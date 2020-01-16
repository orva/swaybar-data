use chrono::prelude::*;
use chrono::Duration;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use structopt::StructOpt;

#[derive(Debug)]
enum TimeAccuracy {
    Seconds,
    Minutes,
}

#[derive(Debug, StructOpt)]
#[structopt(about)]
struct Opt {
    #[structopt(long, parse(from_str = parse_accuracy), default_value = "seconds")]
    /// set timestamp update accuracy: seconds, minutes
    time_accuracy: TimeAccuracy,
}

fn parse_accuracy(arg: &str) -> TimeAccuracy {
    let lowered = arg.to_lowercase();
    if lowered == "minutes" {
        TimeAccuracy::Minutes
    } else if lowered == "seconds" {
        TimeAccuracy::Seconds
    } else {
        eprintln!(
            "Invalid time_accuracy {:?}, defaulting to seconds",
            &lowered
        );
        TimeAccuracy::Seconds
    }
}

fn main() {
    let opt = Opt::from_args();
    let (tx, rx) = channel();

    let timestamp_tx = tx.clone();
    thread::spawn(move || generate_timestamps(timestamp_tx, opt.time_accuracy));

    loop {
        let timestamp = rx.recv().unwrap();
        println!("{}", timestamp);
    }
}

fn generate_timestamps(tx: Sender<String>, accuracy: TimeAccuracy) {
    loop {
        let now = Utc::now();
        let output = now.format("%a %Y-%m-%d - %H:%M:%S").to_string();
        tx.send(output).unwrap();

        match calculate_sleep_duration(&accuracy).to_std() {
            Ok(d) => thread::sleep(d),
            Err(_) => continue,
        }
    }
}

fn calculate_sleep_duration(accuracy: &TimeAccuracy) -> Duration {
    let now = Utc::now();
    let wakeup_point = match accuracy {
        // hardcoded `with_second` parameter inside valid 0-59 range, will never return None
        TimeAccuracy::Minutes => (now + Duration::minutes(1)).with_second(0).unwrap(),
        TimeAccuracy::Seconds => now + Duration::seconds(1),
    };
    wakeup_point.signed_duration_since(now)
}
