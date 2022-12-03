use std::fs;

fn main() {
    let input_file_path = "C:\\Users\\Frytak\\Desktop\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_3\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");
    
    let mut sum: u32 = 0;
    for line in input.lines() {
        let first_half = line.get(..line.len()/2);
        let second_half = line.get(line.len()/2..);

        // Find similiar character
        let mut found: Option<char> = None;
        for i in second_half.unwrap().chars().into_iter() {
            if first_half.unwrap().contains(i) {
                found = Some(i);
                break;
            }
        }

        // Calculate the value of the character
        if found.unwrap().is_ascii_lowercase() {
            sum += (found.unwrap() as u32) - 96;
        } else if found.unwrap().is_ascii_uppercase() {
            sum += (found.unwrap() as u32) - 38;
        }
    }
    
    println!("The sum of the priorities of those items is eaquel to {}", sum);
}
