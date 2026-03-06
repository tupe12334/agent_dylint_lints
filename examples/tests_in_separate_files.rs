fn main() {}

// Covers: #[cfg(test)] attribute not followed by "mod" (false branch of starts_with check)
#[cfg(test)]
fn not_a_mod() {}

// Covers: brace >= semi path in has_inline_cfg_test_module (external module declaration
// embedded in a string so cargo fmt does not need to resolve the module file)
#[allow(dead_code)]
const _COVERAGE: &str = "#[cfg(test)]mod ext_coverage;";

// Should trigger: inline test module body
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}
