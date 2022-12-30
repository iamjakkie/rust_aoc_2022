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

    let mut visited:HashSet<[i32;2]> = HashSet::new();

    let mut head_visited:HashSet<[i32;2]> = HashSet::new();



    let mut pos:[i32;2] = [0,0];

    let mut head = pos;
    let mut tail = pos;

    visited.insert(tail);

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
            (true, true) => {},
            (true, false) => {},
            (false, true) => {},
            (false, false) => {},
        }

        // for _ in slice {
        //     tail = pos;
        //     pos = head;
        //     head[0] += 1;
        //     head_visited.insert(head);
        //     let x_diff = head[0].abs_diff(tail[0]);
        //     let y_diff = head[1].abs_diff(tail[1]);
        //     if x_diff == 1 && y_diff == 0 {
        //         continue;
        //     }
        //     visited.insert(tail);
        // }
    }
    println!("{:?}", head_visited);
    println!("{:?}", visited);

    visited.len() as i32
}