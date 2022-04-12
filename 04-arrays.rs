fn main() {
    // Array literals are declared with square brackets
    // Arrays have a fixed size and are stored on the stack
    // The type of an array is denoted with [T; n] for an array of
    // some type T of size n
    let mut x = [1, 2, 3, 4];

    // Indexing an array (0-based index)
    println!("The second element of x is {}", x[1]);

    // Mutating an array's elements
    x[1] = 3;
    println!("The second element of x is {}", x[1]);

    // Replacing an entire array
    x = [8, 8, 8, 8];
    println!("The second element of x is {}", x[1]);

    // Create an array with repeating values
    let x = [3; 8];
    // Print the contents of an array
    // Using `{:?}` formats the given value in "Debug" mode,
    // which includes more information meant for the programmer
    // Using `{}` is meant to be prettier and is for the end user
    println!("{:?}", x);
}
