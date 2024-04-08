# Sudoku Solver in Rust

This is a toy program I made while I'm learning rust for solving sudoku puzzles made in Rust  

It's 100% memory safe and performant.


## Usage  

    ./sudoku-solver
    >> Enter sudoku grid. Use . for empty cells and ; to signify the end. All other characters will be ignored.
    . . 2 7 8 . . . 3
    . . . . . 9 8 . 1
    4 . . . . 3 . 7 .

    9 . 5 . . 8 . . .
    . . . . 7 . . . .
    . . . 5 . . 4 . 8

    . 6 . 4 . . . . 7
    3 . 9 8 . . . . .
    8 . . . 3 1 6 . .

## Output

    Read your grid
    .  .  2    7  8  .    .  .  3    
    .  .  .    .  .  9    8  .  1    
    4  .  .    .  .  3    .  7  .
    
    9  .  5    .  .  8    .  .  .    
    .  .  .    .  7  .    .  .  .    
    .  .  .    5  .  .    4  .  8
    
    .  6  .    4  .  .    .  .  7    
    3  .  9    8  .  .    .  .  .    
    8  .  .    .  3  1    6  .  .
    
    Solved your grid
    1  9  2    7  8  4    5  6  3    
    5  3  7    6  2  9    8  4  1    
    4  8  6    1  5  3    9  7  2
    
    9  1  5    3  4  8    7  2  6    
    6  4  8    9  7  2    1  3  5    
    7  2  3    5  1  6    4  9  8
    
    2  6  1    4  9  5    3  8  7    
    3  5  9    8  6  7    2  1  4    
    8  7  4    2  3  1    6  5  9  

Copyright Michael Johnston 2024