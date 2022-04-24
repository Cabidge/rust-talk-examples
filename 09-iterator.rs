fn main() {
    let v = vec![1, 2, 3];

    // Create an iterator from a vector
    let iter = v.iter();

    // Loop over iterators with `for..in`
    for i in iter {
        println!("{}", i);
    }

    // Find the sum of all numbers in an iterator with the `sum` method
    let result: i32 = v.iter().sum();
    println!("{}", result);

    // Creating a range iterator with `n..m`
    // Goes from n to m non-inclusive
    for i in 0..10 {
        println!("{}", i);
    }

    // The map method applies a function on every element of an iterator
    for i in (0..10).map(|n| n * n) {
        println!("{}", i);
    }
}
