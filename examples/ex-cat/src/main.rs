struct Cat {
  name: String // name: type
}

impl Cat {
  fn meow(&self) {
    println!("meow");
  }
  // unique access
  fn set_name(&mut self, name: String) {
    self.name = name; // mutating a value
  }
  // shared access
  fn name(&self) -> &str {
    &self.name //expression-oriented return
  }
}

fn main() {
  let name = String::from("peter");
  let mut cat = Cat{name};
  println!("cat's name is {}", cat.name());
  let new_name = String::from("tom");
  cat.set_name(new_name);
  println!("cat's new name is {}", cat.name());
  cat.meow();
}
