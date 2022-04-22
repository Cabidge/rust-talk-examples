// Ignore all unused types
#![allow(dead_code)]

// Define an enumerated type with the `enum` keyword
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// use statement to import enum variants into namespace
// `use Coin::Variant;` to import a specific variant
use Coin::*;

// Enum variants can have associated values
#[derive(Debug)]
enum MaybeInt {
    Nothing,
    Some(i32),
}

fn main() {
    // Instantiating an enum with the `Enum::Variant` syntax
    let coin = Coin::Nickel;
    println!("{:?}", coin);

    // Instantiate imported enum variant
    let coin = Dime;
    println!("{:?}", coin);

    // Instantiating variants with associated types
    let maybe = MaybeInt::Some(12);
    println!("{:?}", maybe);
}
