mod constants;
mod game;
mod ui;
use crate::constants::{APP_DEFAULT_SIZE, APP_ID, APP_MIN_SIZE, APP_NAME};
use eframe::egui;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id(APP_ID)
            .with_resizable(true)
            .with_inner_size(APP_DEFAULT_SIZE)
            .with_min_inner_size(APP_MIN_SIZE),
        ..Default::default()
    };
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(ui::app::ChessRealm::new(cc)))),
    )
}
