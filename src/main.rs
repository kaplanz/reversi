use mcts::Game;
use reversi::{Player, State, Turn};
use std::io::{self, Write};

fn main() {
    let mut game = Reversi::new();

    // Start a new game
    println!("Welcome to Reversi!");

    // Play rounds until the game is over
    while !game.state.is_over() {
        game.play_round();
    }

    // Announce the winner
    print!("Game over: ");
    if let Some(player) = game.state.get_winner() {
        println!("{:?} wins!", player);
    } else {
        println!("It's a tie!");
    }
}

/// Store the current game as a State with a collection of turns.
struct Reversi {
    state: State,
    turns: Vec<Turn>,
}

impl Reversi {
    /// Create a new Reversi game.
    fn new() -> Reversi {
        Reversi {
            state: State::new(),
            turns: Vec::new(),
        }
    }

    /// Play a round of the game.
    fn play_round(&mut self) {
        // Print current state of game
        println!("Turn #{}", self.turns.len() + 1);
        println!();
        println!("{}", self.state);

        // Get all legal actions
        let legal_turns = self.state.get_actions();
        // If none are available...
        if legal_turns.is_empty() {
            println!("No available turns for {:?}", self.state.get_player());
            // ... switch to the next player...
            self.state.switch_player();
            // ... then return
            return;
        }

        // Print available turns
        println!("Available turns for {:?}:", self.state.get_player());
        for turn in self.state.get_actions() {
            println!("{}", turn);
        }

        // Get a turn from either the user or computer
        let mut turn;
        if self.state.get_player() == Player::Black {
            // Use the MCTS algorithm to get a move
            turn = self.state.mcts();
            println!("Computer plays: {}", turn)
        } else {
            // Prompt user to take their turn
            print!("Take your turn: ");
            io::stdout().flush().unwrap();

            turn = self.get_turn();

            while (turn == Turn::Invalid) || !self.state.is_legal(&turn) {
                print!("Invalid. Please try again: ");
                io::stdout().flush().unwrap();

                turn = self.get_turn();
            }
        }

        // Play turn
        self.set_turn(turn);

        println!();
    }

    /// Prompt the player for their turn.
    fn get_turn(&self) -> Turn {
        loop {
            // Get user input
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Error: could not parse input.");

            // Process input
            let input = input.trim().as_bytes();

            if input.len() != 2 {
                print!("Bad input. Please try again: ");
                io::stdout().flush().unwrap();

                continue;
            }

            return Turn::new(
                input[1].checked_sub(b'1').unwrap_or(0) as usize,
                input[0].checked_sub(b'a').unwrap_or(0) as usize,
            );
        }
    }

    /// Play a turn of the game.
    fn set_turn(&mut self, turn: Turn) {
        self.state.play(&turn);

        self.turns.push(turn);
    }
}

impl From<State> for Reversi {
    /// Create a game from an existing State.
    fn from(state: State) -> Self {
        Reversi {
            state,
            turns: Vec::new(),
        }
    }
}
