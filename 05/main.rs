use std::env;
use std::fs;

// https://adventofcode.com/2022/day/4
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Try passing the input file path!");
        }
        2 => {
            if let Ok(file) = fs::read_to_string(&args[1]) {
                let mut file = file.split("\r\n\r\n");
                let boxes = file.next().unwrap();
                // let moves = file.next().unwrap();

                let boxes = boxes.split("\r\n");
                for row in boxes {
                    println!("{}", row);
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
