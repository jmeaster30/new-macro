#[macro_use]
extern crate new_macro;
use new_macro::{New, default};
struct Test<const SCALE: usize> {
    pub a: usize,
    #[default(a*SCALE)]
    pub scaled: usize,
}
impl<const SCALE: usize> Test<SCALE> {
    pub fn new(a: usize) -> Self {
        Self { a, scaled: a * SCALE }
    }
}
