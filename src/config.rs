use crate::timestamp::TimestampConfig;
use serde_derive::Deserialize;
use std::path::Path;
use toml;

#[derive(PartialEq, Debug, Deserialize)]
pub struct Config {
    pub outputs: Vec<Output>,
}

#[derive(PartialEq, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Output {
    Timestamp(TimestampConfig),
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
    fn simple_parse() {
        let expected = Config {
            outputs: vec![Output::Timestamp(TimestampConfig {
                format: "%a %Y-%m-%d - %H:%M:%S".to_string(),
                accuracy: Accuracy::Minutes,
            })],
        };

        let s = r#"
            [[outputs]]
            type = "Timestamp"
            format = "%a %Y-%m-%d - %H:%M:%S"
            accuracy = "Minutes"
        "#;
        let config = Config::parse(s).unwrap();
        assert_eq!(config, expected);
    }
}
