use clippy_utils::diagnostics::span_lint;
use clippy_utils::ty::is_copy;
use rustc_hir::{Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::sym;

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Checks for types that implement both `Copy` and `Iterator`.
    ///
    /// ### Why is this bad?
    ///
    /// If a type implements both `Copy` and `Iterator`, iterating over it will
    /// implicitly copy the iterator, which can lead to unexpected behavior.
    /// For example, calling `.next()` on a copied iterator won't advance the
    /// original. Consider implementing `IntoIterator` instead.
    ///
    /// ### Known problems
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// ```rust
    /// #[derive(Copy, Clone)]
    /// struct MyIter;
    ///
    /// impl Iterator for MyIter {
    ///     type Item = i32;
    ///     fn next(&mut self) -> Option<i32> { None }
    /// }
    /// ```
    pub COPY_ITERATOR,
    Warn,
    "implementing `Iterator` on a `Copy` type"
}

impl<'tcx> LateLintPass<'tcx> for CopyIterator {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if let ItemKind::Impl(impl_block) = &item.kind
            && let Some(trait_ref) = &impl_block.of_trait
            && let Some(trait_def_id) = trait_ref.trait_ref.trait_def_id()
            && cx.tcx.is_diagnostic_item(sym::Iterator, trait_def_id)
        {
            let ty = cx.tcx.type_of(item.owner_id).instantiate_identity();
            if is_copy(cx, ty) {
                span_lint(
                    cx,
                    COPY_ITERATOR,
                    item.span,
                    "you are implementing `Iterator` on a `Copy` type; consider implementing `IntoIterator` instead",
                );
            }
        }
    }
}
