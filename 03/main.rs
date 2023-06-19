use std::env;
use std::fs;

// https://adventofcode.com/2022/day/3
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("My name is 'match_args'. Try passing the input file path!");
        }
        2 => {
            if let Ok(file) = fs::read_to_string(&args[1]) {
                let file = file.split("\n");

            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}

fn calc(val: &str) -> Option<i32> {
    match val {
        "A X" => { Some(3) }
        "A Y" => { Some(4) }
        "A Z" => { Some(8) }
        "B X" => { Some(1) }
        "B Y" => { Some(5) }
        "B Z" => { Some(9) }
        "C X" => { Some(2) }
        "C Y" => { Some(6) }
        "C Z" => { Some(7) }
        _ => { None }
    }
}
