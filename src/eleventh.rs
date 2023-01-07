use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u64>,
    operation: String,
    test_divisible_by: u64,
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

fn parse_input() -> Vec<Monkey>{
    let file = File::open("inputs/11").unwrap();
    let reader = BufReader::new(file);
    let mut ind = 0;

    let mut monkeys: Vec<Monkey> = Vec::new();

    let mut id:usize = 0;
    let mut starting_items:Vec<u64> = Vec::new();
    let mut operation = String::new();
    let mut test:u64 = 0;
    let mut true_output:usize = 0;
    let mut false_output:usize = 0;

    for line in reader.lines() {
        let mut line = line.unwrap();
        line = String::from(line.trim());
        ind += 1;
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
                starting_items = items.split(", ").into_iter().map(|x| x.parse::<u64>().unwrap()).collect();
            },
            3 => {
                let mut split = line.split("= old ");
                let (_, op) = (split.next().unwrap(), split.next().unwrap());
                operation = String::from(op);
            },
            4 => {
                let mut split = line.split("by ");
                let (_, num) = (split.next().unwrap(), split.next().unwrap());
                let num = num.trim().parse::<u64>().unwrap();
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
                // println!("id {} op {} test {} items {:?} true {} false {}",
                // id, operation, test, starting_items, true_output, false_output);
                let mut items = vec![];
                starting_items.clone_into(&mut items);
                let monkey = Monkey{
                    id:id,
                    operation: String::from(&operation),
                    test_divisible_by: test,
                    items: items,
                    true_output:true_output,
                    false_output:false_output
                };
                monkeys.push(monkey);
                ind = 0;
            }
        }
    }
    println!("{:?}", monkeys);
    monkeys

}

pub fn solution_a() -> usize {
    let mut monkeys = &mut parse_input();
    let monkeys_cnt = monkeys.len();
    let mut monkey_business = vec![0;monkeys_cnt];
    for rnd in (0..20) {
        for i in (0..monkeys.len()) {
            // let mut monkey = monkeys.get_mut(i).unwrap();
            let mut monkey = &mut monkeys[i].clone();
            let (op, num) = monkey.operation.split_once(" ").unwrap();
            // let mut num = num.chars().next().unwrap();
            monkey_business[i] += monkey.items.len();
            for j in (0..monkey.items.len()) {
                let curr_val = monkey.items.get_mut(j).unwrap().clone();
                let parsed_num = num.parse::<u64>().unwrap_or(curr_val);

                let mut res:u64 = match op {
                    "+" => curr_val + parsed_num,
                    "-" => curr_val - parsed_num,
                    "*" => curr_val * parsed_num,
                    "/" => curr_val / parsed_num,
                    _ => { continue; }
                };
                //todo divide by 3
                res = res / 3;
                let mut target: usize = i;
                // println! {"monkey: {}, item: {}, test: {}{} res: {}", i, curr_val, op, parsed_num, res};

                if res % monkey.test_divisible_by == 0 {
                    // monkeys[monkey.true_output].items.push(res);
                    target = monkey.true_output;
                } else {
                    target = monkey.false_output;
                }

                monkeys[target].items.push(res)
            }
            monkeys[i].items = vec![];


            // println!("{:?}", monkeys);
        }

        // println!("Round: {} = {:?}", rnd, monkeys);

    }

    monkey_business.sort();
    println!("{:?}", monkey_business);

    monkey_business[monkey_business.len()-1] * monkey_business[monkey_business.len()-2]
}