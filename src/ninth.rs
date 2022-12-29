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
    let file = File::open("inputs/1").unwrap();
    let reader = BufReader::new(file);

    let mut visited:HashSet<[usize;2]> = HashSet::new();

    let mut pos:[usize;2] = [0,0];


    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");



        visited.insert(pos);

        let (dir, num) = (split.next().unwrap(), split.next().unwrap());



        match (dir, num.parse::<usize>().unwrap()) {
            ("R", n) => {
                for i in [pos[0]..n] {
                    pos[0] += 1;
                    visited.insert(pos);
                }
            },
            ("L", n) => {
                for i in [pos[0]..n] {
                    pos[0] -= 1;
                }
            },
            ("U", n) => {
                for i in [pos[1]..n] {
                    pos[1] += 1;
                }
            },
            ("D", n) => {
                for i in [pos[1]..n] {
                    pos[1] -= 1;
                }
            },
            _ => (),
        }
    }
    0
}