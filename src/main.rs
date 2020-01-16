use chrono::prelude::*;
use chrono::Duration;
use std::thread::sleep;

fn main() {
    println!("Hello, world!");
    loop {
        let now = Utc::now();
        let output = now.format("%a %Y-%m-%d - %H:%M:%S").to_string();
        println!("{}", output);
        let sleep_duration = Duration::seconds(1)
            .to_std()
            .expect("sleep duration generation failed");
        sleep(sleep_duration);
    }
}
