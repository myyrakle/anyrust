# anyrust

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-0.2.3-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/anyrust/blob/master/LICENSE)

A library that provides a type system as flexible and powerful as Javascript.

## Usage 

Usage is simple and intuitive.
All you have to do is box the value into the anyrust::Any type and use it appropriately.

Below is a simple example.
```rust
use anyrust::*;

fn main() {
    let a = Any::new(5);
    let b = Any::new("10");
    let result = a + b;

    println!("result: {result}"); // result: 510
}
```

## Primitives

The basic integer type, basic float type, boolean type, and string type support mutual conversion with Any without any problem.

## Array

Arrays are supported through the `anyrust::Array` type. This is compatible with `Vec<Any>`.
```rust
    let mut arr = array![1, 2, 3, 4, 5];
    arr.push(4444);
    arr.push("foo");

    for e in arr {
        println!("{e}");
    }
```

## Map

KV Map is supported through the `anyrust::Map` type. This is compatible with `HashMap<Any,Any>`.
```rust
    let mut map = Any::from(Map::new());
    map.set("name", "John Doe");
    map.set("age", 30);
    map.set("is_adult", true);

    println!("{}", map.to_string());

    for (k, v) in map.to_map() {
        println!("{}: {}", k, v);
    }
```