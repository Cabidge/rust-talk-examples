fn main() {
    let arr = [1, 2, 3];

    // Create an iterator from an array
    let mut iter = arr.iter();

    // Iterators have a `next` function which returns an Option value
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

    // Find the sum of all numbers in an iterator with the `sum` method
    let result: i32 = arr.iter().sum();
    println!("{}", result);

    // Loop over iterators with `for..in`
    for i in arr.iter() {
        println!("{}", i);
    }

    // Creating a range iterator with `n..m`
    // Goes from n to m non-inclusive
    for i in 0..10 {
        println!("{}", i);
    }

    // Functions are allowed to be declared inside other functions
    fn square(n: i32) -> i32 {
        n * n
    }

    // The map method apply a function on every element of an iterator
    for i in (0..10).map(square) {
        println!("{}", i);
    }

    // You can create a function without defining its name by creating a closure
    (0..10).map(|n| n * n)
        .for_each(|n| println!("{}", n));
}
