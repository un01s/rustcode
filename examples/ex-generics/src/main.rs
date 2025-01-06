//!
//! https://doc.rust-lang.org/rust-by-example/generics.html
//!
//! besides type, function, implementation, or traits can all be generic
//!

use std::fmt::{Debug, Display};

struct A; // concrete type A
struct Single(A); // concrete type Single
struct SingleGen<T>(T); // generic type SingleGen

// multiple bounds for a single type
fn compare_prints<T: Debug + Display>(t: &T) {
  println!("Debug: `{:?}`", t);
  println!("Display: `{}`", t);
}

fn compare_types<T: Debug, U: Debug>(t: &T, u: &U) {
  println!("t: `{:?}`", t);
  println!("u: `{:?}`", u);
}

fn main() {
  let _s = Single(A);
  let _char: SingleGen<char> = SingleGen('a');

  let _t = SingleGen(A);
  let _i32 = SingleGen(6);
  let _char = SingleGen('a');

  println!("Hello, world!");

  // multiple bounds
  let string = "words";
  let array = [1, 2, 3];
  let vec = vec![1, 2, 3];

  compare_prints(&string);
  compare_types(&array, &vec);
}
