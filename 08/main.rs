use std::env;
use std::fs;
use std::convert::TryInto;

#[derive(Debug)]
struct Forest {
    w: usize,
    h: usize,
    file: String,
}

impl Forest {
    fn check_up(&self, idx: usize) -> bool {
        let mut n = self.w;
        let bytes = self.file.as_bytes();

        while n <= idx {
            if bytes[idx - n] >= bytes[idx] {
                return false;
            }
            n += self.w;
        }
        true
    }

    fn check_down(&self, idx: usize) -> bool {
        let mut n = idx + self.w;
        let bytes = self.file.as_bytes();

        while n < self.w * self.h {
            if bytes[n] >= bytes[idx] {
                return false;
            }
            n += self.w;
        }
        true
    }

    fn check_left(&self, idx: usize) -> bool {
        let mut n = idx - 1;
        let bytes = self.file.as_bytes();

        loop {
            if bytes[n] >= bytes[idx] {
                return false;
            }
            if n % self.w == 0 { break; }
            n -= 1;
        }
        true
    }

    fn check_right(&self, idx: usize) -> bool {
        let bytes = self.file.as_bytes();
        let x = self.w - (idx % self.w);

        for n in 1..x {
            if bytes[idx + n] >= bytes[idx] {
                return false;
            }
        }
        true
    }

    fn check(&self, idx: usize) -> bool {
        return self.check_up(idx) || self.check_right(idx) || self.check_down(idx) || self.check_left(idx);
    }

    fn count_visible_trees(&self) -> i32 {
        let mut idx = self.w + 1;
        let mut count = 0;

        while idx < self.w * (self.h - 1) - 1 {
            for _ in 0..self.w - 2 {
                if self.check(idx) {
                    dbg!(&idx);
                    dbg!(&self.file.as_bytes()[idx] - 48);
                    count += 1;
                }
                idx += 1;
            }
            idx += 2;
        }
        (count + 2 * (self.w + self.h) - 4).try_into().unwrap()
    }

    fn scenic_score_up(&self, idx: usize) -> u32 {
        let mut res = 0;
        let mut n = self.w;
        let bytes = self.file.as_bytes();

        while n <= idx {
            res += 1;
            if bytes[idx - n] >= bytes[idx] {
                return res;
            }
            n += self.w;
        }
        res
    }

    fn scenic_score_down(&self, idx: usize) -> u32 {
        let mut res = 0;
        let mut n = idx + self.w;
        let bytes = self.file.as_bytes();

        while n < self.w * self.h {
            res += 1;
            if bytes[n] >= bytes[idx] {
                return res;
            }
            n += self.w;
        }
        res
    }

    fn scenic_score_left(&self, idx: usize) -> u32 {
        let mut n = idx - 1;
        let bytes = self.file.as_bytes();
        let mut res = 0;

        loop {
            res += 1;
            if bytes[n] >= bytes[idx] {
                return res;
            }
            if n % self.w == 0 { break; }
            n -= 1;
        }
        res
    }

    fn scenic_score_right(&self, idx: usize) -> u32 {
        let bytes = self.file.as_bytes();
        let x = self.w - (idx % self.w);
        let mut res = 0;

        for n in 1..x {
            res += 1;
            if bytes[idx + n] >= bytes[idx] {
                return res;
            }
        }
        res
    }

    fn scenic_score(&self, idx: usize) -> u32 {
        let u = self.scenic_score_up(idx);
        let d = self.scenic_score_down(idx);
        let l = self.scenic_score_left(idx);
        let r = self.scenic_score_right(idx);
        u * d * l * r
    }

    fn find_max_score(&self) -> u32 {
        let mut idx = self.w + 1;
        let mut max = 0;

        while idx < self.w * (self.h - 1) - 1 {
            for _ in 0..self.w - 2 {
                let x = self.scenic_score(idx);
                if x > max {
                    max = x;
                }
                idx += 1;
            }
            idx += 2;
        }
        max
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            println!("Try passing the input file path!");
        }
        2 => {
            if let Ok(file) = fs::read_to_string(&args[1]) {
                let h = file.matches("\n").count() - 1;
                let w = file.find("\n").unwrap();
                let file = file.replace("\n", "");

                let forest = Forest {
                    w: w,
                    h: h,
                    file: file,
                };
                let res = forest.find_max_score();
                println!("Solution is: {}", res);
            } else {
                println!("Invalid file name: {}", &args[1]);
            }
        }
        _ => {
            println!("Invalid number of arguments.");
        }
    }
}
