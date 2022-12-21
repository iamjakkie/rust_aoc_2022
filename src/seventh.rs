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
    let file = File::open("inputs/7_test").unwrap();
    let reader = BufReader::new(file);

    let mut curr_node:char = '\t';
    let mut visited:Vec<char> = vec!['/'];
    let mut sizes:HashMap<char, i32> = HashMap::new();


    for line in reader.lines() {
        println!("line: {:?}", line);
        let mut line = line.unwrap();
        let mut split = line.split(" ");


        let (first, second) = (split.next().unwrap(), split.next().unwrap());

        let path = split.next();


        match path {
            Some(path) => {
                match path {
                    "/" => {
                        curr_node = path.parse().unwrap();
                    }
                    ".." => {
                        curr_node = visited.pop().unwrap();
                    },
                    _ => {
                        curr_node = path.parse().unwrap();
                        visited.push(curr_node);
                    }
                };
            },
            None => {
                let val = first.parse::<i32>();
                match val {
                    Ok(val) => {
                        for node in visited.iter() {
                            let mut size = sizes.get_mut(node);
                            match size {
                                Some(size) => {
                                    *size += val;
                                },
                                None => {
                                    sizes.insert(*node, val);
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

        println!("curr_node: {:?}", curr_node);
        println!("sizes: {:?}", sizes);
    }

    let mut size = 0;

    for i in sizes.values() {
        if i < &100000 {
            size += i;
        }
    }

    size
}


