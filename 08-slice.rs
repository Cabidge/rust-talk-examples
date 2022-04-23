fn main() {
    let mut arr = [1, 2, 3, 4, 5];

    // Create a "slice" of an array by using the `&..[n..m]` syntax
    // The slice ranges from n to m non-inclusive
    let slice = &arr[1..4];

    // Slices are another type of borrow so ownership rules apply

    // Slices function just like an array
    println!("{}", slice[1]);
    println!("{:?}", slice);

    // Creating a mutable slice
    let slice = &mut arr[1..4];
    slice[0] = 100;
    println!("{:?}", slice);
    println!("{:?}", arr);
}
