use crate::{
    game::{
        piece::PieceSide,
        rules::get_valid_moves,
        state::MoveResult,
    },
    ui::{
        app::ChessRealm,
        state::{GameMode, PieceAnimation, PopupTip},
    },
};
use eframe::egui;

impl ChessRealm {
    pub fn render_board(&mut self, ui: &mut egui::Ui) {
        let available_size: egui::Vec2 = ui.available_size();
        let (response, painter) = ui.allocate_painter(available_size, egui::Sense::click());
        let rect: egui::Rect = response.rect;

        let cols: usize = 9;
        let rows: usize = 10;

        let cell_w = rect.width() / 9.4;
        let cell_h = rect.height() / 10.4;
        let cell_size = cell_w.min(cell_h);

        let draw_width: f32 = cell_size * 8.0;
        let draw_height: f32 = cell_size * 9.0;

        let start_x: f32 = rect.left() + (rect.width() - draw_width) / 2.0;
        let start_y: f32 = rect.top() + (rect.height() - draw_height) / 2.0;

        let to_screen = |col: usize, row: usize| -> egui::Pos2 {
            egui::pos2(
                start_x + col as f32 * cell_size,
                start_y + row as f32 * cell_size,
            )
        };

        let from_screen = |pos: egui::Pos2| -> Option<(usize, usize)> {
            let col_f = (pos.x - start_x + cell_size / 2.0) / cell_size;
            let row_f = (pos.y - start_y + cell_size / 2.0) / cell_size;

            if col_f >= 0.0 && row_f >= 0.0 {
                let col = col_f as usize;
                let row = row_f as usize;
                if col < cols && row < rows {
                    let center = to_screen(col, row);
                    let dist = ((pos.x - center.x).powi(2) + (pos.y - center.y).powi(2)).sqrt();
                    if dist <= cell_size * 0.5 {
                        return Some((row, col));
                    }
                }
            }
            None
        };

        if response.clicked() {
            if let Some(click_pos) = response.interact_pointer_pos() {
                if let Some((row, col)) = from_screen(click_pos) {
                    self.handle_board_click(row, col);
                }
            }
        }

        let stroke: egui::Stroke = egui::Stroke::new(1.5, ui.visuals().text_color());

        for row in 0..rows {
            painter.line_segment([to_screen(0, row), to_screen(cols - 1, row)], stroke);
        }
        painter.line_segment([to_screen(0, 0), to_screen(0, rows - 1)], stroke);
        painter.line_segment(
            [to_screen(cols - 1, 0), to_screen(cols - 1, rows - 1)],
            stroke,
        );

        for col in 1..(cols - 1) {
            painter.line_segment([to_screen(col, 0), to_screen(col, 4)], stroke);
            painter.line_segment([to_screen(col, 5), to_screen(col, rows - 1)], stroke);
        }

        painter.line_segment([to_screen(3, 0), to_screen(5, 2)], stroke);
        painter.line_segment([to_screen(5, 0), to_screen(3, 2)], stroke);

        painter.line_segment([to_screen(3, 7), to_screen(5, 9)], stroke);
        painter.line_segment([to_screen(5, 7), to_screen(3, 9)], stroke);

        let draw_cross = |c: usize, r: usize| {
            let center = to_screen(c, r);
            let len = cell_size * 0.1;
            let gap = cell_size * 0.05;

            if c > 0 {
                painter.line_segment(
                    [
                        center + egui::vec2(-gap - len, -gap),
                        center + egui::vec2(-gap, -gap),
                    ],
                    stroke,
                );
                painter.line_segment(
                    [
                        center + egui::vec2(-gap, -gap - len),
                        center + egui::vec2(-gap, -gap),
                    ],
                    stroke,
                );
            }

            if c < cols - 1 {
                painter.line_segment(
                    [
                        center + egui::vec2(gap + len, -gap),
                        center + egui::vec2(gap, -gap),
                    ],
                    stroke,
                );
                painter.line_segment(
                    [
                        center + egui::vec2(gap, -gap - len),
                        center + egui::vec2(gap, -gap),
                    ],
                    stroke,
                );
            }

            if c > 0 {
                painter.line_segment(
                    [
                        center + egui::vec2(-gap - len, gap),
                        center + egui::vec2(-gap, gap),
                    ],
                    stroke,
                );
                painter.line_segment(
                    [
                        center + egui::vec2(-gap, gap + len),
                        center + egui::vec2(-gap, gap),
                    ],
                    stroke,
                );
            }

            if c < cols - 1 {
                painter.line_segment(
                    [
                        center + egui::vec2(gap + len, gap),
                        center + egui::vec2(gap, gap),
                    ],
                    stroke,
                );
                painter.line_segment(
                    [
                        center + egui::vec2(gap, gap + len),
                        center + egui::vec2(gap, gap),
                    ],
                    stroke,
                );
            }
        };

        draw_cross(1, 2);
        draw_cross(7, 2);
        draw_cross(1, 7);
        draw_cross(7, 7);

        for i in 0..5 {
            draw_cross(i * 2, 3);
            draw_cross(i * 2, 6);
        }

        if let Some(last_move) = self.game.last_move {
            let highlight_color = egui::Color32::from_rgba_unmultiplied(255, 200, 0, 120);
            let corner_len = cell_size * 0.2;
            let stroke = egui::Stroke::new(3.0, highlight_color);

            for &(row, col) in &[last_move.from, last_move.to] {
                let center = to_screen(col, row);
                let half = cell_size * 0.45;

                let corners = [
                    (
                        center + egui::vec2(-half, -half),
                        egui::vec2(corner_len, 0.0),
                        egui::vec2(0.0, corner_len),
                    ),
                    (
                        center + egui::vec2(half, -half),
                        egui::vec2(-corner_len, 0.0),
                        egui::vec2(0.0, corner_len),
                    ),
                    (
                        center + egui::vec2(-half, half),
                        egui::vec2(corner_len, 0.0),
                        egui::vec2(0.0, -corner_len),
                    ),
                    (
                        center + egui::vec2(half, half),
                        egui::vec2(-corner_len, 0.0),
                        egui::vec2(0.0, -corner_len),
                    ),
                ];

                for (corner, h_offset, v_offset) in corners {
                    painter.line_segment([corner, corner + h_offset], stroke);
                    painter.line_segment([corner, corner + v_offset], stroke);
                }
            }
        }

        for &(row, col) in &self.game.valid_moves {
            let center = to_screen(col, row);
            let radius = cell_size * 0.15;

            if self.game.board[row][col].is_some() {
                painter.circle_stroke(
                    center,
                    cell_size * 0.45,
                    egui::Stroke::new(3.0, egui::Color32::from_rgba_unmultiplied(0, 200, 0, 180)),
                );
            } else {
                painter.circle_filled(
                    center,
                    radius,
                    egui::Color32::from_rgba_unmultiplied(0, 200, 0, 180),
                );
            }
        }

        let animation_finished = self
            .ui
            .piece_animation
            .as_ref()
            .map(|a| a.is_done())
            .unwrap_or(false);
        if animation_finished {
            self.ui.piece_animation = None;
        }

        if self.ui.piece_animation.is_some() {
            ui.ctx().request_repaint();
        }

        let anim_target = self.ui.piece_animation.as_ref().map(|a| a.to);

        for row in 0..rows {
            for col in 0..cols {
                if let Some(piece) = self.game.board[row][col] {
                    if anim_target == Some((row, col)) {
                        continue;
                    }

                    let center = to_screen(col, row);
                    let radius = cell_size * 0.4;

                    let is_selected = self.game.selected_piece == Some((row, col));

                    if is_selected {
                        painter.circle_stroke(
                            center,
                            radius + 4.0,
                            egui::Stroke::new(3.0, egui::Color32::from_rgb(255, 215, 0)),
                        );
                    }

                    let bg_color = match piece.side {
                        PieceSide::Red => egui::Color32::from_rgb(200, 50, 50),
                        PieceSide::Black => {
                            if self.ui.window.dark_mode {
                                egui::Color32::from_rgb(120, 120, 130)
                            } else {
                                egui::Color32::from_rgb(50, 50, 50)
                            }
                        }
                    };
                    painter.circle_filled(center, radius, bg_color);

                    let text = piece.label();

                    let text_center = center + egui::vec2(0.0, cell_size * 0.12);
                    painter.text(
                        text_center,
                        egui::Align2::CENTER_CENTER,
                        text,
                        egui::FontId::new(
                            cell_size * 0.65,
                            egui::FontFamily::Name("feibo-zhengdots".into()),
                        ),
                        egui::Color32::WHITE,
                    );
                }
            }
        }

        if let Some(popup) = &self.ui.popup {
            if popup.is_visible() {
                let popup_text = &popup.message;
                let font_size = cell_size * 1.5;
                let font_id =
                    egui::FontId::new(font_size, egui::FontFamily::Name("feibo-zhengdots".into()));

                let popup_center = rect.center();
                let radius = cell_size * 0.8;

                painter.circle_filled(
                    popup_center,
                    radius,
                    egui::Color32::from_rgba_unmultiplied(255, 255, 255, 180),
                );

                let text_color = if popup.use_dark_red {
                    egui::Color32::from_rgb(139, 0, 0)
                } else {
                    egui::Color32::from_rgb(80, 80, 80)
                };

                painter.text(
                    popup_center + egui::vec2(0.0, cell_size * 0.2),
                    egui::Align2::CENTER_CENTER,
                    popup_text,
                    font_id,
                    text_color,
                );

                ui.ctx().request_repaint();
            }
        }

        if let Some(animation) = &self.ui.piece_animation {
            let progress = animation.progress();
            let t = progress * progress * (3.0 - 2.0 * progress);
            let start = to_screen(animation.from.1, animation.from.0);
            let end = to_screen(animation.to.1, animation.to.0);
            let center = start.lerp(end, t);
            let radius = cell_size * 0.4;

            let bg_color = match animation.piece.side {
                PieceSide::Red => egui::Color32::from_rgb(200, 50, 50),
                PieceSide::Black => {
                    if self.ui.window.dark_mode {
                        egui::Color32::from_rgb(120, 120, 130)
                    } else {
                        egui::Color32::from_rgb(50, 50, 50)
                    }
                }
            };
            painter.circle_filled(center, radius, bg_color);

            let text = animation.piece.label();
            let text_center = center + egui::vec2(0.0, cell_size * 0.12);
            painter.text(
                text_center,
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::new(
                    cell_size * 0.65,
                    egui::FontFamily::Name("feibo-zhengdots".into()),
                ),
                egui::Color32::WHITE,
            );
        }
    }

    fn handle_board_click(&mut self, row: usize, col: usize) {
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
        } else {
            if let Some(piece) = self.game.board[row][col] {
                if piece.side == self.game.current_turn {
                    self.game.selected_piece = Some(clicked_pos);
                    self.game.valid_moves =
                        get_valid_moves(&self.game.board, clicked_pos, self.game.current_turn);
                }
            }
        }
    }

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
    fn should_block_input(&self) -> bool {
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
    fn check_ai_turn(&mut self) {
        if self.ui.window.game_mode == GameMode::PlayerVsAI
            && self.game.is_ai_turn()
            && self.game.status == crate::game::state::GameStatus::InProgress
        {
            self.ui.ai_thinking = true;
        }
    }
}
