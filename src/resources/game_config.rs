use bevy::utils::tracing::Level as LogLevel;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PlayerConfig {
    /// The height of the physics capsule for the player
    capsule_height: f32,
    /// The radius of the physics capsule for the player
    capsule_radius: f32,
    /// The force applied to the FirstPersonSubject during movement.
    /// Unsure of units but likely in Newtons
    movement_force: f32,
    /// The vertical force applied to the FirstPersonSubject
    /// to cause it to jump
    jump_force: f32,
    /// The max speed of the FirstPersonSubject.
    /// Unsure of units but likely in meters per second
    max_speed: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        PlayerConfig {
            capsule_height: 8f32,
            capsule_radius: 1f32,
            movement_force: 1000f32,
            jump_force: 10000f32,
            max_speed: 5f32,
        }
    }
}

impl PlayerConfig {
    pub fn capsule_height(&self) -> f32 {
        self.capsule_height
    }

    pub fn capsule_radius(&self) -> f32 {
        self.capsule_radius
    }

    pub fn movement_force(&self) -> f32 {
        self.movement_force
    }

    pub fn jump_force(&self) -> f32 {
        self.jump_force
    }

    pub fn max_speed(&self) -> f32 {
        self.max_speed
    }
}

/// The global runtime configuration of the game. This value
/// is loaded at runtime instead of build time and cannot be edited
/// by the player
#[derive(Debug, Deserialize, Clone)]
pub struct GameConfig {
    window_title: String,
    log_level: String,
    log_filter: String,
    pub player: PlayerConfig,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            window_title: String::from("bevy-fp-template"),
            log_level: String::from("error"),
            log_filter: String::from("none=warn"),
            player: PlayerConfig::default(),
        }
    }
}

impl GameConfig {
    #[allow(dead_code)]
    pub fn try_from_toml(toml_str: String) -> Result<Self, String> {
        match toml::from_str::<GameConfig>(toml_str.as_str()) {
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
    pub fn log_level_raw(&self) -> &String {
        &self.log_level
    }

    pub fn log_level(&self) -> LogLevel {
        match self.log_level.as_str() {
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
        let good_config = GameConfig::try_from_toml(String::from(
            "
        window_title=\"some title\"
        log_level=\"trace\"
        log_filter=\"some=trace\"
        [player]
        capsule_height = 8
        capsule_radius = 1
        movement_force = 1000
        jump_force = 10000
        max_speed = 5
        ",
        ))
        .unwrap();
        assert_eq!(good_config.window_title(), &String::from("some title"));
        assert_eq!(good_config.log_level(), LogLevel::TRACE);
        assert_eq!(good_config.log_level_raw(), "trace");
        assert_eq!(good_config.log_filter(), "some=trace");
        assert_eq!(good_config.player.capsule_height(), 8f32);
        assert_eq!(good_config.player.capsule_radius(), 1f32);
        assert_eq!(good_config.player.movement_force(), 1000f32);
        assert_eq!(good_config.player.jump_force(), 10000f32);
        assert_eq!(good_config.player.max_speed(), 5f32);

        // Test bad configs

        // Missing config property
        match GameConfig::try_from_toml(String::from(
            "
        window_title=\"some title\" \n
        log_level=\"trace\" \n
        ",
        )) {
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
        match GameConfig::try_from_toml(String::from(
            "
        window_title=\"some title\" \n
        log_level=\"trace\" \n
        log_filter=\"some=trace \n
        ",
        )) {
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
