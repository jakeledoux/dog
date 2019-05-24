use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn get_contents(filename: String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match get_contents(String::from("test.txt")) {
        Ok(contents) => contents,
        
    };
    println!("Hello, world!");
}
