# [rust ownership, move and borrow](https://www.openmymind.net/Rust-Ownership-Move-and-Borrow-part-2/)

this blog is good to explain how Rust ownership, move and borrow work. 
it then introduces traits, mutability, scope and lifetime. And how these concepts interact with the ownership.

Check out [this comment](https://users.rust-lang.org/t/for-beginners-an-interesting-article-about-ownership-and-borrowing/108718/2).

## part1 is bit over-generalized

it presents references as immutable/mutable (in contrast with shared/exclusive) without pointing out that shared mutability is possible (also common). it has some bigger misconceptions about reference lifetimes or perhaps lifetimes generally.

## ownership principles

* every value in Rust has only one owner at a time, it is its binding variable.
* typically there is one owner, but shared ownership can also be implemented.
* &mut T is exclusive reference, only one active reference is allowed.
* &T is shared references, it can be aliased.
* values can have move or copy semantics depending on the Copy trait

```rust
fn foo<'v>(v: &'v mut Vec<i32>) {
  v.push(0);         // line 1
  println!("{v:?}"); // line 2
}
```

You're not moving ```v: &mut Vec<i32>``` when you pass it to ```push``` on line 1. But you are not copying it either, because ```&mut _``` does not implement ```Copy``` trait. Instead, ```*v``` is reborrowed for some shorter lifetime than ```'v```, which ends on line1.

An explicit reborrow would look like this:

```rust
Vec::push(&mut *v, 0);
```

```v``` can't be used while the reborrow exists, but after it expires, you can use ```v``` again. In this way, both &mut are still exclusive borrows.

## references

* [Trait Copy](https://doc.rust-lang.org/std/marker/trait.Copy.html)
* [shared mutability](https://doc.rust-lang.org/core/cell/struct.UnsafeCell.html)
* [shared ownership](https://doc.rust-lang.org/std/sync/struct.Arc.html)
* [shared ownership: module rc](https://doc.rust-lang.org/std/rc/index.html)
* [method-call lookup](https://doc.rust-lang.org/stable/reference/expressions/method-call-expr.html)
* [type coercions](https://doc.rust-lang.org/stable/reference/type-coercions.html#coercion-types)
* [reborrow](https://quinedot.github.io/rust-learning/st-reborrow.html)

