use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

pub fn solution_a() -> i32 {
    let file = File::open("inputs/3").unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0;

    for line in reader.lines() {
        let mut first_set = HashSet::new();
        let mut second_set = HashSet::new();

        let line = line.unwrap();
        let (first, second) = line.split_at(line.len()/2);

        for i in 0..first.len(){
            let first_val = first.chars().nth(i).unwrap();
            let second_val = second.chars().nth(i).unwrap();
            if second_set.contains(&first_val) {
                let abs_val = first_val as i32;
                let val = if abs_val < 97 { abs_val - 38} else { abs_val - 96};
                sum += val;
                break;
            }
            if first_set.contains(&second_val) {
                let abs_val = second_val as i32;
                let val = if abs_val < 97 { abs_val - 38} else { abs_val - 96};
                sum += val;
                break;
            }
            first_set.insert(first_val);
            second_set.insert(second_val);
        }
    }

    return sum;
}
