mod config;
mod dbusdata;
mod generated;
mod timestamp;

use config::*;
use dbusdata::*;
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
struct OutputState {
    state: Option<String>,
    output_config: config::Output,
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

    let mut outputs: Vec<OutputState> = config
        .outputs
        .into_iter()
        .map(|output_config| OutputState {
            state: None,
            output_config,
        })
        .collect();

    for (i, output) in outputs.iter().enumerate() {
        match &output.output_config {
            Output::Timestamp(conf) => start_timestamp_generation(tx.clone(), conf.clone(), i),
        };
    }

    start_dbusdata_generation(tx.clone());

    loop {
        let OutputUpdate(update, id) = rx.recv().unwrap();
        outputs[id].state = Some(update);
        let output = outputs
            .iter()
            .map(|o| o.state.clone().unwrap_or("".to_string()))
            .collect::<Vec<String>>()
            .join(" | ");

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

fn start_dbusdata_generation(tx: Sender<OutputUpdate>) {
    info!("Spawning dbusdata generation thread");
    thread::spawn(move || {
        let dd = DbusData::new(tx);
    });
}
