/*
 *  stack: a region of memory where data is allocated for local variables
 *  and function arguments. This memory is  managed automatically and is 
 *  deallocated when a function completes.
 * 
 *  heap: a region of memory where data is dynamically allocated. This memory
 *  is managed manually, requiring the developer to allocate and deallocate.
 *
 *  in Rust, references typically point to data on the heap, as this allows for
 *  dynamic memory allocation. When a reference goes out of scope, it does not
 *  automatically de-allocate the data it points to. Instead, the ownership
 *  rules ensure that the data is only de-allocated when the original owner
 *  goes out of scope.
 * 
 */

#[warn(unused_variables)]
#[warn(dead_code)]

struct Node {
  value: i32,
  next: Option<Box<Node>>,
}

struct LinkedList {
  head: Option<Box<Node>>,
}

fn main() {

  let s = String::from("Hello, world!");

  // Immutable borrow
  let _len = s.len(); // s is still owned by the original variable

  // Mutable borrow
  let mut s = String::from("Hello, world!");
  let r = &mut s;
  r.push_str("!"); // Modifies the original string

  println!("{s}");

  // create a new linked list
  let mut list = LinkedList{ head: None };
  // add a node to the list
  let new_node = Node { value: 1, next: None };
  let new_node = Box::new(new_node);

  list.head = Some(new_node); // ownership is transferred
}
