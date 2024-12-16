fn main() {
  let mut v = vec![1, 2, 3, 4, 9];
  let part = &v[1..3];

  for i in &v {
    println!("vec value={}", i);
  }

  println!("slice of v: {:?}", part);

  let index: usize = 0;
  println!("first of part = {}", part[index]);

  match v.get(7) {
    Some(x) => println!("Item 7 is {}", x),
    None => println!("Sorry, this vector is too short.")
  }
}
