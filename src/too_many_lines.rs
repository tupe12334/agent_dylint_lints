use clippy_utils::diagnostics::span_lint;
use rustc_hir::{Body, FnDecl, intravisit::FnKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{BytePos, Span, def_id::LocalDefId};

dylint_linting::declare_late_lint! {
    /// ### What it does
    ///
    /// Checks that no function body exceeds 100 non-empty lines.
    ///
    /// ### Why is this bad?
    ///
    /// Functions with too many lines are hard to read, understand, and maintain.
    ///
    /// ### Known problems
    ///
    /// None.
    ///
    /// ### Example
    ///
    /// A function with more than 100 non-empty lines will trigger this lint.
    pub TOO_MANY_LINES,
    Warn,
    "function body has too many lines"
}

impl<'tcx> LateLintPass<'tcx> for TooManyLines {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        _decl: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        _id: LocalDefId,
    ) {
        if matches!(kind, FnKind::Closure) {
            return;
        }

        const MAX_LINES: usize = 100;

        let line_count = cx
            .sess()
            .source_map()
            .span_to_snippet(body.value.span)
            .map(|snippet| {
                snippet
                    .lines()
                    .filter(|line| !line.trim().is_empty())
                    .count()
            })
            .unwrap_or(0);
        if line_count > MAX_LINES {
            let report_span = span.with_hi(BytePos(span.lo().0 + 1));
            span_lint(
                cx,
                TOO_MANY_LINES,
                report_span,
                format!(
                    "function body has {line_count} non-empty lines, which exceeds the maximum of {MAX_LINES}"
                ),
            );
        }
    }
}
