use serde::Serialize;

// The global player-editable game configuration.
/// These settings can be edited at runtime
#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct GameSettings {
    horizontal_sensitivity: u8,
    vertical_sensitivity: u8,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            horizontal_sensitivity: 5,
            vertical_sensitivity: 5,
        }
    }
}

impl GameSettings {
    #[allow(dead_code)]
    pub fn horizontal_sensitivity(&self) -> u8 {
        self.horizontal_sensitivity
    }

    #[allow(dead_code)]
    pub fn vertical_sensitivity(&self) -> u8 {
        self.vertical_sensitivity
    }

    #[allow(dead_code)]
    pub fn set_horizontal_sensitivity(&mut self, sensitivity: u8) {
        self.horizontal_sensitivity = sensitivity;
    }

    #[allow(dead_code)]
    pub fn set_vertical_sensitivity(&mut self, sensitivity: u8) {
        self.vertical_sensitivity = sensitivity;
    }

    #[allow(dead_code)]
    pub fn try_to_toml(&self) -> Result<String, String> {
        match toml::to_string(&self) {
            Ok(settings_string) => Ok(settings_string),
            Err(serialization_err) => Err(serialization_err.to_string()),
        }
    }

    #[allow(dead_code)]
    pub fn to_toml(&self) -> String {
        self.try_to_toml().unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default() {
        let default_settings = GameSettings::default();
        assert_eq!(default_settings.horizontal_sensitivity(), 5);
        assert_eq!(default_settings.vertical_sensitivity(), 5);
    }

    #[test]
    fn set_horizontal_sensitivity() {
        let mut settings = GameSettings::default();
        settings.set_horizontal_sensitivity(7);
        assert_eq!(settings.horizontal_sensitivity(), 7);
        settings.set_horizontal_sensitivity(2);
        assert_eq!(settings.horizontal_sensitivity(), 2);
    }

    #[test]
    fn set_vertical_sensitivity() {
        let mut settings = GameSettings::default();
        settings.set_vertical_sensitivity(7);
        assert_eq!(settings.vertical_sensitivity(), 7);
        settings.set_vertical_sensitivity(2);
        assert_eq!(settings.vertical_sensitivity(), 2);
    }

    #[test]
    fn try_to_toml() {
        let settings = GameSettings::default();
        assert_eq!(
            settings.try_to_toml().unwrap(),
            "horizontal_sensitivity = 5\nvertical_sensitivity = 5\n"
        );
    }

    #[test]
    fn to_toml() {
        let settings = GameSettings::default();
        assert_eq!(
            settings.to_toml(),
            "horizontal_sensitivity = 5\nvertical_sensitivity = 5\n"
        );
    }
}
