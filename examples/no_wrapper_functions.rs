// Should trigger: single expression delegating to external method chain
pub fn is_metadata_ok(path: &std::path::Path) -> bool {
    std::fs::metadata(path).is_ok()
}

// Should trigger: direct call to external function
pub fn read_file(path: &std::path::Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

// Should NOT trigger: has let binding
#[allow(dead_code)]
pub fn check_with_binding(path: &std::path::Path) -> bool {
    let result = std::fs::metadata(path);
    result.is_ok()
}

// Should NOT trigger: body is a literal, not a call
#[allow(dead_code)]
fn constant() -> i32 {
    42
}

// Should NOT trigger: indirect call (callee is not a simple path)
#[allow(clippy::redundant_closure_call, dead_code)]
fn calls_indirect() -> i32 {
    (|| 42)()
}

fn main() {}
