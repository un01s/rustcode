# async in rust

## threads

```spawn``` a thread for each call so that several calls are made at the same time. ```join``` means "wait for the thread to finish."

```
$ cargo run  
   Compiling rustcode v0.1.0 (/Users/wb/io/code/rustlang/rustcode)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.89s
     Running `target/debug/rustcode`
start 1
start 4
start 2
start 5
start 6
start 7
start 8
start 3
start 9
start 10
end 4
end 1
end 3
end 9
end 2
end 5
end 6
end 7
end 10
end 8
```

The problem with thread is that each thread uses a lot of memory. So there is a limit on how many threads can be spawn. if there are too many threads like a thousand, the system will panic due to unavailable resources. Another thing is that switching between threads can affect performance. So thread does not scale well.

## asycn/await

For this version to run, we have to add ```tokio``` and ```futures```.

```
$ cargo add tokio
$ cargo add futures
```

Check Cargo.toml file and make sure the following line is in the dependencies.

```
tokio = {version = "1.41.0", features = ["full"]}
```

Here we go:

```
$ cargo run
Running `target/debug/rustcode`
start 1
start 2
start 3
start 4
start 5
start 6
start 7
start 8
start 9
start 10
end 1
end 2
end 3
end 4
end 5
end 6
end 7
end 8
end 9
end 10
```

## breakdown futures

The foo() was still an async function. But it is changed to a regular non-async function. It returns a ```Foo``` struct. It calls ```tokio::time::sleep```, but it does not use ```.await``` for Sleep future that sleep returns. Instead, it stores that future in the ```Foo``` struct.

Implementing the ```Future``` trait is what makes ```Foo``` a future. 

Under the hood, async functions in rust are just regular functions that return a ```Future```. The ```Future``` trait is a type that:

* can be polled
* when it's polled, it might return ```Pending``` or ```Ready```
* if it's pending, poll it again later
* if it's ready, it reponds with a value. This is resolving.

```rust
use std::{future::Future, pin::Pin, task::Context}

/// A future which returns a random number when it resolves.
#[derive(Default)]
struct RandFuture;

impl Future for RandFuture {
  // Every future has to specify what type of value it returns when it resolves.
  // This particular future will return a u16.
  type Output = u16;

  // The Future trait has only one method, named poll
  fn poll(mut self: Pin<&mut Self>, _cx: &mut Context) -> Poll<()> {
    Poll::ready(rand::random())
  }
}
```

