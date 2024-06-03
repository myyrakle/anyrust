use anyrust::*;

fn main() {
    let f1 = function!(arg1, arg2 => {
        let mut sum = Any::from(0);
        sum += arg1;
        sum += arg2;
        sum
    });

    let f2 = function!(arg1, arg2, arg3 => {
        let mut sum = Any::from(0);
        sum += arg1;
        sum += arg2;
        sum += arg3;
        sum
    });

    let f3 = function!( => {
        let sum = Any::from(0);
        sum
    });

    let result = f1.call(array![1, 2]);
    println!("Result: {}", result);

    let result = f2.call(array![1, 2, 3]);
    println!("Result: {}", result);

    let result = f3.call(array![]);
    print!("Result: {}", result);

    let f: Function = Function::new(
        move |args| {
            let mut sum = Any::from(0);
            for arg in args {
                sum += arg;
            }
            sum
        },
        1,
    );

    let a = Any::from(f.clone());
    let b = a.clone();

    let args = array![1, 2, 3, 4, 5];
    let result = a.call(args.clone());
    print!("Result: {}", result);

    let result = a.call(args);
    print!("Result: {}", result);

    let result = b.call(array![1, 2, 3, 4, 4]);
    print!("Result: {}", result);
}
