use clippy_utils::diagnostics::span_lint;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::TyKind;
use rustc_span::sym;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Checks for calls to `.unwrap()` on `Option` or `Result` values.
    ///
    /// ### Why is this bad?
    ///
    /// Calling `.unwrap()` will panic if the value is `None` or `Err(...)`.
    /// Use explicit error handling (e.g., `?`, `match`, `if let`, or `unwrap_or`) instead.
    ///
    /// ### Known problems
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let x: Option<i32> = Some(1);
    /// let _ = x.unwrap(); // panics if None
    /// ```
    pub NO_UNWRAP,
    Warn,
    "use of `.unwrap()` which may panic"
}

impl<'tcx> LateLintPass<'tcx> for NoUnwrap {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let ExprKind::MethodCall(path, recv, [], _) = expr.kind else {
            return;
        };
        if path.ident.name != sym::unwrap {
            return;
        }
        let ty = cx.typeck_results().expr_ty(recv).peel_refs();
        let TyKind::Adt(adt, _) = ty.kind() else {
            return;
        };
        let did = adt.did();
        if cx.tcx.is_diagnostic_item(sym::Option, did)
            || cx.tcx.is_diagnostic_item(sym::Result, did)
        {
            span_lint(
                cx,
                NO_UNWRAP,
                expr.span,
                "called `.unwrap()` which may panic; use `?`, `unwrap_or`, or explicit error handling instead",
            );
        }
    }
}
