// `#[derive(Debug)]` macro allows formatting via {:?}
#[derive(Debug)]
// Define a struct type
struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    // Instantiating named struct
    let rect = Rect {
        width: 12,
        height: 3,
    };

    // Using {:#?} is pretty debug, which formats
    // values nicely over multiple lines
    println!("{:#?}", i);

    // Access struct fields with `.`
    println!("width: {}, height: {}", rect.width, rect.height);
}
