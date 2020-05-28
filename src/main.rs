use std::io::{self, Write};
use reversi::{State, Turn};

fn main() {
    let mut game = Game::new();

    game.start();

    while !game.is_over() {
        game.play();
    }

    println!("Game over!");
}


/// Store the current game as a State with a collection of turns.
struct Game {
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

    /// Start the game.
    fn start(&self) {
        println!("Welcome to Reversi!");
    }

    /// Play a round of the game.
    fn play(&mut self) {
        println!("Turn #{}: {:?}", self.turns.len(), self.state.get_player());
        println!();
        self.state.print();
        println!();

        print!("Take your turn: ");
        io::stdout().flush().unwrap();

        let mut turn = self.get_turn();

        while (turn == Turn::Invalid) ||
              !self.state.is_legal(&turn) {
            print!("Invalid. Please try again: ");
            io::stdout().flush().unwrap();

            turn = self.get_turn();
        }

        self.set_turn(turn);
        println!();
    }

    /// Check if the game is over.
    fn is_over(&self) -> bool {
        false // TODO
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
                print!("Invalid. Please try again: ");
                io::stdout().flush().unwrap();

                continue;
            }

            return Turn::new(input[1].checked_sub(b'1').unwrap_or(0) as usize,
                             input[0].checked_sub(b'a').unwrap_or(0) as usize);
        }
    }

    /// Play a turn of the game.
    fn set_turn(&mut self, turn: Turn) {
        self.state.play_turn(&turn);

        self.turns.push(turn);
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
