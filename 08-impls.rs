#[derive(Debug)]
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
    fn magnitude(&self) -> f64 {
        f64::sqrt(self.real * self.real + self.imag * self.imag)
    }
}

fn main() {
    // To call an associated function, use the `<type>::<func>` syntax
    let comp = Complex::new(3.0, 4.0);
    println!("{:#?}", comp);

    // Calling struct methods
    println!("{}", comp.magnitude());
}
