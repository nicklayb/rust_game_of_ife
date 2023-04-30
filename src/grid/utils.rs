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
