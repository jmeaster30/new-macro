#[macro_use]
extern crate new_macro;

use new_macro::{New, default};

#[derive(New)]
struct Test {
  #[default(100)]
  pub a: i32,
}