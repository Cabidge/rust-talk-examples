fn main() {
    // A Vec, or vector, is a dynamically sized list of elements,
    // meaning you can add or remove as many elements as you want
    // To create a new Vec, use the Vec::new() function
    let mut v = Vec::new();

    // Pushing elements to Vec
    v.push(1);
    v.push(2);
    v.push(3);

    // Get the length of a Vec
    println!("v is {} elements", v.len());

    // Pop the last element of Vec
    // Since pop returns an Option, we must unwrap to get the inner value
    println!("The last element of v is {}", v.pop().unwrap());

    // Print the entire contents of Vec
    println!("{:?}", v);

    // Use the vec macro for concise Vec creation
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // Iterate over Vec with the `.iter()` method
    for element in v.iter() {
        println!("{}", element);
    }
}
