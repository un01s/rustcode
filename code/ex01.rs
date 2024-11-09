enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

enum Location {
  Point(i32),
  Range(i32, i32)
}

fn main() {
  let l: Location = Location::Range(0, 5);
  let n = match l {
    Location::Point(_) => -1,
    Location::Range(_, n) => n,
    Location::Range(0, _) => 0,
    _ => -2
  };
  println!("{n}");
}

