use crate::config::*;
use crate::timestamp::*;

pub struct OutputUpdate {
    pub id: usize,
    pub update: UpdateType,
}

pub enum UpdateType {
    Timestamp(String),
    Percentage(f64),
    OnBattery(bool),
    TimeToFull(i64),
    TimeToEmpty(i64),
}

#[derive(Debug, PartialEq)]
pub enum Output {
    Timestamp(Timestamp),
    Battery(Battery),
}

#[derive(Debug, PartialEq)]
pub struct Timestamp {
    pub state: String,
    pub config: TimestampConfig,
}

#[derive(Debug, PartialEq, Default)]
pub struct Battery {
    pub percentage: f64,
    pub on_battery: bool,
    pub seconds_to_full: i64,
    pub seconds_to_empty: i64,
}

impl Output {
    pub fn update(&mut self, update: UpdateType) -> bool {
        match self {
            Output::Timestamp(ref mut ts) => {
                if let UpdateType::Timestamp(s) = update {
                    let changed = ts.state != s;
                    ts.state = s;
                    changed
                } else {
                    false
                }
            }
            Output::Battery(ref mut bat) => match update {
                UpdateType::Percentage(p) => {
                    let changed = bat.percentage != p;
                    bat.percentage = p;
                    changed
                }
                UpdateType::OnBattery(b) => {
                    let changed = bat.on_battery != b;
                    bat.on_battery = b;
                    changed
                }
                UpdateType::TimeToFull(t) => {
                    let changed = bat.seconds_to_full != t;
                    bat.seconds_to_full = t;
                    changed
                }
                UpdateType::TimeToEmpty(t) => {
                    let changed = bat.seconds_to_empty != t;
                    bat.seconds_to_empty = t;
                    changed
                }
                _ => false,
            },
        }
    }

    pub fn as_plain(&self) -> String {
        match self {
            Output::Timestamp(ref ts) => ts.state.clone(),
            Output::Battery(ref bat) => match bat.on_battery {
                true => format!(
                    "BAT {}%: {} remaining",
                    bat.percentage,
                    secs_to_human(bat.seconds_to_empty)
                ),
                false => format!(
                    "CHR {}%: {} to full",
                    bat.percentage,
                    secs_to_human(bat.seconds_to_full)
                ),
            },
        }
    }
}

fn secs_to_human(secs: i64) -> String {
    let hours = secs / 3600;
    let mins = secs % 3600 / 60;
    let min_str = match mins < 10 {
        true => format!("0{}", mins),
        false => mins.to_string(),
    };
    format!("{}:{}", hours, min_str)
}

impl From<&OutputConfig> for Output {
    fn from(c: &OutputConfig) -> Self {
        match c {
            OutputConfig::Timestamp(c) => Output::Timestamp(Timestamp {
                state: "".to_string(),
                config: c.clone(),
            }),
            OutputConfig::Battery => Output::Battery(Battery::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::timestamp::{Accuracy, TimestampConfig};

    #[test]
    fn matching_timestamp_update() {
        let ts = timestamp_output();
        let mut out = Output::Timestamp(ts);
        let update = UpdateType::Timestamp("all strings are valid, sadly".to_string());
        let changed = out.update(update);
        assert!(changed);

        let expected = Output::Timestamp(Timestamp {
            state: "all strings are valid, sadly".to_string(),
            ..timestamp_output()
        });
        assert_eq!(out, expected);
    }

    #[test]
    fn matching_noop_timestamp_update() {
        let ts = timestamp_output();
        let mut out = Output::Timestamp(ts);
        let update = UpdateType::Timestamp("".to_string());
        let changed = out.update(update);
        assert!(!changed);

        let expected = Output::Timestamp(timestamp_output());
        assert_eq!(out, expected);
    }

    #[test]
    fn nonmatching_timestamp_updates() {
        let ts = timestamp_output();
        let mut out = Output::Timestamp(ts);
        let unchanged_out = Output::Timestamp(timestamp_output());

        let update = UpdateType::Percentage(1.0);
        let changed = out.update(update);
        assert!(!changed);
        assert_eq!(out, unchanged_out);

        let update = UpdateType::OnBattery(true);
        let changed = out.update(update);
        assert!(!changed);
        assert_eq!(out, unchanged_out);

        let update = UpdateType::TimeToFull(10);
        let changed = out.update(update);
        assert!(!changed);
        assert_eq!(out, unchanged_out);

        let update = UpdateType::TimeToEmpty(10);
        let changed = out.update(update);
        assert!(!changed);
        assert_eq!(out, unchanged_out);
    }

    #[test]
    fn matching_battery_updates() {
        let mut out = Output::Battery(battery_output());
        let update = UpdateType::OnBattery(false);
        assert!(out.update(update));
        let expected = Output::Battery(Battery {
            on_battery: false,
            ..battery_output()
        });
        assert_eq!(out, expected);

        let mut out = Output::Battery(battery_output());
        let update = UpdateType::Percentage(66.6);
        assert!(out.update(update));
        let expected = Output::Battery(Battery {
            percentage: 66.6,
            ..battery_output()
        });
        assert_eq!(out, expected);

        let mut out = Output::Battery(battery_output());
        let update = UpdateType::TimeToFull(12);
        assert!(out.update(update));
        let expected = Output::Battery(Battery {
            seconds_to_full: 12,
            ..battery_output()
        });
        assert_eq!(out, expected);

        let mut out = Output::Battery(battery_output());
        let update = UpdateType::TimeToEmpty(12);
        assert!(out.update(update));
        let expected = Output::Battery(Battery {
            seconds_to_empty: 12,
            ..battery_output()
        });
        assert_eq!(out, expected);
    }

    #[test]
    fn matching_noop_battery_updates() {
        let mut out = Output::Battery(battery_output());
        let update = UpdateType::OnBattery(true);
        assert!(!out.update(update));
        let expected = Output::Battery(battery_output());
        assert_eq!(out, expected);

        let mut out = Output::Battery(battery_output());
        let update = UpdateType::Percentage(41.5);
        assert!(!out.update(update));
        let expected = Output::Battery(battery_output());
        assert_eq!(out, expected);

        let mut out = Output::Battery(battery_output());
        let update = UpdateType::TimeToFull(0);
        assert!(!out.update(update));
        let expected = Output::Battery(battery_output());
        assert_eq!(out, expected);

        let mut out = Output::Battery(battery_output());
        let update = UpdateType::TimeToEmpty(36123);
        assert!(!out.update(update));
        let expected = Output::Battery(battery_output());
        assert_eq!(out, expected);
    }

    #[test]
    fn nonmatching_battery_updates() {
        let mut out = Output::Battery(battery_output());
        let update = UpdateType::Timestamp("wow, string".to_string());
        assert!(!out.update(update));
        let expected = Output::Battery(battery_output());
        assert_eq!(out, expected);
    }

    fn timestamp_output() -> Timestamp {
        Timestamp {
            state: "".to_string(),
            config: TimestampConfig {
                accuracy: Accuracy::Minutes,
                format: "%a %Y-%m-%d - %H:%M:%S".to_string(),
            },
        }
    }

    fn battery_output() -> Battery {
        Battery {
            percentage: 41.5,
            on_battery: true,
            seconds_to_full: 0,
            seconds_to_empty: 36123,
        }
    }
}
