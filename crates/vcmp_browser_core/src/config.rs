use std::{default::Default, sync::OnceLock};

#[derive(Debug, Clone)]
pub struct Config {
    pub master: String,
    pub update: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            master: "https://txit.top/vcmp".to_string(),
            update: "https://txit.top/vcmp".to_string()
        }
    }
}

impl From<toml::Table> for Config {
    fn from(value: toml::Table) -> Self {
        Self {
            master: value["master"].as_str().unwrap().to_string(),
            update: value["update"].as_str().unwrap().to_string(),
        }
    }
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init() {
    let config = &CONFIG;
    config.set(Config::default()).unwrap();
}

pub fn get_config() -> Config {
    CONFIG.get().unwrap().clone()
}