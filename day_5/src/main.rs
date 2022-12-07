use std::{fs, vec};

fn main() {
    let input_file_path = "C:\\Users\\Frytak\\Desktop\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_5\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut crates: Vec<Vec<char>> = vec![];

    // Create the correct amount of columns
    for line in input.lines() {
        if line.chars().nth(1).unwrap() == '1' {
            for _ in 0..line.chars().filter(|x| !x.is_whitespace()).count() {
                crates.push(vec![]);
            }
            break;
        }
    }

    // Get the crates arrangement
    let mut crate_index: usize = 0;

    for line in input.lines() {
        if line.contains("1") {break;}

        // Assign each crate to it's column
        for char in line.chars().skip(1).step_by(4) {
            crate_index += 1;
            if char::is_whitespace(char) {continue;}
            crates[crate_index - 1].push(char);
        }
        crate_index = 0;
    }
    

    // Revert all the vectors
    for i in 0..crates.len() {
        crates[i].reverse();
    }

    // Make the moves
    for line in input.lines().skip(crates.len()+1) {
        let text: Vec<_> = line.split(' ').collect();
        
        let move_nums = str::parse::<usize>(text[1]).unwrap() - 1;
        let from = str::parse::<usize>(text[3]).unwrap() - 1;
        let to = str::parse::<usize>(text[5]).unwrap() - 1;
        let mut appending:Vec<char> = vec![];

        for _ in 0..=move_nums {
            let mut_crates = crates[from][crates[from].len() - 1];
            appending.push(mut_crates);
            crates[from].pop();
        }

        // To get the part two answer comment this out:
        //appending.reverse();
        
        crates[to].append(&mut appending);
    }

    // Get the top crates
    let mut top = String::from("");
    for i in 0..crates.len() {
        if crates[i].len() == 0 {continue;}
        top.push(crates[i][crates[i].len() - 1])
    }

    println!("The crates at the top will be: {:?}", top);
}
