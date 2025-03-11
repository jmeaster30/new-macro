#[macro_use]
extern crate new_macro;
use new_macro::New;
struct Test;
impl Test {
    pub fn new() -> Self {
        Self {}
    }
}
