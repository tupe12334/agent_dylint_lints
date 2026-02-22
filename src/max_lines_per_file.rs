use clippy_utils::diagnostics::span_lint;
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{FileName, Span};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Checks that no source file exceeds 100 lines.
    ///
    /// ### Why is this bad?
    ///
    /// Large files are harder to read, review, and maintain.
    ///
    /// ### Known problems
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// A `.rs` file with more than 100 lines will trigger this lint.
    pub MAX_LINES_PER_FILE,
    Warn,
    "file exceeds the maximum of 100 lines"
}

impl<'tcx> LateLintPass<'tcx> for MaxLinesPerFile {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        const MAX_LINES: usize = 100;

        for file in cx.sess().source_map().files().iter() {
            if !matches!(file.name, FileName::Real(_)) {
                continue;
            }
            // cnum == 0 identifies files belonging to the local crate
            if file.cnum.as_u32() != 0 {
                continue;
            }
            let line_count = file.count_lines();
            if line_count > MAX_LINES {
                let span = Span::with_root_ctxt(file.start_pos, file.start_pos);
                span_lint(
                    cx,
                    MAX_LINES_PER_FILE,
                    span,
                    format!(
                        "file has {line_count} lines, which exceeds the maximum of {MAX_LINES}"
                    ),
                );
            }
        }
    }
}
