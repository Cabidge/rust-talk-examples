fn main() {
    // Create an infinite loop with the `loop` keyword
    // Similar to `while true` in other languages
    //loop {
    //    println!("Hey!");
    //}
    // ^ Uncomment me!

    // Exit loops with `break` keyword
    let mut i = 0;
    loop {
        println!("{}", i);
        i += 1;

        if i == 10 {
            break;
        }
    }

    // Loops can also return values with break
    let mut i = 0;
    let result = loop {
        i += 1;

        if i == 10 {
            break i * 2;
        }
    };
    println!("result is {}", result);

    // Iterate over a specific range with ranges
    // 0 to 10 non-inclusive
    for i in 0..10 {
        println!("{}", i);
    }

    // Iterating over Vecs
    let v = vec![1, 2, 3, 4];
    // For loops take ownership of the given value, which will consume the Vec
    //for x in v {
    //    println!("{}", x);
    //}
    // Can't use `v` afterwards

    // To keep ownership of Vec, use the `iter` method to borrow each element
    for x in v.iter() {
        println!("{}", x);
    }
}
