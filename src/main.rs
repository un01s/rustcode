use std::ascii::escape_default;
use std::str;

static BYTES: &[u8] = b"hello world!";

fn show(bs: &[u8]) -> String {
  let mut visible = String::new();
  for &b in bs {
    let part: Vec<u8> = escape_default(b).collect();
    visible.push_str(str::from_utf8(&part).unwrap());
  }
  visible
}

fn read_one_byte(index: usize) -> u8 {
  BYTES[index]
}

/*
// like returning one byte as *(BYTES+index) in C
unsafe fn get_one(index: usize) -> u8 {
  *BYTES.as_ptr().add(index)
}
*/

// object soup
struct Person {
  name: String,
  friends: Vec<usize>,
}

fn new_person(people: &mut Vec<Person>, name: &str) -> usize {
  people.push(Person { name: name.into(), friends: Vec::new() });
  people.len() - 1
}

fn add_friend(people: &mut Vec<Person>, this_id: usize, other_id: usize) {
  if people[other_id].name != people[this_id].name {
    people[this_id].friends.push(other_id);
  }
}

fn main() {
  let arg = std::env::args()
        .skip(1)
        .next()
        .expect("should have one argument");
  println!("{}", arg.to_uppercase());

  let string = "onestring";
  for c in string.chars() {
    print!("{}", c);
  }
  println!();

  println!("Hello, world!");
  println!("{}", read_one_byte(1));
  //println!("{}", read_one_byte(100));

  // remove unsafe code for object soup
  //println!("{}", get_one(1));

  let mut people = Vec::new();
  let alice_id = new_person(&mut people, "Alice");
  let bob_id = new_person(&mut people, "Bob");
    
  add_friend(&mut people, alice_id, bob_id);
  add_friend(&mut people, bob_id, alice_id);
  add_friend(&mut people, alice_id, alice_id);

  // use show()
  let bytes = b"foo\xE2\x98\x83bar\xFFbaz";
  println!("{}", show(bytes));

}
