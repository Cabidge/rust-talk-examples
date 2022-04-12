fn main() {
    // Variables are created with the `let` keyword
    let x = 12;

    // This doesn't work!
    //println!(x);

    // The first argument to println must be a constant string literal
    // To print a variable, the format string must contain `{}`
    // and the variable is passed as the next argument
    println!("{}", x);

    // The `{}` is substituted by the given argument
    println!("The value of x is {}", x);

    // You can pass as many arguments to println as you'd like
    let y = 18;

    println!("{} + {} = {}", x, y, x + y);
}
