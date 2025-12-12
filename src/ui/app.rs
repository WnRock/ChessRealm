use crate::constants::{APP_MIN_SIZE, APP_STATE_KEY};
use crate::engine::uci::EngineHandle;
use crate::game::state::GameState;
use crate::ui::fonts::{font, load_fonts};
use crate::ui::state::UiState;
use crate::ui::state::WindowState;
use eframe::egui;

pub struct ChessRealm {
    pub game: GameState,
    pub ui: UiState,
}

impl ChessRealm {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let window = cc
            .storage
            .and_then(|storage| eframe::get_value::<WindowState>(storage, APP_STATE_KEY))
            .unwrap_or_default();

        cc.egui_ctx
            .send_viewport_cmd(egui::ViewportCommand::InnerSize(egui::vec2(
                window.width,
                window.height,
            )));

        load_fonts(&cc.egui_ctx);

        let visuals = if window.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };
        cc.egui_ctx.set_visuals(visuals);

        let mut engine = None;
        let mut engine_invalid = false;
        let mut engine_path = window.engine_path.clone();
        let mut game_mode = window.game_mode;

        if let Some(ref path) = engine_path {
            if std::path::Path::new(path).exists() {
                match EngineHandle::new(path) {
                    Ok(handle) => {
                        engine = Some(handle);
                    }
                    Err(_) => {
                        engine_invalid = true;
                        engine_path = None;
                        game_mode = crate::ui::state::GameMode::PlayerVsPlayer;
                    }
                }
            } else {
                engine_invalid = true;
                engine_path = None;
                game_mode = crate::ui::state::GameMode::PlayerVsPlayer;
            }
        }

        Self {
            game: GameState::default(),
            ui: UiState {
                window: WindowState {
                    engine_path,
                    game_mode,
                    ..window
                },
                popup: None,
                engine,
                engine_invalid,
                ai_thinking: false,
                ai_request_sent: false,
                piece_animations: Vec::new(),
            },
        }
    }

    fn track_window_size(&mut self, ctx: &egui::Context) {
        if let Some(rect) = ctx.input(|i| i.viewport().inner_rect) {
            if rect.width() > 0.0 && rect.height() > 0.0 {
                self.ui.window.width = rect.width();
                self.ui.window.height = rect.height();
            }
        }
    }

    /// Sends a move request to the engine.
    pub fn request_ai_move(&mut self) {
        if let Some(engine) = &self.ui.engine {
            let moves_uci = self.game.moves_to_uci();
            let elo = if self.ui.window.engine_elo == 0 {
                None
            } else {
                Some(self.ui.window.engine_elo)
            };
            engine.request_move(moves_uci, Some(10), Some(2000), elo);
            self.ui.ai_request_sent = true;
        }
    }

    /// Polls for AI move result and applies it if available.
    pub fn poll_ai_move(&mut self) {
        if !self.ui.ai_thinking {
            return;
        }

        if let Some(engine) = &self.ui.engine {
            if let Some(result) = engine.try_recv_move() {
                self.ui.ai_thinking = false;
                self.ui.ai_request_sent = false;

                if let Ok(move_uci) = result {
                    if let Some(ai_move) = GameState::uci_to_move(&move_uci) {
                        let moving_piece = self.game.board[ai_move.from.0][ai_move.from.1];

                        let result = self.game.make_move(ai_move.from, ai_move.to);

                        if !matches!(result, crate::game::state::MoveResult::Invalid) {
                            if let Some(piece) = moving_piece {
                                self.ui.piece_animations.push(
                                    crate::ui::state::PieceAnimation::new(
                                        piece,
                                        ai_move.from,
                                        ai_move.to,
                                    ),
                                );
                            }
                        }

                        self.handle_move_result(result);
                    }
                }
            }
        }
    }
}

impl eframe::App for ChessRealm {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, APP_STATE_KEY, &self.ui.window);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.track_window_size(ctx);
        if self.ui.piece_animations.is_empty() {
            self.poll_ai_move();
        }

        if self.ui.ai_thinking && !self.ui.ai_request_sent {
            self.request_ai_move();
        }

        if self.ui.ai_thinking {
            ctx.request_repaint();
        }

        let bar_height = if self.ui.window.height < 500.0 {
            let t = (self.ui.window.height - APP_MIN_SIZE[1]) / (500.0 - APP_MIN_SIZE[1]);
            40.0 + t * (50.0 - 40.0)
        } else {
            50.0
        };
        let scale = bar_height / 50.0;
        let font_size = 18.0 * scale;
        let button_padding = egui::vec2(12.0 * scale, 6.0 * scale);

        ctx.style_mut(|style| {
            style.spacing.button_padding = button_padding;
        });

        let panel_frame = egui::Frame {
            inner_margin: egui::Margin {
                left: 15,
                right: 15,
                top: 5,
                bottom: 5,
            },
            fill: ctx.style().visuals.panel_fill,
            ..Default::default()
        };

        egui::TopBottomPanel::top("top_bar")
            .exact_height(bar_height)
            .frame(panel_frame)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        if ui
                            .button(font("新局", "zhuque-fangsong", font_size))
                            .clicked()
                        {
                            self.game = GameState::default();
                            self.ui.ai_thinking = false;
                            self.ui.ai_request_sent = false;
                            self.ui.piece_animations.clear();
                        }
                        let can_toggle_to_ai = self.ui.engine.is_some()
                            || self.ui.window.game_mode == crate::ui::state::GameMode::PlayerVsAI;
                        ui.add_enabled_ui(can_toggle_to_ai, |ui| {
                            if ui
                                .button(font(
                                    self.ui.window.game_mode.label(),
                                    "zhuque-fangsong",
                                    font_size,
                                ))
                                .clicked()
                            {
                                self.ui.window.game_mode = self.ui.window.game_mode.toggle();
                                self.ui.ai_thinking = false;
                                self.ui.ai_request_sent = false;
                            }
                        });

                        ui.add_enabled_ui(!self.game.move_history.is_empty(), |ui| {
                            if ui
                                .button(font("悔棋", "zhuque-fangsong", font_size))
                                .clicked()
                            {
                                self.handle_undo();
                            }
                        });
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .button(font("设置", "zhuque-fangsong", font_size))
                            .clicked()
                        {
                            self.ui.window.show_settings = !self.ui.window.show_settings;
                        }
                    });
                });
            });

        self.render_settings_window(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_board(ui);
        });
    }
}
