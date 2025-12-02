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
            selected_piece: None,
            valid_moves: Vec::new(),
        }
    }
}

impl GameState {
    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> MoveResult {
        if self.status != GameStatus::InProgress {
            return MoveResult::Invalid;
        }
        if !is_valid_move(&self.board, from, to, self.current_turn) {
            return MoveResult::Invalid;
        }

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
