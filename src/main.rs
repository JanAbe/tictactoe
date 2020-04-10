// cargo run -> builds and executes the program
// ./target/debug/tic_tac_toe.exe is the executable that has been made by cargo run

use std::io;
use std::collections::HashMap;

fn main() {
    let mut board_length = String::new();
    io::stdin().read_line(&mut board_length)
        .expect("Failed to read board length input");
    
    let board_length: u8 = match board_length.trim().parse() {
        Ok(num) => num,
        Err(_) => 3, // if no number is provided, give board a default length
    };

    let game = Game {
        is_over: false,
        turn_counter: 0,
        board: Board {
            cells: HashMap::new(),
            length: board_length
        }
    };

    game.play();

    // let mut game_over = false;
    // let mut turn = 1;
    // let mut board = Board{
    //     cells: HashMap::new(), 
    //     length: board_length
    // };

    // while !game_over {
    //     if turn % 2 == 0 {
    //         prompt_player(&mut board, PlayerValue::X);
    //     } else {
    //         prompt_player(&mut board, PlayerValue::O);
    //     }

    //     turn += 1;
    //     if turn == 10 {
    //         game_over = true;
    //     }
    // }
}


struct Game{
    is_over: bool,
    turn_counter: u8,
    board: Board,
}

impl Game{

    fn play(&mut self) {
        while !self.is_over {
            self.prompt_player();
        }
    }

    /// prompt_player prompts a player to input his new move and
    /// updates the board increases the turn counter
    fn prompt_player(&mut self) {
        println!("Enter tile coordinates, like: x,y");
        let mut tile_coords = String::new();
        io::stdin().read_line(&mut tile_coords)
            .expect("Failed to read tile coordinates");
        
        let tile_coords: Vec<&str> = tile_coords.split(',').collect();
        println!("{:?}", tile_coords);
        // can maybe make this function recursive? 
        // todo: parse tile_coords to u8's
        // if no number chosen, or if number is outside of board size -> ask for user input again
        // check whose turn it is, if even -> X, if odd -> O;
        // check if tile wasn't already chosen
        // if already chosen -> ask again for user input 
        // if not chosen already -> add coords to board cells.
        // check if a player has won
        // if player has won -> end game
        // if player hasn't won -> continue to next turn

        // after a succesful turn -> draw board to convey new state

        self.turn_counter += 1; // does turn_counter need to be mut?
    }
}

struct Board {
    cells: HashMap<(u8, u8), PlayerValue>, // not sure if i still want a map
    // maybe i can use a (second) map to store al moves made by player X
    // if this map doesn't contain the move of player O, and the move is within bounds
    // the cell is open.
    length: u8,
}

impl Board {
    /// generate_win_combinations generates all possible win combinations for a board.
    /// It returns a Vector containing Vectors that contain
    /// cell positions in the form of a tuple/pair.
    fn generate_win_combinations(&self) -> Vec<Vec<(u8, u8)>> {
        let n = self.length;
        let mut winnable_combinations = Vec::new();
        let mut left_diagonal_combinations = Vec::new();
        let mut right_diagonal_combinations = Vec::new();
        let mut rows = Vec::new();
        let mut columns = Vec::new();
        for i in 0..n {
            for j in 0..n {
                rows.push((i, j));
                columns.push((j, i));
            }
            
            left_diagonal_combinations.push((i, i));
            right_diagonal_combinations.push((i, n-1-i));
            winnable_combinations.push(rows);
            winnable_combinations.push(columns);
            rows = Vec::new();
            columns = Vec::new();
        }

        winnable_combinations.push(left_diagonal_combinations);
        winnable_combinations.push(right_diagonal_combinations);
        return winnable_combinations;
    }

    /// draw draws the board on standard output
    fn draw(&self) {
        println!("board hellooo");
    }
}

enum PlayerValue {
    X,
    O
}