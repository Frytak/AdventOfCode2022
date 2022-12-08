use std::fs;

fn main() {
    let input_file_path = "C:\\Users\\Frytak\\Desktop\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_8\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut grid: Vec<Vec<usize>> = vec![];
    let mut visible_trees: usize = 0;
    let search_pattern: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    for (i, line) in input.lines().enumerate() {
        grid.push(vec![]);
        for char in line.chars() {
            grid[i].push(char::to_digit(char, 10).unwrap() as usize)
        }
    }

    for i in 1..grid.len()-1 {
        for j in 1..grid[i].len()-1 {
            for x in search_pattern {
                if grid[(i as isize + x.0) as usize][(j as isize + x.1) as usize] <= grid[i][j] {
                    visible_trees += 1;
                    break;
                }
            }
        }
    }

    println!()
}
