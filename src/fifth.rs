use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::BinaryHeap;

pub fn solution_a() -> String {
    let file = File::open("inputs/5").unwrap();
    let reader = BufReader::new(file);

    let mut boxes:Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        match line {
            a if a.is_empty() => continue,
            a if a.starts_with(" 1") => continue,
            a if a.starts_with("move") => {
                println!("{:?}", boxes);
                let mut split = a.split(" ");

                let (number, from, to) = (split.nth(1).unwrap(), split.nth(1).unwrap(), split.nth(1).unwrap());
                // println!("{} {} {}", number, from, to);
                for _ in 0..number.parse::<i32>().unwrap() {
                    let mut from = from.parse::<usize>().unwrap();
                    let mut to = to.parse::<usize>().unwrap();
                    let mut to_move = boxes[from-1].remove(0);
                    boxes[to-1].insert(0, to_move);
                    // println!("{:?}", boxes);
                }
            },
            _ => {
                let len = line.len();
                let mut i = 0;
                while i < len {
                    let potential_box = line[i..i+3].trim().to_string();
                    // println!("chunk: {}", potential_box);
                    // println!("{}", i%3);
                    if (i%3+1) > boxes.len() {
                        boxes.push(Vec::new());
                    }
                    if !potential_box.is_empty(){
                        // println!("{}", potential_box);
                        boxes[i%3].push(line.chars().nth(i+1).unwrap());
                    }
                    i += 4;
                }
                let clean = line.trim().replace("[", "").replace("]", "");
                let clean = clean.split(" ").collect::<Vec<&str>>();

                // println!("clean {:?}", clean);
                // println!("{:?}", boxes);
            }
        }
    }

    let mut result = String::new();

    for stack in boxes {
        result.push(stack[0]);
    }

    return result;
}