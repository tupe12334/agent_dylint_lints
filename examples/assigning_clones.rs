#[allow(unused_variables, unused_assignments)]
fn main() {
    let mut s = String::from("hello");
    let other = String::from("world");
    s = other.clone(); // should warn: use clone_from

    let mut t = String::from("foo");
    let slice: &str = "bar";
    t = slice.to_owned(); // should warn: use clone_into

    // Should NOT warn: not an assignment expression
    let _copy = other.clone();
    let _owned = slice.to_owned();
}
