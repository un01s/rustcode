/*
 * rust design pattern
 * https://rust-unofficial.github.io/patterns/idioms/coercion-arguments.html
 * 
 * SO link:
 * https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str 
 *
 * fat pointer
 *
 * Fat pointer generally refers to a pointer that stores some extra data 
 * besides just the address of the object being pointed to.
 * 
 * The term "fat pointer" is used to refer to references and raw pointers to 
 * dynamically sized types (DSTs) – slices or trait objects. A fat pointer 
 * contains a pointer plus some information that makes the DST "complete" 
 * (e.g. the length).
 *
 * Most commonly used types in Rust are not DSTs but have a fixed size known 
 * at compile time. These types implement the Sized trait. Even types that 
 * manage a heap buffer of dynamic size (like Vec<T>) are Sized, as the 
 * compiler knows the exact number of bytes a Vec<T> instance will take up on 
 * the stack. There are currently four different kinds of DSTs in Rust. 
 *
 * - slices ([T] and str)
 * - trait objects (dyn Trait)
 * - custom DSTs
 * - exception: Extern types 
 *
 * str vs String
 *
 * String is a dynamic heap string type, like vec. use it when you need to own 
 * or modify your string data.
 * str is an immutable sequence of UTF-8 bytes of dynamic length 
 * &str a reference to some UTF-8 data, normally called a string slice.
 * a slice is just a view onto some data, and that data can be anaywhere
 * - in static storage: a string literal "foo" is &'static str. the data is
 *   hardcoded into the executable and loaded into memory when the program 
 *   runs.
 * - inside a heap allocated String
 * - on the stack
 *
 * use String if you have to own string data like passing strings to other
 * threads or building them at runtime., and use &str if you only need a view
 * of a string.
 *
 */

use std::mem;

fn three_vowels(word: &String) -> bool {
    let mut vowel_count = 0;
    for c in word.chars() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                vowel_count += 1;
                if vowel_count >= 3 {
                    return true;
                }
            }
            _ => vowel_count = 0,
        }
    }
    false
}

trait Animal {
    fn speak(&self);
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
}


fn main() {
    dbg!(size_of::<&u32>()); // 8 bytes
    dbg!(size_of::<&[u32; 2]>()); // 8
    dbg!(size_of::<&[u32]>()); // 16, [u32] is a DST
/*
    &[u32] is something like the following

    struct SliceRef {
        ptr: *const u32,
        len: usize,
    }
*/

    // for mutable reference
    dbg!(size_of::<&mut [u32]>());
    // for immutable raw pointers
    dbg!(size_of::<*const [u32]>());
    // for mutable raw pointers
    dbg!(size_of::<*mut [u32]>());

    dbg!(size_of::<&Cat>()); // 8 bytes because Cat is a normal type
    dbg!(size_of::<&dyn Animal>()); // 16 bytes, dyn Animal is a trait object

    let ferris = "Ferris".to_string();
    let curious = "Curious".to_string();
    println!("{}: {}", ferris, three_vowels(&ferris));
    println!("{}: {}", curious, three_vowels(&curious));

    // This works fine, but the following two lines would fail:
    // println!("Ferris: {}", three_vowels("Ferris"));
    // println!("Curious: {}", three_vowels("Curious"));

    println!("{}", mem::size_of::<&str>()); // 16
    println!("{}", mem::size_of::<String>()); // 24

    let string1: &'static str = "abc";
    // string will point to 'static memory which lives throughout the whole program

    let ptr = string1.as_ptr();
    let len = string1.len();

    println!("{}, {}", unsafe { *ptr as char }, len); // a, 3
    // len is 3 characters long so 3
    // pointer to the first character points to letter a

    {
        let mut string2: String = "def".to_string();

        let ptr = string2.as_ptr();
        let len = string2.len();
        let capacity = string2.capacity();
        println!("{}, {}, {}", unsafe { *ptr as char }, len, capacity); // d, 3, 3
        // pointer to the first character points to letter d
        // len is 3 characters long so 3
        // string has now 3 bytes of space on the heap

        string2.push_str("ghijk"); // we can mutate String type, capacity and length will also change
        println!("{}, {}", string2, string2.capacity()); // defghijk, 8

    } // memory of string2 on the heap will be freed here because owner goes out of scope

}

