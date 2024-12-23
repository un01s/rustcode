//!
//! once a trait has been declared, it can be used either as a trait or
//! as a type. 
//! 
//! since Rust 1.27, dyn is introduced.
//!
//! Trait objects are the Rust implementation of dynamic dispatch. 
//! Dynamic dispatch allows one particular implementation of a polymorphic 
//! operation (trait methods) to be chosen at run time. Dynamic dispatch allows
//! a very flexible architecture because we can swap function implementations 
//! out at runtime. However, there is a small runtime cost associated with 
//! dynamic dispatch.
//!
//! The variables/parameters which hold the trait objects are fat pointers 
//! which consists of the following components:
//! 
//! pointer to the object in memory
//! pointer to that object’s vtable, a vtable is a table with pointers which 
//!     point to the actual method(s) implementation(s). 

use std::{fmt::Display, sync::Arc};

struct Point {
    x: i64,
    y: i64,
    z: i64,
}

trait Print {
    fn print(&self);
}

// dyn Print is actually a type and we can implement methods on it
impl dyn Print + 'static {
    fn print_traitobject(&self) {
        println!("from trait object");
    }
}

impl Print for Point {
    fn print(&self) {
        println!("x: {}, y: {}, z: {}", self.x, self.y, self.z);
    }
}

// static dispatch (compile time): compiler must know specific versions
// at compile time generates a version for each type

// compiler will use monomorphization to create different versions of the function
// for each type. However, because they can be inlined, it generally has a faster runtime
// compared to dynamic dispatch
fn static_dispatch<T: Print>(point: &T) {
    point.print();
}

// dynamic dispatch (run time): compiler doesn't need to know specific versions
// at compile time because it will use a pointer to the data and the vtable.
// The vtable contains pointers to all the different different function implementations.
// Because it has to do lookups at runtime it is generally slower compared to static dispatch

// point_trait_obj is a trait object
fn dynamic_dispatch(point_trait_obj: &(dyn Print + 'static)) {
    point_trait_obj.print();
    point_trait_obj.print_traitobject();
}

fn main() {
    let display_ref: &dyn Display = &42;
    let display_box: Box<dyn Display> = Box::new(42);
    let display_arc: Arc<dyn Display> = Arc::new(42);
    println!("Hello, world!");

    //
    let point = Point { x: 1, y: 2, z: 3 };

    // On the next line the compiler knows that the generic type T is Point
    static_dispatch(&point);

    // This function takes any obj which implements Print trait
    // We could, at runtime, change the specfic type as long as it implements the Print trait
    dynamic_dispatch(&point);
}
