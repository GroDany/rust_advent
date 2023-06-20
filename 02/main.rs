use std::env;
use std::fs;

// https://adventofcode.com/2022/day/2
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Try passing the input file path!");
        }
        2 => {
            if let Ok(file) = fs::read_to_string(&args[1]) {
                let file = file.split("\n");
                let mut sum = 0;
                for (i, mut line) in file.enumerate() {
                    line = line.trim();

                    if line.is_empty() {
                        println!("The solution is: {}", sum);
                    } else {
                        if let Some(result) = calc(line) {
                            sum += result;
                        } else {
                            println!("Invalid file content: at line {}: '{}'", i + 1, line);
                        }
                    }
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
