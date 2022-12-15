use std::{fs, cell::RefCell, rc::Rc};

#[derive(Debug)]
struct Head {
    current_pos: (isize, isize),
    previous_pos: (isize, isize),
    knot: Vec<Rc<RefCell<Knot>>>,
}

impl Head {
    fn new(knots: u8) -> Head {
        let mut knots_vec = vec![];
        for _ in 0..knots {
            knots_vec.push(Rc::new(RefCell::new(Knot::new())));
        }
        Head { current_pos: (0, 0), previous_pos: (0, 0), knot: knots_vec }
    }

    fn is_knot_near(&self, pos: u8) -> bool {
        if self.knot.len() <= pos as usize { panic!("Index out of bounds. The provided index was {} while the max index is {}", pos, self.knot.len()); }
        match pos {
            0 => {
                for y in self.current_pos.0-1..=self.current_pos.0+1 {
                    for x in self.current_pos.1-1..=self.current_pos.1+1 {
                        println!("{} {} {:?}", y, x, self.knot[pos as usize].borrow().current_pos);

                        if (y, x) == self.knot[pos as usize].borrow().current_pos {
                            return true;
                        }
                    }
                }
            }
            _ => {                
                let knot = self.knot[(pos-1) as usize].borrow();
                
                for y in knot.current_pos.0-1..=knot.current_pos.0+1 {
                    for x in knot.current_pos.1-1..=knot.current_pos.1+1 {
                        println!("{} {} {:?}", y, x, self.knot[(pos) as usize].borrow().current_pos);

                        if (y, x) == self.knot[(pos) as usize].borrow().current_pos {
                            return true;
                        }
                    }
                }
            }
        }
        
        false
    }

    fn was_knot_cardinal(&self, pos: u8) -> bool {
        if self.knot.len() <= pos as usize { panic!("Index out of bounds. The provided index was {} while the max index is {}", pos, self.knot.len()); }

        match pos {
            0 => {
                for y in self.previous_pos.0-1..=self.previous_pos.0+1 {
                    for x in self.previous_pos.1-1..=self.previous_pos.1+1 {
                        if !((y == self.previous_pos.0-1 || y == self.previous_pos.0+1) && (x == self.current_pos.1-1 || x == self.current_pos.1+1)) {
                            if (y, x) == self.knot[pos as usize].borrow().current_pos {
                                return true;
                            }
                        }
                    }
                }
            }
            _ => {                
                let knot = self.knot[pos as usize].borrow();

                for y in knot.previous_pos.0-1..=knot.previous_pos.0+1 {
                    for x in knot.previous_pos.1-1..=knot.previous_pos.1+1 {
                        if !((y == knot.previous_pos.0-1 || y == knot.previous_pos.0+1) && (x == self.current_pos.1-1 || x == self.current_pos.1+1)) {
                            if (y, x) == self.knot[pos as usize].borrow().current_pos {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        
        false
    }

    fn was_knot_diagonal(&self, pos: u8, knot: &Knot, child_knot: Option<Knot>) -> bool {
        if self.knot.len() <= pos as usize { panic!("Index out of bounds. The provided index was {} while the max index is {}", pos, self.knot.len()); }
        if pos != 0 && child_knot == None { panic!("If pos is not 0 you need to specify the child knot... sadly") }
        match pos {
            0 => {
                for y in (self.previous_pos.0-1..=self.previous_pos.0+1).step_by(2) {
                    for x in (self.previous_pos.1-1..=self.previous_pos.1+1).step_by(2) {
                        if (y, x) == knot.current_pos {
                            return true;
                        }
                    }
                }
            },
            _ => {
                for y in (knot.previous_pos.0-1..=knot.previous_pos.0+1).step_by(2) {
                    for x in (knot.previous_pos.1-1..=knot.previous_pos.1+1).step_by(2) {
                        if (y, x) == child_knot.as_ref().unwrap().current_pos {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    fn move_by(&mut self, direction: char, count: usize) {
        self.previous_pos = self.current_pos;

        let position_vec;
        match direction {
            'R' | 'U' => {position_vec = 1;},
            'L' | 'D' => {position_vec = -1;},
            _ => return,
        }

        match direction {
            'R' | 'L' => {
                for _ in 1..=count {
                    self.previous_pos.1 = self.current_pos.1;
                    self.current_pos.1 += position_vec;

                    self.catch_up_knots();
                }
            },
            'D' | 'U' => {
                for _ in 1..=count {
                    self.previous_pos.0 = self.current_pos.0;
                    self.current_pos.0 += position_vec;
                    
                    self.catch_up_knots();
                }
            },
            _ => return,
        }
    }

    

    fn catch_up_knot(&mut self, pos: u8) {
        if self.knot.len() <= pos as usize { panic!("Index out of bounds. The provided index was {} while the max index is {}", pos, self.knot.len()); }

        if !self.is_knot_near(pos) {
            println!("T");
            let mut knot = self.knot[pos as usize].borrow_mut();
            let mut child_knot = self.knot[(pos+1) as usize].borrow_mut();

            match pos {
                0 => {
                    knot.previous_pos = knot.current_pos;

                    if self.was_knot_diagonal(pos, &knot, None) {
                        knot.current_pos = self.previous_pos;
                    } else {
                        let new_pos_change = (self.current_pos.0 - self.previous_pos.0, self.current_pos.1 - self.previous_pos.1);

                        knot.current_pos.0 += new_pos_change.0;
                        knot.current_pos.1 += new_pos_change.1;
                    }
                },
                _ => {
                    if self.was_knot_diagonal(pos, &knot, Some(child_knot.clone())) {
                        child_knot.current_pos = knot.previous_pos;
                    } else {
                        let new_pos_change = (knot.current_pos.0 - knot.previous_pos.0, knot.current_pos.1 - knot.previous_pos.1);

                        child_knot.current_pos.0 += new_pos_change.0;
                        child_knot.current_pos.1 += new_pos_change.1;
                    }
                }
            }
        }
    }

    fn catch_up_knots(&mut self) {
        for i in 0..self.knot.len()-1 {
            self.catch_up_knot(i as u8);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Knot {
    current_pos: (isize, isize),
    previous_pos: (isize, isize),
    visited_pos: Vec<(isize, isize)>,
}

impl Knot {
    fn new() -> Knot {
        Knot { current_pos: (0, 0), previous_pos: (0, 0), visited_pos: vec![(0, 0)] }
    }

    fn add_visited(&mut self) {
        if !self.visited_pos.contains(&self.current_pos) {
            self.visited_pos.push(self.current_pos);
        }
    }
}

fn main() {
    let input_file_path = "C:\\Users\\fryta\\Pulpit\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_9\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut head = Head::new(3);

    for line in input.lines() {
        let segments = line.split(' ').collect::<Vec<_>>();
        let direction = segments[0].chars().collect::<Vec<_>>()[0];
        let num = str::parse::<usize>(segments[1]).unwrap();
        head.move_by(direction, num);
    }

    println!("\n{:?}", head);
}
