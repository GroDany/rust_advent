use std::env;
use std::fs;

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operands: Vec<String>,
    mult: u128,
    true_monkey: usize,
    false_monkey: usize,
    inspected: u128,
}

impl Monkey {
    fn from(monkey: &str) -> Monkey {
        let monkey = monkey.split("\n").collect::<Vec<&str>>();

        let items = monkey[1].replace("  Starting items: ", "");
        let items = items.split(", ");
        let items = items.map(|item| item.parse::<u128>().unwrap()).collect::<Vec<u128>>();

        let operands = monkey[2].replace("  Operation: new = ", "");
        let operands = operands.split(" ").map(|s| String::from(s)).collect::<Vec<String>>();

        let mult = monkey[3].replace("  Test: divisible by ", "").parse::<u128>().unwrap();
        let tm = monkey[4].replace("    If true: throw to monkey ", "").parse::<usize>().unwrap();
        let fm = monkey[5].replace("    If false: throw to monkey ", "").parse::<usize>().unwrap();

        Monkey {
            items: items, 
            operands: operands,
            mult: mult,
            true_monkey: tm,
            false_monkey: fm,
            inspected: 0,
        }
    }

    fn operate(&self, item: u128, m: u128) -> u128 {
        let a = if self.operands[0] == "old" {
            item
        } else {
            self.operands[0].parse::<u128>().unwrap()
        };
        let b = if self.operands[2] == "old" {
            item
        } else {
            self.operands[2].parse::<u128>().unwrap()
        };
        match self.operands[1].as_str() {
            "+" => (a + b) % m,
            _ => (a * b) % m,
        }
    }

    fn process_item(&mut self, m: u128) -> Option<(usize, u128)> {
        if self.items.is_empty() {
            return None;
        }
        self.inspected += 1;
        let item: u128 = self.items.remove(0);
        let item = self.operate(item, m);
        if item % self.mult == 0 {
            Some((self.true_monkey, item))
        } else {
           Some((self.false_monkey, item))
        }
    }

    fn receive_item(&mut self, item: u128) {
        self.items.push(item);
    }
}

fn process(monkeys: &mut Vec<Monkey>, m: u128) {
    let len = monkeys.len();

    for _ in 0..10000 {
        for n in 0..len {
            while let Some(target) = monkeys[n].process_item(m) {
                monkeys[target.0].receive_item(target.1);
            }
        }
    }
}

fn compute(monkeys: &Vec<Monkey>) -> u128 {
    let mut m0 = monkeys[0].inspected;
    let mut m1 = 0;
    let len = monkeys.len();

    for i in 1..len {
        if m0 < monkeys[i].inspected {
            m1 = m0;
            m0 = monkeys[i].inspected;
        } else if m1 < monkeys[i].inspected {
            m1 = monkeys[i].inspected;
        }
    }
    m0 * m1
}

fn compute_big_mult(monkeys: &Vec<Monkey>) -> u128 {
    let mut mult = 1;
    let len = monkeys.len();

    for i in 0..len {
        mult *= monkeys[i].mult;
    }
    mult
}

// https://adventofcode.com/2022/day/10
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Try passing the input file path!");
        }
        2 => {
            if let Ok(file) = fs::read_to_string(&args[1]) {
                let monkey_confs = file.split("\n\n");
                let mut monkeys: Vec<Monkey> = Vec::new();
                for monkey_conf in monkey_confs {
                    if monkey_conf.is_empty() {
                        break;
                    }
                    monkeys.push(Monkey::from(monkey_conf));
                }
                let m = compute_big_mult(&monkeys);
                process(&mut monkeys, m);
                println!("Solution is: {}", compute(&monkeys));
            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}

