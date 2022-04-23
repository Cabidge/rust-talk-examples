// Everything in rust is private by default,
// so a `pub` keyword is required if you want to use
// it outside of the current module
pub fn add3(n: i32) -> i32 {
    add(n, 3)
}

// This function won't be visible from outside modules
fn add(lhs: i32, rhs: i32) -> i32 {
    lhs + rhs
}