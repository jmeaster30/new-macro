#[macro_use]
extern crate new_macro;
use new_macro::{New, default};
struct Test<'a> {
    pub a: &'a i32,
}
impl<'a> Test<'a> {
    pub fn new(a: &'a i32) -> Self {
        Self { a }
    }
}
