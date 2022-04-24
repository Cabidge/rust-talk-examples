fn main() {
    // Built-in Option enum to represent nullable values
    let maybe = Option::Some(12);
    println!("{:?}", maybe);

    let maybe = Option::<i32>::None;
    println!("{:?}", maybe);

    // Option variants are automatically imported
    let maybe = Some(12);
    println!("{:?}", maybe);

    let maybe: Option<i32> = None; // None needs an explicit type
    println!("{:?}", maybe);
}