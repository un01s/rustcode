/*
 * variable binding: assign some value to a name
 *
 * Rust is a statically typed language = type is checked at the compile time
 * integers such as i32 are signed
 * integers such as u16 are unsigned
 * 
 * by default bindings are immutable
 *
 * scope and shadowing
 * scopes: block, function, global
 * 
 * shadowing: a later variable binding with the same name as another binding 
 * that is currently in scope will override the previous binding
 *  
 */

#![warn(unused_assignments)]

fn main() {
  let n = 5; // type inference, implicit
  let (x, y) = (1, 2); // patterns
  let i: i32 = 16; // type
  let mut m = 5;

  println!("Hello, {n}");
  println!("Hello, {x}, {y}!");
  println!("immutable, {}", i);
  println!("mutable, {m}");
  m = 10;
  println!("mutable after change, {m}");
}
