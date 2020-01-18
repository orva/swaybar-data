mod timestamp;

use timestamp::*;

use env_logger::{Builder, Target};
use log::{info, LevelFilter};
use structopt::StructOpt;
use std::sync::mpsc::{channel, Sender};
use std::thread;

#[derive(Debug, StructOpt)]
#[structopt(about)]
struct Opt {
    /// Set timestamp update accuracy: seconds, minutes
    #[structopt(long, parse(from_str = parse_accuracy), default_value = "seconds")]
    time_accuracy: Accuracy,

    /// Enable debug printing to stderr, same as RUST_LOG="swaybar_data=debug"
    #[structopt(long, short)]
    debug: bool,
}

fn main() {
    let opt = Opt::from_args();

    if opt.debug {
        Builder::from_default_env()
            .target(Target::Stderr)
            .filter(Some("swaybar_data"), LevelFilter::Debug)
            .init();
    } else {
        Builder::from_default_env().target(Target::Stderr).init();
    }

    let (tx, rx) = channel();

    let timestamp_tx = tx.clone();
    let timestamp_config = TimestampConfig {
        debug: opt.debug,
        accuracy: opt.time_accuracy,
        format: "%a %Y-%m-%d - %H:%M:%S".to_string(),
    };
    start_timestamp_generation(timestamp_tx, timestamp_config);

    loop {
        let timestamp: String = rx.recv().unwrap();
        println!("{}", timestamp);
    }
}

fn parse_accuracy(arg: &str) -> Accuracy {
    let lowered = arg.to_lowercase();
    if lowered == "minutes" {
        Accuracy::Minutes
    } else if lowered == "seconds" {
        Accuracy::Seconds
    } else {
        Accuracy::Seconds
    }
}

fn start_timestamp_generation(tx: Sender<String>, config: TimestampConfig) {
    info!("Spawning timestamp generation thread");
    thread::spawn(move || {
        let timestamps = TimestampGenerator::new(config);
        for i in timestamps {
            tx.send(i).unwrap();
        }
    });
}
