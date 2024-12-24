//! 
//! rust ownership, move, and borrow
//!
//! the difference between print! and println! is that the later has a newline
//! 
//! in Rust, we can move or borrow. move is default. to borrow we use &
//! A borrow might be implemented using a reference, but it might not. 
//! by default, all bindings in Rust are immutable.
//!
//! ownership and mutability interact thusly:
//! you can have multiple immutable borrows or one mutable borrow.
//!
//! Borrows, mutable or not, exist for the lifetime of their scope.

#[derive(Debug)] // easy to print out

struct User {
  id: u32,
}

impl Clone for User {
  fn clone(&self) -> Self {
    User{id: self.id}
  }
}

impl Copy for User {

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

  //let u2 = u1; // moved, form here u1 has no data
  // assignment or variable binding moves ownership
  // println!("{:?}", u2);

  //print_user(u2);
  // now, if you try to print u2, Rust won't and it reports compilation error
  //println!("{:?}", u2);

  let u2 = return_ownership(u2);
  println!("{:?}", u2);

  borrow_ownership(&u2);

  // use an integer instead of struct User type
  // the integer is a primitive, it has Copy trait.
  // struct User is a complex and user-created type. 
  // by default, it has no Copy trait.
  let i1 = 9001;
  println!("1st: {}", i1);
  let i2 = i1; // copied instead of moved
  println!("2nd: {}", i2);
  println!("3rd: {}", i1); // this works because the integer has Copy trait

  let u2 = u1; // copied, because of both trait Clone and Copy 
  println!("u2 copied: {:?}", u2);
  println!("u1 remains: {:?}", u1);

  let mut a = 1;
  a += 2;
  println!("{}", a);
}
