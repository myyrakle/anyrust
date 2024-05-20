use anyrust::*;

fn main() {
    let mut map = Any::from(Map::new());
    map.set("name", "John Doe");
    map.set("age", 30);
    map.set("is_adult", true);

    println!("{:?}", map.to_string());

    for (k, v) in map.to_map() {
        println!("{}: {}", k, v);
    }

    let s = Any::from("Hello, World!");
    println!("{:?}", s.to_string());
    for c in s.to_string().chars() {
        println!("{}", c);
    }
}
