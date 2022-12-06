use std::fs;

const LOW_BOUND: u32 = 1;
const HIGH_BOUND: u32 = 3;

fn translate(character: char) -> Option<u32> {
    match character {
        'A' | 'X' => Some(1),
        'B' | 'Y' => Some(2),
        'C' | 'Z' => Some(3),
        _ => None,
    }
}

fn logic(op: u32, me: u32) -> Option<u32> {    
    match me {
        1 => { // Loss
            if op == LOW_BOUND {
                return Some(3);
            }
            Some(op - 1)
        },
        2 => Some(op), // Draw
        3 => { // Win
            if op == HIGH_BOUND {
                return Some(1);
            }
            Some(op + 1)
        },
        _ => None
    }
}

fn main() {
    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_2\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut score: u32 = 0;
    let mut score_part_two: u32 = 0;
    for line in input.lines() {
        let mut chars = line.chars().into_iter();
        let op = translate(chars.nth(0).unwrap());
        let me = translate(chars.last().unwrap());

        // Point for selected shape
        score += me.unwrap();

        // Points for the outcome
        if op.unwrap() == LOW_BOUND && me.unwrap() == HIGH_BOUND { // Loss
        }
        else if (op < me) || (me.unwrap() == LOW_BOUND && op.unwrap() == HIGH_BOUND) { // Win
            score += 6;
        } else if op == me { // Draw
            score += 3;
        }

        // Part Two
        let choice = logic(op.unwrap(), me.unwrap()).unwrap();
        score_part_two += choice;

        if me.unwrap() == 3 {
            score_part_two += 6;
        } else if me.unwrap() == 2 {
            score_part_two += 3;
        }
    }

    println!("My total score is {}", score);
    println!("My total score for the correct strategy would be {}", score_part_two);
}
