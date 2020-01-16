use chrono::prelude::*;
use chrono::Duration;
use std::thread::sleep;

enum TimeAccuracy {
    Seconds,
    Minutes,
}

fn main() {
    loop {
        let now = Utc::now();
        let output = now.format("%a %Y-%m-%d - %H:%M:%S").to_string();

        println!("{}", output);

        match calculate_sleep_duration(TimeAccuracy::Minutes).to_std() {
            Ok(d) => sleep(d),
            Err(_) => continue,
        }
    }
}

fn calculate_sleep_duration(accuracy: TimeAccuracy) -> Duration {
    let now = Utc::now();
    let wakeup_point = match accuracy {
        // hardcoded `with_second` parameter inside valid 0-59 range, will never return None
        TimeAccuracy::Minutes => (now + Duration::minutes(1)).with_second(0).unwrap(),
        TimeAccuracy::Seconds => now + Duration::seconds(1),
    };
    wakeup_point.signed_duration_since(now)
}
