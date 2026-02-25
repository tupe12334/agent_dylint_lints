use rustc_span::FileName;

pub const MAX_LINES: usize = 100;

pub fn should_lint(name: &FileName, cnum: u32, line_count: usize) -> bool {
    matches!(name, FileName::Real(_)) && cnum == 0 && line_count > MAX_LINES
}

pub fn lint_message(line_count: usize) -> String {
    format!("file has {line_count} lines, which exceeds the maximum of {MAX_LINES}")
}
