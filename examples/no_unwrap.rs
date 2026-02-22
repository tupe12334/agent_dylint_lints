fn main() {
    let opt: Option<i32> = std::hint::black_box(Some(1));
    let _ = opt.unwrap();

    let res: Result<i32, &str> = std::hint::black_box(Ok(1));
    let _ = res.unwrap();

    // These should not trigger
    let _ = opt.unwrap_or(0);
    let _ = res.unwrap_or(0);
}
