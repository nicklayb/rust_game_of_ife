use std::fmt;
use crate::grid::utils::parse_i8_tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let size = Size::new(5, 6);
        assert_eq!(size, Size { width: 5, height: 6 })   
    }

    #[test]
    fn parse() {
        assert_eq!(Size::parse("4x5".to_string()), Some(Size { width: 4, height: 5 }));
        assert_eq!(Size::parse("4,".to_string()), None);
        assert_eq!(Size::parse("4".to_string()), None);
        assert_eq!(Size::parse("".to_string()), None)
    }
}
