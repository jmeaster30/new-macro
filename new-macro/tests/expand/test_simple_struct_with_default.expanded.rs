#[macro_use]
extern crate new_macro;
use new_macro::{New, default};
struct Test {
    #[default(100)]
    pub a: i32,
}
impl Test {
    pub fn new() -> Self {
        Self { a: 100 }
    }
}
