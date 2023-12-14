use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the command line argument is provided
    if args.len() != 2 {
        println!("Usage: {} <input_file>", args[0]);
        std::process::exit(1);
    }
    // Extract the filename from the command line argument
    let file_name = &args[1];

    // Attempt to open the file
    let f = File::open(&file_name)?;
    // Create a buffered reader for reading lines of text
    let rdr = io::BufReader::new(f);

    // Initialize variables to keep track of the current block and its sum
    let mut current_sum = 0;

    // Iterate over each line in the file
    for line in rdr.lines() {
        let line = line?;
        // Check if the line is empty, indicating a new block
        if !line.is_empty() {
            let sum: u32 = process_line(&line);
            current_sum += sum;
            // println!("{}", sum)
            println!("sum: {}, current_sum: {}", sum, current_sum)
        }
    }
    // Process the current block and its sum
    println!("sum is: {}", current_sum);
    Ok(())
}
fn process_line(line: &String) -> u32 {
    let mut tuples: (char, char) = (' ', ' ');
    // Iterate chars and find first number
    println!("{}", line);
    for char in line.chars() {
        match char.is_digit(10) {
            true => {
                tuples.0 = char.clone();
                break;
            }
            false => continue,
        }
    }
    for char in line.chars().rev() {
        match char.is_digit(10) {
            true => {
                tuples.1 = char.clone();
                break;
            }
            false => continue,
        }
    }
    let result_str = format!("{}{}", tuples.0, tuples.1);
    return result_str.parse().unwrap();
}
