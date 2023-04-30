use std::{fmt, collections::HashMap};
use cell::Cell;
use size::Size;
use coordinate::Coordinate;

mod cell;
pub mod size;
pub mod coordinate;
mod utils;

pub struct Grid {
    grid: HashMap<Coordinate, Cell>,
    size: Size
}

impl Grid {
    pub fn new(size: Size) -> Grid {
        let mut grid = HashMap::new();
        for row_index in 0..size.height {
            for column_index in 0..size.width {
                grid.insert(Coordinate::new(row_index, column_index), Cell::Dead);
            }
        }
        Grid { grid, size }
    }

    pub fn get(&self, coordinate: Coordinate) -> Option<&Cell> {
        self.grid.get(&self.cycle_coordinate(coordinate))
    }

    fn siblings(&self, coordinate: Coordinate) -> [Option<&Cell>; 8] {
        [
            self.get(Coordinate { row_index: coordinate.row_index - 1, column_index: coordinate.column_index - 1 }),
            self.get(Coordinate { row_index: coordinate.row_index - 1, column_index: coordinate.column_index }),
            self.get(Coordinate { row_index: coordinate.row_index - 1, column_index: coordinate.column_index + 1 }),
            self.get(Coordinate { row_index: coordinate.row_index, column_index: coordinate.column_index - 1 }),
            self.get(Coordinate { row_index: coordinate.row_index, column_index: coordinate.column_index + 1 }),
            self.get(Coordinate { row_index: coordinate.row_index + 1, column_index: coordinate.column_index - 1 }),
            self.get(Coordinate { row_index: coordinate.row_index + 1, column_index: coordinate.column_index }),
            self.get(Coordinate { row_index: coordinate.row_index + 1, column_index: coordinate.column_index + 1 })
        ]
    }

    fn cycle_coordinate(&self, coordinate: Coordinate) -> Coordinate {
        let row_index = self.cycle_row(coordinate.row_index);
        let column_index = self.cycle_column(coordinate.column_index);
        Coordinate { row_index, column_index }
    }

    fn cycle_row(&self, row_index: i8) -> i8 {
        if row_index < 0 {
            return self.size.height - row_index;
        }
        if row_index >= self.size.width {
            return row_index - self.size.height;
        }
        row_index
    }

    fn cycle_column(&self, column_index: i8) -> i8 {
        if column_index < 0 {
            return self.size.width - column_index;
        }
        if column_index >= self.size.width {
            return column_index - self.size.width;
        }
        column_index
    }

    pub fn evolve(&mut self) {
        for coordinate in self.grid.keys() {
            self.evolve_cell(coordinate);
        }
    }

    fn evolve_cell(&mut self, coordinate: Coordinate) {
        let mut count = 0;
        for cell in self.siblings(coordinate) {
            if let Some(Cell::Alive) = cell {
                count = count + 1
            }
        }
        match count {
            3 => self.set_alive(coordinate),
            2 => (),
            _ => self.set_dead(coordinate)
        }
    }

    pub fn set_alive(&mut self, coordinate: Coordinate) {
        if self.grid.contains_key(&coordinate) {
            self.grid.insert(coordinate, Cell::Alive);
        }
    }

    pub fn set_dead(&mut self, coordinate: Coordinate) {
        if self.grid.contains_key(&coordinate) {
            self.grid.insert(coordinate, Cell::Dead);
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        for row_index in 0..self.size.height {
            for column_index in 0..self.size.width {
                let coordinate: Coordinate = Coordinate { row_index, column_index };
                if let Some(cell) = self.get(coordinate) { 
                    output.push_str(&cell.to_string())
                }
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}
