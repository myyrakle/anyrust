use anyrust::*;

fn main() {
    let add = function!(lhs, rhs => {
        lhs + rhs
    });

    let result = add.call(array![1, 2]);
    println!("Result: {}", result);

    let four: Any = function!( => {
        let sum = Any::from(4444);
        sum
    });

    let result = four.call(array![]);
    println!("Result: {}", result);
}
