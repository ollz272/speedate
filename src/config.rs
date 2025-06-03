// src/config.rs

use crate::ConfigError;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum TimestampUnit {
    /// Let the parser infer units based on value length.
    /// Interpret as seconds since the UNIX epoch.
    Second,
    /// Interpret as milliseconds since the UNIX epoch.
    Millisecond,
    #[default]
    Infer,
}

impl TryFrom<&str> for TimestampUnit {
    type Error = ConfigError;
    fn try_from(value: &str) -> Result<Self, ConfigError> {
        match value.to_lowercase().as_str() {
            "s" => Ok(Self::Second),
            "ms" => Ok(Self::Millisecond),
            "infer" => Ok(Self::Infer),
            _ => Err(ConfigError::UnknownTimestampUnitString),
        }
    }
}
