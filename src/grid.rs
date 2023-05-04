use std::{fmt, collections::HashMap};
use cell::Cell;
use size::Size;
use coordinate::Coordinate;

mod cell;
pub mod size;
pub mod coordinate;
mod utils;

type InnerGrid = HashMap<Coordinate, Cell>;

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    grid: InnerGrid,
    size: Size,
    pub generation: u16
}

impl Grid {
    pub fn parse(input: String) -> Option<Grid> {
        let mut grid = HashMap::new();
        let splitted: Vec<&str> = input.trim().split('\n').filter(|&char| !char.is_empty()).collect();
        let mut width = 0;
        let height = splitted.len();

        splitted.iter().enumerate().for_each(|(row_index, line)| {
            if row_index == 0 { width = line.len() }
            if !line.is_empty() {
                line.trim().split("").filter(|&char| !char.is_empty()).enumerate().for_each(|(column_index, cell)| {
                    let value = match cell {
                        "." => Cell::Dead,
                        _ => Cell::Alive
                    };
                    if let Some((row, column)) = dual_usize_to_i8(row_index, column_index) {
                        grid.insert(Coordinate::new(row, column), value);
                    }
                }) 
            }
        });

        match dual_usize_to_i8(width, height) {
            Some((width, height)) => 
                Some(Grid { size: Size::new(width, height), grid, generation: 0 }),
            _ =>
                None
        }
    }

    pub fn new(size: Size) -> Grid {
        let mut grid = HashMap::new();
        for row_index in 0..size.height {
            for column_index in 0..size.width {
                grid.insert(Coordinate::new(row_index, column_index), Cell::Dead);
            }
        }
        Grid { grid, size, generation: 0 }
    }

    pub fn is_empty(&self) -> bool {
        for (_, value) in self.grid.iter() {
            if let Cell::Alive = value {
                return false
            }
        }
        true
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
        let old_grid = self.clone();
        for coordinate in old_grid.grid.keys()  {
            self.evolve_cell(coordinate, &old_grid);
        }
        self.bump_generation();
    }

    fn evolve_cell(&mut self, coordinate: &Coordinate, old_grid: &Grid) {
        let count = old_grid.siblings(*coordinate).into_iter().fold(0, |acc, value| {
            match value {
                Some(Cell::Alive) => acc + 1,
                _ => acc
            }
        });
        match count {
            3 => self.set_alive(*coordinate),
            2 => (),
            _ => self.set_dead(*coordinate)
        }
    }

    fn bump_generation(&mut self) {
        self.generation += 1
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

fn dual_usize_to_i8(first: usize, second: usize) -> Option<(i8, i8)> {
    match (i8::try_from(first), i8::try_from(second)) {
        (Ok(left), Ok(right)) => Some((left, right)),
        _ => None
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn new() {
        let grid = Grid::new(Size::new(3,3));
        assert_eq!(grid, Grid {
            size: Size::new(3, 3),
            generation: 0,
            grid: HashMap::from([
                (Coordinate::new(0, 0), Cell::Dead),
                (Coordinate::new(0, 1), Cell::Dead),
                (Coordinate::new(0, 2), Cell::Dead),
                (Coordinate::new(1, 0), Cell::Dead),
                (Coordinate::new(1, 1), Cell::Dead),
                (Coordinate::new(1, 2), Cell::Dead),
                (Coordinate::new(2, 0), Cell::Dead),
                (Coordinate::new(2, 1), Cell::Dead),
                (Coordinate::new(2, 2), Cell::Dead)
            ])
        })
    }

    #[test]
    fn parse() {
        let grid = Grid::parse(String::from("...\n.O.\n.OO"));
        assert_eq!(grid, Some(Grid {
            size: Size::new(3, 3),
            generation: 0,
            grid: HashMap::from([
                (Coordinate::new(0, 0), Cell::Dead),
                (Coordinate::new(0, 1), Cell::Dead),
                (Coordinate::new(0, 2), Cell::Dead),
                (Coordinate::new(1, 0), Cell::Dead),
                (Coordinate::new(1, 1), Cell::Alive),
                (Coordinate::new(1, 2), Cell::Dead),
                (Coordinate::new(2, 0), Cell::Dead),
                (Coordinate::new(2, 1), Cell::Alive),
                (Coordinate::new(2, 2), Cell::Alive)
            ])
        }))
    }

    #[test]
    fn is_empty() {
        let mut grid = Grid::new(Size::new(3, 3));

        assert!(grid.is_empty());
        
        grid.set_alive(Coordinate::new(0,1));

        assert!(!grid.is_empty());
    }

    #[test]
    fn evolve() {
        let mut grid = Grid::parse(String::from("
                                            .OO
                                            O..
                                            .O.")).expect("Parses correctly");
        assert_eq!(grid.generation, 0);

        let expected_grid = Grid {
            size: Size::new(3, 3),
            grid: HashMap::from([
                (Coordinate::new(0, 0), Cell::Dead),
                (Coordinate::new(0, 1), Cell::Alive),
                (Coordinate::new(0, 2), Cell::Alive),
                (Coordinate::new(1, 0), Cell::Alive),
                (Coordinate::new(1, 1), Cell::Dead),
                (Coordinate::new(1, 2), Cell::Dead),
                (Coordinate::new(2, 0), Cell::Alive),
                (Coordinate::new(2, 1), Cell::Alive),
                (Coordinate::new(2, 2), Cell::Dead)
            ]),
            generation: 1
        };

        grid.evolve();

        assert_eq!(grid, expected_grid);

        let expected_grid = Grid {
            size: Size::new(3, 3),
            grid: HashMap::from([
                (Coordinate::new(0, 0), Cell::Dead),
                (Coordinate::new(0, 1), Cell::Alive),
                (Coordinate::new(0, 2), Cell::Alive),
                (Coordinate::new(1, 0), Cell::Alive),
                (Coordinate::new(1, 1), Cell::Dead),
                (Coordinate::new(1, 2), Cell::Dead),
                (Coordinate::new(2, 0), Cell::Alive),
                (Coordinate::new(2, 1), Cell::Dead),
                (Coordinate::new(2, 2), Cell::Dead)
            ]),
            generation: 2
        };

        grid.evolve();

        assert_eq!(grid, expected_grid);
    }

    #[test]
    fn get() {
        let mut grid = Grid::new(Size::new(3, 3));

        let coordinate = Coordinate::new(0, 0);

        assert_eq!(grid.get(coordinate), Some(&Cell::Dead));

        grid.set_alive(coordinate);

        assert_eq!(grid.get(coordinate), Some(&Cell::Alive));

        assert_eq!(grid.get(Coordinate::new(10, 10)), None);
    }

    #[test]
    fn set_alive() {
        let mut grid = Grid::new(Size::new(3, 3));

        let coordinate = Coordinate::new(0, 0);

        assert_eq!(grid.get(coordinate), Some(&Cell::Dead));

        grid.set_alive(coordinate);

        assert_eq!(grid.get(coordinate), Some(&Cell::Alive));
    }

    #[test]
    fn set_dead() {
        let mut grid = Grid::new(Size::new(3, 3));

        let coordinate = Coordinate::new(0, 0);

        grid.set_alive(coordinate);

        assert_eq!(grid.get(coordinate), Some(&Cell::Alive));

        grid.set_dead(coordinate);

        assert_eq!(grid.get(coordinate), Some(&Cell::Dead));
    }
}
