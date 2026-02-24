use rustc_span::FileName;

pub const MAX_LINES: usize = 100;

pub fn should_lint(name: &FileName, cnum: u32, line_count: usize) -> bool {
    matches!(name, FileName::Real(_)) && cnum == 0 && line_count > MAX_LINES
}

pub fn lint_message(line_count: usize) -> String {
    format!("file has {line_count} lines, which exceeds the maximum of {MAX_LINES}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustc_span::{FileName, RealFileName};

    fn local_file() -> FileName {
        FileName::Real(RealFileName::empty())
    }

    #[test]
    fn non_real_file_not_linted() {
        assert!(!should_lint(
            &FileName::Custom(String::from("macro")),
            0,
            MAX_LINES + 1
        ));
    }

    #[test]
    fn external_crate_file_not_linted() {
        assert!(!should_lint(&local_file(), 1, MAX_LINES + 1));
    }

    #[test]
    fn file_at_limit_not_linted() {
        assert!(!should_lint(&local_file(), 0, MAX_LINES));
    }

    #[test]
    fn file_over_limit_is_linted() {
        assert!(should_lint(&local_file(), 0, MAX_LINES + 1));
    }

    #[test]
    fn lint_message_includes_counts() {
        assert_eq!(
            lint_message(101),
            "file has 101 lines, which exceeds the maximum of 100"
        );
    }
}
