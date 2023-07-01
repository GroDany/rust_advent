use std::env;
use std::fs;

#[derive(Debug)]
struct Item {
    name: String,
    parent: usize,
    size: i32,
}

impl Item {
    fn from(name: String, parent: usize, size: i32) -> Item {
        Item { name: name, parent: parent, size: size }
    }
}

fn command(params: &[&str], items: &mut Vec<Item>, parent: usize) -> usize {
    match params[0] {
        "cd" => {
            if !params[1].eq("..") {
                items.push(Item::from(String::from(params[1]), parent, 0));
                return items.len() - 1;
            } else {
                let parent_parent_idx = items[parent].parent;
                items[parent_parent_idx].size += items[parent].size;
                return items[parent].parent;
            }
        }
        _ => {},
    }
    parent
}

fn item(params: &[&str], items: &mut Vec<Item>, parent: usize) {
    if !params[0].eq("dir") {
        let size = params[0].parse::<i32>().unwrap();
        items[parent].size += size;
    }
}

fn compute(items: &Vec<Item>) {
    let available = 70000000 - items[0].size;
    let mut idx = 0;
    let tree = items.into_iter();
    for folder in tree.enumerate() {
        let new_available = available + folder.1.size;
        if new_available >= 30000000 && folder.1.size < items[idx].size {
            idx = folder.0;
        }
    }
    println!("Solution:");
    dbg!(&items[idx]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Try passing the input file path!");
        }
        2 => {
            if let Ok(file) = fs::read_to_string(&args[1]) {
                let mut items: Vec<Item> = Vec::new();
                let mut parent: usize = 0;
                let file = file.split("\n");
                for line in file {
                    let line = line.split(" ").collect::<Vec<&str>>();
                    match line[0] {
                        "$" => parent = command(&line[1..], &mut items, parent),
                        "" => break,
                        _ => item(&line[..], &mut items, parent),
                    }
                }
                let parent_parent_idx = items[parent].parent;
                items[parent_parent_idx].size += items[parent].size;
                // dbg!(&items);
                compute(&items);
            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}
