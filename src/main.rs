static BYTES: &[u8] = b"hello world!";

fn read_one_byte(index: usize) -> u8 {
  BYTES[index]
}

fn main() {
  println!("Hello, world!");
  println!("{}", read_one_byte(0));
  println!("{}", read_one_byte(100));
}
