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

    // Creating an inclusive range with `n..=m`
    for i in 0..=10 {
        println!("{}", i);
    }

    // Functions are allowed to be declared inside other functions
    // Factorial function created with iterators
    fn factorial(n: u32) -> u32 {
        (1..=n).product()
    }

    let result = factorial(5);
    println!("{}", result);

    // The map method apply a function on every element of an iterator
    // You can create a function without defining a name by creating a closure
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
