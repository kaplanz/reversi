//! # Reversi
//!
//! `reversi` is a library to handle the logic of the board game with the same name.

use mcts::Game;
use std::cmp::Ordering;
use std::fmt::{self, Display};

/// Size of the board's width and height.
pub const BOARDSIZE: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    Black,
    White,
}

struct Piece(Option<Player>);

impl Display for Piece {
    /// Display a game piece.
    ///
    /// - `Player::Black` is displayed as `"B"`
    /// - `Player::White` is displayed as `"W"`
    /// - `None` is displayed as `"_"`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            Some(Player::Black) => write!(f, "B"),
            Some(Player::White) => write!(f, "W"),
            None => write!(f, "_"),
        }
    }
}

/// A single move on the board.
#[derive(Clone, Debug, PartialEq)]
pub enum Turn {
    Valid(usize, usize),
    Invalid,
}

impl Turn {
    /// Create a new Turn.
    ///
    /// Will be of the `Turn::Invalid` variant if either of `row`, `col` are out of bounds.
    pub fn new(row: usize, col: usize) -> Turn {
        if (row < BOARDSIZE) && (col < BOARDSIZE) {
            Turn::Valid(row, col)
        } else {
            Turn::Invalid
        }
    }
}

impl Display for Turn {
    /// Display a game turn.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Turn::Valid(row, col) = *self {
            write!(f, "{}{}", (b'a' + col as u8) as char, row + 1)
        } else {
            write!(f, "{:?}", *self)
        }
    }
}

type Board = [[Option<Player>; BOARDSIZE]; BOARDSIZE];

/// The current state of the game.
///
/// Keeps track of the board, and current player.
#[derive(Clone)]
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
        let mut board = [[None; BOARDSIZE]; BOARDSIZE];

        board[BOARDSIZE / 2 - 1][BOARDSIZE / 2 - 1] = Some(Player::White);
        board[BOARDSIZE / 2 - 1][BOARDSIZE / 2]     = Some(Player::Black);
        board[BOARDSIZE / 2][BOARDSIZE / 2 - 1]     = Some(Player::Black);
        board[BOARDSIZE / 2][BOARDSIZE / 2]         = Some(Player::White);

        State {
            board,
            player: Player::Black,
        }
    }

    /// Check if a turn is valid.
    pub fn is_legal(&self, turn: &Turn) -> bool {
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
            self.board[row][col] != None
        } else {
            false
        }
    }

    /// Check if a position on the board is occupied.
    fn is_legal_in_direction(&self, turn: &Turn, delta_row: isize, delta_col: isize) -> bool {
        if let Turn::Valid(row, col) = *turn {
            let x = (row as isize + delta_row) as usize;
            let y = (col as isize + delta_col) as usize;

            // Check if adjacent piece belongs to the enemy
            if self.get_piece(&Turn::new(x, y)) != Some(self.get_enemy()) {
                return false;
            }

            // Search for the player's piece as a delimiter
            for i in 2..BOARDSIZE {
                let x = (row as isize + (i as isize * delta_row)) as usize;
                let y = (col as isize + (i as isize * delta_col)) as usize;

                match self.get_piece(&Turn::new(x, y)) {
                    Some(player) if player == self.get_player() => return true,
                    Some(_) => continue,
                    None => return false,
                }
            }
        }

        false
    }

    /// Get a piece on the board.
    fn get_piece(&self, turn: &Turn) -> Option<Player> {
        if let Turn::Valid(row, col) = *turn {
            self.board[row][col]
        } else {
            None
        }
    }

    /// Set a piece on the board.
    ///
    /// Does not check legality of `turn`.
    fn set_piece(&mut self, turn: &Turn) {
        if let Turn::Valid(row, col) = *turn {
            self.board[row][col] = Some(self.get_player());

            // Check if pieces should be flipped in each direction
            for delta_row in [-1, 0, 1].iter() {
                for delta_col in [-1, 0, 1].iter() {
                    // Only flip if legal in direction
                    if self.is_legal_in_direction(turn, *delta_row, *delta_col) {
                        // Flip until no longer the enemy's piece
                        for i in 1..BOARDSIZE {
                            let x = (row as isize + (i as isize * delta_row)) as usize;
                            let y = (col as isize + (i as isize * delta_col)) as usize;

                            if self.get_piece(&Turn::new(x, y)) == Some(self.get_enemy()) {
                                self.board[x][y] = Some(self.get_player());
                            } else {
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Get the current enemy player.
    pub fn get_enemy(&self) -> Player {
        match self.player {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    /// Switch to the other player.
    pub fn switch_player(&mut self) {
        self.player = self.get_enemy();
    }
}

impl Display for State {
    /// Display the game board.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print row of letter labels
        write!(f, "  |")?;
        for i in 0..BOARDSIZE {
            write!(f, " {}", (b'a' + i as u8) as char)?;
        }
        writeln!(f)?;
        writeln!(f, "--+{}", "-".repeat(2 * BOARDSIZE))?;

        // Print each for of the board
        for (i, row) in self.board.iter().enumerate() {
            write!(f, "{} |", i + 1)?;
            for piece in row.iter() {
                write!(f, " {}", Piece(*piece))?;
            }
            writeln!(f)?;
        }

        write!(f, "")
    }
}

impl Game for State {
    type Player = Player;
    type Turn = Turn;

    /// Play a turn on the board.
    fn play(&mut self, turn: &Turn) {
        // Play only legal turns
        if self.is_legal(turn) {
            // Set the piece
            self.set_piece(turn);
        }

        // Switch players (or pass)
        self.switch_player();
        // If opponent has no moves, switch back
        if self.get_actions().len() == 0 {
            self.switch_player();
        }
    }

    /// Get all legal turns for the current state.
    fn get_actions(&self) -> Vec<Turn> {
        let mut legal_turns = Vec::new();

        // Iterate through the entire board
        for i in 0..BOARDSIZE {
            for j in 0..BOARDSIZE {
                // Sort by col, then row
                let turn = Turn::new(j, i);

                // Check if each turn would be legal
                if self.is_legal(&turn) {
                    legal_turns.push(turn);
                }
            }
        }

        legal_turns
    }

    /// Get the current player.
    fn get_player(&self) -> Player {
        self.player
    }

    /// Check if the game is over.
    fn is_over(&self) -> bool {
        if !self.get_actions().is_empty() {
            false
        } else {
            // Check if next player has any moves
            let mut next_state = self.clone();
            next_state.switch_player();
            next_state.get_actions().is_empty()
        }
    }

    /// Get the winner of the current state.
    ///
    /// Returns the player with the most pieces on the board.
    /// Does not check if game is over.
    fn get_winner(&self) -> Option<Player> {
        let mut tally = 0;

        for i in 0..=BOARDSIZE {
            for j in 0..=BOARDSIZE {
                tally += match self.get_piece(&Turn::new(i, j)) {
                    Some(Player::Black) => 1,
                    Some(Player::White) => -1,
                    _ => 0,
                }
            }
        }

        match tally.cmp(&0) {
            Ordering::Less => Some(Player::White),
            Ordering::Equal => None,
            Ordering::Greater => Some(Player::Black),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_turn() {
        // Valid turns (within bounds)
        assert_eq!(Turn::new(0, 4), Turn::Valid(0, 4));
        assert_eq!(Turn::new(3, 4), Turn::Valid(3, 4));
        assert_eq!(Turn::new(3, 7), Turn::Valid(3, 7));

        // Invalid turns (out of bounds)
        assert_eq!(Turn::new(3, 8), Turn::Invalid);
        assert_eq!(Turn::new(8, 4), Turn::Invalid);
        assert_eq!(Turn::new(8, 8), Turn::Invalid);
    }

    #[test]
    fn test_mut_state_player() {
        // Start with a new game state
        let mut state = State::new();

        // First player is black, so enemy is white
        assert_eq!(state.get_player(), Player::Black);
        assert_eq!(state.get_enemy(),  Player::White);

        // Switch players
        state.switch_player();

        // First player is black, so enemy is white
        assert_eq!(state.get_player(), Player::White);
        assert_eq!(state.get_enemy(),  Player::Black);
    }

    #[test]
    fn test_set_piece() {
        let mut state = State::new();

        // Play a few moves
        state.set_piece(&Turn::new(2, 3));
        state.set_piece(&Turn::new(5, 4));

        // Manually flip pieces
        let mut manual_state = State::new();
        manual_state.board[2][3] = Some(Player::Black);
        manual_state.board[3][3] = Some(Player::Black);
        manual_state.board[5][4] = Some(Player::Black);
        manual_state.board[4][4] = Some(Player::Black);
        assert_eq!(state.board, manual_state.board);
    }

    #[test]
    fn test_is_occupied() {
        let state = State::new();

        assert_eq!(state.is_occupied(&Turn::new(0, 0)), false);
        assert_eq!(state.is_occupied(&Turn::new(2, 2)), false);
        assert_eq!(state.is_occupied(&Turn::new(4, 4)), true);
        assert_eq!(state.is_occupied(&Turn::new(6, 6)), false);
    }

    #[test]
    fn test_is_legal() {
        let state = State::new();

        // Legal spaces for black's first turn
        assert_eq!(state.is_legal(&Turn::new(2, 3)), true);
        assert_eq!(state.is_legal(&Turn::new(3, 2)), true);
        assert_eq!(state.is_legal(&Turn::new(4, 5)), true);
        assert_eq!(state.is_legal(&Turn::new(5, 4)), true);

        // Legal spaces for white's first turn
        assert_eq!(state.is_legal(&Turn::new(2, 4)), false);
        assert_eq!(state.is_legal(&Turn::new(4, 2)), false);
        assert_eq!(state.is_legal(&Turn::new(5, 3)), false);
        assert_eq!(state.is_legal(&Turn::new(3, 5)), false);

        // Occupied spaces
        assert_eq!(state.is_legal(&Turn::new(3, 3)), false);
        assert_eq!(state.is_legal(&Turn::new(4, 4)), false);

        // Spaces on the edge of the board
        assert_eq!(state.is_legal(&Turn::new(0, 0)), false);
        assert_eq!(state.is_legal(&Turn::new(7, 7)), false);

        // Invalid spaces
        assert_eq!(state.is_legal(&Turn::new(8, 8)), false);
    }

    #[test]
    fn test_get_actions() {
        let mut state = State::new();

        // Note: sorted by col, then row
        assert_eq!(
            state.get_actions(),
            [
                Turn::Valid(3, 2),
                Turn::Valid(2, 3),
                Turn::Valid(5, 4),
                Turn::Valid(4, 5)
            ]
        );

        // No legal turns
        state.board[3][3] = Some(Player::Black);
        state.board[4][4] = Some(Player::Black);
        assert_eq!(state.get_actions(), []);
    }
}
