#![feature(rustc_private)]

extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_lint;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

mod assigning_clones;
mod copy_iterator;
mod lib_tests;
mod logic;
mod logic_tests;
mod max_lines_per_file;
mod max_lines_per_function;
mod max_nesting_depth;
mod no_expect;
mod no_unwrap;
mod tests_in_separate_files;
mod too_many_lines;
mod unused_async;
mod wildcard_imports;

dylint_linting::dylint_library!();

#[allow(clippy::no_mangle_with_rust_abi)]
#[unsafe(no_mangle)]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    assigning_clones::register_lints(sess, lint_store);
    copy_iterator::register_lints(sess, lint_store);
    max_lines_per_file::register_lints(sess, lint_store);
    max_lines_per_function::register_lints(sess, lint_store);
    max_nesting_depth::register_lints(sess, lint_store);
    no_expect::register_lints(sess, lint_store);
    no_unwrap::register_lints(sess, lint_store);
    tests_in_separate_files::register_lints(sess, lint_store);
    too_many_lines::register_lints(sess, lint_store);
    unused_async::register_lints(sess, lint_store);
    wildcard_imports::register_lints(sess, lint_store);
}
