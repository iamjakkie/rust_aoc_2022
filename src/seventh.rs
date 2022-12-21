use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
//
// #[derive(Clone)]
// struct Node {
//     stats: i32,
//     children: Option<RcVec<Node>>,
//     parent: Option<Box<Node>>
// }
//
// impl Node {
//     fn new() -> Node {
//         Node {
//             stats: 0,
//             children: None,
//             parent: None,
//         }
//     }
//
//     fn add_child(&mut self, child: Node) {
//         match self.children {
//             Some(ref mut children) => children.push(child),
//             None => self.children = Some(vec![child]),
//         }
//     }
//
//     fn add_stats(&mut self, stats: i32) {
//         self.stats += stats;
//     }
//
//     fn add_parent(&mut self, parent: Node) {
//         self.parent = Some(Box::new(parent));
//     }
// }

pub fn solution_a() -> i32 {
    let file = File::open("inputs/7").unwrap();
    let reader = BufReader::new(file);

    let mut curr_node:String = String::new();
    let mut visited:Vec<&str> = vec![""];
    let mut sizes:HashMap<&String, i32> = HashMap::new();


    for line in reader.lines() {
        println!("line: {:?}", line);
        let mut line = line.unwrap();
        let mut split = line.split(" ");


        let (first, second) = (split.next().unwrap(), split.next().unwrap());

        let path = split.next();


        match path {
            Some(path) => {
                let path = String::from(path);
                match path.as_str() {
                    "/" => {
                        curr_node = path;
                    }
                    ".." => {
                        curr_node = visited.last_mut().unwrap().parse().unwrap();
                    },
                    _ => {
                        curr_node = path;
                        visited.push(&*curr_node);
                    }
                };
            },
            None => {
                let val = first.parse::<i32>();
                match val {
                    Ok(val) => {
                        for i in 0..visited.len() {
                            let k = String::from(visited[i]);
                            let mut size = sizes.get_mut(&k);
                            match size {
                                Some(size) => {
                                    *size += val;
                                },
                                None => {
                                    let k = String::from(visited[i]).clone();
                                    sizes.insert(&k, val);
                                }
                            }
                        }
                    },
                    Err(_) => {
                        println!("Error parsing val: {:?}", second);
                    }
                }
            },
        };

    }

    let mut size = 0;

    for i in sizes.values() {
        if i < &100000 {
            size += i;
        }
    }

    size
}


