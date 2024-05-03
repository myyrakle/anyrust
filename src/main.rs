use anyrust::{Any, Array};

fn main() {
    let some = Any::new(Array::from(vec![Any::new(5), Any::new(10)]));
    println!("Hello, world!: {some}");
    let some = Any::new("5");
    let foo = Any::new("10");
    let result = some + foo;

    println!("Hello, world!: {result}");
}
