use crate::game::engine::uci::EngineHandle;
use crate::ui::app::{ChessRealm, font};
use eframe::egui;

impl ChessRealm {
    pub fn render_settings_window(&mut self, ctx: &egui::Context) {
        if !self.ui.window.show_settings {
            return;
        }

        let dark_mode = self.ui.window.dark_mode;

        let settings_size = [400.0_f32, 300.0_f32];
        let position = ctx.input(|i| i.viewport().outer_rect).map(|rect| {
            let center = rect.center();
            egui::pos2(
                center.x - settings_size[0] / 2.0,
                center.y - settings_size[1] / 2.0,
            )
        });

        let mut builder = egui::ViewportBuilder::default()
            .with_title("设置")
            .with_inner_size(settings_size)
            .with_resizable(false);

        if let Some(pos) = position {
            builder = builder.with_position(pos);
        }

        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("settings_window"),
            builder,
            |ctx, _class| {
                let visuals = if dark_mode {
                    egui::Visuals::dark()
                } else {
                    egui::Visuals::light()
                };
                ctx.set_visuals(visuals);

                ctx.style_mut(|style| {
                    style.spacing.button_padding = egui::vec2(12.0, 6.0);
                });

                egui::TopBottomPanel::bottom("settings_bottom")
                    .exact_height(50.0)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add_space(8.0);
                            if ui.button(font("关闭", "zhuque-fangsong", 16.0)).clicked() {
                                self.ui.window.show_settings = false;
                            }
                        });
                    });

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add_space(10.0);

                    let row_height = 36.0;

                    ui.horizontal(|ui| {
                        ui.set_min_height(row_height);
                        ui.add_space(20.0);
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(font("主题", "zhuque-fangsong", 16.0));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(20.0);
                            let theme_text = if dark_mode { "深色" } else { "浅色" };
                            if ui
                                .button(font(theme_text, "zhuque-fangsong", 16.0))
                                .clicked()
                            {
                                self.ui.window.dark_mode = !self.ui.window.dark_mode;
                            }
                        });
                    });

                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.set_min_height(row_height);
                        ui.add_space(20.0);
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(font("引擎路径", "zhuque-fangsong", 16.0));
                        });
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.add_space(20.0);
                            if ui.button(font("选择", "zhuque-fangsong", 16.0)).clicked() {
                                if let Some(path) = rfd::FileDialog::new().pick_file() {
                                    let path_str = path.display().to_string();
                                    match EngineHandle::new(&path_str) {
                                        Ok(engine) => {
                                            self.ui.window.engine_path = Some(path_str);
                                            self.ui.engine = Some(engine);
                                            self.ui.engine_invalid = false;
                                        }
                                        Err(_) => {
                                            self.ui.window.engine_path = None;
                                            self.ui.engine = None;
                                            self.ui.engine_invalid = true;
                                        }
                                    }
                                }
                            }
                            if self.ui.window.engine_path.is_some() {
                                if ui.button(font("清除", "zhuque-fangsong", 16.0)).clicked() {
                                    self.ui.window.engine_path = None;
                                    self.ui.engine = None;
                                    self.ui.engine_invalid = false;
                                    self.ui.window.game_mode =
                                        crate::ui::state::GameMode::PlayerVsPlayer;
                                }
                            }
                        });
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                        ui.add_space(20.0);
                        if self.ui.engine_invalid {
                            ui.label(
                                font("引擎无效", "zhuque-fangsong", 14.0)
                                    .color(egui::Color32::from_rgb(220, 50, 50)),
                            );
                        } else if let Some(ref path) = self.ui.window.engine_path {
                            let display_path = truncate_path_display(path, 35);
                            ui.label(font(display_path, "zhuque-fangsong", 14.0));
                        }
                    });
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    self.ui.window.show_settings = false;
                }
            },
        );
    }
}

/// Truncates a path string for display, keeping the end portion if too long.
fn truncate_path_display(path: &str, max_chars: usize) -> String {
    let char_count = path.chars().count();
    if char_count <= max_chars {
        path.to_string()
    } else {
        let skip = char_count - max_chars + 3;
        let truncated: String = path.chars().skip(skip).collect();
        format!("...{}", truncated)
    }
}
