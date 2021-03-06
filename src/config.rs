use crate::error::Error;
use crate::timestamp::TimestampConfig;
use serde::Deserialize;
use std::path::Path;
use toml;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Config {
    pub outputs: Vec<OutputConfig>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputConfig {
    Timestamp(TimestampConfig),
    Battery,
    ActiveConnections,
}

impl Config {
    pub fn read_from(path: &Path) -> Result<Self, Error> {
        let source = std::fs::read_to_string(path)?;
        Config::parse(&source)
    }

    fn parse(source: &str) -> Result<Self, Error> {
        toml::from_str(source).map_err(Error::MalformedConfig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timestamp::*;

    #[test]
    fn simple_timestamp_parse() {
        let expected = Config {
            outputs: vec![
                OutputConfig::Timestamp(TimestampConfig {
                    format: "%a %Y-%m-%d - %H:%M:%S".to_string(),
                    accuracy: Accuracy::Minutes,
                }),
                OutputConfig::Timestamp(TimestampConfig {
                    format: "%a %Y-%m-%d - %H:%M:%S".to_string(),
                    accuracy: Accuracy::Seconds,
                }),
            ],
        };

        let s = r#"
            [[outputs]]
            type = "timestamp"
            format = "%a %Y-%m-%d - %H:%M:%S"
            accuracy = "minutes"

            [[outputs]]
            type = "timestamp"
            format = "%a %Y-%m-%d - %H:%M:%S"
            accuracy = "seconds"
        "#;
        let config = Config::parse(s).unwrap();
        assert_eq!(config, expected);
    }

    #[test]
    fn simple_battery_parse() {
        let expected = Config {
            outputs: vec![OutputConfig::Battery],
        };

        let s = r#"
            [[outputs]]
            type = "battery"
        "#;
        let config = Config::parse(s).unwrap();
        assert_eq!(config, expected);
    }

    #[test]
    fn simple_active_connections_parse() {
        let expected = Config {
            outputs: vec![OutputConfig::ActiveConnections],
        };

        let s = r#"
            [[outputs]]
            type = "active_connections"
        "#;
        let config = Config::parse(s).unwrap();
        assert_eq!(config, expected);
    }
}
