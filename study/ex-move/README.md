# [rust ownership, move and borrow](https://www.openmymind.net/Rust-Ownership-Move-and-Borrow-part-2/)

this blog is good to explain how Rust ownership, move and borrow work. 
it then introduces traits, mutability, scope and lifetime. And how these concepts interact with the ownership.

Check out [this comment](https://users.rust-lang.org/t/for-beginners-an-interesting-article-about-ownership-and-borrowing/108718/2).

## part1 is bit over-generalized

it presents references as immutable/mutable (in contrast with shared/exclusive) without pointing out that shared mutability is possible (also common). it has some bigger misconceptions about reference lifetimes or perhaps lifetimes generally.


