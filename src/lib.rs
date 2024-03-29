//! # Reversi
//!
//! `reversi` is a library to handle the logic of the board game of the same name.

use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::ops::{Index, IndexMut};

use gamesweet::Game;

/// Size of the game board.
const BOARDSIZE: usize = 8;

/// Reversi game.
#[derive(Clone, Debug)]
pub struct Reversi {
    board: Board<BOARDSIZE>,
}

impl Reversi {
    /// Create a new Reversi game.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Reversi {
    fn default() -> Self {
        Self {
            board: Board::new(),
        }
    }
}

impl Display for Reversi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}

impl Game for Reversi {
    type Player = Player;
    type Turn = Turn;

    /// Get the current player.
    fn player(&self) -> Player {
        self.board.player
    }

    /// Get all legal turns.
    fn turns(&self) -> Vec<Turn> {
        self.board.turns()
    }

    /// Play a turn of the game.
    fn play(&mut self, turn: Turn) -> bool {
        self.board.play(&turn)
    }

    /// Check if the game is over.
    fn over(&self) -> bool {
        self.board.over()
    }

    /// Get the winner of the game.
    ///
    /// Returns `None` if the game is still ongoing.
    fn winner(&self) -> Option<Player> {
        self.board.winner()
    }
}

/// Board on which the game is played.
///
/// Responsible for managing the placement of pieces and handling game logic.
#[derive(Clone, Debug, PartialEq)]
struct Board<const BOARDSIZE: usize> {
    squares: [[Square; BOARDSIZE]; BOARDSIZE],
    player: Player,
}

impl<const BOARDSIZE: usize> Board<BOARDSIZE> {
    /// Create a new Board.
    ///
    /// The board starts with 4 pieces in the centre.
    /// The first player is always black.
    fn new() -> Self {
        let mut squares = [[Square::Empty; BOARDSIZE]; BOARDSIZE];

        squares[BOARDSIZE / 2 - 1][BOARDSIZE / 2 - 1] = Square::Piece(Player::White);
        squares[BOARDSIZE / 2 - 1][BOARDSIZE / 2] = Square::Piece(Player::Black);
        squares[BOARDSIZE / 2][BOARDSIZE / 2 - 1] = Square::Piece(Player::Black);
        squares[BOARDSIZE / 2][BOARDSIZE / 2] = Square::Piece(Player::White);

        Self {
            squares,
            player: Player::Black,
        }
    }

    /// Get all legal turns for the current player.
    fn turns(&self) -> Vec<Turn> {
        let mut turns = Vec::new();

        // Iterate through the entire board
        for i in 0..BOARDSIZE {
            for j in 0..BOARDSIZE {
                // Sort by col, then row
                let turn = Turn::new(self.player, Position(j, i));

                // Check if each turn would be legal
                if self.is_legal(&turn) {
                    turns.push(turn);
                }
            }
        }

        turns
    }

    /// Play a turn of the game.
    fn play(&mut self, turn: &Turn) -> bool {
        // Try to play the turn
        let success = self.set_turn(turn);

        // Only switch players if opponent has a turn
        if success && self.has_turn(self.player.opponent()) {
            self.player.switch();
        }

        success
    }

    /// Check if the game is over.
    fn over(&self) -> bool {
        !self.has_turn(self.player) && !self.has_turn(self.player.opponent())
    }

    /// Get the winner of the game.
    ///
    /// Returns `None` if the game is still ongoing.
    fn winner(&self) -> Option<Player> {
        if !self.over() {
            return None;
        }

        // Count who has more pieces
        let mut count = 0;

        for i in 0..self.height() {
            for j in 0..self.width() {
                count += match self[Position(i, j)] {
                    Square::Piece(Player::Black) => 1,
                    Square::Piece(Player::White) => -1,
                    Square::Empty => 0,
                }
            }
        }

        match count.cmp(&0) {
            Ordering::Less => Some(Player::White),
            Ordering::Equal => None,
            Ordering::Greater => Some(Player::Black),
        }
    }

    /// Check if a turn is legal.
    ///
    /// Performs bounds check on `turn`.
    fn is_legal(&self, turn: &Turn) -> bool {
        // Perform bounds check
        if !self.in_bounds(turn.pos) {
            return false;
        }

        // Occupied spaces are never legal
        if self.is_occupied(turn.pos) {
            return false;
        }

        // Check legality in each direction
        for i in [-1, 0, 1].iter() {
            for j in [-1, 0, 1].iter() {
                if self.is_legal_in_direction(turn, (*i, *j)) {
                    return true;
                }
            }
        }

        false
    }

    /// Check if a turn is legal in a direction.
    fn is_legal_in_direction(&self, turn: &Turn, (dx, dy): (isize, isize)) -> bool {
        let Position(row, col) = turn.pos;

        // Check if adjacent square belongs to the opponent
        let x = (row as isize + dx) as usize;
        let y = (col as isize + dy) as usize;
        if self.get(Position(x, y)) != Some(&Square::Piece(turn.player.opponent())) {
            return false;
        }

        // Search for the player's piece as a delimiter
        for i in 2..BOARDSIZE {
            let x = (row as isize + (i as isize * dx)) as usize;
            let y = (col as isize + (i as isize * dy)) as usize;
            match self.get(Position(x, y)) {
                Some(Square::Piece(player)) if player == &turn.player => return true,
                Some(_) => continue,
                None => return false,
            }
        }

        false
    }

    /// Check if a position is in bounds.
    fn in_bounds(&self, pos: Position) -> bool {
        self.get(pos).is_some()
    }

    /// Check if a position on the board is occupied.
    ///
    /// # Panics
    ///
    /// Will panic if `pos` is out of bounds.
    fn is_occupied(&self, pos: Position) -> bool {
        self[pos].occupied()
    }

    /// Check if the current player has a legal turn.
    fn has_turn(&self, player: Player) -> bool {
        // Iterate through the entire board
        for i in 0..BOARDSIZE {
            for j in 0..BOARDSIZE {
                let turn = Turn::new(player, Position(j, i));

                // Check if turn is legal for player
                if self.is_legal(&turn) {
                    return true;
                }
            }
        }

        false
    }

    /// Set a turn on the board.
    ///
    /// Performs legality check on `turn`.
    fn set_turn(&mut self, turn: &Turn) -> bool {
        // Perform legality check
        if !self.is_legal(turn) {
            return false;
        }

        // Set the player
        self.player = turn.player;

        // Set the piece
        self[turn.pos] = Square::Piece(turn.player);

        // Flip pieces in each legal direction
        for x in [-1, 0, 1].iter() {
            for y in [-1, 0, 1].iter() {
                // Only flip if legal in direction
                if self.is_legal_in_direction(turn, (*x, *y)) {
                    // Iterate in direction
                    for i in 1..BOARDSIZE {
                        let x = (turn.pos.0 as isize + (i as isize * x)) as usize;
                        let y = (turn.pos.1 as isize + (i as isize * y)) as usize;
                        let pos = Position(x, y);

                        // Only flip opponent's pieces (performs bounds check)
                        match self[pos] {
                            Square::Piece(ref mut player) if player == &turn.player.opponent() => {
                                player.switch()
                            }
                            _ => break,
                        }
                    }
                }
            }
        }

        true
    }

    /// Get the board height.
    fn height(&self) -> usize {
        self.squares.len()
    }

    /// Get the board width.
    fn width(&self) -> usize {
        self.squares[0].len()
    }

    /// Borrow the square at a position.
    ///
    /// Performs bounds check, and returns `None` variant on invalid position.
    fn get(&self, pos: Position) -> Option<&Square> {
        self.squares.get(pos.0)?.get(pos.1)
    }

    /// Mutably borrow the square at a position.
    ///
    /// Performs bounds check, and returns `None` variant on invalid position.
    #[allow(dead_code)]
    fn get_mut(&mut self, pos: Position) -> Option<&mut Square> {
        self.squares.get_mut(pos.0)?.get_mut(pos.1)
    }
}

impl<const BOARDSIZE: usize> Display for Board<BOARDSIZE> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Print top border
        writeln!(f, "┌───┬{}─┐", "─".repeat(2 * BOARDSIZE))?;

        // Print row of letter labels
        write!(f, "│ {} │", Square::Piece(self.player))?;
        for i in 0..BOARDSIZE {
            write!(f, " {}", (b'a' + i as u8) as char)?;
        }
        writeln!(f, " │")?;
        writeln!(f, "├───┼{}─┤", "─".repeat(2 * BOARDSIZE))?;

        // Print each row of the board
        for (i, row) in self.squares.iter().enumerate() {
            write!(f, "│ {} │", i + 1)?;
            for square in row.iter() {
                write!(f, " {}", square)?;
            }
            writeln!(f, " │")?;
        }

        // Print bottom border
        write!(f, "└───┴{}─┘", "─".repeat(2 * BOARDSIZE))
    }
}

impl<const BOARDSIZE: usize> Index<Position> for Board<BOARDSIZE> {
    type Output = Square;

    fn index(&self, pos: Position) -> &Self::Output {
        &self.squares[pos.0][pos.1]
    }
}

impl<const BOARDSIZE: usize> IndexMut<Position> for Board<BOARDSIZE> {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.squares[pos.0][pos.1]
    }
}

/// A square of the game.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Square {
    Piece(Player),
    Empty,
}

impl Square {
    /// Check if a square is occupied.
    fn occupied(&self) -> bool {
        match self {
            Square::Piece(_) => true,
            Square::Empty => false,
        }
    }
}

impl Display for Square {
    /// Display a game square.
    ///
    /// | Piece                   | Char |
    /// | ----------------------- | ---- |
    /// | `Piece(Player::Black)`  | `●`  |
    /// | `Piece(Player::White)`  | `○`  |
    /// | `Empty`                 | ` `  |
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Square::Piece(player) => write!(f, "{}", player),
            Square::Empty => write!(f, " "),
        }
    }
}

/// A player of the game.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    Black,
    White,
}

impl Player {
    /// Get the opponent of a player.
    pub fn opponent(&self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    /// Switch player to opponent.
    fn switch(&mut self) {
        *self = self.opponent();
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::Black => "●",
                Player::White => "○",
            }
        )
    }
}

/// A board position to play a piece.
#[derive(Clone, Debug, PartialEq)]
pub struct Turn {
    player: Player,
    pos: Position,
}

impl Turn {
    /// Create a new Turn.
    pub fn new(player: Player, pos: Position) -> Self {
        Self { player, pos }
    }
}

impl Display for Turn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.pos)
    }
}

/// A position on the board.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(pub usize, pub usize);

impl Display for Position {
    /// Display a position on the board.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", (b'a' + self.1 as u8) as char, self.0 + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_player_test() {
        let mut game = Reversi::new();

        // Start a new game playing as black
        assert_eq!(game.player(), Player::Black);
        assert_eq!(game.player().opponent(), Player::White);

        // Switch players
        game.board.player.switch();
        assert_eq!(game.player(), Player::White);
        assert_eq!(game.player().opponent(), Player::Black);
    }

    #[test]
    fn game_board_turns_test() {
        let mut game = Reversi::new();

        // Check legal first turns
        // NOTE: ordered by col, then row
        assert_eq!(
            game.board.turns(),
            [
                Turn::new(Player::Black, Position(3, 2)),
                Turn::new(Player::Black, Position(2, 3)),
                Turn::new(Player::Black, Position(5, 4)),
                Turn::new(Player::Black, Position(4, 5)),
            ]
        );

        // Remove all legal turns
        game.board[Position(3, 3)] = Square::Piece(Player::Black);
        game.board[Position(4, 4)] = Square::Piece(Player::Black);
        assert_eq!(game.board.turns(), []);
    }

    #[test]
    fn game_play_test() {
        let mut game = Reversi::new();

        // Play a few moves
        game.play(Turn::new(game.player(), Position(2, 3)));
        game.play(Turn::new(game.player(), Position(4, 2)));

        // Manually play turns
        let mut board = Board::new();
        board[Position(2, 3)] = Square::Piece(Player::Black);
        board[Position(3, 3)] = Square::Piece(Player::Black);
        board[Position(4, 2)] = Square::Piece(Player::White);
        board[Position(4, 3)] = Square::Piece(Player::White);
        assert_eq!(game.board, board);
    }

    #[test]
    fn game_over_test() {
        let mut game = Reversi::new();

        // Play first turn until game is over
        while !game.over() {
            let turn = game.turns()[0].clone();
            game.play(turn);
        }
    }

    #[test]
    fn game_over_early_test() {
        let mut game = Reversi::new();

        // Play game until an early end
        game.play(Turn::new(Player::Black, Position(3, 2)));
        assert!(!game.over());
        game.play(Turn::new(Player::White, Position(2, 2)));
        assert!(!game.over());
        game.play(Turn::new(Player::Black, Position(1, 2)));
        assert!(!game.over());
        game.play(Turn::new(Player::White, Position(3, 1)));
        assert!(!game.over());
        game.play(Turn::new(Player::Black, Position(4, 0)));
        assert!(!game.over());
        game.play(Turn::new(Player::White, Position(3, 5)));
        assert!(!game.over());
        game.play(Turn::new(Player::Black, Position(3, 6)));
        assert!(!game.over());
        game.play(Turn::new(Player::White, Position(4, 2)));
        assert!(!game.over());
        game.play(Turn::new(Player::Black, Position(5, 3)));
        assert!(game.over());
        assert_eq!(game.winner(), Some(Player::Black));
    }

    #[test]
    fn game_board_is_occupied_test() {
        let game = Reversi::new();

        assert!(!game.board.is_occupied(Position(0, 0)));
        assert!(!game.board.is_occupied(Position(2, 2)));
        assert!(game.board.is_occupied(Position(4, 4)));
        assert!(!game.board.is_occupied(Position(6, 6)));
    }

    #[test]
    fn game_board_is_legal_test() {
        let game = Reversi::new();
        let board = &game.board;

        // Legal spaces for black's first turn
        assert!(board.is_legal(&Turn::new(game.player(), Position(2, 3))));
        assert!(board.is_legal(&Turn::new(game.player(), Position(3, 2))));
        assert!(board.is_legal(&Turn::new(game.player(), Position(4, 5))));
        assert!(board.is_legal(&Turn::new(game.player(), Position(5, 4))));

        // Legal spaces for white's first turn
        assert!(!board.is_legal(&Turn::new(game.player(), Position(2, 4))));
        assert!(!board.is_legal(&Turn::new(game.player(), Position(4, 2))));
        assert!(!board.is_legal(&Turn::new(game.player(), Position(5, 3))));
        assert!(!board.is_legal(&Turn::new(game.player(), Position(3, 5))));

        // Occupied spaces
        assert!(!board.is_legal(&Turn::new(game.player(), Position(3, 3))));
        assert!(!board.is_legal(&Turn::new(game.player(), Position(4, 4))));

        // Spaces on the edge of the board
        assert!(!board.is_legal(&Turn::new(game.player(), Position(0, 0))));
        assert!(!board.is_legal(&Turn::new(game.player(), Position(7, 7))));

        // Invalid spaces
        assert!(!board.is_legal(&Turn::new(game.player(), Position(8, 8))));
    }
}
