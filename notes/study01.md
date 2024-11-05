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


