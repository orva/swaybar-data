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

pub enum Output {
    Timestamp(Timestamp),
    Battery(Battery),
}

pub struct Timestamp {
    pub state: String,
    pub config: TimestampConfig,
}

pub struct Battery {
    pub state: BatteryState,
}

#[derive(Default)]
pub struct BatteryState {
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
                    bat.state.percentage = p;
                    bat.state.percentage == p
                }
                UpdateType::OnBattery(b) => {
                    bat.state.on_battery = b;
                    bat.state.on_battery == b
                }
                UpdateType::TimeToFull(t) => {
                    bat.state.seconds_to_full = t;
                    bat.state.seconds_to_full == t
                }
                UpdateType::TimeToEmpty(t) => {
                    bat.state.seconds_to_empty = t;
                    bat.state.seconds_to_empty == t
                }
                _ => false,
            },
        }
    }

    pub fn as_plain(&self) -> String {
        match self {
            Output::Timestamp(ref ts) => ts.state.clone(),
            Output::Battery(ref bat) => match bat.state.on_battery {
                true => format!(
                    "BAT {}%: {} remaining",
                    bat.state.percentage,
                    secs_to_human(bat.state.seconds_to_empty)
                ),
                false => format!(
                    "CHR {}%: {} to full",
                    bat.state.percentage,
                    secs_to_human(bat.state.seconds_to_full)
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
            OutputConfig::Battery => Output::Battery(Battery {
                state: BatteryState::default(),
            }),
        }
    }
}
