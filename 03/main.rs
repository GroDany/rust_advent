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
                let _file = file.split("\n");
                let mut sum = 0;

                for line in _file {
                    let len = line.len();

                    let mut s = String::from(&line[len / 2..]);

                    sum += find_value(&line[..len / 2], &mut s);
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

fn find_value(a: &str, b: &mut str) -> i32 {
    for character in a.chars() {
        if let Some(c) = b.chars().find(|c| *c == character) {
            let offset: i32 = if (c as i32) < 97 { 38 } else { 96 };
            return (c as i32) - offset;
        }
    }

    return 0;
}
