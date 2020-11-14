use std::fs;
use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

#[derive(Deserialize)]
pub struct ServerConfig {
    pub addr: String,
}

impl Config {
    pub fn from_file(path: &str) -> Config {
        let config: Config = toml::from_str(
            &fs::read_to_string(path)
            .unwrap()
        )
        .unwrap();
        
        config
    }
}
