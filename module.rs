// Everything in rust is private by default,
// so a `pub` keyword is required if you want to use
// it outside of the current module
pub fn add3(n: i32) -> i32 {
    add(n, 3)
}

// This function won't be visible from outside modules
fn add(lhs: i32, rhs: i32) -> i32 {
    lhs + rhs
}

// Unit tests are often put inside their own sub module literal
// in the same file as what they are testing
#[cfg(test)]
mod tests {
    // Use `use` to import the add functions
    use super::*;

    // Using the `#[test]` macro marks a function as a unit test
    #[test]
    fn add_test() {
        // Use the `assert` macro to define an expected result
        assert!(1 == 1);

        // This fails!
        //assert!(1 != 1);

        // Use `assert_eq` to create assertions of equality
        assert_eq!(add(1, 1), 2);

        // This fails!
        //assert_eq!(add(1, 1), 3);
    }

    // Using the `#[should_panic]` macro defines a test that should fail
    #[test]
    #[should_panic]
    fn should_fail() {
        assert_eq!(add3(3), 0);
    }
}