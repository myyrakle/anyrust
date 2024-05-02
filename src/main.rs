use anyrust::Any;

fn main() {
    let some = Any::new(5);
    let foo = Any::new(10);
    let result = some + foo;

    println!("Hello, world!: {result}");
}
