use rand::Rng;

fn main() {
  let num = rand::thread_rng().gen_range(0..100);
  println!("{}", num);

  println!("Hello, world!");
}
