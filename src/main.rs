use anyrust::*;

fn main() {
    let a = Any::new(5);
    let b = Any::new("10");
    let result = a + b;

    println!("result: {result}");
}
