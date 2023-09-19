#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Direction {
    North, East, South, West,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct TronBikeState {
    pub x: i32,
    pub y: i32,
    pub d: Direction,
}

impl TronBikeState {
    pub fn new(x: i32, y: i32, d: Direction) -> TronBikeState {
        TronBikeState{x, y, d}
    }

    pub fn turn_left(&mut self) {
        match self.d {
            Direction::East => { self.d = Direction::North},
            Direction::West => { self.d = Direction::South},
            Direction::North => { self.d = Direction::West},
            Direction::South => { self.d = Direction::East},
        }
    }

    pub fn turn_right(&mut self) {
        match self.d {
            Direction::East => { self.d = Direction::South},
            Direction::West => { self.d = Direction::North},
            Direction::North => { self.d = Direction::East},
            Direction::South => { self.d = Direction::West},
        }
    }

    pub fn forward(&mut self) {
        match self.d {
            Direction::East => { self.x += 1},
            Direction::West => { self.x -= 1},
            Direction::North => { self.y += 1},
            Direction::South => { self.x -= 1},
        }
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> Direction {
       self.d
    }

    pub fn commands(&mut self, commandString: &str) {
        for c in commandString.chars() {
            match c {
                'F' => { self.forward()},
                'L' => { self.turn_left()},
                'R' => { self.turn_right()},
                _ => {},
            }
        }
    }

    pub fn print_state(&self) {
        println!("Bike is at {:?}, travelling {:?}", self.position(), self.direction());
    }
}