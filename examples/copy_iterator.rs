// Should trigger: Copy type implementing Iterator
#[derive(Copy, Clone)]
struct CopyIter;

impl Iterator for CopyIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        None
    }
}

// Should NOT trigger: non-Copy type implementing Iterator
struct NonCopyIter;

impl Iterator for NonCopyIter {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        None
    }
}

fn main() {}
