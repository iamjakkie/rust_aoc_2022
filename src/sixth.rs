use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

pub fn solution(part:char) -> i32 {
    let file = File::open("inputs/6").unwrap();
    let reader = BufReader::new(file);
    let mut cnt = 0;
    let mut unique = 0;

    let mut stop = match part { 'a' => 4, 'b' => 14, _ => 0 };

    for line in reader.lines() {
        let mut markers:Vec<char> = Vec::new();
        let line = line.unwrap();

        for char in line.chars() {
            cnt += 1;
            if markers.contains(&char) {
                let ind = markers.iter().position(|&x| x == char).unwrap();
                if ind < markers.len() {
                    for _ in 0..=ind {
                        markers.remove(0);
                        unique-= 1;
                    }
                    markers.push(char);
                    unique += 1;
                } else {
                    markers.clear();
                    markers.push(char);
                    unique = 1;
                }
            } else{
                unique += 1;
                markers.push(char);
            }
            if unique == stop {
                break;
            }
        }
    }
    cnt
}