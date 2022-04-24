fn main() {
    // Option type to represent values that might not exist
    let maybe = Some(12);
    println!("{:?}", maybe);

    let maybe: Option<i32> = None; // None needs an explicit type
    println!("{:?}", maybe);

    // Vec's pop method returns and removes the last element
    // as an Option to represent a value that may or may not exist
    let mut v = vec![1, 2];

    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
    println!("{:?}", v.pop());
}