use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

//todo implement wasm vizualization

pub fn solution_a(part: char) -> String {
    let file = File::open("inputs/5").unwrap();
    let reader = BufReader::new(file);

    let mut boxes: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        match line {
            a if a.is_empty() => {
                println!("{:?}", boxes);
            }
            a if a.starts_with(" 1") => continue,
            a if a.starts_with("move") => {
                match part {
                    'a' => {
                        let mut split = a.split(" ");

                        let (number, from, to) = (
                            split.nth(1).unwrap(),
                            split.nth(1).unwrap(),
                            split.nth(1).unwrap(),
                        );
                        for _ in 0..number.parse::<i32>().unwrap() {
                            let mut from = from.parse::<usize>().unwrap();
                            let mut to = to.parse::<usize>().unwrap();
                            let mut to_move = boxes[from - 1].remove(0);
                            boxes[to - 1].insert(0, to_move);
                        }
                    }
                    'b' => {
                        let mut split = a.split(" ");

                        let (number, from, to) = (
                            split.nth(1).unwrap(),
                            split.nth(1).unwrap(),
                            split.nth(1).unwrap(),
                        );
                        for i in 0..number.parse::<i32>().unwrap() {
                            let mut from = from.parse::<usize>().unwrap();
                            let mut to = to.parse::<usize>().unwrap();
                            let mut to_move = boxes[from - 1].remove(0);
                            boxes[to - 1].insert(i as usize, to_move);
                            // boxes[to-1].push(to_move);
                        }
                    }
                    _ => panic!("Invalid part"),
                }
            }
            _ => {
                let len = line.len();
                let mut i = 1;
                let mut cnt = 0;
                while i < len {
                    let potential_box = line[i..i + 1].to_string();
                    if (cnt + 1) > boxes.len() {
                        boxes.push(Vec::new());
                    }
                    if potential_box != " " {
                        boxes[cnt].push(line.chars().nth(i).unwrap());
                    }
                    i += 4;
                    cnt += 1;
                }
            }
        }
    }
    let mut result = String::new();
    for stack in boxes {
        result.push(stack[0]);
    }

    return result;
}
