/*
 * primitive types are built-in types
 * boolean (bool, true, false)
 * char: a single Unicode value (cahr, 'x'), 4-byte instead of one byte
 * numeric types such as i32, f64, u8
 *     signed and unsigned
 *     fixed-size types
 *     variable-size types: isize and usize
 *     floating types
 * arrays
 * str 
 * tuple
 * function
 * 
 * A slice is a reference (or view into) another data structure.
 * 
 */

#![warn(unused_variables)]

fn main() {
  let _x = true;
  let _y: bool = false;

  let _c = 'x';
  let h = '💕';

  println!("{}", h);

  let a = [1, 2, 3, 4, 5];
  let mut m = [2, 3, 4, 5, 6];
  let b = [0; 20]; // type [T; N], b: [i32; 20]
  let all = &a[..]; 
  let mid = &a[1..4];

  let tuple = (1, 2, 3);

  let x = tuple.0;
  let y = tuple.1;
  let z = tuple.2;

  println!("x is {}", x);
}
