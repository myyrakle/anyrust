use anyrust::Any;

fn main() {
    let some = anyrust::array![1, 2, 3, 4, 5];
    println!("Hello, world!: {some}");
    let some = Any::new("5");
    let foo = Any::new("10");
    let result = some + foo;

    println!("Hello, world!: {result}");
}
