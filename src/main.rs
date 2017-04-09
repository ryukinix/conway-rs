extern crate rand;

pub mod grid;
pub mod cell;
pub mod utils;

use grid::Grid;
use std::io;

fn main() {
    let mut input = String::new();
    let mut grid = Grid::random_grid(80, 25);
    let mut generation = 0;
    loop {
        print!("{}", grid);
        grid = grid.new_generation();
        println!("(generation: {}) press RET to continue and C-c to exit.", generation);
        let _ = io::stdin().read_line(&mut input);
        generation += 1
    }
}