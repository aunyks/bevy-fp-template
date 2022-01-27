use bevy::utils::tracing::Level as LogLevel;
use serde::Deserialize;

/// The global runtime configuration of the game. This value
/// is loaded at runtime instead of build time and cannot be edited
/// by the player
#[derive(Debug, Deserialize, Clone)]
pub struct GameConfig<'a> {
    window_title: String,
    log_level: &'a str,
    log_filter: String,
}

impl<'a> Default for GameConfig<'a> {
    fn default() -> Self {
        GameConfig {
            window_title: String::from("bevy-fp-template"),
            log_level: "error",
            log_filter: String::from("bevy_fp_template::plugins::game::main=trace"),
        }
    }
}

impl<'a> GameConfig<'a> {
    #[allow(dead_code)]
    pub fn try_from_toml(toml_str: &'a str) -> Result<Self, String> {
        match toml::from_str::<GameConfig>(toml_str) {
            Ok(config) => Ok(config),
            Err(toml_de_err) => Err(toml_de_err.to_string()),
        }
    }

    pub fn window_title(&self) -> &String {
        &self.window_title
    }

    // This shouldn't be used much, if at all.
    // It's here for convenience
    #[allow(dead_code)]
    pub fn log_level_raw(&self) -> &str {
        self.log_level
    }

    pub fn log_level(&self) -> LogLevel {
        match self.log_level {
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

    pub fn log_filter(&self) -> &String {
        &self.log_filter
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default() {
        let default_config = GameConfig::default();
        assert_eq!(
            default_config.window_title(),
            &String::from("bevy-fp-template")
        );
        assert_eq!(default_config.log_level(), LogLevel::ERROR);
        assert_eq!(default_config.log_level_raw(), "error");
        assert_eq!(default_config.log_filter(), "none=warn");
    }

    #[test]
    fn try_from_toml() {
        // Test normal conditions
        let good_config = GameConfig::try_from_toml(
            "
        window_title=\"some title\" \n
        log_level=\"trace\" \n
        log_filter=\"some=trace\" \n
        ",
        )
        .unwrap();
        assert_eq!(good_config.window_title(), &String::from("some title"));
        assert_eq!(good_config.log_level(), LogLevel::TRACE);
        assert_eq!(good_config.log_level_raw(), "trace");
        assert_eq!(good_config.log_filter(), "some=trace");

        // Test bad configs

        // Missing config property
        match GameConfig::try_from_toml(
            "
        window_title=\"some title\" \n
        log_level=\"trace\" \n
        ",
        ) {
            Err(err_string) => {
                assert_eq!(
                    err_string,
                    String::from("missing field `log_filter` at line 1 column 1")
                );
            }
            _ => {
                panic!("Bad GameConfig TOML was parsed. An error should have occurred.");
            }
        };

        // Missing trailing quote at the end
        match GameConfig::try_from_toml(
            "
        window_title=\"some title\" \n
        log_level=\"trace\" \n
        log_filter=\"some=trace \n
        ",
        ) {
            Err(err_string) => {
                assert_eq!(
                    err_string,
                    // It's line 6 and not 3, because both the newlines of the string
                    // and the literal newline characters within the string ultimately
                    // add newlines to the resulting string.
                    String::from("newline in string found at line 6 column 32")
                );
            }
            _ => {
                panic!("Bad GameConfig TOML was parsed. An error should have occurred.");
            }
        };
    }
}
