use eframe::egui;

const APP_NAME: &str = "ChessRealm";

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc: &eframe::CreationContext<'_>| Ok(Box::new(ChessRealm::new(cc)))),
    )
}

#[derive(Default)]
struct ChessRealm {}

impl ChessRealm {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for ChessRealm {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {
            ui.heading("Hello, world!");
        });
    }
}
