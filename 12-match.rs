fn main() {
    // Pattern matching against enum variants
    // Evalutates from top to bottom, so order matters
    let opt = Some(12);
    match opt {
        // Matching a specific value
        Some(10) => println!("10 is the magic number"),
        // Binding a value to a variable
        Some(n) => println!("We have the number {}", n),
        None => println!("We don't have a number"),
    }

    // Must be exhaustive
    // This doesn't work!
    //match opt {
    //    Some(10) => println!("10 is the magic number"),
    //}

    // Wildcard pattern
    match opt {
        Some(10) => println!("10 is the magic number"),
        // Matches every single possible pattern
        _ => println!("We didn't get 10"),
    }

    // Wildcard to ignore inner values
    let result = "3.14".parse::<f64>();
    match result {
        Ok(n) => println!("I got a {}!", n),
        Err(_) => println!("An error ocurred..."),
    }
}
