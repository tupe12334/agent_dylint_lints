fn main() {
    let x: Option<i32> = None;
    let _a = x.expect("should have value");

    let y: Result<i32, &str> = Err("error");
    let _b = y.expect("should be ok");
}
