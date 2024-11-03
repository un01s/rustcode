static BYTES: &[u8] = b"hello world!";

fn read_one_byte(index: usize) -> u8 {
  BYTES[index]
}

// like returning one byte as *(BYTES+index) in C
unsafe fn get_one(index: usize) -> u8 {
  *BYTES.as_ptr().add(index)
}

fn main() {
  let string = "onestring";
  for c in string.chars() {
    print!("{}", c);
  }
  println!();

  println!("Hello, world!");
  println!("{}", read_one_byte(1));
  //println!("{}", read_one_byte(100));

  println!("{}", get_one(1));
}
