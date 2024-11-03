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

