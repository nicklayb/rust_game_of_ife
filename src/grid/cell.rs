use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Alive,
    Dead
}

impl Cell {
    pub fn parse(input: &str) -> Option<Cell> {
        if input.len() == 1 {
            return match input {
                "." => Some(Cell::Dead),
                _ => Some(Cell::Alive)
            }
        }

        None
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Cell::Dead => ' ',
            Cell::Alive => '▓'
        };
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(Cell::parse("."), Some(Cell::Dead));
        assert_eq!(Cell::parse("O"), Some(Cell::Alive));
        assert_eq!(Cell::parse("X"), Some(Cell::Alive));
        assert_eq!(Cell::parse(""), None);
        assert_eq!(Cell::parse(".O"), None);
    }
}
