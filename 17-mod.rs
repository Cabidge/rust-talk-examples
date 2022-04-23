// Writeing `mod <name>;` tells rust to look for a
// file called `<name>.rs` or `<name>/mod.rs`
// and put the contents of the file into the module
mod module;

fn main() {
    // To access something inside a module
    // do `<module>::<value>`
    let result = module::add3(8);
    println!("{}", result);

    // Non-public values are not visible
    // This doesn't work!
    //let result = module::add(1, 2);
    //println!("{}", result);
}