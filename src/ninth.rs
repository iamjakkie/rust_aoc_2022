use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;
use indexmap::IndexSet;

/*

R->U -> skip last R
R->D -> skip last R
L->U -> skip last L
L->D -> skip last L
U->R -> skip last U
U->L -> skip last U
D->R -> skip last D
D->L -> skip last D

 */

pub fn solution_a() -> i32 {
    let file = File::open("inputs/9_test").unwrap();
    let reader = BufReader::new(file);

    // let mut visited:IndexSet<[i32;2]> = IndexSet::new();
    // let mut head_visited:IndexSet<[i32;2]> = IndexSet::new();

    let mut visited:Vec<[i32;2]> = Vec::new();
    let mut head_visited:Vec<[i32;2]> = Vec::new();

    // let mut head_visited:HashSet<[i32;2]> = HashSet::new();



    let mut pos:[i32;2] = [0,0];

    let mut head = pos;
    let mut tail = pos;

    let mut first_move = true;

    head_visited.push(head);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");




        let (dir, num) = (split.next().unwrap(), split.next().unwrap());



        let (vertical, opposite, slice) = match (dir, num.parse::<i32>().unwrap()) {
            ("R", n) => {
                (false, false, (0..n).collect::<Vec<i32>>())
                // for i in [pos[0]..n] {
                //     head[0] += 1;
                //     let x_diff = head[0].abs_diff(tail[0]);
                //     let y_diff = head[1].abs_diff(tail[1]);
                //     if x_diff == 1 && y_diff == 0 {
                //         continue;
                //     }
                //     visited.insert(tail);
                // }
            },
            ("L", n) => {
                (false, true, (0..n).collect::<Vec<i32>>())
                // for i in [pos[0]..n] {
                //     tail = pos;
                //     pos = head;
                //     head[0] -= 1;
                // }
            },
            ("U", n) => {
                (true, false, (0..n).collect::<Vec<i32>>())
                // for i in [pos[1]..n] {
                //     tail = pos;
                //     pos = head;
                //     head[1] += 1;
                // }
            },
            ("D", n) => {
                (true, true, (0..n).collect::<Vec<i32>>())
                // for i in [pos[1]..n] {
                //     tail = pos;
                //     pos = head;
                //     head[1] -= 1;
                // }
            },
            _ => { continue; },
        };

        println!("{} {:?}", dir, slice);

        match (vertical, opposite) {
            (true, true) => {
                for i in slice {
                    head[1] -= 1;
                    head_visited.push(head);
                }
            },
            (true, false) => {
                for i in slice {
                    head[1] += 1;
                    head_visited.push(head);
                }
            },
            (false, true) => {
                for i in slice {
                    head[0] -= 1;
                    head_visited.push(head);
                }
            },
            (false, false) => {
                for i in slice {
                    head[0] += 1;
                    head_visited.push(head);
                }
            },
        }

    }
    println!("{:?}", head_visited);


    visited.push([0,0]);
    let mut last = visited[0];

    for (ind, cell) in head_visited.iter().enumerate() {
        let cell = *cell;
        println!("{:?} {:?}", cell, last);
        if ind == head_visited.len() -2 {
            visited.push(cell);
            break;
        }
        let x_diff = cell[0].abs_diff(last[0]);
        let y_diff = cell[1].abs_diff(last[1]);
        if x_diff == 1 && y_diff == 1 {
            visited.push(cell);
        }
        visited.push(cell);
        last = cell;
    }
    println!("{:?}", visited);

    visited.len() as i32
}