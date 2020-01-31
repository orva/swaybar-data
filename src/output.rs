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
}

impl Output {
    pub fn update(&mut self, update: UpdateType) {
        match self {
            Output::Timestamp(ref mut ts) => {
                if let UpdateType::Timestamp(s) = update {
                    ts.state = s;
                }
            }
            Output::Battery(ref mut bat) => match update {
                UpdateType::Percentage(p) => bat.state.percentage = p,
                UpdateType::OnBattery(b) => bat.state.on_battery = b,
                _ => {}
            },
        }
    }
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
