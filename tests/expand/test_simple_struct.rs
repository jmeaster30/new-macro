#[macro_use]
extern crate new_macro;

use new_macro::New;

#[derive(New)]
struct Test {
  pub a: i32,
}