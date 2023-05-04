pub fn parse_i8_tuple(input: String, char: char) -> Option<(i8, i8)> {
    let mut splitted = input.trim().split(char);
    match (splitted.next(), splitted.next()) {
        (Some(left), Some(right)) => {
            match (left.parse::<i8>(), right.parse::<i8>()) {
                (Ok(left_int), Ok(right_int)) =>
                    Some((left_int, right_int)),

                _ => {
                    None
                }
            }
        }

        _ => 
            None
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_i8_tuple() {
        assert_eq!(parse_i8_tuple(String::from("1,2"), ','), Some((1, 2)));
        assert_eq!(parse_i8_tuple(String::from("1x2"), 'x'), Some((1, 2)));
        assert_eq!(parse_i8_tuple(String::from("1x"), 'x'), None);
        assert_eq!(parse_i8_tuple(String::from(""), 'x'), None);
        assert_eq!(parse_i8_tuple(String::from("c,b"), 'x'), None);
    }
}
