use anyrust::*;

fn main() {
    let p = pair!(1, 2);
    println!("Pair: {}", p);

    let add = function!(lhs, rhs => {
        println!("lhs: {}, rhs: {}", lhs, rhs);
        lhs + rhs
    });

    let negative = function!(num => {
        num * any(-1)
    });

    let composite = add >> negative;

    let result = composite.call(params![1, 2]);

    println!("Result: {}", result);

    let m = map!{
        "name" => "John",
        "age" => 20,
    };
    
    println!("Map: {}", m);
}
