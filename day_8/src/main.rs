use std::fs;

// False means it's not visible
#[derive(Debug)]
struct Tree {
    size: usize,
    left: Option<bool>,
    top: Option<bool>,
    right: Option<bool>,
    bottom: Option<bool>,
}

fn main() {
    let input_file_path = "C:\\Users\\Frytak\\Desktop\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_8\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut grid: Vec<Vec<Tree>> = vec![];
    for (i, line) in input.lines().enumerate() {
        grid.push(vec![]);
        for char in line.chars() {
            grid[i].push(Tree {
                size: char::to_digit(char, 10).unwrap() as usize,
                left: None,
                top: None,
                right: None,
                bottom: None,
            })
        }
    }


    let mut highest_seen = 0;
    let grid_width = grid[0].len();
    let grid_height = grid.len();
    // Rigth to left
    for y in 0..grid_height {
        grid[y][0].left = Some(true);
        highest_seen = grid[y][0].size;
        for x in 1..grid_width {
            if (grid[y][x-1].left.unwrap() && grid[y][x-1].size < grid[y][x].size) || highest_seen < grid[y][x].size {
                grid[y][x].left = Some(true);
            } else {
                grid[y][x].left = Some(false);
            }
            if highest_seen < grid[y][x].size {
                highest_seen = grid[y][x].size;
            }
        }
    }

    // Left to right
    for y in 0..grid_height {
        grid[y][grid_width-1].right= Some(true);
        highest_seen = grid[y][grid_width-1].size;
        for x in (0..grid_width-1).rev() {
            if (grid[y][x+1].right.unwrap() && grid[y][x+1].size < grid[y][x].size) || highest_seen < grid[y][x].size {
                grid[y][x].right = Some(true);
            } else {
                grid[y][x].right = Some(false);
            }
            if highest_seen < grid[y][x].size {
                highest_seen = grid[y][x].size;
            }
        }
    }

    // Top to bottom
    for x in 0..grid_width {
        grid[0][x].top = Some(true);
        highest_seen = grid[0][x].size;
        for y in 1..grid_height {
            if (grid[y-1][x].top.unwrap() && grid[y-1][x].size < grid[y][x].size) || highest_seen < grid[y][x].size {
                grid[y][x].top = Some(true);
            } else {
                grid[y][x].top = Some(false);
            }
            if highest_seen < grid[y][x].size {
                highest_seen = grid[y][x].size;
            }
        }
    }

    // Bottom to top
    for x in 0..grid_width {
        grid[grid_height-1][x].bottom = Some(true);
        highest_seen = grid[grid_height-1][x].size;
        //print!("\n{}: {}", x, highest_seen);
        for y in (0..grid_height-1).rev() {
            if (grid[y+1][x].bottom.unwrap() && grid[y+1][x].size < grid[y][x].size) || highest_seen < grid[y][x].size {
                grid[y][x].bottom = Some(true);
            } else {
                grid[y][x].bottom = Some(false);
            }
            if highest_seen < grid[y][x].size {
                highest_seen = grid[y][x].size;
            }
            //print!(" {}", highest_seen);
        }
    }

    // Check how many trees are visible
    let mut sum: usize = 0;
    for y in 0..grid_height {
        //print!("\n{}:", y);
        for x in 0..grid_width {
            //print!(" {}", grid[y][x].bottom.unwrap());
            let c = &grid[y][x];
            if c.left.unwrap() || c.right.unwrap() || c.top.unwrap() || c.bottom.unwrap() {
                sum += 1;
                //println!("{:?}", c);
            }
        }
    }

    // Part two
    let mut best_scenis_score: usize = 0;
    for y in 0..grid_height {
        for x in 0..grid_width {
            let mut left: usize = 0;
            let mut right: usize = 0;
            let mut top: usize = 0;
            let mut bottom: usize = 0;
            if x != 0 {
                for i in (0..x).rev() {
                    left += 1;
                    if grid[y][x].size <= grid[y][i].size {
                        break;
                    }
                }
            }

            if x != grid_width-1 {
                for i in x+1..grid_width {
                    right += 1;
                    if grid[y][x].size <= grid[y][i].size {
                        break;
                    }
                }
            }

            if y != 0 {
                for i in (0..y).rev() {
                    top += 1;
                    if grid[y][x].size <= grid[i][x].size {
                        break;
                    }
                }
            }

            if y != grid_height-1 {
                for i in y+1..grid_height {
                    bottom += 1;
                    if grid[y][x].size <= grid[i][x].size {
                        break;
                    }
                }
            }

            let scenic_score = left * right * top * bottom;
            if scenic_score > best_scenis_score {
                best_scenis_score = scenic_score;
            }

        }
    }

    println!("{:?}", sum);
    println!("{:?}", best_scenis_score);

}
