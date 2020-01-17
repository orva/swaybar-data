use crate::{GeneratorResult, OutputGenerator};

use chrono::prelude::*;
use chrono::Duration;
use std::sync::mpsc::Sender;
use std::thread;
use log::{debug};

#[derive(Debug)]
pub enum Accuracy {
    Seconds,
    Minutes,
}

#[derive(Debug)]
pub struct TimestampConfig {
    pub debug: bool,
    pub accuracy: Accuracy,
}

#[derive(Debug)]
pub struct TimestampGenerator {
    config: TimestampConfig,
    tx: Sender<String>,
}

impl OutputGenerator for TimestampGenerator {
    type Config = TimestampConfig;
    type Error = ();

    fn new(tx: Sender<String>, config: TimestampConfig) -> Self {
        TimestampGenerator { config, tx }
    }

    fn generate(&self) -> GeneratorResult<Self::Error> {
        loop {
            let now = Utc::now();
            let output = now.format("%a %Y-%m-%d - %H:%M:%S").to_string();

            match self.tx.send(output) {
                Err(_) => return Err(()),
                Ok(_) => {}
            };

            match calculate_sleep_duration(&self.config.accuracy).to_std() {
                Ok(sleep_duration) => thread::sleep(sleep_duration),
                // Conversion to std::time::Duration fails if chrono::Duration is negative, which can
                // happen if time has been adjusted to past. Just try again until system time has
                // been stabilized.
                Err(_) => {
                    debug!("Negative sleep duration");
                    continue
                },
            }
        }
    }
}

fn calculate_sleep_duration(accuracy: &Accuracy) -> Duration {
    let now = Utc::now();
    let wakeup_point = match accuracy {
        // hardcoded `with_second` parameter inside valid 0-59 range, will never return None
        Accuracy::Minutes => (now + Duration::minutes(1)).with_second(0).unwrap(),
        Accuracy::Seconds => now + Duration::seconds(1),
    };
    debug!("calculating sleep duration now: {:?}, wakeup_point: {:?}", &now, &wakeup_point);
    wakeup_point.signed_duration_since(now)
}
