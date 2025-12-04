use crate::game::{
    board::{self, BoardState},
    piece::{Piece, PieceSide},
    rules::{is_checkmate, is_in_check, is_stalemate, is_valid_move},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum MoveResult {
    Invalid,
    Success,
    Capture(Piece),
    Check,
    CaptureAndCheck(Piece),
    Checkmate(PieceSide),
    Stalemate(PieceSide),
}

/// Represents a single move in the game history.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub enum GameStatus {
    #[default]
    InProgress,
    RedWins,
    BlackWins,
    Draw,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameState {
    pub board: BoardState,
    pub current_turn: PieceSide,
    pub status: GameStatus,
    pub move_history: Vec<Move>,
    #[serde(skip)]
    pub selected_piece: Option<(usize, usize)>,
    #[serde(skip)]
    pub valid_moves: Vec<(usize, usize)>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: board::init_board(),
            current_turn: PieceSide::Red,
            status: GameStatus::InProgress,
            move_history: Vec::new(),
            selected_piece: None,
            valid_moves: Vec::new(),
        }
    }
}

impl GameState {
    /// Returns true if it's Black's turn (AI's turn in Player vs AI mode).
    pub fn is_ai_turn(&self) -> bool {
        self.current_turn == PieceSide::Black
    }

    /// Converts a board position (row, col) to UCI coordinate format (e.g., "a0", "i9").
    pub fn pos_to_uci(pos: (usize, usize)) -> String {
        let col_char = (b'a' + pos.1 as u8) as char;
        let row_char = char::from_digit(9 - pos.0 as u32, 10).unwrap();
        format!("{}{}", col_char, row_char)
    }

    /// Converts a UCI coordinate (e.g., "a0") to board position (row, col).
    pub fn uci_to_pos(uci: &str) -> Option<(usize, usize)> {
        let chars: Vec<char> = uci.chars().collect();
        if chars.len() != 2 {
            return None;
        }
        let col = (chars[0] as u8).checked_sub(b'a')? as usize;
        let row = 9 - chars[1].to_digit(10)? as usize;
        if col < 9 && row < 10 {
            Some((row, col))
        } else {
            None
        }
    }

    /// Converts a move to UCI format (e.g., "a0a1").
    pub fn move_to_uci(m: &Move) -> String {
        format!("{}{}", Self::pos_to_uci(m.from), Self::pos_to_uci(m.to))
    }

    /// Parses a UCI move string (e.g., "a0a1") into a Move.
    pub fn uci_to_move(uci: &str) -> Option<Move> {
        if uci.len() != 4 {
            return None;
        }
        let from = Self::uci_to_pos(&uci[0..2])?;
        let to = Self::uci_to_pos(&uci[2..4])?;
        Some(Move { from, to })
    }

    /// Returns the move history in UCI format for engine position command.
    pub fn moves_to_uci(&self) -> String {
        self.move_history
            .iter()
            .map(Self::move_to_uci)
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> MoveResult {
        if self.status != GameStatus::InProgress {
            return MoveResult::Invalid;
        }
        if !is_valid_move(&self.board, from, to, self.current_turn) {
            return MoveResult::Invalid;
        }

        self.move_history.push(Move { from, to });

        let piece = self.board[from.0][from.1].take();
        let captured = self.board[to.0][to.1].take();
        self.board[to.0][to.1] = piece;

        let moving_side = self.current_turn;
        self.current_turn = match self.current_turn {
            PieceSide::Red => PieceSide::Black,
            PieceSide::Black => PieceSide::Red,
        };

        let was_stalemate = self.update_game_status();
        let opponent_in_check = is_in_check(&self.board, self.current_turn);
        if self.status == GameStatus::RedWins || self.status == GameStatus::BlackWins {
            if was_stalemate {
                MoveResult::Stalemate(moving_side)
            } else {
                MoveResult::Checkmate(moving_side)
            }
        } else if let Some(captured_piece) = captured {
            if opponent_in_check {
                MoveResult::CaptureAndCheck(captured_piece)
            } else {
                MoveResult::Capture(captured_piece)
            }
        } else if opponent_in_check {
            MoveResult::Check
        } else {
            MoveResult::Success
        }
    }

    fn update_game_status(&mut self) -> bool {
        if is_checkmate(&self.board, self.current_turn) {
            self.status = match self.current_turn {
                PieceSide::Red => GameStatus::BlackWins,
                PieceSide::Black => GameStatus::RedWins,
            };
            false
        } else if is_stalemate(&self.board, self.current_turn) {
            self.status = match self.current_turn {
                PieceSide::Red => GameStatus::BlackWins,
                PieceSide::Black => GameStatus::RedWins,
            };
            true
        } else {
            false
        }
    }
}
