fn main() {
    // Rust can often infer types
    let x = 12;

    // But types can also be explicitly declared
    let y: i32 = 12;

    println!("x is {}, and y is {}", x, y);

    // Integer types:
    // signed | unsigned
    // -------+---------
    // i8     | u8
    // i16    | u16
    // i32    | u32
    // i64    | u64
    // i128   | u128
    //
    // Floating point:
    // ---------------
    // f32, f64
    //
    // Other primitives:
    // -----------------
    // bool, char, str

    // Variables cannot change types
    let mut z = 3;
    // This doesn't work!
    //z = "four"
    z += 1;
    println!("{}", z);
}
