use new_macro::New;

#[derive(Debug, New)]
struct Test<'a> {
    a: &'a i32,
}

fn main() {
    let value = 12;
    let t = Test::new(&value);
    println!("Hello {:#?}", t);
}
