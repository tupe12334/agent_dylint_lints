#![feature(rustc_private)]

extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

mod max_lines_per_file;

dylint_linting::dylint_library!();

#[allow(clippy::no_mangle_with_rust_abi)]
#[unsafe(no_mangle)]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    max_lines_per_file::register_lints(sess, lint_store);
}

#[test]
fn ui() {
    // dylint_testing internally runs `cargo build --verbose` to extract rustc flags.
    // When cargo-llvm-cov sets RUSTC_WRAPPER, the "Running" line shows the wrapper
    // instead of rustc, causing dylint_testing to fail with "Found no rustc invocations".
    // Unsetting RUSTC_WRAPPER here lets dylint_testing find the real rustc invocation.
    unsafe { std::env::remove_var("RUSTC_WRAPPER") };
    dylint_testing::ui_test_examples(env!("CARGO_PKG_NAME"));
}
