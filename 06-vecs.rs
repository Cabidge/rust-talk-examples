fn main() {
    // A Vec, or vector, is a dynamically sized list of elements,
    // meaning you can add or remove as many elements as you want
    // To create a new Vec, use the Vec::new() function
    let mut v = Vec::new();

    // Pushing elements to Vec
    v.push(1);
    v.push(2);
    v.push(3);

    // Print the entire contents of Vec
    println!("{:?}", v);

    // Index Vec with `[]`
    println!("v[1] is {}", v[1]);

    // Remove a specific index
    // This is an O(n) operation because it shifts all subsequent elements over
    println!("Removed v[2]: {}", v.remove(2));

    // Use the vec macro for concise Vec creation
    let v = vec![1, 2, 3];
    println!("{:?}", v);

    // Calling a function that consumes a Vec
    first_element(v);

    // This no longer works!
    //println!("{:?}", v);
}

fn first_element(v: Vec<i32>) {
    println!("The first element is {}", v[0]);
}
