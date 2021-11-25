use std::io::{self, Write};

use gamesweet::{ai, Config, Game, TurnFn};
use reversi::{Player, Position, Reversi, Turn};

fn main() {
    // Initialize logger
    env_logger::Builder::new()
        .default_format()
        .format_indent(Some(12))
        .format_timestamp(None)
        .parse_default_env()
        .init();

    // Create a Reversi game
    let game = Reversi::new();

    // Define the game config
    let p1 = (Player::Black, ask_human as TurnFn<Reversi>);
    let p2 = (Player::White, ai::mcts::run as TurnFn<Reversi>);
    let config = Config::new(p1, p2);

    // Run the game loop
    game.main(config);
}

fn ask_human(game: &Reversi) -> Turn {
    // Print available turns
    println!("Available turns:");
    for turn in game.turns() {
        println!("{}", turn);
    }

    // Query the game for the player
    let player = Game::player(game);

    // Loop until user provides a valid turn
    loop {
        // Print prompt
        print!("[{}] >> ", &player);
        io::stdout().flush().unwrap();

        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        // Process input
        let input = input.trim().as_bytes();
        if input.is_empty() {
            continue;
        }

        // Validate input
        if input.len() != 2 {
            eprintln!("error: invalid input");
            continue;
        }

        // Parse input
        let row = match input[1].checked_sub(b'1') {
            Some(row) => row as usize,
            None => {
                eprintln!("error: invalid row");
                continue;
            }
        };
        let col = match input[0].checked_sub(b'a') {
            Some(col) => col as usize,
            None => {
                eprintln!("error: invalid col");
                continue;
            }
        };
        let pos = Position(row, col);

        return Turn::new(player, pos);
    }
}
