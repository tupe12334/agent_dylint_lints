use clippy_utils::diagnostics::span_lint;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty;
use rustc_span::sym;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Checks for `.expect()` calls on `Option` and `Result` types.
    ///
    /// ### Why is this bad?
    ///
    /// Using `.expect()` panics with a message when the value is `None` or `Err`.
    /// Prefer explicit error handling with `match`, `if let`, `?`, or similar constructs.
    ///
    /// ### Known problems
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// A file with `.expect()` calls on `Option` or `Result` will trigger this lint.
    pub NO_EXPECT,
    Warn,
    "use of `.expect()` on `Option` or `Result`"
}

impl<'tcx> LateLintPass<'tcx> for NoExpect {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::MethodCall(method_path, receiver, _, _) = expr.kind
            && method_path.ident.name.as_str() == "expect"
        {
            let receiver_ty = cx.typeck_results().expr_ty(receiver).peel_refs();
            if let ty::Adt(adt_def, _) = receiver_ty.kind() {
                let def_id = adt_def.did();
                if cx.tcx.is_diagnostic_item(sym::Option, def_id)
                    || cx.tcx.is_diagnostic_item(sym::Result, def_id)
                {
                    span_lint(
                        cx,
                        NO_EXPECT,
                        expr.span,
                        "use of `.expect()` on `Option` or `Result`; prefer explicit error handling",
                    );
                }
            }
        }
    }
}
