use std::fs::File;

fn get_contents(filename: String) -> std::io::Result<()> {
    let mut file = File::open("test.txt")?;
    let mut contents = String::new();

    Ok(())
}

fn main() {
    
    println!("Hello, world!");
}
