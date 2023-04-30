use std::fmt;

pub enum Cell {
    Alive,
    Dead
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
