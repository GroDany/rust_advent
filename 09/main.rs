use std::env;
use std::fs;
use std::collections::HashSet;

fn move_tail(head: &(i32, i32), tail: &mut (i32, i32), pos: &mut HashSet<(i32, i32)>) {
    let diff = (head.0 - tail.0, head.1 - tail.1);
    if diff.0 == 2 || diff.0 == -2 {
        tail.0 += diff.0 / 2;
        if diff.1 == 1 || diff.1 == -1 {
            tail.1 += diff.1;
        }
        pos.insert((tail.0, tail.1));
        return;    
    }
    if diff.1 == 2 || diff.1 == -2 {
        tail.1 += diff.1 / 2;
        if diff.0 == 1 || diff.0 == -1 {
            tail.0 += diff.0;
        }
        pos.insert((tail.0, tail.1));
        return; 
    }
}

// https://adventofcode.com/2022/day/9
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Try passing the input file path!");
        }
        2 => {
            if let Ok(file) = fs::read_to_string(&args[1]) {
                let mut head: (i32, i32) = (0, 0);
                let mut tail: (i32, i32) = (0, 0);
                let file = file.split("\n");
                let mut pos: HashSet<(i32, i32)> = HashSet::new();
                pos.insert((0, 0));
                for line in file {
                    if line.is_empty() { break; }
                    let line = line.split(" ").collect::<Vec<&str>>();
                    let n = line[1].parse::<i32>().unwrap();
                    match line[0] {
                        "U" => {
                            for _ in 0..n {
                                head.1 += 1;
                                move_tail(&head, &mut tail, &mut pos);
                            }
                        },
                        "D" => {
                            for _ in 0..n {
                                head.1 -= 1;
                                move_tail(&head, &mut tail, &mut pos);
                            }
                        },
                        "L" => {
                            for _ in 0..n {
                                head.0 -= 1;
                                move_tail(&head, &mut tail, &mut pos);
                            }
                        },
                        "R" => {
                            for _ in 0..n {
                                head.0 += 1;
                                move_tail(&head, &mut tail, &mut pos);
                            }
                        },
                        _ => {},
                    }
                }
                dbg!(&pos.len());
            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}

