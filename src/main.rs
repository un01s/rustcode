fn main() {
  let a = 10; // default i32, immutable
  let b: i32 = 20; // explicit type i32, immutable
  let mut c = 30i32; // value 3o type i32, mutable
  let d = 30_i32;
  let e = add(add(a, b), add(c, d));

  println!("sum of a, b, c and d is 0x{:x}", e);

  // destructing assignment
  let (a, mut b): (bool, bool) = (true, false);
  println!("a = {:?}, b = {:?}", a, b);
  b = true;
  assert_eq!(a, b);

  // _ representing a match, but we do not care about its specific value
  let (x, y);
  [x, .., y, _] = [1, 2, 3, 4, 5];
  println!("x = {}, y = {}", x, y); 

  // slice
  let mut s = String::from("hello world");
  let hello = &s[0..5];
  let world = &s[6..11];
  println!("{} + {}", hello, world);
  
  let word = first_word(&s); // 5
  s.clear(); // empty the string
  println!("length of the first word: {}", word);
}

// do not add ; after i+j
// that makes {} returned instead an i32
// "" for string
// '' for char
fn add(i: i32, j: i32) -> i32 {
  // return is not in use here
  i + j
}

fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}
