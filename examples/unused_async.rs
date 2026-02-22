// Should trigger: async fn with no await
async fn no_await() -> i32 {
    42
}

// Should NOT trigger: async fn that uses await
async fn with_await() -> i32 {
    std::future::ready(42).await
}

// Should NOT trigger: not async
fn sync_fn() -> i32 {
    42
}

// Should NOT trigger: trait impl method (must match trait signature)
trait AsyncTrait {
    async fn trait_method(&self) -> i32;
}

struct Foo;

impl AsyncTrait for Foo {
    async fn trait_method(&self) -> i32 {
        42
    }
}

fn main() {
    let _ = no_await();
    let _ = with_await();
    let _ = sync_fn();
    let _ = Foo.trait_method();
}
