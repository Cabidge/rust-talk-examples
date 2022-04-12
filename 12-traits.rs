// Import special built-in traits
// Traits in the ops module allow you to do operator overloading
use std::ops;

#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Self {
            real,
            imag,
        }
    }
}

// To implement a trait, use the `impl...for` statement
// Implementing Add allows using the `+` operator for custom types
impl ops::Add for Complex {
    // Define the resulting type of Add
    // Complex + Complex = Self
    type Output = Self;

    // Required function
    // Self::Output refers to the Output type defined above
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

// Add is a generic trait, so it can be implemented with different types
impl ops::Add<f64> for Complex {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        Self {
            real: self.real + rhs,
            imag: self.imag,
        }
    }
}

// Traits can also be implemented for existing types*
// * This is only possible when the trait or generic type parameter of the trait is
//   a type defined in the current project
impl ops::Add<Complex> for f64 {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            real: self + rhs.real,
            imag: rhs.imag,
        }
    }
}

// This doesn't work!
//impl ops::Add<i32> for f64 { ... }

fn main() {
    let a = Complex::new(3.0, 4.0);
    let b = Complex::new(-8.0, 7.0);

    // Same as `let result = a.add(b);`
    let result = a + b;
    println!("{:#?}", result);

    // Complex + f64
    let result = a + 4.0;
    println!("{:#?}", result);

    // f64 + Complex
    let result = 9.0 + b;
    println!("{:#?}", result);
}