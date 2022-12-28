use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Dir {
    path: String,
    size: usize,
    parent: usize,
}

fn get_dirs() -> Vec<Dir> {
    let file = File::open("inputs/7").unwrap();
    let reader = BufReader::new(file);

    let mut dirs: Vec<Dir> = vec![Dir {
        path: "/".to_string(),
        size: 0,
        parent: 0,
    }];
    let root = 0;

    let mut curr = root;

    let directory_regex = Regex::new(r"\s*\$ cd (?P<target>.*)").unwrap();
    let file_regex = Regex::new(r"\s*(?P<size>\d+)\s+(?P<file>.+)").unwrap();

    for line in reader.lines() {
        let str = line.unwrap();
        let str = str.as_str();
        if let Some(cap) = directory_regex.captures(str) {
            match &cap["target"] {
                "/" => curr = root,
                ".." => curr = dirs[curr].parent,
                name => {
                    dirs.push(Dir {
                        path: name.to_string(),
                        size: 0,
                        parent: curr,
                    });
                    curr = dirs.len() - 1;
                }
            }
        } else if let Some(cap) = file_regex.captures(str) {
            let size = &cap["size"].parse().unwrap();
            let mut p = curr;
            loop {
                dirs[p].size += size;
                if dirs[p].parent == p {
                    break;
                }
                p = dirs[p].parent;
            }
        }
    }
    dirs
}

pub fn solution_a() -> usize {
    let dirs = get_dirs();
    dirs.iter()
        .map(|dir| dir.size)
        .filter(|size| *size <= 100000)
        .sum()
}

pub fn solution_b() -> usize {
    let dirs = get_dirs();

    let space = 70000000 - dirs[0].size;

    let lacking = (30000000 - space);
    match lacking {
        0 => 0,
        _ => dirs
            .iter()
            .map(|dir| dir.size)
            .filter(|size| *size > lacking)
            .min()
            .unwrap(),
    }
}
