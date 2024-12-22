use std::{fs::read_to_string, process};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub user: User,
    pub auth: Auth,
    pub display: Display,
    pub internal: Internal,
}

#[derive(Deserialize)]
pub struct User {
    pub username: String,
    pub tag: String,
    pub region: Option<String>,
    pub platform: Option<String>,
}

#[derive(Deserialize)]
pub struct Auth {
    pub key: String,
}

#[derive(Deserialize)]
pub struct Display {
    pub template: String,
    pub file_path: String,
}

#[derive(Deserialize)]
pub struct Internal {
    pub interval: Option<u64>,
}

impl Config {
    pub fn from_file(path: &str) -> Self {
        let s = read_to_string(path).expect("cannot read file");
        return Config::from_string(&s);
    }
    pub fn from_string(raw_string: &str) -> Self {
        let value: Result<Self, toml::de::Error> = toml::from_str(raw_string);
        match value {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[PANIC] Unrecoverable error, exiting. See the error message below for more information.");
                eprintln!("[CONFIG] failed to parse config string: {}", e.message());
                eprintln!("[HELP] Please take a look at the documentation for the config file, or open an issue on github.");
                process::exit(1)
            }
        }
    }
}
