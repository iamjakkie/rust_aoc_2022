use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use regex::Regex;


#[derive(Debug)]
struct Dir{
    path: String,
    size: usize,
    parent: usize
}

pub fn solution_a() -> i32 {
    let file = File::open("inputs/7").unwrap();
    let reader = BufReader::new(file);

    let mut dirs:Vec<Dir> = vec![Dir {
        path: "/".to_string(),
        size: 0,
        parent: 0
    }];
    let root = 0;

    let mut curr = root;

    let directory_regex = Regex::new(r"\s*\$ cd (P<target>.*)").unwrap();
    let file_regex = Regex::new(r"\s*(?P<size>\d+)\s+(?P<file>.+)").unwrap();


    for line in reader.lines() {
        match directory_regex.captures(line.unwrap().as_str()) {
            Some(cap) => {
                match &cap["target"] {
                    "/" => curr = root,
                    ".." => curr = dirs[curr].parent,
                    name => {
                        dirs.push(Dir {
                            path: name.to_string(),
                            size: 0,
                            parent: curr
                        });
                        curr = dirs.len() - 1;
                    }
                }
            },
            None => {
                match file_regex.captures(line.unwrap().as_str()) {
                    Some(cap) => {
                        let size = &cap["size"].parse().unwrap();
                        let mut p = curr;
                        loop {
                            dirs[p].size += size;
                            if dirs[p].parent == p {
                                break;
                            }
                            p = dirs[p].parent;
                        }
                    },
                    None => {}
                }
            }
        }
    }

    0
}


