/*
 * higher order function example
 *
 */

fn plus_one(n: i32) -> i32 {
  n + 1
}

fn add(n: i32, m: i32) -> i32 {
  n + m
}

fn binary_operator<F>(n: i32, m: i32, op: F) -> i32 
                     where F: Fn(i32, i32) -> i32 {
  op(n, m)
}

fn main() {
  let add_one = plus_one; // store a function in a variable
  println!("{}", add_one(1));
  // named function: 11
  println!("{}", binary_operator(5, 6, add));
  // anonymous: -1
  println!("{}", binary_operator(5, 6, |a: i32, b: i32| a - b));
}
