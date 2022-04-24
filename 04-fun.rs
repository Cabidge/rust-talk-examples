fn main() {
    println!("{}", double(3));
}

// Functions must contain type annotations for parameters
// i32 represents a signed 32 bit integer
fn double(x: i32) -> i32 {
    // The last expression of a function is implicitly returned
    x * 2
}
