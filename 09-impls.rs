#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

// Create associated functions and methods with `impl` keyword
impl Complex {
    // `Self` keyword is an alias for the impl type,
    // which in this case is `Complex`
    // The "constructor function" is named `new` by convention
    fn new(real: f64, imag: f64) -> Self {
        Self {
            real,
            imag,
        }
    }

    // Using the `self` keyword as the first parameter turns
    // this function into a method that can be called with the dot syntax
    fn magnitude(self) -> f64 {
        // `.sqrt()` is a f64 method for taking the sqaure root
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}

fn main() {
    // To call an associated function, use the `T::func` syntax
    let mut comp = Complex::new(3.0, 4.0);
    println!("{:#?}", comp);

    // Calling struct methods
    println!("{}", comp.magnitude());

    // We can update its fields after and get a new result
    comp.real = -5.0;
    comp.imag = 12.0;
    println!("{}", comp.magnitude());
}
