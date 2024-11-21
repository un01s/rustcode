/*
 * str is a primitive type in Rust. it is a dynamically-sized type (DST).
 * the size str takes up cannot be known at compile time and depends on 
 * runtime information - it cannot be stored in a variable because the compiler
 * needs to know at the compile time what the size of each variable is.
 *
 * &str or any other pointer to a str like Box<str> does exist at runtime. 
 * This is so-called fat-pointer. it is a pointer with extra information. So it
 * is twice large. 
 *
 *
 */

fn say_hello(to_whom: &str) { //type coercion
  println!("Hey {}!", to_whom) 
}

// using String type means the program must copy the value.
// when using a reference, such as &str, no copy is made.
fn print_me(msg: String) {
    println!("the message is {}", msg);
}

// &str is a reference type, means we are borrowing the variable
// when print_str() is done with the variable, ownership will return to the
// original owner.
// use a reference is more efficeint. 
// A String type can be magically turned into a &str type using the Deref trait
// and type coercion. this will make more sense with the following function.

fn print_str(msg: &str) {
    println!("the message is {}", msg);
}

struct Person<'a> {
  name: &'a str,
}

fn say_hello(name: &str) -> String {
  format!("hello {name}!)
}

fn main(){
  let hello = "hello";
  // compiler error
  //let any_char = hello[0];

  for c in hello.chars() {
    println!("{}",c);
  }

  let any_char = &hello[4..5];
  println!("{:?}", any_char);

  // shadowing
  let s: &str = "hello"; // &str
  let s: String = s.to_uppercase(); // String
  println!("{}", s); // HELLO

  let ss_slice: &'static str = "you";
  let ss: String = ss_slice.into(); // &str => String
  say_hello(ss_slice);
  say_hello(&ss);// &String

  // str vs. String 
  let msg = "helloworld"; // this string literal is of type &str
  // expected `String`, found `&str`
  //print_me(msg);
  print_str(msg);
  let msg1 = "hello".to_string();
  print_me(msg1); // OK
  let msg2 = String::from("hey");
  print_str(&msg2); // OK too

  // struct with &str
  // compiler error: missing lifetime specifier

  // concat with format!
  say_hello(&msg);
}

