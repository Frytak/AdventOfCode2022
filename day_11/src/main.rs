use std::fs;

#[derive(Debug)]
struct Operation {
    first: String,
    sign: Option<char>,
    second: String,
}

impl Operation {
    fn new() -> Operation {
        Operation { first: String::new(), sign: None, second: String::new() }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Option<usize>,
    if_true: Option<usize>,
    if_false: Option<usize>,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey { items: vec![], operation: Operation::new(), test: None, if_true: None, if_false: None }
    }

    fn get_operation_values<>(&self, old: &mut usize, item_index: usize) -> (Option<&mut usize>, Option<&mut usize>) {
        let mut output = [None, None];
        for (i, item) in [self.operation.first.clone(), self.operation.second.clone()].into_iter().enumerate() {
            match item.as_str() {
                "old" => {
                    old = &mut self.items[item_index];
                    output[i] = Some(old);
                }
                _ => { output[i] = Some(&mut str::parse::<usize>(item.as_str()).unwrap()); }
            }
        }
        (output[0], output[1])
    }

    fn perform_operation(&mut self, old: &mut usize, item_index: usize) -> usize {
        let (first, second) = self.get_operation_values(old, item_index);
        match self.operation.sign.unwrap() {
            '+' => {*first.unwrap() + *second.unwrap()}
            '*' => {*first.unwrap() * *second.unwrap()}
            _ => {panic!("Unknown operation sign '{}'", self.operation.sign.unwrap())}
        }
    }

    fn inspect_items(&mut self, old: &mut usize) {
        for item in 0..self.items.len() {
            let mut stress_level = self.perform_operation(old, item);
            //stress_level /= 3;
            println!("{}", stress_level);
        }
    }
}

fn main() {
    let input_file_path = "C:\\Users\\Frytak\\Desktop\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_11\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");
    
    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkey_index: usize = 0;
    let mut stress_level: usize = 0;

    for line in input.lines() {
        let segments = line.trim().split(' ').collect::<Vec<&str>>();

        if segments[0] == "Monkey" {
            monkey_index += 1;
            monkeys.push(Monkey::new());
        }

        match segments[0] {
            "Starting" => {
                for item in segments.into_iter().skip(2) {
                    monkeys[monkey_index-1].items.push(str::parse::<usize>(item.trim_end_matches(',')).unwrap()); // Read items
                }
            },
            "Operation:" => {
                monkeys[monkey_index-1].operation.first = String::from(segments[3]);
                monkeys[monkey_index-1].operation.sign = Some(str::parse::<char>(segments[4]).unwrap());
                monkeys[monkey_index-1].operation.second = String::from(segments[5]);
            }
            "Test:" => { monkeys[monkey_index-1].test = Some(str::parse::<usize>(segments[3]).unwrap()); }
            "If" => {
                match segments[1] {
                    "true:" => { monkeys[monkey_index-1].if_true = Some(str::parse::<usize>(segments[5]).unwrap()); }
                    "false:" => { monkeys[monkey_index-1].if_false = Some(str::parse::<usize>(segments[5]).unwrap()); }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    for mut monkey in monkeys {
        println!("{:?}", monkey.inspect_items(&mut stress_level));
    }

    //println!("{:?}", monkeys);
}
