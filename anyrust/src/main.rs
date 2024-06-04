use anyrust::*;

fn main() {
    let add = function!(lhs, rhs => {
        println!("lhs: {}, rhs: {}", lhs, rhs);
        lhs + rhs
    });

    let negative = function!(num => {
        num * Any::from(-1)
    });

    let composite = add >> negative;

    let result = composite.call(params![1, 2]);

    println!("Result: {}", result);
}
