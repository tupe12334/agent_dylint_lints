use clippy_utils::diagnostics::span_lint;
use clippy_utils::is_def_id_trait_method;
use rustc_hir::intravisit::{FnKind, Visitor, walk_body, walk_expr};
use rustc_hir::{
    Body, ClosureKind, CoroutineKind, CoroutineSource, Expr, ExprKind, FnDecl, IsAsync, YieldSource,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;
use rustc_span::def_id::LocalDefId;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Checks for `async` functions that don't contain any `await` expressions.
    ///
    /// ### Why is this bad?
    ///
    /// The `async` keyword makes a function return a `Future`. If no `await` is used,
    /// the `async` keyword is unnecessary and adds overhead by wrapping the return
    /// value in a future that resolves immediately.
    ///
    /// ### Known problems
    ///
    /// Does not trigger on trait method implementations, as they need to match the
    /// trait signature.
    ///
    /// ### Example
    ///
    /// ```rust
    /// async fn foo() -> i32 {
    ///     42
    /// }
    /// ```
    /// Use instead:
    /// ```rust
    /// fn foo() -> i32 {
    ///     42
    /// }
    /// ```
    pub UNUSED_ASYNC,
    Warn,
    "async functions with no await expressions"
}

impl<'tcx> LateLintPass<'tcx> for UnusedAsync {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        fn_kind: FnKind<'tcx>,
        _fn_decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        _span: Span,
        def_id: LocalDefId,
    ) {
        let header = match fn_kind {
            FnKind::ItemFn(_, _, header) => header,
            FnKind::Method(_, sig) => sig.header,
            FnKind::Closure => return,
        };

        let IsAsync::Async(async_span) = header.asyncness else {
            return;
        };

        // Skip trait method implementations, as they need to match the trait signature.
        if is_def_id_trait_method(cx, def_id) {
            return;
        }

        let mut visitor = AwaitVisitor {
            tcx: cx.tcx,
            found_await: false,
        };
        walk_body(&mut visitor, body);

        if !visitor.found_await {
            span_lint(
                cx,
                UNUSED_ASYNC,
                async_span,
                "this function is declared `async` but does not use `await`",
            );
        }
    }
}

struct AwaitVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    found_await: bool,
}

impl<'tcx> Visitor<'tcx> for AwaitVisitor<'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if self.found_await {
            return;
        }
        match expr.kind {
            ExprKind::Yield(_, YieldSource::Await { .. }) => {
                self.found_await = true;
            }
            ExprKind::Closure(closure) => {
                // The body of an async fn is wrapped in a coroutine (CoroutineSource::Fn).
                // We must recurse into it to find await expressions.
                // We do NOT recurse into user-written async blocks (CoroutineSource::Block)
                // or closures, to avoid false negatives from nested async code.
                if matches!(
                    closure.kind,
                    ClosureKind::Coroutine(CoroutineKind::Desugared(_, CoroutineSource::Fn))
                ) {
                    let inner_body = self.tcx.hir_body(closure.body);
                    walk_body(self, inner_body);
                }
            }
            _ => walk_expr(self, expr),
        }
    }
}
