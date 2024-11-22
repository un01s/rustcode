/*
 * Rust fold
 * apply a function to each element of the iteration
 * accumulating the result into a new value
 *
 * each iteration of fold takes two arguments.
 * the first is an initial value for the accumulator.
 * the second is a closure which itself takes two arguments: 
 * the accumulator and an item from the iterator.
 * once the iterator drains, the final value of the accumulator is returned.
 *
 */

fn main() {
  // imperative: what
  let mut sum = 0;
  for i in 1..11 {
    sum += i;
  }
  println!("{sum}");

  // declarative: how
  println!("{}", (1..11).fold(0, |a, b| a + b));

  // another fold example
  let even_sum = (1..=10).fold(0, |acc, num| 
    if num%2==0 { 
      acc + num
    } else { 
      acc
    }); 
  println!("{even_sum:?}");

  // use filter to do the same
  //println!("{:?}", (0..11).filter(|n| n%2==0).sum());
  let numbers = vec![1, 2, 3, 4, 5, 6, 7];
  let even_numbers: Vec<_> = numbers.iter().filter(|&x| x % 2 == 0).collect();
  println!("Even numbers: {:?}", even_numbers);

  let week_days = vec!["Monday", 
                        "Tuesday", 
                        "Wednesday", 
                        "Thursday", 
                        "Friday", 
                        "Saturday", 
                        "Sunday"];
  let filtered_week_days:Vec<_> = 
        week_days.iter()                     
        .filter(|days| days.len() < 8)
        .collect();
  println!("{:?}", filtered_week_days);
  // ["Monday", "Tuesday", "Friday", "Sunday"]
}
