#[macro_use]
extern crate new_macro;

use new_macro::{New, default};

#[derive(New)]
struct Test<const SCALE: usize> {
  pub a: usize,
  #[default(a * SCALE)]
  pub scaled: usize,
}