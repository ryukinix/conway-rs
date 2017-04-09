use std::fmt;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Cell {
    pub alive: bool,
    pub point: Point,
}

//* Automata 2D Cell implementation methods 
impl Cell {
    // create a new cell
    pub fn new(state: bool, x: i32, y: i32) -> Cell {
        Cell{alive: state, point: Point{x: x, y: y}}
    }

    // apply logic for a new state based on the number of neighbors
    pub fn update(&self, neighbors: u8) -> Cell {
        let mut new_state: bool = self.alive;
        if self.alive && (neighbors < 2 || neighbors > 3) {
            new_state = false;
        } else if self.alive == false && neighbors == 3 {
            new_state = true;
        }
        Cell{alive: new_state, point: Point{x:self.point.x, y:self.point.y}}
    }
}

impl fmt::Display for Cell {
    // Write a dot if cell is alive, otherwise whitespace
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.alive {
            write!(f, "•")
        }
        else {
            write!(f, " ")
        }
    }
}