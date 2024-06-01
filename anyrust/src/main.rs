use anyrust::*;

fn main() {
    let mut f: Function = Function::new(|args| {
        let mut sum = Any::from(0);
        for arg in args {
            sum += arg;
        }
        sum
    });

    let args = array![1, 2, 3, 4, 5];
    let result = f.call(args.clone());
    print!("Result: {}", result);

    let result = f.call(args);
    print!("Result: {}", result);
}
