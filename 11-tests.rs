fn main() {
    println!("Compile with `rustc --test`");
}

// `#[test]` macro marks a function as a unit test
#[test]
fn add_test() {
    // Use the `assert` macro to define expected behavior
    assert!(true);

    // This will fail!
    //assert!(false);

    // Use the `assert_eq` macro to define expected equality
    assert_eq!(1 + 1, 2);

    // This will fail!
    //assert_eq!(1 + 1, 3);
}

// `#[should_panic]` defines a test that should fail
#[test]
#[should_panic]
fn pop_test() {
    let mut v = vec![1, 2, 3];
    assert_eq!(v.pop(), None);
}
