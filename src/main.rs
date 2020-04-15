use std::io;
use std::cmp::Eq;
use std::collections::HashMap;

fn main() {
    const BOARD_LENGTH: u8 = 3;
    let mut game = Game::new(BOARD_LENGTH);
    game.board.draw(); // show empty board, before anyone has made a move
    game.play();
}

struct Game{
    turn_counter: u8,
    board: Board,
    win_combinations: Vec<Vec<(u8,u8)>>,
}

#[derive(PartialEq, Eq)]
enum GameState {
    Playing,
    Drawn,
    Over,
}

impl Game{
    /// new creates a new instance of the Game struct
    fn new(length: u8) -> Game {
        let board = Board::new(length);
        Game {
            turn_counter: 0,
            win_combinations: board.generate_win_combinations(),
            board,
        }
    }

    /// play starts a game of tic tac toe
    fn play(&mut self) {
        loop {
            match self.play_turn() {
                GameState::Playing => {
                    continue;
                },
                GameState::Drawn => {
                    println!("Game over, it is a draw!");
                    break;
                },
                GameState::Over => {
                    if self.turn_counter % 2 == 0 {
                        println!("Game over, player X has won!");
                    } else {
                        println!("Game over, player O has won!");
                    }
                    break;
                }
            }
        }
    }

    /// get_state gets the state of the game that is being played.
    /// It looks if the state is over by looking if a player has won by looking at all possible win combinations,
    /// for each win combination it looks which player has chosen the tile, and if all tiles within the
    /// winnable combination are chosen by the same player.
    /// If no player can win, the state becomes Drawn.
    /// If neither are the case, the state stays Playing.
    fn get_state(&self) -> GameState {
        let mut player_won = false;
        for i in 0..self.win_combinations.len() {
            player_won = match self.board.cells.get(&self.win_combinations[i][0]) {
                Some(first_tile) => 
                    match self.board.cells.get(&self.win_combinations[i][1]) {
                        Some(second_tile) => 
                            match self.board.cells.get(&self.win_combinations[i][2]) {
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

        if player_won {
            return GameState::Over;
        }

        if self.turn_counter == self.board.length*self.board.length {
            return GameState::Drawn;
        }

        return GameState::Playing;
    }

    /// play_turn plays a turn of the game. It prompts a player to input his new move, 
    /// updates the board and increases the turn counter, returning the new GameState.
    fn play_turn(&mut self) -> GameState {
        self.turn_counter += 1;

        let is_xs_turn = self.turn_counter % 2 == 0;
        if is_xs_turn {
            println!("(Player X's turn)");
        } else {
            println!("(Player O's turn)");
        }

        let tile_coords = self.prompt_player();

        if is_xs_turn {
            self.board.cells.insert(tile_coords, PlayerValue::X);
        } else {
            self.board.cells.insert(tile_coords, PlayerValue::O);
        }

        println!("");
        self.board.draw();
        return self.get_state();
    }

    /// prompt_player prompts a player to provide the coords of a tile
    /// and returns these coords, correctly parsed
    fn prompt_player(&self) -> (u8,u8) {
        loop {
            // ask and get input
            println!("Enter tile coordinates, like: x,y");
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read tile coordinates");
            
            // turn provided input into usable datastructure
            let tile_coords: Vec<&str> = input.trim().split(',').collect();
            if tile_coords.len() != 2 {
                continue;
            }

            // parse input into u8
            let x: u8 = match tile_coords[0].parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let y: u8 = match tile_coords[1].parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            let n = self.board.length;
            if x > n || y > n {
                println!("Provided coords are outside of board space. Choose again:");
                continue;
            }

            let tile_is_taken = self.board.cells.contains_key(&(x,y));
            if tile_is_taken {
                println!("Tile is already chosen. Choose another tile.");
                continue;
            }

            return (x,y);
        }
    }
}

struct Board {
    cells: HashMap<(u8, u8), PlayerValue>,
    length: u8,
}

impl Board {
    /// new creates a new instance of the Board struct
    fn new(length: u8) -> Board {
        Board {
            cells: HashMap::new(),
            length,
        }
    }
    
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