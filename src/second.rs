use std::cmp::Ordering;
use std::collections::HashMap;


struct Shape {
    encrypted: char,
    value: u8
}

let scores = HashMap!

impl Eq for Shape {}

impl PartialEq<Self> for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.encrypted == other.encrypted
    }
}

impl PartialOrd<Self> for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Shape {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

pub fn solution_a() -> i32 {
    let mut map:HashMap<char, u8> = HashMap::new();
    let mut sum = 0;

    for line in include_str!("../inputs/2_test").lines() {
        let mut split = line.split_whitespace();
        let (his, my) = (split.next().unwrap(), split.next().unwrap());
        println!("his: {}, my: {}", his, my);
    }

    return 0;
}