struct Wrapper(i32);
impl Wrapper {
    fn expect(self, _msg: &str) -> i32 {
        self.0
    }
}

fn main() {
    let x: Option<i32> = std::hint::black_box(None);
    let _a = x.expect("should have value");

    let y: Result<i32, &str> = std::hint::black_box(Err("error"));
    let _b = y.expect("should be ok");

    // Should NOT trigger: custom type with expect method
    let _c = Wrapper(42).expect("msg");
}
