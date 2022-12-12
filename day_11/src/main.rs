use std::fs;

const MONKEY_STEP_BY: usize = 7;

struct Operation {
    first: String,
    sign: char,
    second: String,
}

struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: usize,
    if_true: usize,
    if_false: usize,
}

fn main() {
    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_11\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");
    
    for line in input.lines() {
        
    }
}
