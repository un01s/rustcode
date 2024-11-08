/*
 * deal with huge number
 * https://github.com/rust-num/num
 *
 * https://friendlyuser.github.io/posts/tech/rust/getting_started_with_bigint/
 * 
 */

extern crate num_bigint;
use num_bigint::{BigInt, BigUint, Sign};
use num_traits::Zero;
use num_traits::One;

fn main() {
  //let x = BigInt::new(Sign::Plus, vec![1, 0]);
  //println!(num::pow(x, 100).to_str_radix(10));
  
  let a: BigInt = "12345678901234567890".parse().unwrap();
  let b: BigInt = "98765432109876543210".parse().unwrap();

  let sum = &a + &b; // Use references for arithmetic operations
  let difference = &b - &a;
  let product = &a * &b;

  // Ensure b is not zero before dividing
  let quotient = if !b.is_zero() {
    Some(&a / &b)
  } else {
    None
  };

  println!("Sum: {}", sum);
  println!("Difference: {}", difference);
  println!("Product: {}", product);
  println!("Quotient: {:?}", quotient);

  let exp = BigUint::one()<<136279841 - 1;
  println!("2^136279841 - 1 = {}", exp);
}
