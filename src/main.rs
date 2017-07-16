//! # Just the main loop
//! RET do a new generation
//! C-c (CTRL-C) exits.
extern crate conway_rs;

use conway_rs::grid::Grid;
use std::io;

fn main() {
    let mut input = String::new();
    let mut grid = Grid::random_grid(78, 20);
    let mut generation = 0;
    loop {
        print!("{}", grid);
        grid = grid.new_generation();
        println!(
            "(generation: {}) press RET to continue and C-c to exit.",
            generation
        );
        let _ = io::stdin().read_line(&mut input);
        generation += 1
    }
}
