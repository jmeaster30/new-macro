#[macro_use]
extern crate new_macro;

use std::io::Error;
use new_macro::New;

#[derive(New)]
struct Test {
  pub a: i32,
  pub message: String,
  #[default(Ok(HashMap::new()))]
  pub error: Result<HashMap<String, u64>, Error>,
  woah: u32,
}