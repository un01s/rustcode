/*
 * ownership and borrowing
 * what is the safe code? 
 *
 * variables live in stack
 *
 */

#[warn(unused_variables)]

fn read(y: bool) {
  if y {
    println!("true");
  }
}

fn plus_one(x: i32) -> i32 {
  x + 1
}

fn greet(g1: &String, g2: &String) {
  println!("{} {}!", g1, g2);
}

fn main() {
  let x = true;
  read(x);

  let n = 5;
  let y = plus_one(n);
  println!("y = {y}");

  let m1 = String::from("Hello");
  let m2 = String::from("world");
  greet(&m1, &m2);
  let _s = format!("{} {}", m1, m2);
}
