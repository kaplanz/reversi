use std::io::{self, Write};
use reversi::{State, Turn};

fn main() {
    let mut game = Game::new();

    loop {
        println!("Turn #{}", game.turns.len());
        println!();
        game.state.print();
        println!();

        game.take_turn();
        println!();
    }
}


/// Store the current game as a State with a collection of turns.
pub struct Game {
    state: State,
    turns: Vec<Turn>,
}

impl Game {
    /// Create a new Game.
    pub fn new() -> Game {
        Game {
            state: State::new(),
            turns: Vec::new(),
        }
    }

    pub fn take_turn(&mut self) {
        print!("Take your turn: ");
        io::stdout().flush().unwrap();

        let mut turn = self.get_turn();

        while let Turn::Invalid = turn {
            print!("Invalid. Please try again: ");
            io::stdout().flush().unwrap();

            turn = self.get_turn();
        }

        self.state.play_turn(&turn);

        self.turns.push(turn);
    }

    /// Prompt the player for their turn.
    fn get_turn(&self) -> Turn {
        loop {
            // Get user input
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                       .expect("Error: coult not parse input.");

            // Process input
            let input = input.trim().as_bytes();

            if input.len() != 2 {
                continue;
            }

            return Turn::new(input[1].checked_sub(b'1').unwrap_or(0) as usize,
                             input[0].checked_sub(b'a').unwrap_or(0) as usize);
        }
    }
}

impl From<State> for Game {
    /// Create a game from an existing State.
    fn from(state: State) -> Self {
        Game {
            state,
            turns: Vec::new(),
        }
    }
}
