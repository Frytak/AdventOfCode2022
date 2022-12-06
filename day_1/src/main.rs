use std::fs;

fn main() {
    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_1\\src\\input.txt";
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
    elfs_calories.sort();
    let mut top_three: u32 = 0;
    for i in 0..3 {
        top_three += elfs_calories[elfs_calories.len()-i-1] 
    }

    println!("Elf carrying the most has {} calories of food!", elfs_calories[elfs_calories.len()-1]);
    println!("Top three Elves are carrying {} calories in total.", top_three);
}
