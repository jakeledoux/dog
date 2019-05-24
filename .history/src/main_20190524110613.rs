// For reading files
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
// For making HTTP requests
use reqwest;
// For launch arguments
use std::env;
// For path operations
use std::path::PathBuf;
// For color printing
use colored::*;

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
    let mut silent: bool = false;
    let mut color: bool = true;
    // Get current path
    let path = env::current_dir()?;
    // Collect args
    let args: Vec<String> = env::args().collect();

    // Assemble absolute paths
    let mut filepaths: Vec<PathBuf> = Vec::new();
    for argument in &args[1..] {
        // Help argument
        if argument.to_lowercase() == "-h" || argument.to_lowercase() == "--help"
        || argument.to_lowercase() == "-?" || argument.to_lowercase() == "?" {
            println!("dog | a cat clone by Jake Ledoux, 2019\nUsage:\n\tdog [file]\n\tdog [file1] [file2] [etc...]\n\tdog https://[url]");
            println!("Flags:\n\t-H, --help: Show this help screen\n\t-S, --silent: Silence filenames in output\n\t-N, --nocolor: Leave filepaths gray");
            return Ok(());
        }
        // Silent argument
        else if argument.to_lowercase() == "-s" || argument.to_lowercase() == "--silent" {
            silent = true;
        }
        // No-color argument
        else if argument.to_lowercase() == "-n" || argument.to_lowercase() == "--nocolor" {
            color = false;
        }
        else if argument.starts_with("http") {
            filepaths.push(PathBuf::from(argument));
        }
        else {
            filepaths.push(path.clone());
            filepaths.last_mut().unwrap().push(argument);
        }
    }

    // A bit of fun
    if filepaths.len() == 0 && !silent{
        println!("woof!");
        return Ok(());
    }

    // Get file contents
    for path in filepaths {
        // Print file path
        if !silent {
            if color {
                println!("{}", {format!("[{}]", path.display()).bright_cyan().bold()});
            } else {
                println!("[{}]", path.display());
            }
        }
        // Get contents and print
        let mut contents: String;
        if path.starts_with("http") {
            contents = match reqwest::get(path.to_str().unwrap()) {
                Ok(mut result) => result.text().unwrap(),
                Err(error) => format!("Failed to get contents of {}. {}", path.display(), error)
            }
        }
        else {
            contents = match get_contents(&path) {
                Ok(result) => result,
                Err(error) => format!("Failed to get contents of {}. {}", path.display(), error)
            };
        }
        println!("{}", contents);
    }

    Ok(())
}
