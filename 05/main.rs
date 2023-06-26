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
                let mut file = file.split("\r\n\r\n");
                let boxes = file.next().unwrap();
                // let moves = file.next().unwrap();

                let boxes = boxes.rsplit("\r\n");
                let mut items = Vec::<char>::new();
                for row in boxes {
                    items.append(
                        &mut row
                            .chars()
                            .collect::<Vec<char>>()
                            .chunks(4)
                            .map(|chunk| chunk[1])
                            .collect::<Vec<char>>(),
                    );
                }

                let mut stacks: Vec<Vec<char>> = Vec::new();
                let mut stacks_number = 0;
                let mut stacks_total = 0;
                for (i, item) in items.into_iter().enumerate() {
                    if item.is_numeric() {
                        stacks_total += 1;
                    } else {
                        if stacks_number < stacks_total {
                            let mut v: Vec<char> = Vec::new();
                            v.push(item);
                            stacks.push(v);
                            stacks_number += 1;
                        } else if item != ' ' {
                            stacks[i % stacks_total].push(item);
                        }
                    }
                }

                println!("{:?}", stacks);
            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}
