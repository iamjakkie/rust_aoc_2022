use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn solution_a() -> i32 {
    let file = File::open("inputs/1").unwrap();
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut sum = 0;

    for line in reader.lines() {}
    0
}