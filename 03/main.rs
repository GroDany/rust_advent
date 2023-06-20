use std::collections::HashSet;
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
                let file: Vec<&str> = file.split("\r\n").collect();
                let mut sum = 0;

                for i in (0..file.len()).step_by(3) {
                    if file[i].is_empty() {
                        break;
                    }
                    let set0: HashSet<char> = file[i].chars().collect();
                    let set1: HashSet<char> = file[i + 1].chars().collect();
                    let set2: HashSet<char> = file[i + 2].chars().collect();

                    let set = set0
                        .iter()
                        .filter(|k| set1.contains(k))
                        .filter(|k| set2.contains(k));
                    for character in set.enumerate() {
                        sum += if (*character.1 as i32) < 97 {
                            (*character.1 as i32) - 38
                        } else {
                            (*character.1 as i32) - 96
                        };
                    }
                }

                println!("Solution is: {}", sum);
            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}

