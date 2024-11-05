# learn in rust

## function return

```rust
fn add(i: i32, j: i32) -> i32 {
  i + j // expression
}
```

No ```;``` after ```i+j```. If there is a ```;``` after ```i+j```, that function returns ```()``` implicitly instead of an ```i32```. Another way is to write ```return i+j;```.

```rust
fn add(i: i32, j: i32) -> i32 {
  return i + j; // statement
}
```

## variable: mutable or immutable

* let

A variable by default is immutable. This means you as a programmer cannot assign another value to the immutable variable. The usual assignment statement like ```let a = "hello";``` is called ```variable binding```.
 
Use ```_``` to tell Rust to ignore it. 

* const 

```rust
const MAX: u32 = 100_000;
```

* shadowing

```rust
let x = 5;
let x = x + 1; // shadowing
```

