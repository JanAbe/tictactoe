// cargo run -> builds and executes the program
// ./target/debug/tic_tac_toe.exe is the executable that has been made by cargo run

use std::io;
use std::cmp::Eq;
use std::collections::HashMap;

fn main() {
    println!("Enter your wishes board length below: (default length is 3)");
    let mut board_length = String::new();
    io::stdin().read_line(&mut board_length)
        .expect("Failed to read board length input");
    
    let board_length: u8 = match board_length.trim().parse() {
        Ok(num) => num,
        Err(_) => 3, // if no number is provided, give board a default length
    };

    let mut board = Board {
        cells: HashMap::new(),
        length: board_length
    };

    println!("\n");
    board.draw();

    let mut game = Game {
        is_over: false,
        turn_counter: 1,
        board: board,
    };

    game.play();
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

        // why does this do the reversed of what i expect?
        // maybe the turn counter doesn't get increased before i check if game won or something, idk :c
        if self.turn_counter % 2 == 0 {
            println!("Game over, player O has won!");
        } else {
            println!("Game over, player X has won!");
        }
    }

    // check_player_won checks if a player has won by looking at all possible win combinations.
    // for each win combination it looks which player has chosen the tile, and if all tiles within the
    // winnable combination are chosen by teh same player.
    fn check_player_won(&self) -> bool {
        // need to store this in game/board field. It is unecessary to generate all winnable combination each time this function is called,
        // as it doesn't change.
        let win_combinations = self.board.generate_win_combinations();
        let mut player_won = true;
        // todo: atm is only works for a board of size 3
        // need to make this size independent!!!
        for i in 0..win_combinations.len() {
        //     for j in 0..win_combinations[i].len()-1 {
        //         player_won = match self.board.cells.get(&win_combinations[i][j]) {
        //             Some(current_tile) =>
        //                 match self.board.cells.get(&win_combinations[i][j+1]) {
        //                     Some(next_tile) => (current_tile == next_tile) && player_won,
        //                     _ => false
        //                 }
        //             _ => false
        //         };

        //     }
        //     if player_won {
        //         break;
        //     }

            player_won = match self.board.cells.get(&win_combinations[i][0]) {
                Some(first_tile) => 
                    match self.board.cells.get(&win_combinations[i][1]) {
                        Some(second_tile) => 
                            match self.board.cells.get(&win_combinations[i][2]) {
                                Some(third_tile) => (first_tile == second_tile) && (first_tile == third_tile),
                                _ => false
                            }
                        _ => false
                    }
                _ => false
            };

            if player_won {
                break;
            }
        }

        return player_won;
    }

    /// prompt_player prompts a player to input his new move, 
    /// updates the board and increases the turn counter
    fn prompt_player(&mut self) {
        println!("Enter tile coordinates, like: x,y");
        let mut tile_coords = String::new();
        io::stdin().read_line(&mut tile_coords)
            .expect("Failed to read tile coordinates");
        
        let tile_coords: Vec<&str> = tile_coords.trim().split(',').collect();
        if tile_coords.len() != 2 {
            self.prompt_player();
        }

        // dit in een loop stoppen, dan bij Err(_) => continue schrijven ?
        // maar dan heb ik wel een loop in een loop, omdat deze functie wordt aangeroepen in de game loop.
        // Weet niet of dat erg is.
        let x: u8 = match tile_coords[0].parse() {
            Ok(num) => num,
            Err(_) => 0 //self.prompt_player(), // if parsing fails -> reprompt the user
        };

        let y: u8 = match tile_coords[1].parse() {
            Ok(num) => num,
            Err(_) => 0 //self.prompt_player(),
        };

        let n = self.board.length;
        if x > n || y > n {
            println!("Provided coords are outside of board space. Choose again:");
            self.prompt_player();
        }

        let tile_is_taken = self.board.cells.contains_key(&(x,y));
        if tile_is_taken {
            println!("Tile is already chosen. Chose another tile.");
            self.prompt_player();
        }

        if self.turn_counter % 2 == 0 {
            self.board.cells.insert((x,y), PlayerValue::X);
        } else {
            self.board.cells.insert((x,y), PlayerValue::O);
        }

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
        println!("");
        self.board.draw();
        self.is_over = self.check_player_won();
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

    /// draw draws the board and displays it on standard output
    fn draw(&self) {
        /*
            // todo: improve display of first row
            // atm each cell has a width of 3
            // and for the first row in the middle, like _i_, the column number is placed.
            // Problem: it doesn't work when the column_number becomes longer than 1 digit
            // e.g. 12 or 44. It screws up the mark up of the first row.
            // so need to build something that checks if the board_length is 1 digit, 2 digits or 3 digits long
            // (can use the fact that the board length can never be longer than 255 (so need to support 3 digits max))
            // let w: usize;
            // if n < 10 {
            //     w = 3; // -5-
            // } else if n > 99 {
            //     w = 5; // -244-
            // } else {
            //     w = 4; // -23-
            // }
        */

        let mut output: Vec<String> = Vec::new();
        output.push(String::from("  "));

        let n = self.length;
        for i in 0..n {
            output.push(format!(" {}  ", i));
        }
        output.push(String::from("\n"));

        output.push(String::from(" "));
        for _ in 0..n {
            output.push(String::from("----"));
        }
        output.push(String::from("-"));
        output.push(String::from("\n"));

        for i in 0..n {
            output.push(format!("{}|", i));
            for j in 0..n {
                match self.cells.get(&(i,j)) {
                    Some(value) if value == &PlayerValue::O => output.push(String::from(" O |")),
                    Some(value) if value == &PlayerValue::X => output.push(String::from(" X |")),
                    _ => output.push(String::from("   |")),
                }
            }

            output.push(String::from("\n"));
            output.push(String::from(" "));
            for _ in 0..n {
                output.push(String::from("----"));
            }
            output.push(String::from("-"));
            output.push(String::from("\n"));
        }

        println!("{}", output.join(""));
    }
}

#[derive(PartialEq, Eq)]
enum PlayerValue {
    X,
    O
}