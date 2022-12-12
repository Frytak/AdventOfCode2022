use std::fs;

const FIRST_CYCLE: usize = 20;
const NEXT_CYCLES: usize = 40;
const NUMBER_OF_CYCLES: usize = 6;

const MONITOR_WIDTH: usize = 40;
const MONITOR_HEIGHT: usize = 6;

fn calc_signal_strength(cycle: &usize, x: &isize) -> Option<isize> {
    for y in 0..NUMBER_OF_CYCLES {
        let nth_cycle = FIRST_CYCLE + NEXT_CYCLES * y;
        if nth_cycle == *cycle {
            return Some(*x * (nth_cycle as isize));
        }
    }
    None
}

fn cycles(cycle: &mut usize, x: &isize, signal_strengths: &mut Vec<isize>, monitor: &mut [[char; MONITOR_WIDTH]; MONITOR_HEIGHT], crt_count: &mut usize, num: usize) {
    for _ in 0..num {
        *cycle += 1;

        let y =
        if *cycle <= 40 { 1 }
        else if *cycle >= 40 && *cycle <= 80 { 2 }
        else if *cycle >= 80 && *cycle <= 120 { 3 }
        else if *cycle >= 120 && *cycle <= 160 { 4 }
        else if *cycle >= 160 && *cycle <= 200 { 5 }
        else if *cycle >= 200 && *cycle <= 240 { 6 }
        else { 100 };

        if *x == (*crt_count%MONITOR_WIDTH) as isize || *x-1 == (*crt_count%MONITOR_WIDTH) as isize || *x+1 == (*crt_count%MONITOR_WIDTH) as isize {
            monitor[y-1 as usize][*crt_count%MONITOR_WIDTH] = '#';
        }

        *crt_count += 1;

        match calc_signal_strength(&cycle, &x) {
            Some(x) => {signal_strengths.push(x);},
            None => {}
        }
    }
}

fn main() {
    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_10\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut cycle: usize = 0;
    let mut x = 1;
    let mut signal_strengths: Vec<isize> = vec![];
    let mut monitor = [['.'; MONITOR_WIDTH]; MONITOR_HEIGHT];
    let mut crt_count: usize = 0;

    for (i, line) in input.lines().enumerate() {
        let line_segments = line.split(' ').collect::<Vec<&str>>();
        match line_segments[0] {
            "noop" => {cycles(&mut cycle, &x, &mut signal_strengths, &mut monitor, &mut crt_count, 1);},
            "addx" => {
                let v = str::parse::<isize>(line_segments[1]).unwrap();
                cycles(&mut cycle, &x, &mut signal_strengths, &mut monitor, &mut crt_count, 2);
                x += v;
            },
            _ => {panic!("There is an error in the input on line {}! Only 'noop' and 'addx' commands are supported.", i)}
        }
    }

    let mut sum = 0;
    for i in 0..signal_strengths.len() {
        sum += signal_strengths[i];
    }

    println!("Sum of all the signals strength is {}", sum);
    for y in 0..MONITOR_HEIGHT {
        for x in 0..MONITOR_WIDTH {
            print!("{}", monitor[y][x]);
        }
        print!("\n");
    }
}
