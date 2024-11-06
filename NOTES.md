# notes on important concepts in rust

memory safety and efficiency

## ownership, references and borrowing

* each value has a single owner
* there can only be one owner at a time. ownership is tranferrable from one variable to another
* when the owner goes out of scope, the value will be dropped. data is de-allocated when ownership ends

### variable scope

* global scope
* function scope
* block scope

### borrowing: sharing data without ownership tranferred

this is basically creating a pointer to the value.

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

let s2 = s.clone();; // deep copy
```

in case of string literal, the content is known at compile time, so the text is hardcoded directly into the final executable. this is why string literals are fast and efficient. but these properties only come from the string literal's immutability. 

with the ```String``` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

* the memory must be requested from the memory allocator at runtime
* we need a way of returning this memory to the allocator when we're done with our ```String```

The first part is done by us when we call ```String::from```, its implementation requests the memory it needs. This is pretty universal in programming languages.

However, the second part is different for different programming languages. Rust return the memory automatically once the variable that owns it goes out of scope. Basically when a variable goes out of scope, Rust calls a special function for us. That function is called ```drop()```, and it's where the author of ```String``` can put the code to return the memory. Rust calls ```drop``` automatically at the closing curly bracket.


### references, the stacka and the heap

stack: a region of memory where data is allocated for local variables and function arguments. this memory is managed automatically and is deallocated when a function completes.

heap: a region of memory where data is dynamically allocated. this memory is managed manually, requiring the developer to allocate and deallocate memory blocks.

In Rust, references typically point to data on the heap, as this allows for dynamic memory allocation. When a reference goes out of scope, it does not automatically de-allocate the data it points to. Instead, the ownership rules ensure that the data is only de-allocated when the original owner goes out of the scope.

```rust
let x = 5;
let y = x; // copy because integer is a primitive type, both x and y pushed to stack

let data = vec![1, 2, 3];
let data2 = data; // move
let reference = &data2; // Immutable borrow

let mut value = 36;
let reference = &mut value; // mutable borrow
*reference += 1;
```

A ```String``` is not a primitive type. It is made up of three parts:

* a pointer to the memory that holds the contents of the string
* a length
* a capacity

This group of data is stored on the stack. The data itself is the memory on the heap.
