#![cfg(test)]

use crate::logic::{lint_message, should_lint, MAX_LINES};
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
