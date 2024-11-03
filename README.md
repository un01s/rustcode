# exercises in rust

## setup

```
$ cargo new rustcode
$ cd rustcode
$ cargo run
   Compiling rustcode v0.1.0 (/Users/wb/io/code/rustlang/rustcode)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/rustcode`
Hello, world!
```

## read one byte out of a static string

```
$ cargo run
   Compiling rustcode v0.1.0 (/Users/wb/io/code/rustlang/rustcode)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/rustcode`
Hello, world!
104
thread 'main' panicked at src/main.rs:4:3:
index out of bounds: the len is 12 but the index is 100
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

There is a bound-check for the index value. It is automatic in rust. The function ```read_one-byte()``` will never cause undefined behavior without the programmer writing ```unsafe``` first. The function is sound. Rust guarantees that functions like ```read_one_byte()```, which does not use any unsafe code either directly or indirectly, will always be sound.

## unsafe code

```
$ cargo run
   Compiling rustcode v0.1.0 (/Users/wb/io/code/rustlang/rustcode)
error[E0133]: call to unsafe function `get_one` is unsafe and requires unsafe function or block
  --> src/main.rs:23:18
   |
23 |   println!("{}", get_one(1));
   |                  ^^^^^^^^^^ call to unsafe function
   |
   = note: consult the function's documentation for information on how to avoid undefined behavior

For more information about this error, try `rustc --explain E0133`.
error: could not compile `rustcode` (bin "rustcode") due to 1 previous error
```
