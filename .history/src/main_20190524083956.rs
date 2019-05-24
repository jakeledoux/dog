use std::fs::File;
use std::io::BufReader;

fn get_contents(filename: String) -> std::io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(())
}

fn main() {
    
    println!("Hello, world!");
}
