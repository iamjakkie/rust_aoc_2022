use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;

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

    let mut visited:HashSet<[usize;2]> = HashSet::new();

    let mut head_visited:HashSet<[usize;2]> = HashSet::new();



    let mut pos:[usize;2] = [0,0];

    let mut head = pos;
    let mut tail = pos;

    visited.insert(tail);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");




        let (dir, num) = (split.next().unwrap(), split.next().unwrap());




        let mut slice= [0..0];
        match (dir, num.parse::<usize>().unwrap()) {
            ("R", n) => {
                slice = [pos[0]..n];
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
                slice = [pos[0]..n];
                // for i in [pos[0]..n] {
                //     tail = pos;
                //     pos = head;
                //     head[0] -= 1;
                // }
            },
            ("U", n) => {
                slice = [pos[1]..n];
                // for i in [pos[1]..n] {
                //     tail = pos;
                //     pos = head;
                //     head[1] += 1;
                // }
            },
            ("D", n) => {
                slice = [pos[1]..n];
                // for i in [pos[1]..n] {
                //     tail = pos;
                //     pos = head;
                //     head[1] -= 1;
                // }
            },
            _ => (),
        }

        for _ in slice {
            tail = pos;
            pos = head;
            head[0] += 1;
            head_visited.insert(head);
            let x_diff = head[0].abs_diff(tail[0]);
            let y_diff = head[1].abs_diff(tail[1]);
            if x_diff == 1 && y_diff == 0 {
                continue;
            }
            visited.insert(tail);
        }
    }
    println!("{:?}", head_visited);
    println!("{:?}", visited);

    visited.len() as i32
}