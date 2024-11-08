struct Point {
  x: i32, 
  y: i32,
}

impl Point {
  fn x(&self) -> i32 { self.x }
  fn y(&self) -> i32 { self.y }
}

fn main() {
  let p = Point{x:1, y:2,};
  
  println!("(x = {},y = {})", p.x(), p.y());
}
