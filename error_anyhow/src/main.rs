fn main() {
    let number = read_number_file("my_number.txt").unwrap();
    println!("Number from file: {}", number);
}

use std::fs::File;
use anyhow::Error;

fn read_number_file(file_path: &str) -> Result<u32, Error> {
    // Open the file
    // File::open(file_path)?;

    let number1 = std::fs::read_to_string(file_path)?;

    // Parse the string into a u32
    let number = number1.trim().parse::<u32>()?;

    Ok(number)
}