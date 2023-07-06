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
                let mut cache = 0;

                for line in file {
                    if cache != 0 {
                        x += cache;
                    }
                    if line.is_empty() { break; }
                    let line = line.split(" ").collect::<Vec<&str>>();
                    match line[0] {
                        "addx" => {
                            cache = line[1].parse::<i32>().unwrap();
                            if cycle % 40 == 0 {
                                print!("\n");
                            }
                            cycle += 1;
                        },
                        _ => cache = 0,
                    }
                    if cycle % 40 == 0 {
                        print!("\n");
                    }
                    cycle += 1;
                }
                println!("Solution is: {}", strength);
            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}

