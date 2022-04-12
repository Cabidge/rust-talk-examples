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
        if i == 10 {
            break;
        }
        println!("{}", i);
        i += 1;
    }

    // Loops can also return values with break
    let mut i = 1;
    let result = loop {
        if i > 100 {
            break i;
        }

        i *= 2;
    };
    println!("result is {}", result);

    // While loops exist
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    // Iterate over Iterators with `for..in`
    // More on Iterators later...
    let arr = [1, 2, 3, 4];
    for x in arr {
        println!("{}", x);
    }

    // Iterate over a specific range with...ranges
    // 0 to 10 non-inclusive
    for i in 0..10 {
        println!("{}", i);
    }
}
