fn main() {
  let a = "Hello";
  let b = "world";
  let result = format!("{} {}\n", a, b);
  print!("{}", result);
  let result2 = [a, b].join("\n");
  print!("{}\n", result2);

  println!("{}", format!("{} {}\n", a, b));
}
