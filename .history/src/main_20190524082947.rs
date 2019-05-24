use std::fs::File;

fn main() {
    let mut file = File::open("test.txt")?;
    println!("Hello, world!");
}
