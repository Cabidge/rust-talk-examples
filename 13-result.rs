#[derive(Debug)]
struct Error;

fn main() {
    // panic! to halt the program
    //panic!();
    // ^ Uncomment me!

    // Convey recoverable errors with the Result enum

    // Ok for a successful result
    let result: Result<i32, Error> = Ok(12);
    println!("{:?}", result);

    // Err for an unsuccessful result
    let result: Result<i32, Error> = Err(Error);
    println!("{:?}", result);

    // Parsing returns a Result
    let result = "24".parse::<i32>();
    println!("{:?}", result);
}