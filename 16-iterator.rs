fn main() {
    let arr = [1, 2, 3, 4];

    // Create an iterator from an array
    let mut iter = arr.iter();
    println!("{:?}", iter.next());
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

    // Consume an iterator to find the sum of elements
    let sum: i32 = (0..10).sum();
    println!("The sum is {}", sum);

    // Nested function (NOT a closure!)
    fn square(n: i32) -> i32 {
        n * n
    }

    // Apply a function on every element of an iterator with map
    // The map function is a "higher order function" meaning that
    // it is a function that takes another function as a parameter
    let squares = (0..10).map(square);
    for i in squares {
        println!("{}", i);
    }

    // Simplify higher order functions by using lambda expressions (closures)
    (0..10).map(|n| n * n)
        .for_each(|n| println!("{}", n));

    // Create a Vec from iterators with collect
    // Need to specify collect to a Vec
    // Vec<_> means "collect to a Vec, but don't care what kind"
    let even_squares: Vec<_> = (0..100)
        .map(|n| n * n)
        .filter(|n| n % 2 == 0)
        .collect();
    println!("{:?}", even_squares);
}
