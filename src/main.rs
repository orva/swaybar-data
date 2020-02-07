mod config;
mod dbusdata;
mod error;
mod generated;
mod output;
mod timestamp;

use config::*;
use dbusdata::*;
use output::*;
use timestamp::*;

use env_logger;
use log::{error, info, LevelFilter};
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

    let config = match Config::read_from(&opt.config) {
        Ok(conf) => conf,
        Err(err) => {
            error!("Error while parsing config: {}", err);
            std::process::exit(1);
        }
    };

    let (tx, rx) = channel();

    let mut outputs: Vec<Output> = config
        .outputs
        .into_iter()
        .map(|output_config| Output::from(&output_config))
        .collect();

    let mut dbusdata_builder = match DBusdata::new() {
        Ok(builder) => builder,
        Err(err) => {
            error!("Error while connecting to DBus {}", err);
            std::process::exit(2);
        }
    };

    for (i, output) in outputs.iter().enumerate() {
        match output {
            Output::Timestamp(ref ts) => {
                start_timestamp_generation(tx.clone(), ts.config.clone(), i);
            }
            Output::Battery(ref _bat) => {
                dbusdata_builder
                    .with_config((i, OutputConfig::Battery))
                    .unwrap();
            }
        }
    }

    start_dbusdata_generation(tx.clone(), dbusdata_builder);

    loop {
        let OutputUpdate { id, update } = match rx.recv() {
            Err(err) => {
                error!("Error while receiving mspc messages {}", err);
                break;
            }
            Ok(up) => up,
        };

        let output_changed = outputs[id].update(update);

        if output_changed {
            let output = outputs
                .iter()
                .map(|out| out.as_plain())
                .collect::<Vec<String>>()
                .join(" | ");

            println!("{}", output);
        }
    }
}

fn start_timestamp_generation(tx: Sender<OutputUpdate>, config: TimestampConfig, id: usize) {
    info!("Spawning timestamp generation thread");
    thread::spawn(move || {
        let timestamps = TimestampGenerator::new(config);
        for update in timestamps {
            tx.send(OutputUpdate {
                id,
                update: UpdateType::Timestamp(update),
            })
            .unwrap();
        }
    });
}

fn start_dbusdata_generation(tx: Sender<OutputUpdate>, mut builder: DBusdata) {
    info!("Spawning dbusdata generation thread");
    thread::spawn(move || {
        if let Err(err) = builder.start_listening(tx) {
            error!("DBus data processing stopped with error {}", err);
        }
    });
}
