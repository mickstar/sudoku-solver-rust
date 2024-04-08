/*
 * Copyright (c) Michael Johnston 2024.
 */

type Cell = char;
type SudokuGrid = [[Cell; 9]; 9];

struct CellLocation {
    x: usize,
    y: usize,
}

const EMPTY_CELL: Cell = '.';
const INITIAL_VALUE: Cell = '1';
const MAX_VALUE: Cell = '9';

trait Sudoku {
    fn is_grid_valid(&self) -> bool;

    fn solve_grid(&mut self) -> bool;
    fn is_valid(&self, location: &CellLocation, candidate_value: Cell) -> bool;

    fn read_grid_from_input() -> Option<SudokuGrid>;
    fn create_empty_grid() -> SudokuGrid;
    fn get_next_empty_location(&self) -> Option<CellLocation>;
    fn set_cell(&mut self, location: &CellLocation, value: Cell);
    fn clear_cell(&mut self, location: &CellLocation);

    fn print_grid(&self);

}

impl Sudoku for SudokuGrid {
    fn is_grid_valid(&self) -> bool {
        for x in 0..9 {
            for y in 0..9 {
                if self[y][x] != EMPTY_CELL {
                    if !self.is_valid(&CellLocation { x, y }, self[y][x]) {
                        println!("cell at {x} {y} value={cell} is invalid",x=x,y=y,cell=self[y][x]);
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn solve_grid(&mut self) -> bool {
        let mut solved = false;

        // we will test every possible value starting with 1.
        let mut current_value = INITIAL_VALUE;
        while !solved && current_value >= INITIAL_VALUE && current_value <= MAX_VALUE {
            if let Some(location) = self.get_next_empty_location() {
                if !self.is_valid(&location, current_value) {
                    current_value = increment_cell_value(current_value);
                    continue;
                }

                self.set_cell(&location, current_value);
                let is_solved = self.solve_grid();
                if is_solved {
                    solved = true;
                    break;
                } else {
                    self.clear_cell(&location)
                }
            } else {
                solved = true;
                break;
            }


            current_value = increment_cell_value(current_value);
        }
        return solved;
    }

    fn is_valid(&self, location: &CellLocation, candidate_value: Cell) -> bool {
        // we will determine if the grid is valid with the given value at location

        if candidate_value == EMPTY_CELL {
            return false;
        }


        // first we will check the row

        for x in 0..9 {
            if self[location.y][x] == candidate_value && x != location.x {
                return false;
            }
        }

        // now we check column

        for y in 0..9 {
            if self[y][location.x] == candidate_value && y != location.y as usize {
                return false;
            }
        }

        // now we need to check the local square. this is a little tricky.
        // we will determine the square using this definition
        // 0 1 2
        // 3 4 5
        // 6 7 8

        let starting_x = 3 * (location.x / 3);
        let starting_y = 3 * (location.y / 3);

        for dx in 0..3 {
            for dy in 0..3 {
                let testing_cell_x = starting_x + dx;
                let testing_cell_y = starting_y + dy;
                if self[testing_cell_y][testing_cell_x] == candidate_value &&
                    testing_cell_x != location.x &&
                    testing_cell_y != location.y {
                    return false;
                }
            }
        }

        return true;
    }

    fn read_grid_from_input() -> Option<SudokuGrid> {
        let stdin = std::io::stdin();
        let mut finished_reading = false;
        let mut sudoku_grid = SudokuGrid::create_empty_grid();

        let mut x = 0;
        let mut y = 0;

        println!("Enter sudoku grid. Use . for empty cells and ; to signify the end. All other characters will be ignored.");

        while !finished_reading {
            let mut line = String::new();
            stdin.read_line(&mut line)
                .expect("error reading input");

            for c in line.chars() {
                if c == ';' || (x == 8 && y == 8) {
                    finished_reading = true;
                    break;
                }
                if (c >= INITIAL_VALUE && c <= MAX_VALUE) || c == EMPTY_CELL {
                    sudoku_grid[y][x] = c;
                    x += 1;
                    if x == 9 {
                        x = 0;
                        y += 1;
                    }
                }
            }
        }

        if !sudoku_grid.is_grid_valid() {
            sudoku_grid.print_grid();
            println!("The grid you have entered is invalid. One or more cells cannot be placed there.");
            return None;
        }
        println!("Read your grid");
        sudoku_grid.print_grid();
        return Some(sudoku_grid);
    }

    fn create_empty_grid() -> SudokuGrid {
        return [[EMPTY_CELL; 9]; 9];
    }

    fn get_next_empty_location(&self) -> Option<CellLocation> {
        for x in 0..9 {
            for y in 0..9 {
                if self[y][x] == EMPTY_CELL {
                    return Some(CellLocation {
                        x,
                        y,
                    });
                }
            }
        }
        return None;
    }

    fn set_cell(&mut self, location: &CellLocation, value: Cell) {
        self[location.y][location.x] = value
    }

    fn clear_cell(&mut self, location: &CellLocation) {
        self.set_cell(location, EMPTY_CELL)
    }

    fn print_grid(&self) {
        for y in 0..9 {
            for i in 0..3 {
                for x in 0..3 {
                    print!("{cell}  ", cell = self[y][i * 3 + x]);
                }
                print!("  ");
            }
            println!();
            if y == 2 || y == 5 {
                println!()
            }
        }
        println!()
    }
}

fn increment_cell_value(value: Cell) -> Cell {
    if value == EMPTY_CELL || value == '9' {
        return EMPTY_CELL;
    }
    return ((value as u8) + 1) as crate::Cell;
}


fn main() {
    if let Some(mut grid) = SudokuGrid::read_grid_from_input() {
        let is_solved = grid.solve_grid();
        if is_solved {
            println!("Solved your grid");
            grid.print_grid();
        } else {
            println!("Could not solve your board. There is likely an error in it.")
        }
    };
}


