# types in rust

rust is a static language. each value has its exact type.

## primitive types

* numeric (integer and float): 
  * signed: i8, i16, i32, i64, isize
  * unsigned: u8, u16, u32, u64, usize
  * float: f32, f64
* textual (string and char)
  * str: use "", and &str (a slice is a kind of reference without ownership)
  * char: use ''
* boolean: bool, true and false
* never: ```!``` a type with no value

## sequence types:

* tuple: unit () and others like (i32,), (f32, String), (i32, Vec<String>, Option<bool>) 
* array: [type;expression]
* slice

## user-defined types

* struct
* enum
* union

## function types

* functions
* closures

## pointer types

* references
* raw pointers
* function pointers

## trait types

* trait objects
* impl trait

