use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn solution() -> i32 {
    let file = File::open("inputs/1").unwrap();
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut sum = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            if sum > max {
                max = sum;
            }
            sum = 0;
            continue;
        }
        sum += line.parse::<i32>().unwrap();
    }

    return max;
}