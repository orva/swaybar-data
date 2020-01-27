use crate::timestamp::TimestampConfig;
use serde::Deserialize;
use std::path::Path;
use toml;

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct Config {
    pub outputs: Vec<Output>,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Output {
    Timestamp(TimestampConfig),
    Battery,
}

impl Config {
    pub fn read_from(path: &Path) -> Option<Config> {
        let source = std::fs::read_to_string(path).ok()?;
        Config::parse(&source)
    }

    fn parse(source: &str) -> Option<Config> {
        toml::from_str(source).ok()
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
                Output::Timestamp(TimestampConfig {
                    format: "%a %Y-%m-%d - %H:%M:%S".to_string(),
                    accuracy: Accuracy::Minutes,
                }),
                Output::Timestamp(TimestampConfig {
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
            outputs: vec![Output::Battery],
        };

        let s = r#"
            [[outputs]]
            type = "battery"
        "#;
        let config = Config::parse(s).unwrap();
        assert_eq!(config, expected);
    }
}
