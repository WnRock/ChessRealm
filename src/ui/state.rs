use crate::constants::APP_DEFAULT_SIZE;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub width: f32,
    pub height: f32,
    pub dark_mode: bool,
    #[serde(skip)]
    pub show_settings: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: APP_DEFAULT_SIZE[0],
            height: APP_DEFAULT_SIZE[1],
            dark_mode: true,
            show_settings: false,
        }
    }
}

pub struct PopupTip {
    pub message: String,
    pub shown_at: Instant,
    pub duration_secs: f32,
    pub use_dark_red: bool,
}

impl PopupTip {
    pub fn new(message: String) -> Self {
        Self {
            message,
            shown_at: Instant::now(),
            duration_secs: 1.0,
            use_dark_red: false,
        }
    }

    pub fn new_game_end(message: String) -> Self {
        Self {
            message,
            shown_at: Instant::now(),
            duration_secs: 3.0,
            use_dark_red: true,
        }
    }

    pub fn is_visible(&self) -> bool {
        self.shown_at.elapsed().as_secs_f32() < self.duration_secs
    }
}

pub struct UiState {
    pub window: WindowState,
    pub popup: Option<PopupTip>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            window: WindowState::default(),
            popup: None,
        }
    }
}
