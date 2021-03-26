use reversi::*;
use std::io::{self, Write};

fn main() {
    println!("Welcome to Reversi!");
    println!();

    let mut game = Reversi::new();

    while !game.over() {
        println!("{}", game);
        let turn = get_turn(&game);
        game.play(turn);
    }

    println!("{}", game);
    match game.winner() {
        Some(player) => println!("Winner: {:?}", player),
        None => println!("It's a tie!"),
    }
}

fn get_turn(game: &Reversi) -> Turn {
    loop {
        // Print prompt
        print!("[{:?}] >> ", game.player());
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
            },
        };
        let col = match input[0].checked_sub(b'a') {
            Some(col) => col as usize,
            None => {
                eprintln!("error: invalid col");
                continue;
            },
        };
        let pos = Position(row, col);

        return Turn::new(game.player(), pos);
    }
}
