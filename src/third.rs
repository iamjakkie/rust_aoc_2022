use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn solution_a() -> i32 {
    let file = File::open("inputs/3").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (first, second) = line.split_at(line.len() / 2);

        let first: HashSet<char> = HashSet::from_iter(first.chars());
        let second: HashSet<char> = HashSet::from_iter(second.chars());

        let common = first.intersection(&second).next();

        match common {
            Some(c) => {
                let abs_val = *c as i32;
                let val = if abs_val < 97 {
                    abs_val - 38
                } else {
                    abs_val - 96
                };
                sum += val;
            }
            None => continue,
        }
    }

    return sum;
}

pub fn solution_b() -> i32 {
    let file = File::open("inputs/3").unwrap();
    let reader = BufReader::new(file);

    let mut cnt = 0;
    let mut sum = 0;
    let mut curr_sum = 0;

    let mut vec: Vec<String> = Vec::new();

    for line in reader.lines() {
        cnt += 1;
        vec.push(line.unwrap());
        cnt += 1;
        if cnt % 3 == 0 {
            let first: HashSet<char> = HashSet::from_iter(vec[0].clone().chars());
            let second: HashSet<char> = HashSet::from_iter(vec[1].clone().chars());
            let third: HashSet<char> = HashSet::from_iter(vec[2].clone().chars());

            let mut common_set = first
                .intersection(&second)
                .cloned()
                .collect::<HashSet<char>>();
            let common = common_set.intersection(&third).next();

            match common {
                Some(c) => {
                    let abs_val = *c as i32;
                    let val = if abs_val < 97 {
                        abs_val - 38
                    } else {
                        abs_val - 96
                    };
                    sum += val;
                }
                None => continue,
            }

            cnt = 0;
            vec = Vec::new();
        }
    }

    return sum;
}
