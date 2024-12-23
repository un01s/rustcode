/*
 * Rust traits
 * a trait tells the compiler about a type must provide
 * a trait is a promise that one type makes
 *
 * trait object
 *  
 */


#![allow(unused_variables)]

struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaah!"
        }
    }
    
    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

// define an example struct, make it printable
#[derive(Debug)]
struct Foo;

// an example trait
trait Bar {
    fn baz(&self);
}

// implement the trait for Foo
impl Bar for Foo {
    fn baz(&self) {
        println!("{:?}", self)
    }
}

// This is a generic function that takes any T that implements trait Bar.
// It must resolve to a specific concrete T at compile time.
// The compiler creates a different version of this function
// for each concrete type used to call it so &T here is NOT
// a trait object (as T will represent a known, sized type
// after compilation)
fn static_dispatch<T>(t: &T)
where
    T: Bar,
{
    t.baz(); // we can do this because t implements Bar
}

// This function takes a pointer to a something that implements trait Bar
// (it'll know what it is only at runtime). &dyn Bar is a trait object.
// There's only one version of this function at runtime, so this
// reduces the size of the compiled program if the function
// is called with several different types vs using static_dispatch.
// However performance is slightly lower, as the &dyn Bar that
// dynamic_dispatch receives is a pointer to the object +
// a vtable with all the Bar methods that the object implements.
// Calling baz() on t means having to look it up in this vtable.
fn dynamic_dispatch(t: &dyn Bar) {
    // ----------------^
    // this is the trait object! It would also work with Box<dyn Bar> or
    // Rc<dyn Bar> or Arc<dyn Bar>
    //
    t.baz(); // we can do this because t implements Bar
}

fn main() {
    // Type annotation is necessary in this case.
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();

    // trait objects
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);
}

