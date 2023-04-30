use std::fmt;
use crate::grid::utils::parse_i8_tuple;

pub struct Size {
    pub width: i8,
    pub height: i8
}

impl Size {
    pub fn new(width: i8, height: i8) -> Size {
        Size { width, height }
    }

    pub fn parse(input: String) -> Option<Size> {
        parse_i8_tuple(input, 'x')
            .map(|(width, height)| Size::new(width, height))
    }
}

impl fmt::Display for Size {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "<{}x{}>", self.width, self.height)
    }
}
