use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

pub fn solution_a() -> i32 {
    let file = File::open("inputs/6_test").unwrap();
    let reader = BufReader::new(file);
    let mut cnt = 0;
    let mut unique = 0;


    for line in reader.lines() {
        let mut markers:Vec<char> = Vec::new();
        let line = line.unwrap();

        for char in line.chars() {
            cnt += 1;
            if markers.contains(&char) {
                let ind = markers.iter().position(|&x| x == char).unwrap();
                println!("{} {}", ind, char);
            } else{
                unique += 1;
                markers.push(char);
            }
            if unique == 4 {
                break;
            }
            println!("{:?}" , markers);
        }
    }
    cnt
}