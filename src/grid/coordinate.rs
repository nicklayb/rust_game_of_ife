use std::fmt;
use crate::grid::utils::parse_i8_tuple;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub row_index: i8,
    pub column_index: i8
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.row_index, self.column_index)
    }
}

impl Coordinate {
    pub fn new(row_index: i8, column_index: i8) -> Coordinate {
        Coordinate { row_index, column_index }
    }

    pub fn parse(input: String) -> Option<Coordinate> {
        parse_i8_tuple(input, ',')
            .map(|(row_index, column_index)| Coordinate::new(row_index, column_index))
    }

    pub fn origin() -> Coordinate {
        Coordinate { row_index: 0, column_index: 0 }
    }
}
