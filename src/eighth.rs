use std::fs::File;
use std::io::{self, prelude::*, BufReader};



pub fn solution_a() -> i32 {
    let file = File::open("inputs/8_test").unwrap();
    let reader = BufReader::new(file);

    let mut lines = 0;
    let mut cell = 0;
    let mut visible = 0;

    let mut area: Vec<Vec<usize>> = Vec::new();


    for line in reader.lines() {
        let line = line.unwrap();
        let area_line: Vec<usize> = line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect();
        area.push(area_line);
    }

    let mut i_x = 0;
    let mut i_y = 0;
    let mut dir = 0; // 0 = right, 1 = down, 2 = left, 3 = up
    let mut start = [0, 0];
    let stop_point = [(area[0].len()/2) , (area.len()/2)];

    loop {
        print!("{:?} ", area[i_y][i_x]);
        match dir {
            0 => {
                i_x += 1;
                if i_x == area[i_y].len() - (1+start[0]) {
                    dir = 1;
                }
            },
            1 => {
                i_y += 1;
                if i_y == area.len() - (1+start[1])  {
                    dir = 2;
                }
            },
            2 => {
                i_x -= 1;
                if i_x == 0 {
                    dir = 3;
                }
            },
            3 => {
                println!("_{} {} {:?}_", i_x, i_y, start);
                if [i_y, i_x] == stop_point {
                    break;
                }
                if [i_y, i_x] == start {
                    println!();
                    i_y += 1;
                    i_x += 1;
                    start = [i_y, i_x];
                    dir = 0;
                    continue;
                }
                i_y -= 1;
            },
            _ => {
                break;
            }
        }
    }

    /*

    0 0
    0 1
    0 2
    0 3
    0 4
    1 4
    2 4
    3 4
    4 4
    4 3
    4 2

     */

    println!("{:?}", area);
    0
}
