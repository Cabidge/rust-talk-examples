fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let slice = &mut v[1..4];
    slice[0] = 100;

    println!("{:?}", v);
}

