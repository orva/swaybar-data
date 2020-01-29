mod config;
mod dbusdata;
mod error;
mod generated;
mod timestamp;

use config::*;
use dbusdata::*;
use timestamp::*;

use env_logger;
use log::{error, info, LevelFilter};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use structopt::StructOpt;

pub struct OutputUpdate {
    id: usize,
    update: UpdateType,
}

pub enum UpdateType {
    Timestamp(String),
    Percentage(f64),
    OnBattery(bool),
}

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

enum Output {
    Timestamp(Timestamp),
    Battery(Battery),
}

struct Timestamp {
    state: String,
    config: timestamp::TimestampConfig,
}

struct Battery {
    state: BatteryState,
}

#[derive(Debug, Clone, Default)]
pub struct BatteryState {
    percentage: f64,
    on_battery: bool,
}

impl Output {
    fn update(&mut self, update: UpdateType) {
        match self {
            Output::Timestamp(ref mut ts) => {
                if let UpdateType::Timestamp(s) = update {
                    ts.state = s;
                }
            }
            Output::Battery(ref mut bat) => match update {
                UpdateType::Percentage(p) => bat.state.percentage = p,
                UpdateType::OnBattery(b) => bat.state.on_battery = b,
                _ => {}
            },
        }
    }
}

impl From<&config::OutputConfig> for Output {
    fn from(c: &config::OutputConfig) -> Self {
        match c {
            OutputConfig::Timestamp(c) => Output::Timestamp(Timestamp {
                state: "".to_string(),
                config: c.clone(),
            }),
            OutputConfig::Battery => Output::Battery(Battery {
                state: BatteryState::default(),
            }),
        }
    }
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

        outputs[id].update(update);

        let output = outputs
            .iter()
            .map(|out| match out {
                Output::Timestamp(ref ts) => ts.state.clone(),
                Output::Battery(ref bat) => format!(
                    "discharging: {}, percentage: {}",
                    bat.state.on_battery,
                    bat.state.percentage.to_string()
                ),
            })
            .collect::<Vec<String>>()
            .join(" | ");

        println!("{}", output);
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
