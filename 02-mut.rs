fn main() {
    // This doesn't work!
    //let x = 12;
    //x = 3;
    //println!("x is {}", x);

    // Variables are immutable by default
    // To create a mutable binding, you must add the `mut` keyword
    let mut x = 12;
    println!("x is {}", x);

    x = 3;
    println!("x is now {}", x);
}
