fn main() {
    // Array literals are declared with square brackets
    let mut x = [1, 2, 3, 4];

    // Indexing an array (0-based index)
    println!("The second element of x is {}", x[1]);

    // Mutating an array's elements
    x[1] = 3;
    println!("The second element of x is {}", x[1]);

    // Print the contents of an array
    println!("{:?}", x);

    // `{}` is display mode for user-facing output
    // `{:?}` is debug mode for programmer-facing output
}
