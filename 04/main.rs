use std::env;
use std::fmt;
use std::fs;

// https://adventofcode.com/2022/day/4
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Try passing the input file path!");
        }
        2 => {
            if let Ok(file) = fs::read_to_string(&args[1]) {
                let file = file.split("\r\n");
                let mut sum = 0;

                for line in file.enumerate() {
                    if line.1.is_empty() {
                        break;
                    }
                    if let Some((r1, r2)) = line.1.split_once(',') {
                        if let (Some(i1), Some(i2)) = (Intervale::from(r1), Intervale::from(r2)) {
                            if i1.overlaps(&i2) || i2.overlaps(&i1) {
                                sum += 1;
                            }
                        } else {
                            println!("Invalid file");
                            std::process::exit(1);
                        }
                    } else {
                        println!("Invalid file");
                        std::process::exit(1);
                    }
                }

                println!("The solution is: {}", sum);
            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}

struct Intervale {
    range: (i32, i32),
}

impl Intervale {
    fn from(range: &str) -> Option<Intervale> {
        if let Some((r1, r2)) = range.split_once('-') {
            let v1 = r1.parse::<i32>();
            let v2 = r2.parse::<i32>();
            match (v1, v2) {
                (Ok(v1), Ok(v2)) => Some(Intervale { range: (v1, v2) }),
                _ => None,
            }
        } else {
            None
        }
    }

    fn overlaps(&self, other: &Intervale) -> bool {
        if (self.range.0 <= other.range.0 && self.range.1 >= other.range.0)
            || (self.range.0 <= other.range.1 && self.range.1 >= other.range.1)
        {
            return true;
        }
        return false;
    }
}

impl fmt::Display for Intervale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.range.0, self.range.1)
    }
}
