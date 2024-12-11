/*
 * there are three types of variables in Rust
 * - values
 * - shared references
 * - mutable refereces
 *
 * in turn, it maps to three different iteration functions
 * - .into_iter() returns an iterator over values
 * - .iter() returns an iterator over shared references
 * - .iter_mut() returns an iterator over mutable references
 *
 *   
 */

fn some_function<K>(f: impl Fn(K) -> K, vs: Vec<K>) -> Vec<K> {
  let mut index = 0;

  vs
  .into_iter()
  .map(|mut x| {
    index += 1;
    for _ in 1..index { x = f(x); }
    x
  }).collect()
}

fn main() {
  let v = vec![1, 2, 3];
  for i in &v {
    println!("{i}");
  }
  println!("Hello, world!");
}
