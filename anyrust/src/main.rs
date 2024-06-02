use anyrust::*;

use std::rc::Rc;

fn main() {
    // let mut rc_f: Rc<dyn Fn(Vec<i32>) -> i32> = Rc::new(|args: Vec<i32>| {
    //     let mut sum = 0;
    //     for arg in args {
    //         sum += arg;
    //     }
    //     sum
    // });
    // let foo = rc_f(vec![1, 2, 3, 4, 5]);
    // println!("Result: {}", foo);

    let mut f: Function = Function::new(|args| {
        let mut sum = Any::from(0);
        for arg in args {
            sum += arg;
        }
        sum
    });

    let a = Any::from(f);
    let b = a.clone();

    let args = array![1, 2, 3, 4, 5];
    let result = f.call(args.clone());
    print!("Result: {}", result);

    let result = f.call(args);
    print!("Result: {}", result);
}
