# memory management and safe code

* garbage collection: Java, Go
* manual memory allocation and release: C++
* ownership: compiler check: Rust

## stack and heap

## ownership: reference and borrowing

* in Rust, every value is owned by a variable. this variable is the owner of this value
* a value could be owned only by a variable at a time. one value has only one owner.
* when a owner (varaible) goes out of the scope, this value will be dropped.

