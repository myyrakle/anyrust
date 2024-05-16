use anyrust::*;

fn main() {
    let mut map = Any::from(Map::new());
    map.set("name".into(), "John Doe".into());
    map.set("age".into(), 30.into());
    map.set("is_adult".into(), true.into());

    println!("{}", map.to_string());

    for e in map {
        let (k, v) = e.to_pair().to_tuple();

        println!("{}: {}", k, v);
    }
}
