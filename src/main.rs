use anyrust::*;

fn main() {
    let mut arr = array![1, 2, 3, 4, 5];
    arr.push(4444);
    arr.push("foo");

    for e in arr {
        println!("{e}");
    }
}
