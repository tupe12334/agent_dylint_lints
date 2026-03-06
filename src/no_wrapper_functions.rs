use clippy_utils::diagnostics::span_lint;
use rustc_hir::{Body, Expr, ExprKind, FnDecl, intravisit::FnKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use rustc_span::def_id::LocalDefId;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Checks for functions whose entire body is a single call to an external
    /// (library) function or method, with no additional logic.
    ///
    /// ### Why is this bad?
    ///
    /// Thin wrapper functions that merely delegate to a library function add
    /// unnecessary indirection. Callers can invoke the library function directly.
    ///
    /// ### Known problems
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// ```rust
    /// pub fn is_accessible(path: &std::path::Path) -> bool {
    ///     std::fs::metadata(path).is_ok()
    /// }
    /// ```
    pub NO_WRAPPER_FUNCTIONS,
    Deny,
    "wrapper function that simply delegates to a single library function"
}

impl<'tcx> LateLintPass<'tcx> for NoWrapperFunctions {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        fn_kind: FnKind<'tcx>,
        _fn_decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        _span: Span,
        _def_id: LocalDefId,
    ) {
        let name_span = match fn_kind {
            FnKind::ItemFn(ident, ..) | FnKind::Method(ident, ..) => ident.span,
            FnKind::Closure => return,
        };

        // Body must be a block with no statements and a single expression
        let ExprKind::Block(block, _) = body.value.kind else {
            return;
        };
        if !block.stmts.is_empty() {
            return;
        }
        let Some(expr) = block.expr else {
            return;
        };

        if calls_external_fn(cx, expr) {
            span_lint(
                cx,
                NO_WRAPPER_FUNCTIONS,
                name_span,
                "this function is a thin wrapper around a library function; call it directly instead",
            );
        }
    }
}

fn calls_external_fn<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> bool {
    match expr.kind {
        ExprKind::MethodCall(..) => cx
            .typeck_results()
            .type_dependent_def_id(expr.hir_id)
            .is_some_and(|did| !did.is_local()),
        ExprKind::Call(func, _) => {
            if let ExprKind::Path(qpath) = func.kind {
                cx.qpath_res(&qpath, func.hir_id)
                    .opt_def_id()
                    .is_some_and(|did| !did.is_local())
            } else {
                false
            }
        }
        _ => false,
    }
}
