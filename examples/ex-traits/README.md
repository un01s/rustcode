# trait object

## dynamic dispatch vs static dispatch

dynamic dispatch is the process of selecting which implementation of a polymorphic operation (method or function) to call at run time. It is commonly employed in, and considered a prime characteristic of, object-oriented programming (OOP) languages and systems.

Object-oriented systems model a problem as a set of interacting objects that enact operations referred to by name. Polymorphism is the phenomenon wherein somewhat interchangeable objects each expose an operation of the same name but possibly differing in behavior. 

static dispatch, in which the implementation of a polymorphic operation is selected at compile time. The purpose of dynamic dispatch is to defer the selection of an appropriate implementation until the run time type of a parameter (or multiple parameters) is known.

## dynamic dispatch vs late binding or dynamic binding

Name binding associates a name with an operation. A polymorphic operation has several implementations, all associated with the same name. Bindings can be made at compile time or (with late binding) at run time. With dynamic dispatch, one particular implementation of an operation is chosen at run time. While dynamic dispatch does not imply late binding, late binding does imply dynamic dispatch, since the implementation of a late-bound operation is not known until run time.

## fat pointer

In Rust, the term fat pointer simply refers to a pointer with additional associated information. The additional information may be a vtable pointer for dynamic dispatch described above, but is more commonly the associated object's size to describe e.g. a slice.

## trait object

You have trait objects when you have a pointer to a trait.  Box, Arc, Rc and the reference & are all, at their core, pointers. In terms of defining a "trait object" they work in the same way. "Trait objects" are Rust's take on dynamic dispatch.

types do not "act" like trait objects; it's rather that any pointer to a trait is a trait object, and there can be different kinds of pointers. Box<T> is an owning pointer, Rc<T> is a shared ownership-pointer, Arc<T> is a multithreaded shared ownership-pointer, etc. In principle, each of these can be used to define trait objects, but currently only references and Boxes work for this. So no, right now you can't create custom pointer types which could be used to create trait objects.


