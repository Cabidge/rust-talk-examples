#[derive(Debug)]
struct Error;

fn main() {
    // panic! to halt the program
    //panic!();
    // ^ Uncomment me!

    // Convey recoverable errors with the Result enum
    // A Result is either an `Ok` for success or `Err` for failure

    // The parse method returns a Result
    let result = "12".parse::<i32>();
    println!("{:?}", result);
}