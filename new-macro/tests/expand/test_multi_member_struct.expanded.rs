#[macro_use]
extern crate new_macro;
use std::io::Error;
use new_macro::New;
struct Test {
    pub a: i32,
    pub message: String,
    pub error: Result<HashMap<String, u64>, Error>,
    woah: u32,
}
impl Test {
    pub fn new(
        a: i32,
        message: String,
        error: Result<HashMap<String, u64>, Error>,
        woah: u32,
    ) -> Self {
        Self { a, message, error, woah }
    }
}
