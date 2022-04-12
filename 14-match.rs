#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    // Pattern matching against enum variants
    let coin = Coin::Dime;
    match coin {
        Coin::Penny => println!("1 cent"),
        Coin::Nickel => println!("5 cents"),
        Coin::Dime => println!("10 cents"),
        Coin::Quarter => println!("25 cents"),
    }

    let coin = Coin::Dime;
    // `match`es are expressions
    let value = match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
    println!("A {:?} is worth {} cent(s)", coin, value);

    // Matching variants with associated values
    // match expressions check from top to bottom, so order matters
    let opt = Some(12);
    match opt {
        // Matching a specific value
        Some(10) => println!("10 is the magic number"),
        // Binding a value to a variable
        Some(n) => println!("We have the number {}", n),
        None => println!("We don't have a number"),
    }

    // Wildcard pattern
    match opt {
        Some(10) => println!("10 is the magic number"),
        // Matches every single possible pattern
        _ => println!("We didn't get 10"),
    }

    // match expressions must be exhaustive
    // This doesn't work!
    //match opt {
    //    Some(10) => println!("10 is the magic number"),
    //}

    // Use unit type to represent no action
    match opt {
        Some(10) => println!("10 is the magic number"),
        _ => (),
    }

    // Wildcard to ignore values
    match opt {
        Some(_) => println!("We have some number"),
        _ => (),
    }

    // if let for ignoring other patterns
    if let Some(n) = opt {
        println!("We have the number {}", n);
    }

    // Other patterns
    let n = 12;
    match n {
        // Multi-pattern
        12 | 15 => println!("Twelve or fifteen"),
        // Ranges
        0..=9 => println!("Single digit number"),
        // if guards
        n if n < 0 => println!("Negative number"),
        _ => println!("Zero or positive"),
    }

    // Option::unwrap returns the inner value,
    // but will panic (crash) if the Option is None
    let opt = Some(12);
    let value = opt.unwrap();
    println!("{}", value);

    // Option::expect for descriptive error message
    let opt = Some(12);
    let value = opt.expect("Didn't have a number");
    println!("{}", value);
}
