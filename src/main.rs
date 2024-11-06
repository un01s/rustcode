/*
 * it's how to cast pointer to struct
 *
 */

use std::ptr::addr_of;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]

struct A<'text> {
  foo: &'text str,
  baz: i32,
}

fn ptr_to_struct(p: *const A) {
  let b: A = unsafe { *p }; // main part of this example!
  println!("{:#?}", b);
}

fn ptr_to_struct_2(p: usize) {
  let b = unsafe { *(p as *const A) }; // main part of this example!
  println!("{:#?}", b);
}

fn main() {
  let a = A {
    foo: "Hello",
    baz: 2000,
  };

  ptr_to_struct(&a);
  ptr_to_struct_2(&a as *const _ as usize);

  // how to get the address of a variable?
  let my_var = 100;
  println!("address = {:#x}", (&my_var) as *const i32 as usize);

  let num = 32;
  println!("Address: {:?}", addr_of!(num));
  let borrow = &num;
  println!("Address: {:?}", addr_of!(borrow));
}

