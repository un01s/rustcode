/*
 * rust book
 * ch5.2 struct
 *
 */

// add a trait for debugging info
#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

// method
impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
  fn square(size: u32) -> Self {
    Self {
      width: size,
      height: size,
    }
  }
}

// function
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    // use struct to make it better
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect is {rect:?}");
    println!("rect is {rect:#?}");

    // use the function
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );

    // use the method
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));

    // to call this associated function, we use :: syntax with the struct name
    let square = Rectangle::square(10);
    println!("Square area {}", square.area());
}

