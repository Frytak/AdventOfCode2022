use std::fs;

fn main() {
    let input_file_path = "C:\\Users\\Frytak\\Desktop\\~\\Important\\Programming Projects\\AdventOfCode2022\\Day_1\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut elfs_calories: Vec<u32> = vec![];

    let mut current_elf = 0;
    for line in input.lines() {
        if line.len() > 3 {
            current_elf += line.parse::<u32>().unwrap();
        } else {
            elfs_calories.push(current_elf);
            current_elf = 0;
        }
    }

    println!("Elf carrying the most has {} calories of food!", elfs_calories.iter().max().unwrap());
}
