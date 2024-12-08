/*
 * unboxed closures are the only kind of closures that 
 * do not need a feature flag.
 * moreover, static functions can be easily converted to 
 * unboxed closures.
 * therefore, the correct way to call functions from a vector of functions is
 * as follows.
 *
 */

fn f1(i: i32) -> i32 { i * 2 }

fn f2(i: i32) -> i32 { i * 4 }

fn main() {
    let arr: Vec<&dyn Fn(i32) -> i32> = vec![&f1, &f2];

    for f in &arr {
        println!("{}", (f)(1));
    }
}

