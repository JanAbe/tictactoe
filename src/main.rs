// cargo run -> builds and executes the program
// ./target/debug/tic_tac_toe.exe is the executable that has been made by cargo run

use std::io;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    println!("Your input {}", input);
}


enum CellValue {
    X,
    O
}

struct Board {
    cells: HashMap<(u8, u8), CellValue>,
    length: u8,
}


// n is the length of the board
fn win_combinations(n: u8) -> Vec<Vec<(u8, u8)>> {
    let mut winnable_combinations = Vec::new();
    let mut left_diagonal_combinations = Vec::new();
    for i in 0..n {
        left_diagonal_combinations.push((i, i));
        let mut row = Vec::new();
        let mut column = Vec::new();
        for j in 0..n {
            column.push((j, i));
            row.push((i, j))
        }
        winnable_combinations.push(row);
        winnable_combinations.push(column);
    }

    winnable_combinations.push(left_diagonal_combinations);

    let mut right_diagonal_combinations = Vec::new();
    for i in 1..n-1 {
        let j = (n-1)-i;
        right_diagonal_combinations.push((i, j));
    }
    winnable_combinations.push(right_diagonal_combinations);

    return winnable_combinations;
}


// reverseVecItems() [(0,0), (0,1), (0,2)] -> [(0,0), (1,0), (2,0)]

// ask user for board size, e.g 3 means a 3x3 board
// input must be larger than 2, must be smaller than 256+1 (2^8+1)
// 