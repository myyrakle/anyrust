use anyrust::*;

fn main() {
    let mut f: Function = Function::new(|args| {
        let mut sum = Any::from(0);
        for arg in args {
            sum += arg;
        }
        sum
    });

    let a = Any::from(f.clone());
    let b = a.clone();

    let args = array![1, 2, 3, 4, 5];
    let result = a.call(args.clone());
    print!("Result: {}", result);

    let result = a.call(args);
    print!("Result: {}", result);
}
