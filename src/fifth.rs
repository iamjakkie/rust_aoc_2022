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
            a if a.is_empty() => { println!("{:?}", boxes); },
            a if a.starts_with(" 1") => continue,
            a if a.starts_with("move") => {
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
                let mut i = 1;
                let mut cnt = 0;
                while i < len {
                    let potential_box = line[i..i+1].to_string();
                    // println!("chunk: {}", potential_box);
                    if (cnt+1) > boxes.len() {
                        boxes.push(Vec::new());
                        // println!("push");
                    }
                    if potential_box!= " " {
                        // println!("{}", potential_box);
                        boxes[cnt].push(line.chars().nth(i).unwrap());
                    }
                    i += 4;
                    cnt += 1;
                }
                // println!("{:?}", boxes);
                // [['N'], [], ['C'], [], [], [], [], ['Q']]
                // let clean = line.trim().replace("[", "").replace("]", "");
                // let clean = clean.split(" ").collect::<Vec<&str>>();

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