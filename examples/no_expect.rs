fn main() {
    let x: Option<i32> = std::hint::black_box(None);
    let _a = x.expect("should have value");

    let y: Result<i32, &str> = std::hint::black_box(Err("error"));
    let _b = y.expect("should be ok");
}
