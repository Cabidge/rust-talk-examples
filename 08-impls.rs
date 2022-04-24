#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

// Create associated functions and methods with `impl` keyword
impl Rect {
    // `Self` keyword is an alias for the impl type,
    // which in this case is `Rect`
    // The "constructor function" is named `new` by convention
    fn new(width: u32, height: u32) -> Self {
        // When the field and variable names are the same,
        // there is no need to write `field: value`
        Self { width, height }
    }

    // Using the `self` keyword as the first parameter turns
    // this function into a method that can be called with the dot syntax
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // To call an associated function, use the `<type>::<func>` syntax
    let rect = Rect::new(12, 3);
    println!("{:#?}", rect);

    // Calling struct methods
    println!("{}", rect.area());
}
