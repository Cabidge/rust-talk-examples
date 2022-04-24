// `#[derive(Debug)]` macro allows formatting via {:?}
#[derive(Debug)]
// Create a struct with named fields
struct Complex {
    real: f64,
    imag: f64,
}

fn main() {
    // Instantiating named struct
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
