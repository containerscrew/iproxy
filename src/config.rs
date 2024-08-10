use serde::Deserialize;
use std::fs;
use mongodb::Database;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub(crate) address: String,
    pub(crate) port: u16,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub(crate) log_level: String,
}

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub(crate) endpoint: String,
    pub(crate) db_name: String,
    pub(crate) collection_name: String,
}


#[derive(Deserialize)]
pub struct Config {
    pub(crate) server: ServerConfig,
    pub(crate) logging: LoggingConfig,
    pub(crate) database: DatabaseConfig,
}

impl Config {
    pub(crate) fn from_file(path: &str) -> Self {
        let config_content = fs::read_to_string(path)
            .expect("Failed to read configuration file");
        toml::from_str(&config_content)
            .expect("Failed to parse configuration file")
    }
}