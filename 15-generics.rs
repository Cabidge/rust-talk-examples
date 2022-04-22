// Use angle brackets to define a generic type parameter
struct Point<T> {
    x: T,
    y: T,
}

// Generic implementation requires type parameter after `impl` keyword
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

fn main() {
    // Rust can often infer the generic type
    let p = Point { x: 1, y: 2 };
    println!("x = {}, y = {}", p.x, p.y);

    // Calling generic constructor
    let p = Point::new(1.2, 2.1);
    println!("x = {}, y = {}", p.x, p.y);

    // Calling a generic function
    println!("{}", identity(1));
    println!("{}", identity(2.3));
    print_len(vec![1, 2, 5]);
    print_len(vec!["hi", "wowee"]);
}

// Generic function definition
fn identity<T>(x: T) -> T { x }

// This doesn't work!
//fn add<T>(a: T, b: T) -> T { a + b }

// Type constraints for specifying expected type behavior
fn print_len<T>(v: Vec<T>) {
    println!("The vec has {} elements", v.len());
}
