use std::cmp::Ordering;
use std::collections::HashMap;


struct Shape {
    encrypted: char,
    value: u32
}



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

pub fn solution_a() -> u32 {
    let scores: HashMap<char, u32> = HashMap::from([
        ('A', 1),
        ('B', 2),
        ('C', 3),
        ('X', 1),
        ('Y', 2),
        ('Z', 3)]);

    let mut sum:u32 = 0;

    for line in include_str!("../inputs/2_test").lines() {
        let mut split = line.split_whitespace();
        let (his, my) = (split.next().unwrap().chars().next().unwrap(), split.next().unwrap().chars().next().unwrap());
        println!("his: {}, my: {}", his, my);
        let his = Shape{ encrypted: his, value: *scores.get(&his).unwrap() };
        let my_value = *scores.get(&my).unwrap();
        let my = Shape{ encrypted: my, value: my_value };

        sum += my_value;

        match my.cmp(&his) {
            Ordering::Greater => sum += 6,
            Ordering::Equal => sum += 3,
            _ => continue
        }

    }

    return sum;
}