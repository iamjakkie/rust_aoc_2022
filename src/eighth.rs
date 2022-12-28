use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn parse_map() -> Vec<Vec<usize>> {
    let file = File::open("inputs/8_test").unwrap();
    let reader = BufReader::new(file);

    let mut area: Vec<Vec<usize>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let area_line: Vec<usize> = line
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        area.push(area_line);
    }

    area
}

pub fn solution_a() -> i32 {
    let area = parse_map();
    let mut visible = 0;

    let max_x = area.len() - 1;
    let max_y = area[0].len() - 1;

    let mut i_x = 0;
    let mut i_y = 0;
    let mut dir = 0; // 0 = right, 1 = down, 2 = left, 3 = up
    let mut start = [0, 0];
    let stop_point = [(area[0].len() / 2), (area.len() / 2)];

    loop {
        // println!("{} {} {:?} {}", i_x, i_y, start, area[i_x][i_y]);

        if i_x != 0 && i_y != 0 && i_x != max_x && i_y != max_y {
            // let left:Vec<usize> = area[i_x][0..i_y].iter().map(|x| *x).collect();
            // let right:Vec<usize> = area[i_x][i_y+1..=max_y].iter().map(|x| *x).collect();
            // let up = area[0..i_x].iter().map(|x| x[i_y]).collect::<Vec<usize>>();
            // let down = area[i_x+1..=max_x].iter().map(|x| x[i_y]).collect::<Vec<usize>>();

            let left = *area[i_x][0..i_y].iter().max().unwrap();
            let right = *area[i_x][i_y + 1..=max_y].iter().max().unwrap();
            let up = area[0..i_x].iter().map(|x| x[i_y]).max().unwrap();
            let down = area[i_x + 1..=max_x].iter().map(|x| x[i_y]).max().unwrap();
            let curr = area[i_x][i_y];

            // println!("curr: {} {:?} {:?} {:?} {:?}",curr, left, right, up, down);

            if left < curr || right < curr || up < curr || down < curr {
                // println!("Visible");
                visible += 1;
            }
        } else {
            visible += 1;
        }

        if [i_x, i_y] == stop_point {
            break;
        }
        match dir {
            0 => {
                i_y += 1;
                if i_y == area[i_y].len() - 1 - start[0] {
                    dir = 1;
                }
            }
            1 => {
                i_x += 1;
                if i_x == area.len() - 1 - start[1] {
                    dir = 2;
                }
            }
            2 => {
                i_y -= 1;
                if i_y == start[0] {
                    dir = 3;
                }
            }
            3 => {
                if [i_x, i_y] == [start[0] + 1, start[1]] {
                    // println!();
                    i_x = start[0] + 1;
                    i_y = start[1] + 1;
                    start = [i_x, i_y];
                    dir = 0;
                    continue;
                }
                i_x -= 1;
            }
            _ => {
                break;
            }
        }
    }

    visible
}

fn calculate_scenic(
    curr: usize,
    left: Vec<usize>,
    right: Vec<usize>,
    up: Vec<usize>,
    down: Vec<usize>,
) -> i32 {
    let combined = vec![left, right, up, down];

    let mut score = 0;

    for dir in combined {
        let mut curr_score = 0;
        for i in dir {
            println!("{} {}", i, curr);
            curr_score += 1;
            if i >= curr {
                score *= curr_score;
                break;
            }
        }
    }

    score
}

pub fn solution_b() -> i32 {
    let area = parse_map();

    let mut best = 0;

    let max_x = area.len() - 1;
    let max_y = area[0].len() - 1;

    let mut i_x = 0;
    let mut i_y = 0;
    let mut dir = 0; // 0 = right, 1 = down, 2 = left, 3 = up
    let mut start = [0, 0];
    let stop_point = [(area[0].len() / 2), (area.len() / 2)];

    loop {
        let left: Vec<usize> = area[i_x][0..i_y].iter().map(|x| *x).collect();
        let right: Vec<usize> = area[i_x][i_y + 1..=max_y].iter().map(|x| *x).collect();
        let up = area[0..i_x].iter().map(|x| x[i_y]).collect::<Vec<usize>>();
        let down = area[i_x + 1..=max_x]
            .iter()
            .map(|x| x[i_y])
            .collect::<Vec<usize>>();

        let curr = area[i_x][i_y];

        //todo skip edge

        let score = calculate_scenic(curr, left, right, up, down);

        println!("[{},{}]={}: {}", i_x, i_y, curr, score);

        if score > best {
            best = score;
        }

        if [i_x, i_y] == stop_point {
            break;
        }
        match dir {
            0 => {
                i_y += 1;
                if i_y == area[i_y].len() - 1 - start[0] {
                    dir = 1;
                }
            }
            1 => {
                i_x += 1;
                if i_x == area.len() - 1 - start[1] {
                    dir = 2;
                }
            }
            2 => {
                i_y -= 1;
                if i_y == start[0] {
                    dir = 3;
                }
            }
            3 => {
                if [i_x, i_y] == [start[0] + 1, start[1]] {
                    // println!();
                    i_x = start[0] + 1;
                    i_y = start[1] + 1;
                    start = [i_x, i_y];
                    dir = 0;
                    continue;
                }
                i_x -= 1;
            }
            _ => {
                break;
            }
        }
    }

    best
}
