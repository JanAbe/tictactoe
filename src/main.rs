// cargo run -> builds and executes the program
// ./target/debug/tic_tac_toe.exe is the executable that has been made by cargo run

use std::io;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // let mut input = String::new();
    // io::stdin().read_line(&mut input)
    //     .expect("Failed to read line");

    // println!("Your input {}", input);

    let mut x = Vec::new();
    x.push((0,0));
    x.push((1,0));
    x.push((2,0));
    x.push((3,0));
    x.push((4,0));
    let size3_win_combos = win_combinations(3);
    println!("{:?}", size3_win_combos);
    /* output of win_combinations(3) =
        [
            [(0, 0), (0, 1), (0, 2)], 
            [(0, 0), (1, 0), (2, 0)], 

            [(1, 0), (1, 1), (1, 2)], 
            [(0, 1), (1, 1), (2, 1)], 

            [(2, 0), (2, 1), (2, 2)], 
            [(0, 2), (1, 2), (2, 2)], 

            [(0, 0), (1, 1), (2, 2)], 
            [(0, 2), (1, 1), (2, 0)]
        ]
    */

    let size5_win_combos = win_combinations(5);
    let won = size5_win_combos.contains(&x);
    println!("{:?}", size5_win_combos);
    println!("{}", won);
    /* output of win_combinations(5) =
        [
            [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)], 
            [(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)], 

            [(1, 0), (1, 1), (1, 2), (1, 3), (1, 4)], 
            [(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)], 

            [(2, 0), (2, 1), (2, 2), (2, 3), (2, 4)], 
            [(0, 2), (1, 2), (2, 2), (3, 2), (4, 2)], 

            [(3, 0), (3, 1), (3, 2), (3, 3), (3, 4)], 
            [(0, 3), (1, 3), (2, 3), (3, 3), (4, 3)], 

            [(4, 0), (4, 1), (4, 2), (4, 3), (4, 4)], 
            [(0, 4), (1, 4), (2, 4), (3, 4), (4, 4)], 

            [(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)], 
            [(0, 4), (1, 3), (2, 2), (3, 1), (4, 0)]
        ]
    */
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
        let mut rows = Vec::new();
        let mut columns = Vec::new();
        for j in 0..n {
            rows.push((i, j));
            columns.push((j, i));
        }
        
        left_diagonal_combinations.push((i, i));
        winnable_combinations.push(rows);
        winnable_combinations.push(columns);
    }

    winnable_combinations.push(left_diagonal_combinations);

    // see if this can be executed/placed in the for loop above.
    let mut right_diagonal_combinations = Vec::new();
    for i in 0..n {
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