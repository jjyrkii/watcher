use chrono::Local;
use std::{
    env,
    fs::{self, OpenOptions},
    io::prelude::*,
    thread::sleep,
    time::Duration,
};

use dotenvy::dotenv;
mod picture;

fn main() {
    dotenv().expect(".env file not found");

    let data_dir = env::var("DATA_DIR").expect("DATA_DIR not set correctly");
    if let Err(_) = fs::metadata(&data_dir) {
        fs::create_dir(data_dir).expect("failed creating data directory");
    }

    let log_dir = env::var("LOG_DIR").expect("LOG_DIR not set correctly");
    if let Err(_) = fs::metadata(&log_dir) {
        fs::create_dir(log_dir).expect("failed creating log directory");
    }

    let interval: u64 = env::var("INTERVAL")
        .expect("INTERVAL not set correctly")
        .parse()
        .unwrap();

    let duration = Duration::new(interval, 0);
    loop {
        match picture::take_picture() {
            Ok(res) => logger("pictures", res),
            Err(err) => logger("pictures_error", err.to_string()),
        };
        sleep(duration);
    }
}

fn logger(file_name: &str, msg: String) {
    let log_dir = env::var("LOG_DIR").unwrap();
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(format!("{}/{}.log", log_dir, file_name))
        .unwrap();
    if let Err(e) = writeln!(file, "[{}]: {}", timestamp, msg) {
        eprintln!("Couldn't write to file: {}", e)
    }
}
