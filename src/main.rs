mod config;
mod timestamp;

use config::*;
use timestamp::*;

use env_logger;
use log::{info, LevelFilter};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about)]
struct Opt {
    /// Enable debug printing to stderr, same as RUST_LOG="swaybar_data=debug"
    #[structopt(long, short)]
    debug: bool,

    /// Config file location
    #[structopt(long, short)]
    config: std::path::PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    if opt.debug {
        env_logger::Builder::from_default_env()
            .target(env_logger::Target::Stderr)
            .filter(Some("swaybar_data"), LevelFilter::Debug)
            .init();
    } else {
        env_logger::Builder::from_default_env()
            .target(env_logger::Target::Stderr)
            .filter_level(LevelFilter::Error)
            .init();
    }

    let config = Config::read_from(&opt.config).unwrap();
    let (tx, rx) = channel();

    for output in config.outputs {
        match output {
            Output::Timestamp(conf) => start_timestamp_generation(tx.clone(), conf),
        };
    }

    loop {
        let timestamp: String = rx.recv().unwrap();
        println!("{}", timestamp);
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
