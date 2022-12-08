use std::fs;

// Just change to 14 for part two
const BUFFER_LENGHT: usize = 4;

fn main() {
    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_6\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut found_same = false;
    let chars: Vec<_> = input.chars().collect();
    for k in 0..chars.len()-BUFFER_LENGHT-1 {
        let mut four_chars = [' '; BUFFER_LENGHT];
        for i in 0..BUFFER_LENGHT {
            four_chars[i] = chars[i+k];
        }

        for i in 0..BUFFER_LENGHT {
            for j in i+1..BUFFER_LENGHT {
                if four_chars[i] == four_chars[j] {
                    found_same = true;
                    break;
                }
            }

            if found_same {break;}
        }

        if !found_same {
            println!("{} have to be processed before the first start-of-packet marker is detedted!", k+BUFFER_LENGHT);
            break;
        }
        found_same = false;
    }
}
