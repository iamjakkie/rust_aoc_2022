use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_head_path() -> Vec<[i32;2]> {
    let file = File::open("inputs/9_test").unwrap();
    let reader = BufReader::new(file);

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

    head_visited
}

pub fn solution_a() -> i32 {
    let head_visited = get_head_path();
    let mut visited: Vec<[i32; 2]> = Vec::new();

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

pub fn solution_b() -> i32 {
    let head_visited = get_head_path();

    println!("{:?}", head_visited);
    let mut visited: Vec<[i32; 2]> = Vec::new();

    let mut snake:[[i32;2];9] = [[0,4];9];
    let mut last_snake;
    let mut last = head_visited[0];


    for cell in head_visited {
        last_snake = snake[snake.len() - 1];
        let x_diff = cell[0].abs_diff(last_snake[0]);
        let y_diff = cell[1].abs_diff(last_snake[1]);
        // println!("Last: {:?}, curr: {:?}, to_insert: {:?}", last_snake, cell, last);

        if (x_diff < 2 && y_diff < 2) {
            last = cell;
            continue;
        } else {
            visited.push(snake[0]);
            for val in snake.iter().rev() {

            }
            let mut snake_temp:[[i32;2];9] = [[0;2];9];
            snake_temp[0..8].copy_from_slice(&snake[1..]);
            snake_temp[8] = last;
            snake = snake_temp;
            last = cell;
        }
        println!("{:?}", snake);
    }

    0

}