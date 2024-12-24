//! 
//! rust ownership, move, and borrow
//!
//! the difference between print! and println! is that the later has a newline
//! 
//! in Rust, we can move or borrow. move is default. to borrow we use &
//! A borrow might be implemented using a reference, but it might not. 
//!

#[derive(Debug)] // easy to print out

struct User {
  id: u32,
}

// this function takes the ownership away of u
fn print_user(u: User) {
  println!("take-away: {:?}", u);
}

// this function returns the ownership
fn return_ownership(u: User) -> User {
  println!("returned: {:?}", u);
  u
}

// this function borrows the ownership
fn borrow_ownership(u: &User) {
  println!("borrowed: {:?}", u);
}

fn main() {
  let u1 = User{id: 1000};
  println!("{:?}", u1);

  let u2 = u1; // moved, form here u1 has no data
  // assignment or variable binding moves ownership
  println!("{:?}", u2);

  //print_user(u2);
  // now, if you try to print u2, Rust won't and it reports compilation error
  //println!("{:?}", u2);

  let u2 = return_ownership(u2);
  println!("{:?}", u2);

  borrow_ownership(&u2);
}
