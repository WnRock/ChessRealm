use crate::{
    game::{piece::PieceSide, rules::get_valid_moves, state::MoveResult},
    ui::{
        app::ChessRealm,
        state::{GameMode, PieceAnimation, PopupTip},
    },
};

impl ChessRealm {
    /// Handles a click on the board at the given position.
    pub fn handle_board_click(&mut self, row: usize, col: usize) {
        if self.should_block_input() {
            return;
        }

        let clicked_pos = (row, col);

        if let Some(selected_pos) = self.game.selected_piece {
            if self.game.valid_moves.contains(&clicked_pos) {
                let moving_piece = self.game.board[selected_pos.0][selected_pos.1];

                let result = self.game.make_move(selected_pos, clicked_pos);

                if !matches!(result, MoveResult::Invalid) {
                    if let Some(piece) = moving_piece {
                        self.ui.piece_animation =
                            Some(PieceAnimation::new(piece, selected_pos, clicked_pos));
                    }
                }

                self.handle_move_result(result);

                self.game.selected_piece = None;
                self.game.valid_moves.clear();

                self.check_ai_turn();
                return;
            }

            if selected_pos == clicked_pos {
                self.game.selected_piece = None;
                self.game.valid_moves.clear();
                return;
            }

            if let Some(piece) = self.game.board[row][col] {
                if piece.side == self.game.current_turn {
                    self.game.selected_piece = Some(clicked_pos);
                    self.game.valid_moves =
                        get_valid_moves(&self.game.board, clicked_pos, self.game.current_turn);
                    return;
                }
            }

            self.game.selected_piece = None;
            self.game.valid_moves.clear();
        } else if let Some(piece) = self.game.board[row][col] {
            if piece.side == self.game.current_turn {
                self.game.selected_piece = Some(clicked_pos);
                self.game.valid_moves =
                    get_valid_moves(&self.game.board, clicked_pos, self.game.current_turn);
            }
        }
    }

    /// Handles the result of a move, showing appropriate popups.
    pub fn handle_move_result(&mut self, result: MoveResult) {
        match result {
            MoveResult::Capture(_piece) => {
                self.ui.popup = Some(PopupTip::new("吃".to_string()));
            }
            MoveResult::Check => {
                self.ui.popup = Some(PopupTip::new("将".to_string()));
            }
            MoveResult::CaptureAndCheck(_piece) => {
                self.ui.popup = Some(PopupTip::new("将".to_string()));
            }
            MoveResult::Checkmate(winner) | MoveResult::Stalemate(winner) => {
                let message = match winner {
                    PieceSide::Red => "胜",
                    PieceSide::Black => "负",
                };
                self.ui.popup = Some(PopupTip::new_game_end(message.to_string()));
            }
            MoveResult::Success | MoveResult::Invalid => {}
        }
    }

    /// Returns true if player input should be blocked.
    pub fn should_block_input(&self) -> bool {
        if self.ui.ai_thinking {
            return true;
        }
        if self.ui.window.game_mode == GameMode::PlayerVsAI && self.game.is_ai_turn() {
            return true;
        }
        if self.ui.piece_animation.is_some() {
            return true;
        }
        false
    }

    /// Checks if it's AI's turn and sets the thinking flag.
    pub fn check_ai_turn(&mut self) {
        if self.ui.window.game_mode == GameMode::PlayerVsAI
            && self.game.is_ai_turn()
            && self.game.status == crate::game::state::GameStatus::InProgress
        {
            self.ui.ai_thinking = true;
        }
    }
}
