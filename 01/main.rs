use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("My name is 'match_args'. Try passing the input file path!");
        }
        2 => {
            if let Ok(file) = fs::read_to_string(&args[1]) {
                let file = file.split("\n");
                let mut max: [i32; 3] = [0, 0, 0];
                let mut sum = 0;
                for (i, mut line) in file.enumerate() {
                    line = line.trim();

                    if line.is_empty() {
                        check_for_max_three(&mut max, sum);
                        sum = 0;
                    } else if let Ok(value) = line.parse::<i32>() {
                        sum += value;
                    } else {
                        println!("Invalid file content: at line {}: '{}'", i + 1, line);
                        std::process::exit(1);
                    }
                }
                println!("The solution is: {}", max[0] + max[1] + max[2]);
            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}

fn check_for_max_three(max: &mut [i32; 3], value: i32) {
    if max[0] < value {
        *max = [value, max[0], max[1]]
    } else if max[0] > value && max[1] < value {
        *max = [max[0], value,  max[1]]
    } else if max[2] < value {
        *max = [max[0],  max[1], value]
    }
}
