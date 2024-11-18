/*
 * Rust traits
 * a trait tells the compiler about a type must provide
 * a trait is a promise that one type makes
 * 
 */


#![allow(unused_variables)]

// trait: define a type signature with no body
trait HasArea {
  fn area(&self) -> f64;
}

struct Circle {
  x: f64, // center x
  y: f64, // center y
  radius: f64, // radius
}

// method for type Circle
impl Circle {
  fn size(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }
}

// to implement a trait, 
// use impl trait-name for tyep-name
impl HasArea for Circle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }
}

// another type
struct Square {
  x: f64,
  y: f64,
  side: f64,
}

impl HasArea for Square {
  fn area(&self) -> f64 {
    self.side * self.side
  }
}

// <T: HasArea> means "any type that implements the HasArea trait
fn print_area<T: HasArea>(shape: T) {
  println!("This shape has an area of {}", shape.area());
}

fn main() {
  let c = Circle {
    x: 0.0f64,
    y: 0.0f64,
    radius: 1.0f64,
  };

  let s = Square {
    x: 0.0f64,
    y: 0.0f64,
    side: 1.0f64,
  };

  print_area(c);
  print_area(s);
}
