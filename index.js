const fs = require('fs')

const DEAD = "."
const ALIVE = "O"
const SIZE_DELIMITER = "x"

function extractSize(input) {
    const [width, height] = input.split(SIZE_DELIMITER)

    return { width: parseInt(width), height: parseInt(height) }
}

function randomCell() {
    const random = Math.round(Math.random() * 1)

    return (random == 0) ? DEAD : ALIVE;
}

function generate({ width, height }) {
    let output = ""

    for (let row_index = 0; row_index < height; row_index++) {
        for (let column_index = 0; column_index < width; column_index++) {
            output += randomCell();
        }
        output += "\n"
    }

    return output;
}

function write(file, output) {
    fs.writeFileSync(file, output)   
}

const SIZE = process.argv[2]
const FILE = process.argv[3]

const size = extractSize(SIZE)
const grid = generate(size)

write(`./templates/${FILE}`, grid);

