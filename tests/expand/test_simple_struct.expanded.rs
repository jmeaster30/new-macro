#[macro_use]
extern crate new_macro;
use new_macro::New;
struct Test {
    pub a: i32,
}
impl Test {
    pub fn new(a: i32) -> Self {
        Self { a }
    }
}
