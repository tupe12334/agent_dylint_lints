use clippy_utils::diagnostics::span_lint_and_help;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Checks for assignments where a cloned or `to_owned` value is assigned to an
    /// existing binding, when `clone_from` or `clone_into` could be used instead.
    ///
    /// ### Why is this bad?
    ///
    /// `Clone::clone_from` and `ToOwned::clone_into` can reuse existing allocations.
    /// Assigning `a = b.clone()` always allocates a new value and drops the old one,
    /// missing the optimization opportunity.
    ///
    /// ### Known problems
    ///
    /// This lint does not check whether the type actually has a specialized
    /// `clone_from`/`clone_into` implementation, so it may fire for types where
    /// the default implementation would not be more efficient.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let mut s = String::from("hello");
    /// let other = String::from("world");
    /// s = other.clone();
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// let mut s = String::from("hello");
    /// let other = String::from("world");
    /// s.clone_from(&other);
    /// ```
    pub ASSIGNING_CLONES,
    Warn,
    "assigning a cloned value when `clone_from` or `clone_into` could be used"
}

impl<'tcx> LateLintPass<'tcx> for AssigningClones {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let ExprKind::Assign(_, rhs, _) = expr.kind
            && let ExprKind::MethodCall(method_path, _, _, _) = rhs.kind
        {
            let method_name = method_path.ident.name.as_str();
            if method_name == "clone" {
                span_lint_and_help(
                    cx,
                    ASSIGNING_CLONES,
                    expr.span,
                    "assigning a cloned value, consider using `clone_from`",
                    None,
                    "use `lhs.clone_from(&rhs)` to potentially reuse existing allocations",
                );
            } else if method_name == "to_owned" {
                span_lint_and_help(
                    cx,
                    ASSIGNING_CLONES,
                    expr.span,
                    "assigning a `to_owned` value, consider using `clone_into`",
                    None,
                    "use `rhs.clone_into(&mut lhs)` to potentially reuse existing allocations",
                );
            }
        }
    }
}
