use bevy::utils::tracing::Level as LogLevel;
use serde::Deserialize;

/// The global runtime configuration of the game. This value
/// is loaded at runtime instead of build time and cannot be edited
/// by the player
#[derive(Deserialize, Clone)]
pub struct GameConfig {
    window_title: String,
    log_level: String,
    log_filter: String,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            window_title: String::from("bevy-fp-template"),
            log_level: String::from("error"),
            log_filter: String::from("none=warn"),
        }
    }
}

impl GameConfig {
    #[allow(dead_code)]
    pub fn from_toml(toml_str: &str) -> Result<Self, String> {
        match toml::from_str::<GameConfig>(toml_str) {
            Ok(config) => Ok(config),
            Err(toml_de_err) => Err(toml_de_err.to_string()),
        }
    }

    pub fn window_title(&self) -> String {
        self.window_title.clone()
    }

    #[allow(dead_code)]
    pub fn log_level_raw(&self) -> String {
        self.log_level.clone()
    }

    pub fn log_level(&self) -> LogLevel {
        match self.log_level.clone().as_str() {
            "trace" => LogLevel::TRACE,
            "debug" => LogLevel::DEBUG,
            "info" => LogLevel::INFO,
            "warn" => LogLevel::WARN,
            "error" => LogLevel::ERROR,
            _ => {
                panic!("Unrecognized log level found! Must be \"trace\", \"debug\", \"info\", \"warn\", or \"error\"");
            }
        }
    }

    pub fn log_filter(&self) -> String {
        self.log_filter.clone()
    }
}
