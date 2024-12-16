
fn main() {
  // range pattern
  for (index, value) in (5..10).enumerate() {
    println!("index = {} and value = {}", index, value);
  }

  for x in 0..10 {
    println!( "{} ", x);
  }

  let mut x = 5;
  let mut done = false;
  while !done {
    x += x - 3;
    println!("{x}");
    if x%5 == 0 {
      done = true;
    }
  }

  // loop labels
  'outer: for x in 0..10 {
    'inner: for y in 0..10 {
      if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
      if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
      println!("x: {}, y: {}", x, y);
    }
  }

//  loop {
//    println!("forever loop");
//  }

}
