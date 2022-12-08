use std::fs;

#[derive(Debug, Clone)]
struct Directory {
    size: usize,
    parent_id: Option<usize>,
}

impl Directory {
    fn main() -> Directory {
        Directory {
            size: 0,
            parent_id: None,
        }
    }
}

fn main() {
    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_7\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut directories: Vec<Directory> = vec![Directory::main()];
    let mut current_directory: usize = 0;
    let mut sum: usize = 0;

    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect();

        // Check if it's a command or a file
        match chars[0] {
            '$' => {
                // If it's the "cd" command...
                if chars[2] == 'c' {
                    // Check what does it do and execute
                    match chars[5] {
                        '/' => {current_directory = 0;}
                        '.' => {current_directory = directories[current_directory].parent_id.unwrap_or(0)}
                        'a'..='z' => {
                            // Adds the directory to the list
                            let mut name = String::from("");
                            name.extend(line.split(' ').collect::<Vec<_>>()[2].chars().into_iter());
                            directories.push(
                                Directory {
                                    size: 0,
                                    parent_id: Some(current_directory),
                                }
                            );
                            current_directory = directories.len()-1;
                        }
                        _ => {}
                    }
                }
            }
            // If it's a file add its' size to the current directory
            '0'..='9' => {
                let mut size = String::from("");
                size.extend(line.split(' ').collect::<Vec<_>>()[0].chars().into_iter());
                directories[current_directory].size += str::parse::<usize>(&size).unwrap();
            }
            _ => {}
        }
    }

    // Update all the directories so they also have size of nested directories
    for i in (0..directories.len()).rev() {
        if directories[i].parent_id == None {continue;}
        let id = directories.clone()[i].parent_id.unwrap();
        directories[id].size += directories[i].size;
    }

    // Get the sum of directories that are at most of 100000 size 
    for i in 0..directories.len() {
        if directories[i].size <= 100000 {
            sum += directories[i].size;
        }
    }

    println!("The sum of directories that are at most of 100000 size is {}", sum);
}
