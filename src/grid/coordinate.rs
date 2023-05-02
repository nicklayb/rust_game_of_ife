use std::fmt;
use crate::grid::utils::parse_i8_tuple;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
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
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let coordinate = Coordinate::new(5, 6);
        assert_eq!(coordinate, Coordinate { row_index: 5, column_index: 6 })   
    }

    #[test]
    fn parse_valid() {
        assert_eq!(Coordinate::parse("4,5".to_string()), Some(Coordinate { row_index: 4, column_index: 5 }));
        assert_eq!(Coordinate::parse("4,".to_string()), None);
        assert_eq!(Coordinate::parse("4".to_string()), None);
        assert_eq!(Coordinate::parse("".to_string()), None)
    }
}
