# notes on important concepts in rust

memory safety and efficiency

## ownership, references and borrowing

* each value has a single owner
* there can only be one owner at a time. ownership is tranferrable from one variable to another
* when the owner goes out of scope, the value will be dropped. data is de-allocated when ownership ends

## variable scope

* global scope
* function scope
* block scope

### borrowing: sharing data without ownership tranferred

this is basically create a pointer to the value.

### types of references:

* immutable references
* mutable references

```rust
let s = String::from("Hello, world!");

// Immutable borrow
let len = s.len(); // s is still owned by the original variable

// Mutable borrow
let mut s = String::from("Hello, world!");
let r = &mut s;
r.push_str("!"); // Modifies the original string
```

### references, the stacka and the heap

stack: a region of memory where data is allocated for local variables and function arguments. this memory is managed automatically and is deallocated when a function completes.

heap: a region of memory where data is dynamically allocated. this memory is managed manually, requiring the developer to allocate and deallocate memory blocks.

In Rust, references typically point to data on the heap, as this allows for dynamic memory allocation. When a reference goes out of scope, it does not automatically de-allocate the data it points to. Instead, the ownership rules ensure that the data is only de-allocated when the original owner goes out of the scope.

```rust
let data = vec![1, 2, 3];
let data2 = data; // move
let reference = &data2; // Immutable borrow

let mut value = 36;
let reference = &mut value; // mutable borrow
*reference += 1;
```

