use std::path::PathBuf;
use std::fs::File;
use std::io::Read;  // read_to_string

use serde::{Serialize, Deserialize};
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub url: String,
    pub key: String,
}

impl Config {
    pub fn default() -> Config {
        Config {
            url: "https://uri".to_string(),
            key: "s3cr3tk3y".to_string(),
        }
    }
    pub fn parse(&mut self, config_file: &PathBuf) -> Config {
        let mut file = File::open(config_file).unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s).unwrap();
        let myconfig: Self = toml::from_str(&s).unwrap();
        myconfig
    }
}

