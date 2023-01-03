use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn parse_input() -> HashMap<usize, i32> {
    let file = File::open("inputs/10_test").unwrap();
    let reader = BufReader::new(file);

    // let mut ops = vec![(0,1)];
    let mut ops:HashMap<usize, i32> = HashMap::from([(1,1)]);
    let mut last = (1 as usize,1);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");
        let op = split.next().unwrap();



        match op {
            "noop" => {
                last = (last.0+1, last.1);
                ops.insert(last.0, last.1);
            },
            "addx" => {
                let num = (split.next().unwrap()).parse::<i32>().unwrap();
                last = (last.0+1, last.1);
                ops.insert(last.0, last.1);
                last = (last.0+1, last.1+num);
                ops.insert(last.0, last.1);
            },
            _ => {continue;}
        }


        // println!("{:?}", ops);
    }

    ops
}

pub fn solution_a() -> i32 {
    let ops = parse_input();
    let a = 20*ops.get(&20).unwrap();
    let b = 60*ops.get(&60).unwrap();
    let c = 100*ops.get(&100).unwrap();
    let d = 140*ops.get(&140).unwrap();
    let e = 180*ops.get(&180).unwrap();
    let f = 220*ops.get(&220).unwrap();

    println!("20: {}", a);
    println!("60: {}", b);
    println!("100: {}", c);
    println!("140: {}", d);
    println!("180: {}", e);
    println!("220: {}", f);
    println!("{}", ops.get(&219).unwrap());

    a+b+c+d+e+f
}

pub fn solution_b() -> String{
    const COLS:u16 = 40;
    const ROWS:u16 = 6;

    let mut crt = ['.';COLS as usize*ROWS as usize];

    let ops = parse_input();
    let mut pix = 0;

    let mut ind = 1;

    loop {
        let x = pix % COLS as i32;
        if x >= ops.get(&ind).unwrap() - 1 && x <= ops.get(&ind).unwrap() + 1 {
            // crt[pix as usize] = '#';
            print!("X ");
        } else {
            print!("  ");
        }

        pix += 1;
        if pix % COLS as i32 == 0 {
            println!("");
        }

        ind += 1;
        if ind == ops.len() {
            break;
        }
    }


    for ind in [0..ops.len()] {

    }



    println!("{:?}", crt);

    String::from("Test")
}