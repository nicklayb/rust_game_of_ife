use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Alive,
    Dead
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = match self {
            Cell::Dead => '_',
            Cell::Alive => '▓'
        };
        write!(f, "{}", output)
    }
}
