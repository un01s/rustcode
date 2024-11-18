/*
 * from and into traits on generic types
 * from: convert type A from type B
 * into: convert a type into another
 *
 */

use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);

    let int = 5;
    let n: Number = int.into();
    println!("My number is {:?}", n);
}
