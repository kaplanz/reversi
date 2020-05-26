//! # Reversi
//!
//! `reversi` is a library to handle the logic of the board game with the same name.

use std::fmt;

/// Size of the board's width and height.
pub const BOARDSIZE: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Player {
    Black,
    White,
    Empty,
}

impl fmt::Display for Player {
    /// Display each player as a piece.
    ///
    /// - `Player::Black` is displayed as `"B"`
    /// - `Player::White` is displayed as `"W"`
    /// - `Player::Empty` is displayed as `"_"`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::Black => write!(f, "B"),
            Player::White => write!(f, "W"),
            Player::Empty => write!(f, "_"),
        }
    }
}

/// A single move on the board.
#[derive(Debug, PartialEq)]
pub enum Turn {
    Valid(usize, usize),
    Invalid,
}

impl Turn {
    /// Create a new Turn.
    ///
    /// Will be of the `Turn::Invalid` variant if either of
    /// `row`, `col` are out of bounds.
    pub fn new(row: usize, col: usize) -> Turn {
        if (row < BOARDSIZE) && (col < BOARDSIZE) {
            Turn::Valid(row, col)
        } else {
            Turn::Invalid
        }
    }
}

type Board = [[Player; BOARDSIZE]; BOARDSIZE];

/// The current state of the game.
///
/// Keeps track of the board, and current player.
pub struct State {
    board: Board,
    player: Player,
}

impl State {
    /// Create a new State for the beginning of a game.
    ///
    /// The board starts with 4 pieces in the centre.
    /// The first player is always black.
    pub fn new() -> State {
        let mut board = [[Player::Empty; BOARDSIZE]; BOARDSIZE];

        board[BOARDSIZE / 2 - 1][BOARDSIZE / 2 - 1] = Player::White;
        board[BOARDSIZE / 2 - 1][BOARDSIZE / 2]     = Player::Black;
        board[BOARDSIZE / 2][BOARDSIZE / 2 - 1]     = Player::Black;
        board[BOARDSIZE / 2][BOARDSIZE / 2]         = Player::White;

        State {
            board,
            player: Player::Black,
        }
    }

    /// Print the game board.
    pub fn print(&self) {
        // Print row of letter labels
        print!(" ");
        for i in 0..BOARDSIZE {
            print!(" {}", (b'a' + i as u8) as char);
        }
        print!("\n");

        // Print each for of the board
        for (i, row) in self.board.iter().enumerate() {
            print!("{}", i + 1);
            for piece in row.iter() {
                print!(" {}", piece);
            }
            print!("\n");
        }
    }

    /// Play a turn on the board.
    pub fn play_turn(&mut self, turn: &Turn) {
        if self.is_legal(turn) {
            self.set_piece(turn);
        }

        self.switch_player();
    }

    /// Check if a turn is valid.
    fn is_legal(&self, turn: &Turn) -> bool {
        // Occupied spaces are always not legal
        if self.is_occupied(turn) {
            return false;
        }

        // Check legality in each direction
        for i in [-1, 0, 1].iter() {
            for j in [-1, 0, 1].iter() {
                if self.is_legal_in_direction(turn, *i, *j) {
                    return true;
                }
            }
        }

        false
    }

    /// Check if a position on the board is occupied.
    fn is_occupied(&self, turn: &Turn) -> bool {
        if let Turn::Valid(row, col) = *turn {
            self.board[row][col] != Player::Empty
        } else {
            false
        }
    }

    /// Check if a position on the board is occupied.
    fn is_legal_in_direction(&self, turn: &Turn, delta_row: isize, delta_col: isize) -> bool {
        if let Turn::Valid(row, col) = *turn {
            let a = (row as isize + delta_row) as usize;
            let b = (col as isize + delta_col) as usize;

            // Check if adjacent piece belongs to the enemy
            if self.get_piece(&Turn::new(a, b)) != Some(self.get_enemy()) {
                return false;
            }

            // Search for the player's piece as a delimiter
            for i in 2..BOARDSIZE {
                let a = (row as isize + (i as isize * delta_row)) as usize;
                let b = (col as isize + (i as isize * delta_col)) as usize;

                if let Some(piece) = self.get_piece(&Turn::new(a, b)) {
                    if piece == self.player {
                        return true;
                    }
                } else {
                    return false
                }
            }
        }

        false
    }

    /// Get a piece on the board.
    fn get_piece(&self, turn: &Turn) -> Option<Player> {
        if let Turn::Valid(row, col) = *turn {
            Some(self.board[row][col])
        } else {
            None
        }
    }

    /// Set a piece on the board.
    fn set_piece(&mut self, turn: &Turn) {
        if let Turn::Valid(row, col) = *turn {
            self.board[row][col] = self.player
        }
    }

    /// Switch to the other player.
    ///
    /// # Panics
    ///
    /// The `switch_player` function will panic if the current player is
    /// the `Player::Empty` variant.
    fn switch_player(&mut self) {
        self.player = self.get_enemy();
    }

    /// Get the enemy player.
    ///
    /// # Panics
    ///
    /// The `get_enemy` function will panic if the current player is
    /// the `Player::Empty` variant.
    fn get_enemy(&self) -> Player {
        match self.player {
            Player::Black => Player::White,
            Player::White => Player::Black,
            _ => panic!("Current player is invalid."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_valid_turn() {
        assert_eq!(Turn::new(0, 4), Turn::Valid(0, 4));
        assert_eq!(Turn::new(3, 4), Turn::Valid(3, 4));
        assert_eq!(Turn::new(3, 7), Turn::Valid(3, 7));
    }

    #[test]
    fn new_invalid_turn() {
        assert_eq!(Turn::new(3, 8), Turn::Invalid);
        assert_eq!(Turn::new(8, 4), Turn::Invalid);
        assert_eq!(Turn::new(8, 8), Turn::Invalid);
    }

    #[test]
    fn mut_state_player_works() {
        // Start with a new game state
        let mut state = State::new();

        // First player is black, so enemy is white
        assert_eq!(state.player, Player::Black);
        assert_eq!(state.get_enemy(), Player::White);

        // Switch players
        state.switch_player();

        // First player is black, so enemy is white
        assert_eq!(state.player, Player::White);
        assert_eq!(state.get_enemy(), Player::Black);
    }

    #[test]
    fn is_occupied_works() {
        let state = State::new();

        assert_eq!(state.is_occupied(&Turn::new(0, 0)), false);
        assert_eq!(state.is_occupied(&Turn::new(2, 2)), false);
        assert_eq!(state.is_occupied(&Turn::new(4, 4)), true);
        assert_eq!(state.is_occupied(&Turn::new(6, 6)), false);
    }

    #[test]
    fn is_legal_works() {
        let state = State::new();

        // Valid spaces for black's first turn
        assert_eq!(state.is_legal(&Turn::new(2, 3)), true);
        assert_eq!(state.is_legal(&Turn::new(3, 2)), true);
        assert_eq!(state.is_legal(&Turn::new(4, 5)), true);
        assert_eq!(state.is_legal(&Turn::new(5, 4)), true);
        // Valid spaces for white's first turn
        assert_eq!(state.is_legal(&Turn::new(2, 4)), false);
        assert_eq!(state.is_legal(&Turn::new(3, 5)), false);
        // Already occupied spaces
        assert_eq!(state.is_legal(&Turn::new(3, 3)), false);
        assert_eq!(state.is_legal(&Turn::new(4, 4)), false);
        // Spaces on the edge of the board
        assert_eq!(state.is_legal(&Turn::new(0, 0)), false);
        assert_eq!(state.is_legal(&Turn::new(7, 7)), false);
        // Invalid spaces
        assert_eq!(state.is_legal(&Turn::new(8, 8)), false);
    }
}
