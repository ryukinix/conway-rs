//! This module handle the individual life of each cell.
//!
//! The higher abstraction structure is called Cell.

use std::fmt;

/// A simple 2D point on plane
#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// Automata 2D Cell implementation methods
#[derive(Debug)]
pub struct Cell {
    pub alive: bool,
    pub point: Point,
}

impl Point {
    /// Create a new Point structure
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

impl Cell {
    /// Create a new cell element.
    pub fn new(state: bool, x: i32, y: i32) -> Cell {
        Cell {
            alive: state,
            point: Point { x: x, y: y },
        }
    }

    /// Update the state of creating a new cell.
    pub fn state(&self, s: bool) -> Cell {
        Cell {
            alive: s,
            point: Point {
                x: self.point.x,
                y: self.point.y,
            },
        }
    }

    /// Apply logic for a new state based on the number of neighbors
    /// The core of Conway's game of life.
    ///
    /// Based on the number of neighbors:
    ///
    /// * if the cell is alive, it will die if the neighbors
    ///   are lesser than 2 (loneliness) or greater than 3 (super population).
    ///
    /// * if the cell is dead but has 3 neighbors, it will born
    ///   a new cell (reproduction).
    pub fn update(&self, neighbors: u8) -> Cell {
        let state = match self.alive {
            true => !(neighbors < 2 || neighbors > 3),
            false => (neighbors == 3),
        };
        Cell {
            alive: state,
            point: Point {
                x: self.point.x,
                y: self.point.y,
            },
        }
    }
}

impl fmt::Display for Cell {
    /// Write a dot if cell is alive, otherwise white-space.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.alive {
            write!(f, "•")
        } else {
            write!(f, " ")
        }
    }
}
