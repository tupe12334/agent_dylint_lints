use crate::logic;
use clippy_utils::diagnostics::span_lint;
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{BytePos, Span};

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
        for file in cx.sess().source_map().files().iter() {
            let line_count = file.count_lines();
            if logic::should_lint(&file.name, file.cnum.as_u32(), line_count) {
                let span = Span::with_root_ctxt(file.start_pos, BytePos(file.start_pos.0 + 1));
                span_lint(
                    cx,
                    MAX_LINES_PER_FILE,
                    span,
                    logic::lint_message(line_count),
                );
            }
        }
    }
}
