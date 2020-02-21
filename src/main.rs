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
use std::sync::mpsc::{channel, Sender, TryRecvError};
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

    // sleep a bit to give generators time to output stuff, then consume those updates
    std::thread::sleep(std::time::Duration::from_millis(1000));
    loop {
        match rx.try_recv() {
            Ok(OutputUpdate { id, update }) => {
                outputs[id].update(update);
            }
            Err(err) => match err {
                TryRecvError::Empty => break,
                TryRecvError::Disconnected => {
                    error!("Error while receiving mspc messages");
                    std::process::exit(3);
                }
            },
        }
    }
    output_plain_prompt(&outputs);

    loop {
        let OutputUpdate { id, update } = match rx.recv() {
            Err(_) => {
                error!("Error while receiving mspc messages");
                std::process::exit(3);
            }
            Ok(up) => up,
        };

        let output_changed = outputs[id].update(update);

        if output_changed {
            output_plain_prompt(&outputs);
        }
    }
}

fn output_plain_prompt(outputs: &Vec<Output>) {
    let output = outputs
        .iter()
        .map(|out| out.as_plain())
        .collect::<Vec<String>>()
        .join(" | ");

    println!("{}", output);
}

fn start_timestamp_generation(tx: Sender<OutputUpdate>, config: TimestampConfig, id: usize) {
    info!("Spawning timestamp generation thread");
    thread::spawn(move || {
        let timestamps = TimestampSource::new(config, id);
        if let Err(err) = timestamps.start_generating(tx) {
            error!("Timestamp generation stopped with error {}", err);
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
