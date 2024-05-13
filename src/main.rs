use anyrust::*;

fn main() {
    let mut map = Any::from(_null);
    println!("{}", map.to_string());

    map.set("key".into(), 123.into());
    println!("{}", map.to_string());

    map.set("key".into(), "value".into());
    println!("{}", map.to_string());
}
