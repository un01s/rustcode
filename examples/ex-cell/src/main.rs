use std::cell::Cell;

fn main() {
  let cell = Cell::new(5);

  assert_eq!(cell.get(), 5);
  assert_eq!(cell.replace(10), 5);
  assert_eq!(cell.get(), 10);

  // unwraps the value, consuming the cell
  let v = cell.into_inner();
  assert_eq!(v, 10);

  println!("Hello, world!");
}
