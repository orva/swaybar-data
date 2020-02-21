use crate::error::Error;
use crate::output::{OutputUpdate, UpdateType};
use chrono::prelude::*;
use chrono::Duration;
use log::{debug, info};
use serde::Deserialize;
use std::sync::mpsc::Sender;
use std::thread;

#[derive(PartialEq, Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Accuracy {
    Seconds,
    Minutes,
}

#[derive(PartialEq, Debug, Clone, Deserialize)]
pub struct TimestampConfig {
    pub accuracy: Accuracy,
    pub format: String,
}

#[derive(Debug)]
pub struct TimestampSource {
    config: TimestampConfig,
    id: usize,
}

impl TimestampSource {
    pub fn new(config: TimestampConfig, id: usize) -> Self {
        TimestampSource { config, id }
    }

    pub fn start_generating(&self, tx: Sender<OutputUpdate>) -> Result<(), Error> {
        tx.send(OutputUpdate {
            id: self.id,
            update: UpdateType::Timestamp(self.generate_output()),
        })?;

        loop {
            thread::sleep(calculate_sleep_duration(&self.config.accuracy));
            tx.send(OutputUpdate {
                id: self.id,
                update: UpdateType::Timestamp(self.generate_output()),
            })?;
        }
    }

    fn generate_output(&self) -> String {
        let now = Local::now();
        now.format(&self.config.format).to_string()
    }
}

fn calculate_sleep_duration(accuracy: &Accuracy) -> std::time::Duration {
    let now = Utc::now();
    let wakeup_point = match accuracy {
        // hardcoded `with_second` parameter inside valid 0-59 range, will never return None
        Accuracy::Minutes => (now + Duration::minutes(1)).with_second(0).unwrap(),
        Accuracy::Seconds => now + Duration::seconds(1),
    };

    debug!(
        "calculating sleep duration (now: {:?}, wakeup_point: {:?})",
        &now, &wakeup_point
    );

    match wakeup_point.signed_duration_since(now).to_std() {
        Ok(sleep_duration) => sleep_duration,
        // Conversion to std::time::Duration fails if chrono::Duration is negative, which can
        // happen if time has been adjusted to past. Just try again until system time has
        // been stabilized.
        Err(_) => {
            info!("Calculated negative sleep duration!");
            std::time::Duration::from_secs(1)
        }
    }
}
