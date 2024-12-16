/*
 * function
 *
 * expression vs. statement
 * expressions return a value, and statements do not.
 *
 * function pointer
 *  
 */

fn print_number(x: i32) {
  println!("the number is: {}", x);
}

fn print_sum(x: i32, y: i32) {
  println!("sum of {x} and {y} is: {}", x + y);
}

// last expression as the return of the function
fn add_one(x: i32) -> i32 {
  x + 1
}

fn main() {
  print_number(32);
  print_sum(2, 3);
  println!("add_one: {}", add_one(1));

  let f: fn(i32) -> i32 = add_one; // explicit type
  let six = f(5);
  print_number(six);
}
