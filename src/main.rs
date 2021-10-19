use std::io::{self, Write};

use mcts::Mcts;
use reversi::*;

fn main() {
    println!("Welcome to Reversi!");
    println!();

    let mut game = Reversi::new();
    let mut success = true;

    while !game.over() {
        if success {
            println!("{}", game);

            println!("Available turns:");
            for turn in game.turns() {
                println!("{}", turn);
            }
        } else {
            println!("error: could not play turn");
        }

        let turn = match game.player() {
            Player::Black => get_turn(game.player()),
            Player::White => game.mcts(),
        };

        success = game.play(turn);
    }

    println!("{}", game);
    match game.winner() {
        Some(player) => println!("Winner: {:?}", player),
        None => println!("It's a tie!"),
    }
}

fn get_turn(player: Player) -> Turn {
    loop {
        // Print prompt
        print!("[{:?}] >> ", &player);
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
