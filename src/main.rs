use anyrust::*;

fn main() {
    let mut map = Any::new(Pair::new("key".into(), "value".into()));
    println!("{}", map.to_string());

    for pair in map {
        let pair = pair.to_pair()

        println!("{}: {}", k, v);
    }
}
