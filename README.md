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

## object soup

Check the python version first.

## rust printing

### let's start from C code

```print.c``` prints its arguments.

```c
$ cat print.c 
#include <stdio.h> // for printf

int main(int argc, char **argv) {
  for (int i = 0; i < argc; i++) {
    char *arg = argv[i];
    printf("%s\n", arg);
  }

  return 0;
}
$ gcc print.c -o p
$ ./p "ready" "go"
./p
ready
go
$ ./p "ready" "go" | xxd -g 1
00000000: 61 72 67 76 20 3d 20 30 78 31 36 64 33 34 33 36  argv = 0x16d3436
00000010: 39 30 0a 61 72 67 76 5b 30 5d 20 3d 20 30 78 31  90.argv[0] = 0x1
00000020: 36 64 33 34 33 38 32 38 0a 2e 2f 70 0a 61 72 67  6d343828../p.arg
00000030: 76 5b 31 5d 20 3d 20 30 78 31 36 64 33 34 33 38  v[1] = 0x16d3438
00000040: 32 63 0a 72 65 61 64 79 0a 61 72 67 76 5b 32 5d  2c.ready.argv[2]
00000050: 20 3d 20 30 78 31 36 64 33 34 33 38 33 32 0a 67   = 0x16d343832.g
00000060: 6f 0a 
```

Each argument is terminated by the value of 0. In C every string is termined by ```null```.

Then we introduce UTF-8 or unicode into the argument.

```
$ node print.js "élément"
ÉLÉMENT
$ node print.js "élément" | xxd -g 1
00000000: c3 89 4c c3 89 4d 45 4e 54 0a                    ..L..MENT.
```

Note that ```É``` is coded as ```c3 89```.

## reference

* [object soup](https://jacko.io/object_soup.html)

* [strings in rust](https://fasterthanli.me/articles/working-with-strings-in-rust)
