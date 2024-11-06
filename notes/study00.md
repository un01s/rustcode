# learn in rust

## setup on mac

* use rustup to install rust

```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ xcode-select --install
$ rustup update
$ rustc -V
rustc 1.81.0 (eeb90cda1 2024-09-04)
$ cargo -V
cargo 1.81.0 (2dbb1af80 2024-08-20)
```

## hello cargo

* use cargo to create a new project

```
$ cargo new hello
$ cd hello
$ tree
.
├── .git
├── .gitignore
├── Cargo.toml
└── src
    └── main.rs
```

* use cargo to run the code

```
$ cd hello
$ cargo run
Running `target/debug/hello`
Hello, world!
```

* compile and run the code

```
$ cd hello
$ cargo build
$ ./target/debug/hello
Hello, world!
```

* rustc to build the code

```
$ cat main.rs
fn main() {
  println!("Hello, world!");
}
$ rustc main.rs
./main
Hello, world!
```

* notes: rust has the native support for UTF-8

* notes: println! is a macro, take note of that ```!```

