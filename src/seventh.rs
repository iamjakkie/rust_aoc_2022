use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

//todo implement BTree for the file system statistics

pub fn solution_a() -> i32 {
    let file = File::open("inputs/7_test").unwrap();
    let reader = BufReader::new(file);

    let mut traverse_path:Vec<&str> = Vec::new();
    let mut path_stats:HashMap<char, u32> = HashMap::new();

    for line in reader.lines() {
        println!("line: {:?}", line);
        let mut line = line.unwrap();
        let mut split = line.split(" ");


        let (first, second) = (split.next().unwrap(), split.next().unwrap());

        match (first, second) {
            ("$", "cd") => {
                let mut rel_path = split.next().unwrap();
                match rel_path {
                    ".." => {
                        traverse_path.pop();
                    }
                    "." => {}
                    _ => {
                        traverse_path.push(rel_path);
                        path_stats.insert(rel_path.chars().next().unwrap(), 0);
                    }
                }
                println!("cd");
                println!("{:?}", traverse_path);
            },
            ("$", "ls") => {
                println!("ls");
            },
            ("dir", _) => {
                println!("dir");
            },
            (_, _) => {
                println!("unknown");
                let curr_dir = traverse_path.last().unwrap();
                let curr_dir = curr_dir.chars().next().unwrap();
                let size = split.next().unwrap();
                let size = (size).parse::<u32>().unwrap();
                let mut curr_size = path_stats.get_mut(&curr_dir).unwrap();
                *curr_size += size;
            }
        }

    }
    0
}


