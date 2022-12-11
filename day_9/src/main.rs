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

    fn knot(&mut self, num: u8) -> &mut Knot {
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
        self.knot.current_pos = self.previous_pos
    }
}

#[derive(PartialEq)]
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
        let mut num: u8 = 0;
        loop {
            let knot = self.knot(num);
            if knot.knot.as_ref().unwrap().knot == None {break;}
            knot.knot.as_mut().unwrap().catch_up_knot( knot.previous_pos, knot.current_pos);
            num += 1;
        }
    }

    fn knot(&mut self, num: u8) -> &mut Knot {
        if self.knot.as_ref() == None {panic!("Cannot find knot of index {}, there are only {} knots.", num, num+1)}
        if num == 0 {return self;}
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

    fn catch_up_knot(&mut self, previous: (isize, isize), current: (isize, isize)) {
        let new_pos_change = (current.0 - previous.0, current.1 - previous.1);
        let knot = self.knot.as_mut().unwrap();

        knot.previous_pos.0 = knot.current_pos.0;
        knot.previous_pos.1 = knot.current_pos.1;

        let new_y = knot.current_pos.0 + new_pos_change.0;
        let new_x = knot.current_pos.1 + new_pos_change.1;

        println!("{:?} {:?} {:?}", new_pos_change, current, previous);

        self.knot(0).current_pos.0 = new_y;
        self.knot(0).current_pos.1 = new_x;

        self.knot.as_mut().unwrap().add_visited((new_y, new_x));
    }
}

fn main() {
    let input_file_path = "C:\\Users\\Frytak\\Desktop\\~\\Important\\Programming Projects\\AdventOfCode2022\\day_9\\src\\input.txt";
    let input = fs::read_to_string(input_file_path)
        .expect("Something went wrong reading the file");

    let mut head = Head::new(3);

    for line in input.lines() {
        let segments = line.split(' ').collect::<Vec<_>>();
        head.r#move(segments[0].chars().collect::<Vec<_>>()[0], str::parse::<usize>(segments[1]).unwrap())
    }

    println!("\n{:?}",head.knot(1).current_pos);
}
