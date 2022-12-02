use std::fs;

fn main() {

    // Get minimum of 50 stars by December 25th
    // Each puzzle grants one star
    // Puzzle input is how much calories each elf is carrying

    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\Day_1\\src\\input.txt";
    
    let input = fs::read_to_string(input_file_path)
        .expect("Should have been able to read the file");

    let mut elfs_calories: Vec<u16> = vec![];

    let mut i = 0;
    for line in input.lines() {
        if line != "\n" {
            i += line.parse::<u16>().unwrap();
        } else {
            elfs_calories.push(i);
        }
    }
}
