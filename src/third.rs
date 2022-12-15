use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

pub fn solution_a() -> i32 {
    let file = File::open("inputs/3").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {

        let line = line.unwrap();
        let (first, second) = line.split_at(line.len()/2);

        let first: HashSet<char> = HashSet::from_iter(first.chars());
        let second: HashSet<char> = HashSet::from_iter(second.chars());

        let common = first
            .intersection(&second)
            .next();

        match common {
            Some(c) => {
                let abs_val = *c as i32;
                let val = if abs_val < 97 { abs_val - 38} else { abs_val - 96};
                sum += val;
            },
            None => continue,
        }
    }

    return sum;
}
