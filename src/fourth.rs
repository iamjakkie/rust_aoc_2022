use std::borrow::Borrow;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;

pub fn solution_a() -> i32 {
    let file = File::open("inputs/4_test").unwrap();
    let reader = BufReader::new(file);

    let mut cnt = 0;


    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(",");
        let (first, second) = (split.next().unwrap(), split.next().unwrap());

        split = first.split("-");
        let (first_start, first_end) = (split.next().unwrap(), split.next().unwrap());

        let first_start: u32 = FromStr::from_str(first_start).unwrap();
        let first_end: u32 = FromStr::from_str(first_end).unwrap();

        split = second.split("-");
        let (second_start, second_end) = (split.next().unwrap(), split.next().unwrap());
        let second_start: u32 = FromStr::from_str(second_start).unwrap();
        let second_end: u32 = FromStr::from_str(second_end).unwrap();

        if (first_start >= second_start) & (first_end <= second_end) |
            (second_start >= first_start) & (second_end <= first_end) {
            cnt+=1;
        }

    }

    return cnt;
}