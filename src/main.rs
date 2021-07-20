use std::{io, process};
use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut game = create_default_tic_tac_toe();

    let result = game.run_game();
    game.default_result_game(result);
}

pub fn create_default_tic_tac_toe() -> TicTacToe {
    let mut game = TicTacToe { 
        moves: HashMap::new(),
        remaining_moves: vec![
            String::from("a1"), String::from("a2"), String::from("a3"),
            String::from("b1"), String::from("b2"), String::from("b3"),
            String::from("c1"), String::from("c2"), String::from("c3"),
        ],
        screen: [
            String::from("  A B C"),
            String::from("1 - - -"),
            String::from("2 - - -"),
            String::from("3 - - -"),
        ],
    };

    game.moves.insert(String::from("a1"), 0);
    game.moves.insert(String::from("a2"), 0);
    game.moves.insert(String::from("a3"), 0);
    game.moves.insert(String::from("b1"), 0);
    game.moves.insert(String::from("b2"), 0);
    game.moves.insert(String::from("b3"), 0);
    game.moves.insert(String::from("c1"), 0);
    game.moves.insert(String::from("c2"), 0);
    game.moves.insert(String::from("c3"), 0);

    return game;
}

/// Create a TicTacToe game
/// moves = states of all moves,
/// remaining_moves = player's and ai's remaining moves
/// screen = screen of the current state game
#[derive(Debug)]
pub struct TicTacToe {
    pub moves: HashMap<String, u32>,
    pub remaining_moves: Vec<String>,
    pub screen: [String; 4],
}

impl TicTacToe {
    /// makes next ai move
    fn ai_move(&mut self, field: &str) -> bool {
        self.moves.insert(String::from(field), 2_u32);
        let position = self.remaining_moves.iter()
            .position(|value| { value == field });
        self.remaining_moves.remove(position.unwrap());
        self.update_screen(field, "O");
        println!("Computer selected '{}'", field);

        return true;
    }

    /// This function checks wheter ai can do next move.
    /// mode: must be 1 or 2
    /// mode = 1: checks moves against the player ex: a1=X, b1=X, c1=N -> c1=O
    /// mode = 2: the move where the computer wins ex: a1=O, b1=O, c1=N -> c1=O
    /// return false when it cannot make specific move
    fn check_ai_move(&mut self, mode: &u32) -> bool {
        //horizontal
        //a1, b1, c1
        if self.moves.get("a1") == Some(mode) && self.moves.get("b1") == Some(mode) && self.moves.get("c1") == Some(&0_u32) {
            return self.ai_move("c1");
        } else if self.moves.get("a1") == Some(mode) && self.moves.get("b1") == Some(mode) && self.moves.get("c1") == Some(mode) {
            return self.ai_move("b1");
        } else if self.moves.get("a1") == Some(&0_u32) && self.moves.get("b1") == Some(&1_u32) && self.moves.get("c1") == Some(mode) {
            return self.ai_move("a1");
        }
        //a2, b2, c2
        else if self.moves.get("a2") == Some(mode) && self.moves.get("b2") == Some(mode) && self.moves.get("c2") == Some(&0_u32) {
            return self.ai_move("c2");
        } else if self.moves.get("a2") == Some(mode) && self.moves.get("b2") == Some(&0_u32) && self.moves.get("c2") == Some(mode) {
            return self.ai_move("b2");
        } else if self.moves.get("a2") == Some(&0_u32) && self.moves.get("b2") == Some(mode) && self.moves.get("c2") == Some(mode) {
            return self.ai_move("a2");
        }
        // a3, b3, c3
        else if self.moves.get("a3") == Some(mode) && self.moves.get("b3") == Some(mode) && self.moves.get("c3") == Some(&0_u32) {
            return self.ai_move("c3");
        } else if self.moves.get("a3") == Some(mode) && self.moves.get("b3") == Some(&0_u32) && self.moves.get("c3") == Some(mode) {
            return self.ai_move("b3");
        } else if self.moves.get("a3") == Some(&0_u32) && self.moves.get("b3") == Some(mode) && self.moves.get("c3") == Some(mode) {
            return self.ai_move("a3");
        }
        //vertical
        //a1, a2, a3
        else if self.moves.get("a1") == Some(mode) && self.moves.get("a2") == Some(mode) && self.moves.get("a3") == Some(&0_u32) {
            return self.ai_move("a3");
        } else if self.moves.get("a1") == Some(mode) && self.moves.get("a2") == Some(&0_u32) && self.moves.get("a3") == Some(mode) {
            return self.ai_move("a2");
        } else if self.moves.get("a1") == Some(&0_u32) && self.moves.get("a2") == Some(mode) && self.moves.get("a3") == Some(mode) {
            return self.ai_move("a1");
        }
        //b1, b2, b3
        else if self.moves.get("b1") == Some(mode) && self.moves.get("b2") == Some(mode) && self.moves.get("b3") == Some(&0_u32) {
            return self.ai_move("b3");
        } else if self.moves.get("b1") == Some(mode) && self.moves.get("b2") == Some(&0_u32) && self.moves.get("b3") == Some(mode) {
            return self.ai_move("b2");
        } else if self.moves.get("b1") == Some(&0_u32) && self.moves.get("b2") == Some(mode) && self.moves.get("b3") == Some(mode) {
            return self.ai_move("b1");
        }
        //c1, c2, c3
        else if self.moves.get("c1") == Some(mode) && self.moves.get("c2") == Some(mode) && self.moves.get("c3") == Some(&0_u32) {
            return self.ai_move("c3");
        } else if self.moves.get("c1") == Some(mode) && self.moves.get("c2") == Some(&0_u32) && self.moves.get("c3") == Some(mode) {
            return self.ai_move("c2");
        } else if self.moves.get("c1") == Some(&0_u32) && self.moves.get("c2") == Some(mode) && self.moves.get("c3") == Some(mode) {
            return self.ai_move("c1");
        }
        //oblique
        //a1, b2, c3
        else if self.moves.get("a1") == Some(mode) && self.moves.get("b2") == Some(mode) && self.moves.get("c3") == Some(&0_u32) {
            return self.ai_move("c3");
        } else if self.moves.get("a1") == Some(mode) && self.moves.get("b2") == Some(&0_u32) && self.moves.get("c3") == Some(mode) {
            return self.ai_move("b2");
        } else if self.moves.get("a1") == Some(&0_u32) && self.moves.get("b2") == Some(mode) && self.moves.get("c3") == Some(mode) {
            return self.ai_move("a1");
        }
        //a3, b2, c1
        else if self.moves.get("a3") == Some(mode) && self.moves.get("b2") == Some(mode) && self.moves.get("c1") == Some(&0_u32) {
            return self.ai_move("c1");
        } else if self.moves.get("a3") == Some(mode) && self.moves.get("b2") == Some(&0_u32) && self.moves.get("c1") == Some(mode) {
            return self.ai_move("b2");
        } else if self.moves.get("a3") == Some(&0_u32) && self.moves.get("b2") == Some(mode) && self.moves.get("c1") == Some(mode) {
           return self.ai_move("a3");
        }

        return false;
    }

    /// Check wheter ai or player win the game.
    /// Return false otherwise.
    fn check_win_status(&self, select: &u32) -> bool {
        // horizontal
        if self.moves.get("a1") == Some(select) && self.moves.get("b1") == Some(select) && self.moves.get("c1") == Some(select) {
            return true;
        } else if self.moves.get("a2") == Some(select) && self.moves.get("b2") == Some(select) && self.moves.get("c2") == Some(select) {
            return true;
        } else if self.moves.get("a3") == Some(select) && self.moves.get("b3") == Some(select) && self.moves.get("c3") == Some(select) {
            return true;
        }
        //vertical
        else if self.moves.get("a1") == Some(select) && self.moves.get("a2") == Some(select) && self.moves.get("a3") == Some(select) {
            return true;
        } else if self.moves.get("b1") == Some(select) && self.moves.get("b2") == Some(select) && self.moves.get("b3") == Some(select) {
            return true;
        } else if self.moves.get("c1") == Some(select) && self.moves.get("c2") == Some(select) && self.moves.get("c3") == Some(select) {
            return true;
        }
        //oblique
        else if self.moves.get("a1") == Some(select) && self.moves.get("b2") == Some(select) && self.moves.get("c3") == Some(select) {
            return true;
        } else if self.moves.get("a3") == Some(select) && self.moves.get("b2") == Some(select) && self.moves.get("c1") == Some(select) {
            return true;
        }
        false
    }

    /// This method is optional. The method print text depending on the parameter result.
    /// By default, the result should be the result of the method run_game()
    pub fn default_result_game(&self, result: i32) {
        match result {
            0 => println!("Draw! Try Again."),
            1 => println!("Congratulation You won!"),
            -1 => println!("You lost. Try Again."),
            2 => return,
            _ => {
                eprintln!("Result must be -1, 0, 1, 2!!! Got {}", result);
                process::exit(1);
            },
        }
    }

    /// Asks the player to enter his next move.
    /// If input is correct then makes selected move.
    fn next_move(&mut self) {
        loop {
            println!("Availabe moves: {:?}", self.remaining_moves);
            println!("Enter your move:");
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line!");

            let position = self.remaining_moves.iter()
                .position(|value| { value == input.trim() });

            match position {
                None => {
                    continue;
                },
                _ => {
                    self.update_screen(input.trim(), "X");
                    self.moves.insert(String::from(input.trim()), 1);
                    self.remaining_moves.remove(position.unwrap());
                    break;
                },
            }
        }
    }

    /// Generates random ai's move
    fn random_ai_move(&mut self) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.remaining_moves.len() - 1);

        let current_move = self.remaining_moves.get(index).unwrap().clone();
        self.moves.insert(current_move.clone(), 2_u32);
        self.update_screen(&current_move, "O");
        println!("Computer selected '{}'", current_move);
        self.remaining_moves.remove(index);
    }

    /// run tic-tac-toe game
    /// return -1 if player lost, 0 when draw, 1 when player win, 2 when player select 2(exit).
    pub fn run_game(&mut self) -> i32 {
        println!("--- Tic Tac Toe ---");

        loop {
            let mut input = String::new();
            println!("1. Play Game\n2. Exit");

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line!");

            match input.trim() {
                "1" => {
                    loop {
                        self.next_move();
                        if self.remaining_moves.len() == 0 {
                            println!("{}\n{}\n{}\n{}", self.screen[0], self.screen[1], self.screen[2], self.screen[3]);
                            return 0;
                        }
                        let win = self.check_win_status(&1_u32);
                        if win == true {
                            println!("{}\n{}\n{}\n{}", self.screen[0], self.screen[1], self.screen[2], self.screen[3]);
                            return 1;
                        }
                        self.check_ai_move(&2_u32);
                        let win = self.check_win_status(&2_u32);
                        if win == true {
                            println!("{}\n{}\n{}\n{}", self.screen[0], self.screen[1], self.screen[2], self.screen[3]);
                            return -1;
                        }
                        if self.check_ai_move(&1_u32) == false {
                            self.random_ai_move();
                        }

                        println!("{}\n{}\n{}\n{}", self.screen[0], self.screen[1], self.screen[2], self.screen[3]);
                    }
                },
                "2" => return 2,
                _ => println!("Enter '1' or '2'"),
            }
        }
    }

    /// update game's screen
    /// position: position to be changed in the string
    /// mark: must be "X" or "O"
    fn update_screen(&mut self, position: &str, mark: &str) {
        match position {
            "a1" => { self.screen[1].replace_range(2..3, mark)},
            "a2" => { self.screen[2].replace_range(2..3, mark)},
            "a3" => { self.screen[3].replace_range(2..3, mark)},
            "b1" => { self.screen[1].replace_range(4..5, mark)},
            "b2" => { self.screen[2].replace_range(4..5, mark)},
            "b3" => { self.screen[3].replace_range(4..5, mark)},
            "c1" => { self.screen[1].replace_range(6..7, mark)},
            "c2" => { self.screen[2].replace_range(6..7, mark)},
            "c3" => { self.screen[3].replace_range(6..7, mark)},
            _ => {},
        }
    }
}
