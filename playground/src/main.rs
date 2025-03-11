
use new_macro::New;

#[derive(Debug, New)]
struct Test {
    a: i32,
}

fn main() {
    let t = Test::new(12);
    println!("Hello {:#?}", t);
}
