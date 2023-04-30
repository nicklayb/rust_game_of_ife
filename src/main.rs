use grid::size::Size;
use grid::coordinate::Coordinate;
use grid::Grid;
mod grid;

fn main() -> std::io::Result<()> {
    if let Some(size) = read_size() {
        let mut grid = Grid::new(size);
        while let Some(coordinate) = read_coordinate() {
            grid.set_alive(coordinate);
        }
        println!("{}", grid);

        println!("Press enter to evolve. Enter `q` to quit.");
        while ! read_equals("q") {
            println!("{}", grid);
        }
    }

    Ok(())
}

fn read_equals(expected: &str) -> bool {
    let mut exit_buffer = String::new();
    match std::io::stdin().read_line(&mut exit_buffer) {
        Ok(_) =>
            exit_buffer.trim() == expected,

        _ =>
            false
    }
}

fn read_size() -> Option<Size> {
    let mut size_buffer = String::new();
    match std::io::stdin().read_line(&mut size_buffer) {
        Ok(_) =>
            Size::parse(size_buffer.clone()),

        _ =>
            None
    }
}

fn read_coordinate() -> Option<Coordinate> {
    let mut coordinate_buffer = String::new();
    match std::io::stdin().read_line(&mut coordinate_buffer) {
        Ok(_) =>
            Coordinate::parse(coordinate_buffer.clone()),

        _ =>
            None
    }
}
