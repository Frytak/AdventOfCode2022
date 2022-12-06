use std::fs;

fn main() {
    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_3\\src\\input.txt";
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

    let mut sum_part_two: u32 = 0;
    let mut lines:Vec<&str> = input.lines().collect();
    for (i, _) in lines.iter().enumerate().step_by(3) {
        let first = lines[i];
        let second = lines[i+1];
        let third = lines[i+2];

        let mut common: Vec<char> = vec![];
        for i in first.chars().into_iter() {
            if second.contains(i) {
                common.push(i);
            }
        }
        println!("{:?}", common);

        for i in third.chars().into_iter() {
            if common.contains(&i) {
                if i.is_ascii_lowercase() {
                    sum_part_two += (i as u32) - 96;
                    println!("Char: {}\nPoints: {}", i, (i as u32) - 96);
                } else if i.is_ascii_uppercase() {
                    sum_part_two += (i as u32) - 38;
                    println!("Char: {}\nPoints: {}", i, (i as u32) - 38);
                }
                break;
            }
        }
    }
    
    println!("The sum of the priorities of those items is equal to {}", sum);
    println!("The sum of the priorities of those items is equal to {}", sum_part_two);
}
