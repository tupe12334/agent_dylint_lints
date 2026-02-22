#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod copy_iterator;
mod max_lines_per_file;
mod max_lines_per_function;
mod no_expect;
mod no_unwrap;
mod too_many_lines;
mod wildcard_imports;

dylint_linting::dylint_library!();

#[allow(clippy::no_mangle_with_rust_abi)]
#[unsafe(no_mangle)]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    copy_iterator::register_lints(sess, lint_store);
    max_lines_per_file::register_lints(sess, lint_store);
    max_lines_per_function::register_lints(sess, lint_store);
    no_expect::register_lints(sess, lint_store);
    no_unwrap::register_lints(sess, lint_store);
    too_many_lines::register_lints(sess, lint_store);
    wildcard_imports::register_lints(sess, lint_store);
}

#[test]
fn ui() {
    dylint_testing::ui_test_examples(env!("CARGO_PKG_NAME"));
}
