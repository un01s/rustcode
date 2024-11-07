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
}

