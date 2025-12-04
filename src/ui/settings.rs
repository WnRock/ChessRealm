use crate::ui::app::{ChessRealm, font};
use eframe::egui;

impl ChessRealm {
    pub fn render_settings_window(&mut self, ctx: &egui::Context) {
        if !self.ui.window.show_settings {
            return;
        }

        let dark_mode = self.ui.window.dark_mode;

        let settings_size = [300.0_f32, 200.0_f32];
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
                });

                if ctx.input(|i| i.viewport().close_requested()) {
                    self.ui.window.show_settings = false;
                }
            },
        );
    }
}
