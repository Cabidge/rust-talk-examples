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

// Specific implementation of generic type
impl Point<i32> {
    fn origin() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

fn main() {
    // Rust can often infer the generic type
    let p = Point { x: 1, y: 2 };
    println!("x = {}, y = {}", p.x, p.y);

    // This doesn't work!
    // An i32 != f64
    //let p = Point { x: 1, y: 2.0 };

    // You can be explicit about types by using
    // the turbo fish operator (::<>)
    let p = Point::<u8> { x: 1, y: 2 };
    println!("x = {}, y = {}", p.x, p.y);

    // Calling generic constructor
    let p = Point::new(1.2, 2.1);
    println!("x = {}, y = {}", p.x, p.y);

    // Constructor with turbo fish
    let p = Point::<f32>::new(24.5, 22.5);
    println!("x = {}, y = {}", p.x, p.y);

    // Calling specific implementation function
    let p = Point::<i32>::origin();
    println!("x = {}, y = {}", p.x, p.y);

    // Calling a generic function
    println!("{}", identity(1));
    println!("{}", identity::<u8>(1));

    // Generic add function
    println!("{}", add(1, 2));
}

// Generic function definition
fn identity<T>(x: T) -> T { x }

// This doesn't work!
//fn add<T>(a: T, b: T) -> T { a + b }

// Type constraints for specifying expected type behavior
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
