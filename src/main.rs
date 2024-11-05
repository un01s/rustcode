fn main() {
  let a = 10; // default i32, immutable
  let b: i32 = 20; // explicit type i32, immutable
  let mut c = 30i32; // value 3o type i32, mutable
  let d = 30_i32;
  let e = add(add(a, b), add(c, d));

  println!("sum of a, b, c and d is 0x{:x}", e);
}

// do not add ; after i+j
// that makes {} returned instead an i32
// "" for string
// '' for char
fn add(i: i32, j: i32) -> i32 {
  // return is not in use here
  i + j
}

