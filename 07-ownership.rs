#![allow(unused_variables, unused_mut)]

fn main() {
    // By default, all values have a singular owner that is
    // responsible for cleaning up the value it goes out of scope

    {
        // The variable `v` is the owner of the Vec object
        let v = vec![1, 2, 3];
    } // Once `v` goes out of scope, it destroys the Vec and it can no longer be accessed

    // Ownership can be transferred, but once transferred,
    // the original owner may not access the original value
    let mut v = vec![1, 2, 3];
    println!("{:?}", v);

    // `v` gives up ownership of the Vec to the function `take`
    take(v);

    // This doesn't work!
    //v.push(4);
    //println!("{:?}", v);

    // Ownership of values can be kept by giving a clone instead of the original value
    let mut v = vec![1, 2, 3];
    take(v.clone());

    // `v` still retains ownership of the original vector
    v.push(4);
    println!("I still have the Vec: {:?}", v);

    // Instead of copying an entire object, we can borrow values instead
    // The `&` means "reference to"
    // References are like snapshots of the current value with which the borrowers
    // can look at the state of the original value, but cannot update it
    borrow(&v);

    // You can also give mutable references `&mut` to allow borrowers to update
    // the original value without giving up ownership
    change(&mut v);

    // However, it is not possible to give a mutable reference to an immutable value
    // This doesn't work!
    //let v = Vec::new();
    //change(&mut v);
}

// A function that takes ownership of a Vec
fn take(v: Vec<i32>) {
    println!("I took the Vec: {:?}", v);
} // The Vec `v` is disposed of at the end of this function

// A function can borrow a value by taking a reference `&` to the value
fn borrow(v: &Vec<i32>) {
    println!("I am borrowing the Vec: {:?}", v);

    // This doesn't work!
    //v.push(4);
} // The Vec does not get disposed of because `borrow` does not own the value

// A function can borrow mutably byt taking a mutable reference `&mut`
fn change(v: &mut Vec<i32>) {
    println!("I am updating the Vec: {:?}", v);
    v.push(4);
} // The Vec does not get disposed of because `change` does not own the value