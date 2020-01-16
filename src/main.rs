mod timestamp;

use std::sync::mpsc::{channel, Sender};
use std::thread;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about)]
struct Opt {
    /// Set timestamp update accuracy: seconds, minutes
    #[structopt(long, parse(from_str = parse_accuracy), default_value = "seconds")]
    time_accuracy: timestamp::Accuracy,

    /// Enable debug printing to stderr
    #[structopt(long, short)]
    debug: bool,
}

fn parse_accuracy(arg: &str) -> timestamp::Accuracy {
    let lowered = arg.to_lowercase();
    if lowered == "minutes" {
        timestamp::Accuracy::Minutes
    } else if lowered == "seconds" {
        timestamp::Accuracy::Seconds
    } else {
        eprintln!("Invalid accuracy {:?}, defaulting to seconds", &lowered);
        timestamp::Accuracy::Seconds
    }
}

pub type GeneratorResult<T> = Result<(), T>;

pub trait OutputGenerator {
    type Config;
    type Error;

    fn new(tx: Sender<String>, config: Self::Config) -> Self;
    fn generate(&self) -> GeneratorResult<Self::Error>;
}

fn main() {
    let opt = Opt::from_args();
    if opt.debug {
        eprintln!("Cli options: {:?}", opt);
    }

    let (tx, rx) = channel();

    let timestamp_tx = tx.clone();
    let timestamp_config = timestamp::TimestampConfig {
        debug: opt.debug,
        accuracy: opt.time_accuracy,
    };

    thread::spawn(move || {
        let t = timestamp::TimestampGenerator::new(timestamp_tx, timestamp_config);
        t.generate().unwrap();
    });

    loop {
        let timestamp = rx.recv().unwrap();
        println!("{}", timestamp);
    }
}
