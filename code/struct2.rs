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

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // use tuple
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rect1)
    );

    // use struct to make it better
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect2 is {rect2:?}");
    println!("rect2 is {rect2:#?}");

    // now just borrow rect2 to calculate the area
    // the main function still has the ownership of rect2
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect2)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// use tuple
fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

