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

// Should trigger: async fn with closure but no await
async fn with_closure() -> i32 {
    let _f = || 42;
    42
}

#[allow(clippy::let_underscore_future)]
fn main() {
    let _ = no_await();
    let _ = with_await();
    let _ = sync_fn();
    let _ = Foo.trait_method();
    let _ = with_closure();
}
