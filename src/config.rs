use serde::Deserialize;
use std::fs;
use tracing::trace;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub(crate) address: String,
    pub(crate) port: u16,
    pub(crate) use_proxy: bool,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub(crate) log_level: String,
    pub(crate) log_type: String,
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
    // This function loads the configuration from the file
    pub(crate) fn load_config() -> Self {
        let path = Config::get_config_path(); // Get the config file path
        let config_content = fs::read_to_string(path).expect("Failed to read configuration file");
        match toml::from_str(&config_content) {
            Ok(config) => {
                trace!("Configuration loaded successfully");
                config
            }
            Err(e) => {
                panic!("Failed to parse configuration file: {}", e);
            }
        }
    }

    // Helper function to get the configuration file path
    fn get_config_path() -> String {
        std::env::var("CONFIG_FILE_PATH").unwrap_or_else(|_| "./config.toml".to_string())
    }
}
