fn main() {
    // A Vec, or vector, is a dynamically sized list of elements
    // To create a new Vec, use the Vec::new() function
    let mut v = Vec::new();

    // Pushing elements to a vector
    v.push(1);
    v.push(2);
    v.push(3);

    // Print the entire contents of a vector
    println!("{:?}", v);

    // Use the vec macro for concise Vec creation
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // Calling a function that consumes a vector
    print_first(v);

    // This no longer works!
    //println!("{:?}", v);
}

fn print_first(v: Vec<i32>) {
    println!("The first element is {}", v[0]);
}
