fn main() {
    let arr = [1, 2, 3];

    // Create an iterator from an array
    let mut iter = arr.iter();
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());
    println!("{:?}", iter.next());

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
    // You can create a function without defining a name by creating a closure
    (0..10).map(square)
        .for_each(
            |n: i32| {
                println!("{}", n);
            }
        );

    // Closures don't always need type definitions and braces can be removed
    // if it has only one expression
    (0..10).map(|n| n * n)
        .for_each(|n| println!("{}", n));
}
