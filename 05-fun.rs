fn main() {
    println!("{}", double(3));
}

// Functions must contain type annotations for parameters
// Functions that don't return a value don't need a return type
// i32 represents a signed 32 bit integer
fn double(x: i32) -> i32 {
    // This also works!
    //return x * 2;

    // The last expression of a function is implicitly returned
    x * 2

    // This doesn't work!
    // The last expression cannot have a semicolon, as that makes
    // it a statement
    //x * 2;
}
