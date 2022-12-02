use std::fs;

fn translate(character: char) -> Option<u8> {
    match character {
        'A' | 'X' => Some(1),
        'B' | 'Y' => Some(2),
        'C' | 'Z' => Some(3),
        _ => None,
    }
}

fn main() {
    let input_file_path = "C:\\Users\\Frytak\\Desktop\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_2\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut score:u32 = 0;
    for line in input.lines() {
        let mut chars = line.chars().into_iter();
        let mut op = translate(chars.nth(0).unwrap_or(' '));
        let mut me = translate(chars.last().unwrap_or(' '));

        if op == None || me == None {
            continue;
        }

        // Point for selected shape
        score += me.unwrap_or(0) as u32;

        // Points for the outcome
        let low_bound = 1;
        let high_bound = 3;

        // Circle logic!
        if me.unwrap() == low_bound && op.unwrap() == high_bound {
            me = Some(high_bound + 1);
        }
        if op.unwrap() == low_bound && me.unwrap() == high_bound {
            op = Some(high_bound + 1);
        }

        if op < me { // Win
            score += 6;
        } else if op == me { // Draw
            score += 3;
        }
    }

    println!("My total score is {}", score);
}
