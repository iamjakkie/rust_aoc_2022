use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::rc::Rc;

struct Node {
    stats: i32,
    children: Option<Vec<Node>>,
    parent: Rc<RefCell<Node>>,
}

impl Node {
    fn new() -> Node {
        Node {
            stats: 0,
            children: None,
            parent: Rc::new(RefCell::new(Node::new())),
        }
    }

    fn add_child(&mut self, child: Node) {
        match self.children {
            Some(ref mut children) => children.push(child),
            None => self.children = Some(vec![child]),
        }
    }

    fn add_stats(&mut self, stats: i32) {
        self.stats += stats;
    }

    fn add_parent(&mut self, parent: Node) {
        self.parent = Rc::new(RefCell::new(parent));
    }
}

pub fn solution_a() -> i32 {
    let file = File::open("inputs/7_test").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("line: {:?}", line);
        let mut line = line.unwrap();
        let mut split = line.split(" ");


        let (first, second) = (split.next().unwrap(), split.next().unwrap());

        let path = split.next();

        let mut curr_node = Node::new();

        match path {
            Some(path) => {
                let curr_node =
                match path {
                    ".." => {
                        curr_node = *curr_node.parent.get_mut();
                    },
                    _ => {
                        let mut new_node = Node::new();
                        new_node.add_parent(curr_node);
                        curr_node.add_child(new_node);
                        let curr_node = new_node;
                    }
                };

            }

            //     match (first, second, path) {
            //         ("$", "cd", _) => {
            //
            //
            //             match rel_path {
            //                 ".." => {
            //                     traverse_path.remove(0);
            //                 }
            //                 "." => {}
            //                 _ => {
            //                     traverse_path.push(rel_path);
            //                     path_stats.insert(rel_path.chars().next().unwrap(), 0);
            //                 }
            //             }
            //             println!("cd");
            //             println!("{:?}", traverse_path);
            //         },
            //         ("$", "ls") => {
            //             println!("ls");
            //         },
            //         ("dir", _) => {
            //             println!("dir");
            //         },
            //         (_, _) => {
            //             println!("unknown");
            //             let curr_dir = traverse_path.last().unwrap();
            //             let curr_dir = curr_dir.chars().next().unwrap();
            //             let size = split.next().unwrap();
            //             let size = (size).parse::<u32>().unwrap();
            //             let mut curr_size = path_stats.get_mut(&curr_dir).unwrap();
            //             *curr_size += size;
            //         }
            //     }
            // }
            None => (),
        };



    }
    0
}


