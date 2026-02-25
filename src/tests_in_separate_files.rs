use clippy_utils::diagnostics::span_lint;
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{BytePos, FileName, Span};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Checks that test code is placed in dedicated `<X>_tests.rs` files
    /// rather than inline `#[cfg(test)]` modules within source files.
    ///
    /// ### Why is this bad?
    ///
    /// Mixing production logic and test modules in the same file reduces
    /// readability and separation of concerns. Keeping tests in their own
    /// files makes it easier to navigate the codebase and review production
    /// code in isolation.
    ///
    /// ### Known problems
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// ```rust
    /// // Bad: lib.rs contains both logic and tests
    /// pub fn add(a: i32, b: i32) -> i32 {
    ///     a + b
    /// }
    ///
    /// #[cfg(test)]
    /// mod tests {
    ///     use super::*;
    ///     #[test]
    ///     fn test_add() {
    ///         assert_eq!(add(1, 2), 3);
    ///     }
    /// }
    /// ```
    ///
    /// Move tests to `lib_tests.rs` and declare the module in `lib.rs`:
    ///
    /// ```rust
    /// // lib.rs
    /// pub fn add(a: i32, b: i32) -> i32 {
    ///     a + b
    /// }
    ///
    /// #[cfg(test)]
    /// mod lib_tests;
    /// ```
    pub TESTS_IN_SEPARATE_FILES,
    Warn,
    "test modules should be in separate `<X>_tests.rs` files"
}

/// Returns true if the source text contains a `#[cfg(test)]` attribute
/// directly preceding an inline module body (i.e., `mod name { ... }`).
/// Module declarations like `#[cfg(test)] mod name;` are not flagged.
fn has_inline_cfg_test_module(src: &str) -> bool {
    const MARKER: &str = "#[cfg(test)]";
    let mut pos = 0;
    while let Some(found) = src[pos..].find(MARKER) {
        let start = pos + found + MARKER.len();
        let rest = src[start..].trim_start();
        if rest.starts_with("mod ") || rest.starts_with("mod\t") {
            // Find the first `{` or `;` to distinguish inline from external
            let brace = rest.find('{').unwrap_or(usize::MAX);
            let semi = rest.find(';').unwrap_or(usize::MAX);
            if brace < semi {
                return true;
            }
        }
        pos = start;
    }
    false
}

impl<'tcx> LateLintPass<'tcx> for TestsInSeparateFiles {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        for file in cx.sess().source_map().files().iter() {
            // Only local crate files
            if file.cnum.as_u32() != 0 {
                continue;
            }
            // Only real files
            if !matches!(file.name, FileName::Real(_)) {
                continue;
            }
            // Allow files that are themselves dedicated test files
            let path_str = file.name.prefer_local_unconditionally().to_string_lossy();
            if path_str.ends_with("_tests.rs") {
                continue;
            }
            // Check source text for inline #[cfg(test)] module bodies
            // (module declarations like `#[cfg(test)] mod foo;` are allowed)
            let has_inline_test_mod = file
                .src
                .as_ref()
                .map(|src| has_inline_cfg_test_module(src.as_str()))
                .unwrap_or(false);
            if has_inline_test_mod {
                let span = Span::with_root_ctxt(file.start_pos, BytePos(file.start_pos.0 + 1));
                span_lint(
                    cx,
                    TESTS_IN_SEPARATE_FILES,
                    span,
                    "file contains an inline `#[cfg(test)]` module; move tests to a separate `<name>_tests.rs` file",
                );
            }
        }
    }
}
