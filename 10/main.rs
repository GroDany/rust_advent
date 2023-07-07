use std::env;
use std::fs;

// https://adventofcode.com/2022/day/10
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Try passing the input file path!");
        }
        2 => {
            if let Ok(file) = fs::read_to_string(&args[1]) {
                let file = file.split("\n");
                let mut cycle = 1;
                let mut x = 1;

                for line in file {
                    if line.is_empty() {
                        break;
                    }
                    let line = line.split(" ").collect::<Vec<&str>>();
                    let acc = if line[0] == "addx" {
                       2
                    } else {
                        1 
                    };
                    for n in 0..acc {
                        if x - 1 <= (cycle - 1) % 40 && (cycle - 1) % 40 <= x + 1 {
                            print!("#");
                        } else {
                            print!(".");
                        }
                        if cycle % 40 == 0 {
                            print!("\n");
                        }
                        cycle += 1;
                        if n == 1 {
                            x += line[1].parse::<i32>().unwrap();
                        }
                    }
                }

                // cache = line[1].parse::<i32>().unwrap();
            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}

