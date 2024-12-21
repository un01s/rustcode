//!
//! macros look like functions, except that their names end with a bang !.
//! instead of generating a function call, macros are expanded into source
//! code that gets compiled with the rest of program.
//!
//! macros are created using the macro_rules! macro.
//!
//! patterns and designators
//! overloading
//! repetition
//!

macro_rules! say_hello {
  // `()` indicates that the macro takes no argument.
  () => {
    // the macro will expand into the contents of this block.
      println!("Hello!")
  };
}

fn main() {
  say_hello!();
}
