use heapless::Vec; // fixed capacity `std::Vec`

fn main() {
  println!("Hello, world!");

  // on the stack
  let mut xs: Vec<u8, 8> = Vec::new(); // can hold up to 8 elements
  xs.push(42).unwrap();
  assert_eq!(xs.pop(), Some(42));

  // in a `static` variable
  static mut XS: Vec<u8, 8> = Vec::new();

  let xs = unsafe { &mut XS };

  xs.push(42);
  assert_eq!(xs.pop(), Some(42));

  // in the heap (though kind of pointless because no reallocation)
  let mut ys: Box<Vec<u8, 8>> = Box::new(Vec::new());
  ys.push(42).unwrap();
  assert_eq!(ys.pop(), Some(42));
}
