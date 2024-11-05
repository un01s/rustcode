// ownership: stack and heap, reference and borrowing
fn main() {
  // integer is a primitive type, both x amd y are on the stack
  // copy: the size of the integer is known
  // x is copied to y on the stack instead of the heap
  let x = 5;
  let y = x;
  println!("{}, {}", x, y);

  // wrong  
  //let s1 = String::from("hello");
  //let s2 = s1; // ownership is moved from s1 to s2 in this case
  //println!("{}, world!", s1); // error here! s1 is dropped!
  // corrected
  let s1 = String::from("hello");
  let s2 = s1.clone(); // ownership is moved from s1 to s2 in this case
  println!("{}, world!", s1); // error here! s1 is dropped!
  
  // String is allocated on the heap
  let mut s = String::from("hello");
  s.push_str(", world!"); // push_str() 
  println!("{}", s);

  // reference with slice: it is borrowing for both a and b
  let a: &str = "hello world";
  let b = a;
  println!("{}, {}", a, b);
}
