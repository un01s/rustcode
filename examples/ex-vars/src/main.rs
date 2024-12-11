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
  vs
  .into_iter()
  .map(|(i, mut x)| {
    for _ in 0..i { x = f(x); }
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
