# new-macro
This is a macro to automate creating a 'new' function for structs

![Crates.io Version](https://img.shields.io/crates/v/lilys-new-macro) [![Run tests uwu](https://github.com/jmeaster30/new-macro/actions/workflows/run_tests.yml/badge.svg)](https://github.com/jmeaster30/new-macro/actions/workflows/run_tests.yml)

## Examples
Here is a basic sample of the macro in action!

### Source Code:
```
use new_macro::New;
#[derive(New)]
struct Test {
    a: i32,
    #[default(a * 2)]
    b: i32,
}
```

### Generated Code:

```
struct Test {
    a: i32,
    b: i32,
}

impl Test {
    pub fn new(a: i32) -> Self {
      Self {
        a,
        b: a * 2,
      }
    }
}
```
