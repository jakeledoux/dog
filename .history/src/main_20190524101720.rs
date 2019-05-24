// For reading files
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
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
            println!("dog | a cat clone by Jake Ledoux, 2019\nUsage:\n\tdog [file]\n\tdog [file1] [file2] [etc...]");
            println!("Flags:\n\t-H, --help: Show this help screen\n\t-S, --silent: Silence filenames in output");
            return Ok(());
        }
        // Silent argument
        else if argument.to_lowercase() == "-s" || argument.to_lowercase() == "--silent" {
            silent = true;
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

    for path in filepaths {
        // Print file path
        if !silent {
            println!("{}", format!("[{}]", path.display()).bright_cyan().bold());
        }
        // Get contents and print
        let contents: String = match get_contents(&path) {
            Ok(contents) => contents,
            Err(error) => format!("Failed to get contents of {}. {}", path.display(), error)
        };
        println!("{}", contents);
    }

    Ok(())
}
