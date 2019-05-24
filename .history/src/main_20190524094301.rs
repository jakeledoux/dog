// For reading files
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
// For launch arguments
use std::env;
// For path operations
use std::path::PathBuf;

fn get_contents(filename: &PathBuf) -> std::io::Result<String> {
    // Instantiate variables for file, buffered reader, and file contents
    let file = File::open(filename)?; // Question mark is shorthand for try!(), and will return early on Err.
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    // Read contents and return
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> std::io::Result<()> {
    // Get current path
    let path = env::current_dir()?;
    // Collect args
    let args: Vec<String> = env::args().collect();

    // Assemble absolute paths
    let mut filepaths: Vec<PathBuf> = Vec::new();
    for argument in &args[1..] {
        if argument == "-h" {
            Ok()
        }
        filepaths.push(path.clone());
        filepaths.last_mut().unwrap().push(argument);
    }

    for path in filepaths {
        let contents: String = match get_contents(&path) {
            // String::from(path.to_str().unwrap())) {
                Ok(contents) => contents,
                Err(error) => format!("Failed to get contents of {}. {}", path.display(), error)
            };
        println!("{}", contents);
    }

    Ok(())
}
