# anyrust

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-0.1.0%20alpha-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/anyrust/blob/master/LICENSE)

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

## Array, Map

Arrays are supported through the `anyrust::Array` type. This is compatible with `Vec<Any>`.

KV Map is supported through the `anyrust::Map` type. This is compatible with `HashMap<Any,Any>`.