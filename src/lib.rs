//! Macro to generate constructor functions
//!
//! Adds a derive macro `New` that generates a 'new' function for constructing
//! structs. Also, allows you to control which variables are defaulted
//! and which are required through the sub-attribute `default`. The
//! `default` sub-attribute takes an expression as an argument and uses
//! that to calculate the member variable's value on construction.
//!
//! # Panics
//!
//! When the macro panics, you will see a compile error. It might be
//! vague while I make good compile errors and iron out bugs, but I
//! will work to improve error outputs.
//!
//! # Examples
//!
//! Here is a basic sample of the macro in action!
//!
//! *Source Code:*
//! ```
//! use new_macro::New;
//!
//! #[derive(New)]
//! struct Test {
//!     a: i32,
//!     #[default(a * 2)]
//!     b: i32,
//! }
//! ```
//!
//! *Generated Code:*
//! ```
//! struct Test {
//!     a: i32,
//!     b: i32,
//! }
//!
//! impl Test {
//!     pub fn new(a: i32) -> Self {
//!       Self {
//!         a,
//!         b: a * 2,
//!       }
//!     }
//! }
//! ```
//!

mod new;

use proc_macro::TokenStream;
use crate::new::new_macro;

#[proc_macro_derive(New, attributes(default))]
/// This macro generates a `new` function for the annotated struct.
///
/// `default` is a sub-attribute that takes an expression
/// as an argument, which controls the default value of a
/// member and causes that variable to be excluded from the
/// `new` method's parameter.
///
/// # Example
/// Here is a basic sample of the macro in action!
///
/// *Source Code:*
/// ```
/// use new_macro::New;
///
/// #[derive(New)]
/// struct Test {
///     a: i32,
///     #[default(a * 2)]
///     b: i32,
/// }
/// ```
///
/// *Generated Code:*
/// ```
/// struct Test {
///     a: i32,
///     b: i32,
/// }
///
/// impl Test {
///     pub fn new(a: i32) -> Self {
///       Self {
///         a,
///         b: a * 2,
///       }
///     }
/// }
/// ```
///
pub fn new_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    new_macro(&ast).into()
}
