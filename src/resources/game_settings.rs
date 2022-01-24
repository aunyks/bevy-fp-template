/// The global player-editable game configuration.
/// These settings can be edited at runtime
#[derive(Debug, PartialEq, Eq)]
pub struct GameSettings;

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {}
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn default() {
        let default_settings = GameSettings::default();
        assert_eq!(default_settings, GameSettings {});
    }
}
