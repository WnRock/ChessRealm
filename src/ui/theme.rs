use eframe::egui::Color32;

/// Colors for rendering pieces on the board.
#[derive(Clone, Copy)]
pub struct PieceColors {
    pub red_background: Color32,
    pub black_background: Color32,
    pub text: Color32,
}

/// Colors for board highlights and indicators.
#[derive(Clone, Copy)]
pub struct HighlightColors {
    pub selected_piece: Color32,
    pub last_move: Color32,
    pub valid_move: Color32,
}

/// Colors for popup messages.
#[derive(Clone, Copy)]
pub struct PopupColors {
    pub background: Color32,
    pub text_normal: Color32,
    pub text_game_end: Color32,
}

/// Colors for error/warning states.
#[derive(Clone, Copy)]
pub struct StatusColors {
    pub error: Color32,
}

/// Complete theme configuration for the chess board UI.
#[derive(Clone, Copy)]
pub struct Theme {
    pub piece: PieceColors,
    pub highlight: HighlightColors,
    pub popup: PopupColors,
    pub status: StatusColors,
}

impl Theme {
    /// Creates the default theme for dark mode.
    pub fn dark() -> Self {
        Self {
            piece: PieceColors {
                red_background: Color32::from_rgb(200, 50, 50),
                black_background: Color32::from_rgb(120, 120, 130),
                text: Color32::WHITE,
            },
            highlight: HighlightColors {
                selected_piece: Color32::from_rgb(255, 215, 0),
                last_move: Color32::from_rgba_unmultiplied(255, 200, 0, 120),
                valid_move: Color32::from_rgba_unmultiplied(0, 200, 0, 180),
            },
            popup: PopupColors {
                background: Color32::from_rgba_unmultiplied(255, 255, 255, 180),
                text_normal: Color32::from_rgb(80, 80, 80),
                text_game_end: Color32::from_rgb(139, 0, 0),
            },
            status: StatusColors {
                error: Color32::from_rgb(220, 50, 50),
            },
        }
    }

    /// Creates the default theme for light mode.
    pub fn light() -> Self {
        Self {
            piece: PieceColors {
                red_background: Color32::from_rgb(200, 50, 50),
                black_background: Color32::from_rgb(50, 50, 50),
                text: Color32::WHITE,
            },
            highlight: HighlightColors {
                selected_piece: Color32::from_rgb(255, 215, 0),
                last_move: Color32::from_rgba_unmultiplied(255, 200, 0, 120),
                valid_move: Color32::from_rgba_unmultiplied(0, 200, 0, 180),
            },
            popup: PopupColors {
                background: Color32::from_rgba_unmultiplied(255, 255, 255, 180),
                text_normal: Color32::from_rgb(80, 80, 80),
                text_game_end: Color32::from_rgb(139, 0, 0),
            },
            status: StatusColors {
                error: Color32::from_rgb(220, 50, 50),
            },
        }
    }

    /// Returns the appropriate theme based on dark mode setting.
    pub fn from_dark_mode(dark_mode: bool) -> Self {
        if dark_mode {
            Self::dark()
        } else {
            Self::light()
        }
    }

    /// Returns the background color for a piece based on its side.
    pub fn piece_background(&self, side: crate::game::piece::PieceSide) -> Color32 {
        match side {
            crate::game::piece::PieceSide::Red => self.piece.red_background,
            crate::game::piece::PieceSide::Black => self.piece.black_background,
        }
    }
}
