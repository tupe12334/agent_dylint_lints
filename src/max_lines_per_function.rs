use clippy_utils::diagnostics::span_lint;
use rustc_hir::{Body, FnDecl, intravisit::FnKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{BytePos, Span, def_id::LocalDefId};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Checks that no function exceeds 60 lines.
    ///
    /// ### Why is this bad?
    ///
    /// Large functions are harder to read, test, and maintain.
    ///
    /// ### Known problems
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// A function with more than 60 lines will trigger this lint.
    pub MAX_LINES_PER_FUNCTION,
    Warn,
    "function exceeds the maximum of 60 lines"
}

impl<'tcx> LateLintPass<'tcx> for MaxLinesPerFunction {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _kind: FnKind<'tcx>,
        _decl: &'tcx FnDecl<'tcx>,
        _body: &'tcx Body<'tcx>,
        span: Span,
        _id: LocalDefId,
    ) {
        const MAX_LINES: usize = 60;

        let line_count = cx
            .sess()
            .source_map()
            .span_to_lines(span)
            .map(|fl| fl.lines.len())
            .unwrap_or(0);
        if line_count > MAX_LINES {
            let report_span = span.with_hi(BytePos(span.lo().0 + 1));
            span_lint(
                cx,
                MAX_LINES_PER_FUNCTION,
                report_span,
                format!(
                    "function has {line_count} lines, which exceeds the maximum of {MAX_LINES}"
                ),
            );
        }
    }
}
