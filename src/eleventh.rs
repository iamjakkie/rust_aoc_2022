use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<i32>,
    operation: String,
    test_divisible_by: i32,
    true_output: usize,
    false_output: usize,
}

impl Monkey{

}

impl fmt::Display for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"Monkey {}, starting items: {:?}, operation: old {}, divisible by {}, if true to monkey {} if false to monkey {}",
        self.id, self.items, self.operation, self.test_divisible_by, self.true_output, self.false_output)
    }
}

fn parse_input() {
    let file = File::open("inputs/11_test").unwrap();
    let reader = BufReader::new(file);
    let mut ind = 1;

    let mut monkeys: Vec<Monkey> = Vec::new();

    for line in reader.lines() {
        let mut line = line.unwrap();
        line = String::from(line.trim());
        let mut id:usize = 0;
        let mut starting_items:Vec<i32> = Vec::new();
        let mut operation = String::new();
        let mut test:i32 = 0;
        let mut true_output:usize = 0;
        let mut false_output:usize = 0;
        match ind {
            1 => {
                let mut split = line.split(" ");
                let (_, str_id) = (split.next().unwrap(), split.next().unwrap());
                let str_id = str_id.chars().next().unwrap().to_digit(10).unwrap() as usize;
                id = str_id;
            },
            2 => {
                let mut split = line.split(": ");
                let (_, items) = (split.next().unwrap(), split.next().unwrap());
                starting_items = items.split(", ").into_iter().map(|x| x.parse::<i32>().unwrap()).collect();
            },
            3 => {
                let mut split = line.split("= old ");
                let (_, op) = (split.next().unwrap(), split.next().unwrap());
                operation = String::from(op);
            },
            4 => {
                let mut split = line.split("by ");
                let (_, num) = (split.next().unwrap(), split.next().unwrap());
                let num = num.trim().parse::<i32>().unwrap();
                test = num;
            },
            5 => {
                let mut split = line.split("monkey ");
                let (_, monkey_num) = (split.next().unwrap(), split.next().unwrap());
                true_output = monkey_num.parse::<usize>().unwrap();
            },
            6 => {
                let mut split = line.split("monkey ");
                let (_, monkey_num) = (split.next().unwrap(), split.next().unwrap());
                false_output = monkey_num.parse::<usize>().unwrap();
            },
            _ => {
                let monkey = Monkey{
                    id:id,
                    operation:operation,
                    test_divisible_by: test,
                    items:starting_items,
                    true_output:true_output,
                    false_output:false_output
                };
                println!("{:?}", monkey);
                monkeys.push(monkey);
                println!("{:?}", monkeys);
                ind = 0;
            }
        }
        ind += 1;
        println!("{}", line);
    }
}

pub fn solution_a() -> String{
    parse_input();

    String::from("OK")
}