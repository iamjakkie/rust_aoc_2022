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
