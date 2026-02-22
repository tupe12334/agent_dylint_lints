use clippy_utils::diagnostics::span_lint;
use rustc_hir::{Item, ItemKind, UseKind};
use rustc_lint::{LateContext, LateLintPass};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Checks for wildcard imports (`use some_module::*`).
    ///
    /// ### Why is this bad?
    ///
    /// Wildcard imports pollute the namespace, making it unclear where names
    /// come from. They can also cause unexpected name conflicts when new items
    /// are added to the imported module.
    ///
    /// ### Known problems
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// ```rust
    /// use std::io::*;
    /// ```
    pub WILDCARD_IMPORTS,
    Warn,
    "use of a wildcard import"
}

impl<'tcx> LateLintPass<'tcx> for WildcardImports {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if let ItemKind::Use(use_path, UseKind::Glob) = item.kind {
            // Allow prelude imports (e.g., `use foo::prelude::*` or `use std::prelude::rust_2021::*`)
            if use_path
                .segments
                .iter()
                .any(|seg| seg.ident.name.as_str() == "prelude")
            {
                return;
            }
            span_lint(cx, WILDCARD_IMPORTS, item.span, "usage of wildcard import");
        }
    }
}
