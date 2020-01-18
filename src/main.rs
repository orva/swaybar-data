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
    /// Enable debug printing to stderr, same as `RUST_LOG="swaybar_data=debug" swaybar-data`
    #[structopt(long, short)]
    debug: bool,

    /// Config file location
    #[structopt(long, short)]
    config: std::path::PathBuf,
}

struct OutputUpdate(String, usize);

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

    let mut outputs: Vec<String> = config.outputs.iter().map(|_| "".to_string()).collect();

    for (i, output_conf) in config.outputs.into_iter().enumerate() {
        match output_conf {
            Output::Timestamp(conf) => start_timestamp_generation(tx.clone(), conf, i),
        };
    }

    loop {
        let OutputUpdate(update, id) = rx.recv().unwrap();
        outputs[id] = update;
        let output = outputs.join(" | ");
        println!("{}", output);
    }
}

fn start_timestamp_generation(tx: Sender<OutputUpdate>, config: TimestampConfig, id: usize) {
    info!("Spawning timestamp generation thread");
    thread::spawn(move || {
        let timestamps = TimestampGenerator::new(config);
        for ts in timestamps {
            tx.send(OutputUpdate(ts, id)).unwrap();
        }
    });
}
