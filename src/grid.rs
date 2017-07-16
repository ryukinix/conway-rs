use cell::{Point, Cell};
use utils::random_state;
use std::fmt;

#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
    width: usize,
    heigth: usize,
}


//* Grid abstraction as 2D plane containing the cells and its state
impl Grid {
    // Create a new grid based on the width and heigth
    pub fn new(width: usize, heigth: usize) -> Grid {
        let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(width);
        for y in 0..heigth as i32 {
            let mut line: Vec<Cell> = Vec::new();
            for x in 0..width as i32 {
                line.push(Cell::new(true, x, y));
            }
            grid.push(line);
        }
        Grid {
            cells: grid,
            width: width,
            heigth: heigth,
        }
    }

    // Generate a new grid using random numbers to determine
    // the state of the cells
    pub fn random_grid(width: usize, heigth: usize) -> Grid {
        let mut grid = Grid::new(width, heigth);
        grid.cells = grid.cells
            .iter()
            .map(|line| {
                line.iter().map(|cell| cell.state(random_state())).collect()
            })
            .collect();
        grid
    }

    // Create a new generation for all cells applying the
    // rules of Conway's Game of Life (defined on Cell::update)
    // based on the old state and the neighbors
    pub fn new_generation(self) -> Grid {
        let mut generation = Grid::new(self.width, self.heigth);
        for (y, line) in generation.cells.iter_mut().enumerate() {
            for (x, cell) in line.iter_mut().enumerate() {
                *cell = self.cell(x, y).update(self.neighbors(cell));
            }
        }
        generation
    }

    // access the internal vector of grid based on the
    // 2D point (x,y). Returns the cell
    pub fn cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    // Return the number of neighbors alive from a
    // given cell
    pub fn neighbors(&self, cell: &Cell) -> u8 {
        let x = cell.point.x;
        let y = cell.point.y;
        let mut neighbors_count = 0;
        let neighbors_points = [
            Point { x: x, y: y + 1 },
            Point { x: x, y: y - 1 },
            Point { x: x + 1, y: y },
            Point { x: x - 1, y: y },
            Point { x: x + 1, y: y + 1 },
            Point { x: x - 1, y: y - 1 },
            Point { x: x + 1, y: y - 1 },
            Point { x: x - 1, y: y + 1 },
        ];

        for point in neighbors_points.iter() {
            if point.x >= 0 && point.x < self.width as i32 && point.y >= 0 &&
                point.y < self.heigth as i32
            {
                if self.cell(point.x as usize, point.y as usize).alive {
                    neighbors_count += 1;
                }
            }
        }

        neighbors_count
    }
}

impl fmt::Display for Grid {
    // Display all the grid and its cell state
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut horizontal_limiter = String::from("+");
        for _ in 0..(self.width) {
            horizontal_limiter.push('-');
        }
        horizontal_limiter.push_str("+\n");

        write!(f, "{}", horizontal_limiter)?;
        for line in self.cells.iter() {
            write!(f, "|")?;
            for cell in line.iter() {
                write!(f, "{}", cell)?;
            }
            write!(f, "|")?;
            write!(f, "\n")?;
        }
        write!(f, "{}", horizontal_limiter)
    }
}
