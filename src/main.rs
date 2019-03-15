mod pwgen;
mod config;
mod runner;
mod qrwifi;
mod unifi;

use config::Config;
use std::env::args;
use std::fs::read_to_string;
use std::error::Error;
use toml;
use runner::Runner;
use std::{thread, time};

fn main() -> Result<(), Box<Error>> {
    let config_path = args().nth(1).expect("USAGE: wifi-pw-gen <config file path>");
    let mut config: Config = toml::from_str(&read_to_string(config_path)?)?;
    let output = Runner::spawn(config)?;
    thread::sleep(time::Duration::from_secs(10));
    println!("{}", output.lock().unwrap());
    Ok(())
}
