use indexmap::IndexSet;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn solution_a() -> i32 {
    let file = File::open("inputs/9").unwrap();
    let reader = BufReader::new(file);

    let mut visited: Vec<[i32; 2]> = Vec::new();
    let mut head_visited: Vec<[i32; 2]> = Vec::new();

    let mut head = [0, 4];

    head_visited.push(head);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");

        let (dir, num) = (split.next().unwrap(), split.next().unwrap());

        let (vertical, opposite, slice) = match (dir, num.parse::<i32>().unwrap()) {
            ("R", n) => (false, false, (0..n).collect::<Vec<i32>>()),
            ("L", n) => (false, true, (0..n).collect::<Vec<i32>>()),
            ("U", n) => (true, true, (0..n).collect::<Vec<i32>>()),
            ("D", n) => (true, false, (0..n).collect::<Vec<i32>>()),
            _ => {
                continue;
            }
        };

        match (vertical, opposite) {
            (true, true) => {
                for _ in slice {
                    head[1] -= 1;
                    head_visited.push(head);
                }
            }
            (true, false) => {
                for _ in slice {
                    head[1] += 1;
                    head_visited.push(head);
                }
            }
            (false, true) => {
                for _ in slice {
                    head[0] -= 1;
                    head_visited.push(head);
                }
            }
            (false, false) => {
                for _ in slice {
                    head[0] += 1;
                    head_visited.push(head);
                }
            }
        }
    }

    visited.push(head_visited[0]);
    let mut tail = head_visited[0];
    let mut last = head_visited[1];
    let mut ind = 2;
    loop {
        if ind >= head_visited.len() {
            break;
        }
        let cell = head_visited[ind];
        let x_diff = cell[0].abs_diff(tail[0]);
        let y_diff = cell[1].abs_diff(tail[1]);
        if (x_diff < 2 && y_diff < 2) {
            last = cell;
            ind += 1;
            continue;
        } else {
            visited.push(tail);
            tail = last;
            last = cell;
            ind += 1;
        }
    }
    visited.push(tail);

    visited.sort();
    visited.dedup();
    visited.len() as i32
}
