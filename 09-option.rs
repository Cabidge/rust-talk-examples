fn main() {
    // Option type to represent values that might not exist
    let maybe = Some(12);
    println!("{:?}", maybe);

    let maybe: Option<i32> = None; // None needs an explicit type
    println!("{:?}", maybe);
}