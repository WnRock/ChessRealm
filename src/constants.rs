/// The application ID used for window management and storage
pub const APP_ID: &str = "dev.wnrock.chess_realm";

/// The name of the application, shown in the window title
pub const APP_NAME: &str = "Chess Realm";

/// Default window size [width, height] in pixels
pub const APP_DEFAULT_SIZE: [f32; 2] = [1024.0, 768.0];

/// Minimum window size [width, height] in pixels
pub const APP_MIN_SIZE: [f32; 2] = [300.0, 380.0];

/// Storage key for saving/loading app state
pub const APP_STATE_KEY: &str = "chess_realm_state";

/// Available fonts
pub const AVAILABLE_FONTS: &[(&str, &str)] = &[
    ("zhuque-fangsong", "ZhuqueFangsong-Regular.ttf"),
    ("feibo-zhengdots", "FeiboZhengDots.ttf"),
];
