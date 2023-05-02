# Game of life

Implementation of Conway's Game Of Life to learn Rust.

## Running the game

### Running with a template

The folder `templates` includes a couple of template you can use in the project. Run one using 

```sh
cargo run -- templates/1.txt
```

### Running interactively

You can run it interactively by entering every alive cell coordinates. 

It'll first prompt for the size using format `8x8`. 

Then it'll prompt for every alive cell using format `1,2`. Enter a blank line to start the generation.

## Generating a template

Run `node index.js 8x5 4.txt` to generate a new template. You can change the size as long as it has the format `[int]x[int]`.
