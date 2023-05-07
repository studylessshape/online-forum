pub mod entity;

use std::sync::Mutex;

use clap::Parser;
use once_cell::sync::Lazy;

use self::entity::ServerConfiguration;

#[derive(Parser, Debug)]
#[command(version, long_about = None)]
pub struct ServeCommandArgs {
    /// Network port to use, range is `[1..65565]`
    #[arg(short, long, default_value_t = 8082, value_parser = clap::value_parser!(u16).range(1..))]
    pub port: u16,
    /// Allow orign, default is allow any orign
    #[arg(short, long)]
    pub origins: Vec<String>,
    /// Allow method, Default is allow any method
    #[arg(short, long)]
    pub methods: Vec<String>,
}

pub static SERVER_CONFIGURATION: Lazy<Mutex<ServerConfiguration>> =
    Lazy::new(|| Mutex::new(Default::default()));

pub static SERVER_CONFIGURATION_PATH: &str = "server.conf.toml";
