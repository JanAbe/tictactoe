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
        Err(_) => 3, // if no number is provided, give board a default length of 3
    };

    let mut game = Game::new(board_length);
    println!("");
    game.board.draw(); // show empty board, before anyone has made a move
    game.play();
}

struct Game{
    status: GameStatus,
    turn_counter: u8,
    board: Board,
    win_combinations: Vec<Vec<(u8,u8)>>,
}

#[derive(PartialEq, Eq)]
enum GameStatus {
    Playing,
    Drawn,
    Over,
}

impl Game{
    /// new creates a new instance of the Game struct
    fn new(length: u8) -> Game {
        let board = Board::new(length);
        Game {
            status: GameStatus::Playing,
            turn_counter: 0,
            win_combinations: board.generate_win_combinations(),
            board,
        }
    }

    /// play starts a game of tic tac toe
    fn play(&mut self) {
        while self.status == GameStatus::Playing {
            self.play_turn();
        }

        if self.status == GameStatus::Drawn {
            println!("Game over, it is a draw!");
            return;
        }

        if self.turn_counter % 2 == 0 {
            println!("Game over, player X has won!");
        } else {
            println!("Game over, player O has won!");
        }
    }

    /// get_status gets the status of the game that is being played.
    /// It looks if the status is over by looking if a player has won by looking at all possible win combinations,
    /// for each win combination it looks which player has chosen the tile, and if all tiles within the
    /// winnable combination are chosen by the same player.
    /// If no player can win, the status becomes Drawn.
    /// If neither are the case, the status stays Playing.
    fn get_status(&self) -> GameStatus {
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
            return GameStatus::Over;
        }

        if self.turn_counter == self.board.length*self.board.length {
            return GameStatus::Drawn;
        }

        return GameStatus::Playing;
    }

    /// play_turn plays a turn of the game. It prompts a player to input his new move, 
    /// updates the board and increases the turn counter.
    fn play_turn(&mut self) {
        self.turn_counter += 1;

        let is_xs_turn = self.turn_counter % 2 == 0;
        if is_xs_turn {
            println!("(Player X's turn)");
        } else {
            println!("(Player O's turn)");
        }

        println!("Enter tile coordinates, like: x,y");
        let mut tile_coords = String::new();
        io::stdin().read_line(&mut tile_coords)
            .expect("Failed to read tile coordinates");
        
        let tile_coords: Vec<&str> = tile_coords.trim().split(',').collect();
        if tile_coords.len() != 2 {
            self.play_turn()
        }

        let x: u8 = match tile_coords[0].parse() {
            Ok(num) => num,
            Err(_) => 0 // improve this
        };

        let y: u8 = match tile_coords[1].parse() {
            Ok(num) => num,
            Err(_) => 0
        };

        let n = self.board.length;
        if x > n || y > n {
            println!("Provided coords are outside of board space. Choose again:");
            self.play_turn();
        }

        let tile_is_taken = self.board.cells.contains_key(&(x,y));
        if tile_is_taken {
            println!("Tile is already chosen. Choose another tile.");
            self.play_turn();
        }

        if is_xs_turn {
            self.board.cells.insert((x,y), PlayerValue::X);
        } else {
            self.board.cells.insert((x,y), PlayerValue::O);
        }

        println!("");
        self.board.draw();
        self.status = self.get_status();
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