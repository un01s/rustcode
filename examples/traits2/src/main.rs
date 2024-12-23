// define a trait
trait Animal {
  fn make_noise(&self);
}

// implement a trait
// for a type
struct Dog;

impl Animal for Dog {
  fn make_noise(&self) {
    println!("woof!");
  }
}

// create a trait object
fn make_it_speak(animal: &dyn Animal) {
  animal.make_noise();
}

// type
struct Duck;
impl Duck {
  fn swim(&self) {
    println!("duck swimming");
  }
}

// type
struct Swan;
impl Swan {
  fn fly(&self) {
    println!("swan flying");
  }
}

// trait
trait Bird {
  fn quack(&self) -> String;
}

impl Bird for Duck {
  fn quack(&self) -> String {
    "duck quack".to_string()
  }
}

impl Bird for Swan {
  fn quack(&self) -> String {
    "swan quack".to_string()
  }
}

fn main() {
  let dog = Dog;
  dog.make_noise();

  make_it_speak(&dog);

  // 
  let duck = Duck;
  duck.swim();
}
