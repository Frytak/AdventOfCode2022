use std::{fs, rc::Rc, cell::RefCell};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Option<usize>,
    if_true: Option<usize>,
    if_false: Option<usize>,
    inspection_count: usize,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey { items: vec![], operation: Operation::new(), test: None, if_true: None, if_false: None, inspection_count: 0 }
    }

    fn get_operation_values(&self) -> (Option<usize>, Option<usize>) {
        let mut output = [None, None];

        for (i, segment) in [&self.operation.first, &self.operation.second].into_iter().enumerate() {
            match segment.as_str() {
                "old" => { output[i] = Some(self.items[0]); }
                _ => { output[i] = Some(str::parse::<usize>(segment.as_str()).unwrap()); }
            }
        }

        (output[0], output[1])
    }

    fn perform_operation(&self) -> usize {
        let (first, second) = self.get_operation_values();
        match self.operation.sign.unwrap() {
            '+' => {first.unwrap() + second.unwrap()}
            '*' => {first.unwrap() * second.unwrap()}
            _ => {panic!("Unknown operation sign '{}'", self.operation.sign.unwrap())}
        }
    }

    fn inspect_items(&mut self, monkeys: Vec<Rc<RefCell<Monkey>>>, divider: usize) {
        for item in 0..self.items.len() {
            let mut stress_level = self.perform_operation();
            stress_level %= divider;

            self.items.remove(0);
            self.inspection_count += 1;
            if stress_level % self.test.unwrap() == 0 {
                monkeys[self.if_true.unwrap()].borrow_mut().items.push(stress_level);
            } else {
                monkeys[self.if_false.unwrap()].borrow_mut().items.push(stress_level);
            }

            //println!("Test: {}", stress_level);
        }
    }
}

fn main() {
    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_11\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");
    
    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkey_index: usize = 0;

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

    let m = monkeys.iter().map(|f| {
        return Rc::new(RefCell::new(f.clone()));
    }).collect::<Vec<_>>();

    let mut divider = m[0].borrow().test.unwrap();
    for test in m.clone().into_iter().skip(1) {
        divider *= test.borrow().test.unwrap();
    }
    println!("{}", divider);

    for _ in 0..10000 {
        for monkey in m.clone() {
            monkey.borrow_mut().inspect_items(m.clone(), divider);
        }
    }
    
    let mut most_active = (0, 0);
    for monkey in m.clone() {
        let monkey_insp_count = monkey.borrow().inspection_count;
        if monkey_insp_count > most_active.0 && most_active.0 < most_active.1 {
            most_active.0 = monkey_insp_count;
        } else if monkey_insp_count > most_active.1 {
            most_active.1 = monkey_insp_count;
        }
    }
    println!("Monkey business: {:?}", most_active.0 * most_active.1);
}
