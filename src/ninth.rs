use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn get_head_path() -> Vec<[i32;2]> {
    let file = File::open("inputs/9").unwrap();
    let reader = BufReader::new(file);

    let mut moves: Vec<[i32; 2]> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        for (dir, count) in line.trim().split_once(" "){
            for _ in 0..count.parse().unwrap() {
                match dir {
                    "R" => {moves.push([1,0])},
                    "L" => {moves.push([-1,0])},
                    "U" => {moves.push([0,1])},
                    "D" => {moves.push([0,-1])},
                    _ => {continue;}
                }
            }
        }
    }
    moves
}

fn has_to_move(curr:&[i32;2], neighbour:&[i32;2]) -> bool {
    let x_diff = curr[0].abs_diff(neighbour[0]);
    let y_diff = curr[1].abs_diff(neighbour[1]);

    if (x_diff < 2 && y_diff < 2) {
        false;
    }
    true
}

fn make_moves(moves: Vec<[i32;2]>, len: usize) -> Vec<[i32;2]>{
    let mut snake:Vec<[i32;2]> = vec![[0,0];len];

    let mut visited:Vec<[i32;2]> = Vec::from([[0,0]]);
    for [mx, my] in moves {
        snake[0][0] += mx;
        snake[0][1] += my;
        for i in 1..len {
            let dx = snake[i - 1][0]-snake[i][0] as i32;
            let dy = snake[i - 1][1]-snake[i][1] as i32;
            if dx.abs() < 2 && dy.abs() < 2 { continue; }
            // println!("{}, sig:{}; {}, sig:{}", dx, dx.signum(), dy, dy.signum());
            if dx != 0 { snake[i][0] += dx.signum(); }
            if dy != 0 { snake[i][1] += dy.signum(); }
        }

        visited.push(snake[len - 1]);
    }
    visited
}

pub fn solution_a() -> i32 {
    let head_visited = get_head_path();
    let mut visited = make_moves(head_visited, 2);
    visited.sort();
    visited.dedup();

    visited.len() as i32
}


pub fn solution_b() -> i32 {
    let head_visited = get_head_path();
    let mut visited = make_moves(head_visited, 10);
    visited.sort();
    visited.dedup();

    visited.len() as i32
}