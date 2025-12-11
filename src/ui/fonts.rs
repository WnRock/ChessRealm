use crate::constants::AVAILABLE_FONTS;
use eframe::egui::{self, FontData, FontDefinitions, FontFamily, FontId, RichText};

/// Loads custom fonts into the egui context.
pub fn load_fonts(ctx: &egui::Context) {
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

/// Creates a RichText with the specified font and size.
pub fn font(text: impl Into<String>, font_name: &str, size: f32) -> RichText {
    RichText::new(text).font(FontId::new(size, FontFamily::Name(font_name.into())))
}
