use anyrust::*;

fn main() {
    let add = function!(lhs, rhs => {
        lhs + rhs
    });

    let result = add.call(params![1, 2]);
    println!("Result: {}", result);

    let four: Any = function!( => {
        let sum = Any::from(4444);
        sum
    });

    let result = four.call(params![]);
    println!("Result: {}", result);
}
