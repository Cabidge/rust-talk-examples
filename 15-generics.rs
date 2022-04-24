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
    // Creating a Point<i32>
    let p = Point::new(1, 2);
    println!("{}, {}", p.x, p.y);

    // Creating a Point<f64>
    let p = Point::new(1.2, 2.1);
    println!("{}, {}", p.x, p.y);

    // Calling generic functions
    println!("{}", identity(1));
    println!("{}", identity(2.3));

    print_len(&[1, 2, 5]);
    print_len(&["hi", "wowee"]);
}

// Generic function definition
fn identity<T>(x: T) -> T { x }

// This doesn't work!
//fn add<T>(a: T, b: T) -> T { a + b }

// Type constraints for specifying expected type behavior
fn print_len<T>(a: &[T]) {
    println!("The slice has {} elements", a.len());
}
