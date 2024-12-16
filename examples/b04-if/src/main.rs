///
/// document comments supporting mark down
///

fn main() {
  let x = 5;

  if x == 5 {
    println!("x is five!");
  } else if x == 6 {
    println!("x is six!");
  } else {
    println!("x is not five or six :(");
  }

  let y = if x == 5 {10} else {20};
  println!("y = {}", y);

  match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
  }
}
