use crate::constants::APP_DEFAULT_SIZE;
use crate::engine::uci::EngineHandle;
use crate::game::piece::Piece;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Game mode: Player vs Player or Player vs AI
#[derive(Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameMode {
    #[default]
    PlayerVsPlayer,
    PlayerVsAI,
}

impl GameMode {
    pub fn label(&self) -> &'static str {
        match self {
            GameMode::PlayerVsPlayer => "玩家",
            GameMode::PlayerVsAI => "AI",
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            GameMode::PlayerVsPlayer => GameMode::PlayerVsAI,
            GameMode::PlayerVsAI => GameMode::PlayerVsPlayer,
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WindowState {
    pub width: f32,
    pub height: f32,
    pub dark_mode: bool,
    pub engine_path: Option<String>,
    pub game_mode: GameMode,
    pub engine_elo: u32,
    #[serde(skip)]
    pub show_settings: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            width: APP_DEFAULT_SIZE[0],
            height: APP_DEFAULT_SIZE[1],
            dark_mode: true,
            engine_path: None,
            game_mode: GameMode::default(),
            engine_elo: 3000,
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

pub struct PieceAnimation {
    pub piece: Piece,
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub start_time: Instant,
    pub duration_secs: f32,
}

impl PieceAnimation {
    pub fn new(piece: Piece, from: (usize, usize), to: (usize, usize)) -> Self {
        Self {
            piece,
            from,
            to,
            start_time: Instant::now(),
            duration_secs: 0.35,
        }
    }

    pub fn progress(&self) -> f32 {
        (self.start_time.elapsed().as_secs_f32() / self.duration_secs).min(1.0)
    }

    pub fn is_done(&self) -> bool {
        self.progress() >= 1.0
    }
}

pub struct UiState {
    pub window: WindowState,
    pub popup: Option<PopupTip>,
    pub engine: Option<EngineHandle>,
    pub engine_invalid: bool,
    pub ai_thinking: bool,
    pub ai_request_sent: bool,
    pub piece_animations: Vec<PieceAnimation>,
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            window: WindowState::default(),
            popup: None,
            engine: None,
            engine_invalid: false,
            ai_thinking: false,
            ai_request_sent: false,
            piece_animations: Vec::new(),
        }
    }
}
