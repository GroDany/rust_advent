use std::env;
use std::fs;

// https://adventofcode.com/2022/day/5
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Try passing the input file path!");
        }
        2 => {
            if let Ok(file) = fs::read_to_string(&args[1]) {
                for w in file.chars().collect::<Vec<char>>().windows(4) {
                    dbg!(w);
                }
            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}
