use std::fs;

struct Head {
    current_pos: (isize, isize),
    previous_pos: (isize, isize),
    knot: Knot,
}

impl Head {
    fn new(knots: u8) -> Head {
        Head { current_pos: (0, 0), previous_pos: (0, 0), knot: Knot::new(knots) }
    }

    fn r#move(&mut self, direction: char, count: usize) {
        self.previous_pos = self.current_pos;

        let position_vec;
        match direction {
            'R' | 'U' => {position_vec = 1;},
            'L' | 'D' => {position_vec = -1;},
            _ => return,
        }

        if direction == 'R' || direction == 'L' {
            for _ in 1..=count {
                self.previous_pos.1 = self.current_pos.1;
                self.current_pos.1 += position_vec;
                if !self.is_knot_near() {
                    self.catch_up_knot();
                    self.knot.catch_up_knots();
                    self.knot.add_visited(self.previous_pos);
                }
            }
        } else {
            for _ in 1..=count {
                self.previous_pos.0 = self.current_pos.0;
                self.current_pos.0 += position_vec;

                if !self.is_knot_near() {
                    self.catch_up_knot();
                    self.knot.catch_up_knots();
                    self.knot.add_visited(self.previous_pos);
                }
            }
        }
    }

    fn knot(&mut self, num: u8) -> Option<&mut Knot> {
        self.knot.knot(num)
    }

    fn is_knot_near(&self) -> bool {
        for y in self.current_pos.0-1..=self.current_pos.0+1 {
            for x in self.current_pos.1-1..=self.current_pos.1+1 {
                if (y, x) == self.knot.current_pos {
                    return true;
                }
            }
        }
        false
    }

    fn catch_up_knot(&mut self) {
        self.knot.previous_pos = self.knot.current_pos;
        self.knot.current_pos = self.previous_pos;
    }
}

#[derive(PartialEq, Clone)]
struct Knot {
    current_pos: (isize, isize),
    previous_pos: (isize, isize),
    visited_pos: Vec<(isize, isize)>,
    knot: Option<Box<Knot>>,
}

impl Knot {
    fn new(level: u8) -> Knot {
        if level == 0 {return Knot { current_pos: (0, 0), previous_pos: (0, 0), visited_pos: vec![(0, 0)], knot: None }}
        Knot { current_pos: (0, 0), previous_pos: (0, 0), visited_pos: vec![(0, 0)], knot: Some(Box::new(Knot::new(level - 1))) }
    }

    fn add_visited(&mut self, position: (isize, isize)) {
        if !self.visited_pos.contains(&position) {
            self.visited_pos.push(position);
        }
    }

    fn catch_up_knots(&mut self) {
        for i in 0..self.knot_count() {
            self.knot(i as u8).unwrap().catch_up_knot();
        }
    }

    fn knot_count(&mut self) -> usize {
        let mut count: usize = 0;
        let mut knot = self.knot.as_mut();
        loop {
            let knot_copy = knot.unwrap().knot(0);
            if knot_copy == None { return count+1; }
            
            knot = knot_copy.unwrap().knot.as_mut();
            count += 1;
        }
    }

    // Get the knot in the chain by it's index (0 is the first knot from the current one, sadly it doesn't go up)
    fn knot(&mut self, num: u8) -> Option<&mut Knot> {
        if self.knot.as_ref() == None { return None; }
        if num == 0 {return Some(self);}
        self.knot.as_mut().unwrap().knot(num-1)
    }

    fn is_knot_near(&self) -> bool {
        for y in self.current_pos.0-1..=self.current_pos.0+1 {
            for x in self.current_pos.1-1..=self.current_pos.1+1 {
                if (y, x) == self.knot.as_ref().unwrap().current_pos {
                    return true;
                }
            }
        }
        false
    }

    // fn was_knot_cardinal(&self) -> bool {
    //     println!("{:?} {:?}", self.previous_pos, self.knot.as_ref().unwrap().current_pos);

    //     for y in self.previous_pos.0-1..=self.previous_pos.0+1 {
    //         for x in self.previous_pos.1-1..=self.previous_pos.1+1 {
    //             if !((y == self.previous_pos.0-1 || y == self.previous_pos.0+1) && (x == self.current_pos.1-1 || x == self.current_pos.1+1)) {
    //                 if (y, x) == self.knot.as_ref().unwrap().current_pos {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }
    //     false
    // }

    // fn was_knot_diagonal(&self) -> bool {
    //     for y in (self.previous_pos.0-1..=self.previous_pos.0+1).step_by(2) {
    //         for x in (self.previous_pos.1-1..=self.previous_pos.1+1).step_by(2) {
    //             if (y, x) == self.knot.as_ref().unwrap().current_pos {
    //                 return true;
    //             }
    //         }
    //     }
    //     false
    // }

    // Calculates the vector of how it moved if it was cardinal to the parent knot, and if not it sets child position to the parent previos position
    fn catch_up_knot(&mut self) {
        if self.is_knot_near() {
            // let was_knot_diagonal = self.was_knot_diagonal();
            // let was_knot_cardinal = self.was_knot_cardinal();

            let knot = self.knot.as_mut().unwrap();
            knot.previous_pos = knot.current_pos;
            knot.current_pos = self.previous_pos;

            println!("TE");

            // println!("{} {}", was_knot_diagonal, was_knot_cardinal);

            // if !was_knot_cardinal {
            //     knot.current_pos = self.previous_pos;
            // } 
            // } else if is_knot_diagonal {
            //     let new_pos_change = (self.current_pos.0 - self.previous_pos.0, self.current_pos.1 - self.previous_pos.1);

            //     knot.current_pos.0 += new_pos_change.0;
            //     knot.current_pos.1 += new_pos_change.1;

            // }

            knot.add_visited(knot.current_pos);
        }
    }
}

fn main() {
    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_9\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut head = Head::new(2);

    for line in input.lines() {
        let segments = line.split(' ').collect::<Vec<_>>();
        head.r#move(segments[0].chars().collect::<Vec<_>>()[0], str::parse::<usize>(segments[1]).unwrap())
    }

    println!("\n{:?}", head.knot.knot(1).unwrap().current_pos);
}
