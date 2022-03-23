use serde::Deserialize;
use std::{fs, net::SocketAddr, path::Path};

#[derive(Deserialize, Clone)]
pub struct Config {
    pub hosts: Vec<Host>,
    pub port: Option<u16>,
}

#[derive(Deserialize, Clone)]
pub struct Host {
    pub source: String,          // ip/domain string
    pub destination: SocketAddr, // ip:port string that is parsed
}

pub fn get_config(path: Option<&str>) -> Config {
    let raw_config = read_config(path.unwrap_or("/etc/embudo/config.toml"));
    let parsed_config: Config = toml::from_slice(&raw_config).expect("Unable to parse config");
    return parsed_config;
}

fn read_config(path: &str) -> Vec<u8> {
    return fs::read(Path::new(path)).expect("Unable to read config");
}
