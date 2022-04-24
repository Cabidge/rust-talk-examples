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

    // Pipe to define alternatives
    match opt {
        Some(10) | Some(-10) => println!("10 is the magic number"),
        Some(n) => println!("We have the number {}", n),
        None => println!("We don't have a number"),
    }

    // Patterns can be nested
    match opt {
        Some(10 | -10) => println!("10 is the magic number"),
        Some(n) => println!("We have the number {}", n),
        None => println!("We don't have a number"),
    }

    // Wildcard to ignore values
    match opt {
        Some(10 | -10) => println!("10 is the magic number"),
        // Matches every single possible inner value and ignores it
        Some(_) | None => println!("We didn't get 10"),
    }

    // General wildcard
    match opt {
        Some(10 | -10) => println!("10 is the magic number"),
        // Matches every single possible pattern
        _ => println!("We didn't get 10"),
    }
}
