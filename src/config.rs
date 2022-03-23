use serde::Deserialize;
use std::{fs, net::SocketAddr, path::Path};

#[derive(Deserialize, Clone)]
pub struct Config {
    pub hosts: Vec<Host>,
    pub listen_addr: Option<SocketAddr>,
}

#[derive(Deserialize, Clone)]
pub struct Host {
    pub source: String,          // ip/domain string
    pub destination: SocketAddr, // ip:port string that is parsed
}

pub fn get_config(path: Option<String>) -> Config {
    let raw_config = read_config(&path.unwrap_or("/etc/embudo/config.toml".to_string()));
    let parsed_config: Config = toml::from_slice(&raw_config).expect("Unable to parse config");
    return parsed_config;
}

fn read_config(path: &str) -> Vec<u8> {
    return fs::read(Path::new(path)).expect(&format!("Unable to read config from {}", path));
}
