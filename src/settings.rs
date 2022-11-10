use config::{Config, ConfigError};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Settings {
    pub server: Server,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub ip: String,
    pub port: u32,
}

impl Server {
    pub fn get_ip(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}


impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        const CURRENT_DIR: &str = "./config/Settings.toml";

        let s = Config::builder().add_source(config::File::with_name(CURRENT_DIR)).build()?;

        s.try_deserialize()
    }
}
