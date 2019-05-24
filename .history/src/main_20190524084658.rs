// For reading files
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
// For launch arguments
use std::env;

fn get_contents(filename: String) -> std::io::Result<String> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let contents: String = match get_contents(String::from("test.txt")) {
        Ok(contents) => contents,
        Err(_) => String::from("Failed to get contents.")
    };
    println!("{}", contents);
}
