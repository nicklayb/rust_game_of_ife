use grid::size::Size;
use grid::coordinate::Coordinate;
use grid::Grid;
use std::env;
use std::fs;
mod grid;

fn main() -> std::io::Result<()> {
    if let Some(mut grid) = initialize_grid() {
        println!("{}", grid);

        println!("Press enter to evolve. Enter `q` to quit.");
        while !read_equals("q") && !grid.is_empty() {
            grid.evolve();
            println!("Generation {}\n{}", grid.generation, grid);
        }
    }

    Ok(())
}

fn initialize_grid() -> Option<Grid> {
    if let Some(grid) = read_argument() {
        return Some(grid)
    }
    if let Some(grid) = prompt_game() {
        return Some(grid)
    }
    None
}

fn read_argument() -> Option<Grid> {
    let args: Vec<String> = env::args().collect();
    if let Some(argument) = args.get(1) {
        println!("{}", argument);
        if let Ok(content) = fs::read_to_string(argument) {
            return Grid::parse(content)
        }
    }
    None
}

fn prompt_game() -> Option<Grid> {
    println!("Enter size with format 8x8");
    if let Some(size) = read_size() {
        let mut grid = Grid::new(size);
        println!("Enter the coordinate of an alive cell using coordinate format 1,2. Enter blank to start evolving");
        while let Some(coordinate) = read_coordinate() {
            println!("Enter the coordinate of an alive cell using coordinate format 1,2. Enter blank to start evolving");
            grid.set_alive(coordinate);
        }

        return Some(grid)
    }
    None
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
