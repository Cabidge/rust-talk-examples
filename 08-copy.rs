// Deriving Clone allows us to duplicate a value completely with its own fields
// Deriving Copy tells the compiler that copying the type is a cheap operation
// because all its fields are stack allocated
#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

fn main() {
    let comp = Complex { real: 3.0, imag: 4.0 };
    
    // Same as: `let mut x = comp.clone();`
    let mut x = comp;

    // Mutating x doesn't affect comp
    x.real = 8.0;
    x.imag = 10.0;

    println!("comp is {:#?}", comp);
    println!("x is {:#?}", x);
}