use crate::constants::{APP_STATE_KEY, AVAILABLE_FONTS};
use crate::game::state::GameState;
use crate::ui::state::{UiState, WindowState};
use eframe::egui::{self, FontData, FontDefinitions, FontFamily, FontId, RichText};

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

        cc.egui_ctx.style_mut(|style| {
            style.spacing.button_padding = egui::vec2(12.0, 6.0);
        });

        Self {
            game: GameState::default(),
            ui: UiState {
                window,
                popup: None,
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
}

impl eframe::App for ChessRealm {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, APP_STATE_KEY, &self.ui.window);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.track_window_size(ctx);

        egui::TopBottomPanel::top("top_bar")
            .exact_height(50.)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        if ui.button(font("新局", "zhuque-fangsong", 18.0)).clicked() {
                            self.game = GameState::default();
                        }
                    });
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(font("设置", "zhuque-fangsong", 18.0)).clicked() {
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

fn load_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    for (name, data) in AVAILABLE_FONTS {
        fonts
            .font_data
            .insert(name.to_string(), FontData::from_static(data).into());

        fonts
            .families
            .insert(FontFamily::Name((*name).into()), vec![name.to_string()]);
    }

    ctx.set_fonts(fonts);
}

pub fn font(text: impl Into<String>, font_name: &str, size: f32) -> RichText {
    RichText::new(text).font(FontId::new(size, FontFamily::Name(font_name.into())))
}
