// Create a struct without fields
// Use the derive macro to automatically implement Debug trait
// Allows formatting via {:?}
#[derive(Debug)]
struct Error;

// Create a struct with anonymous fields
#[derive(Debug)]
struct Point(i32, i32);

// Create a struct with named fields
#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

fn main() {
    // Instantiating unit struct
    let err = Error;
    println!("{:?}", err);

    // Instantiating tuple struct
    let p = Point(2, 4);
    println!("{:?}", p);

    // Instantiating normal struct
    let i = Complex {
        real: 0.0,
        imag: 1.0,
    };
    // Using {:#?} is pretty debug, which formats struct-like
    // values nicely over multiple lines
    println!("{:#?}", i);

    // Access struct fields with dot syntax
    println!("i.real is {}", i.real);
    println!("i.imag is {}", i.imag);
}
