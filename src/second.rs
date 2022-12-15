use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


pub fn solution_a() -> u32 {
    let file = File::open("inputs/2").unwrap();
    let reader = BufReader::new(file);


    let scores: HashMap<String, u32> = HashMap::from([
        (String::from("A X"), 4),
        (String::from("A Y"), 8),
        (String::from("A Z"), 3),
        (String::from("B X"), 1),
        (String::from("B Y"), 5),
        (String::from("B Z"), 9),
        (String::from("C X"), 7),
        (String::from("C Y"), 2),
        (String::from("C Z"), 6)]);

    let mut sum:u32 = 0;

    for line in reader.lines() {

        let line_u = match line {
            Ok(l) => l,
            Err(_) => continue
        };

        let total = *scores.get(&line_u).unwrap();
        sum += total;

    }


    return sum;
}

pub fn solution_b() -> u32 {
    let file = File::open("inputs/2").unwrap();
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;

    for line in reader.lines() {
        let line_u = match line {
            Ok(l) => l,
            Err(_) => continue
        };

        let mut split = line_u.split_whitespace();
        let (left, res) = (split.next().unwrap().chars().next().unwrap(), split.next().unwrap().chars().next().unwrap());

        match (left, res) {
            ('A', 'X') => sum += 3,
            ('A', 'Y') => sum += 4,
            ('A', 'Z') => sum += 8,
            ('B', 'X') => sum += 1,
            ('B', 'Y') => sum += 5,
            ('B', 'Z') => sum += 9,
            ('C', 'X') => sum += 2,
            ('C', 'Y') => sum += 6,
            ('C', 'Z') => sum += 7,
            _ => continue
        }

    }


    return sum;
}