use std::{default::Default, path::PathBuf, sync::OnceLock};

use tracing::{Level, event};

use crate::utils::get_home_dir;

pub mod index_url;

use index_url::IndexUrl;

#[derive(Debug, Clone)]
pub struct Config {
    pub index_url: IndexUrl
}


impl From<toml::Table> for Config {
    fn from(value: toml::Table) -> Self {
        Self {
            index_url: IndexUrl::from(value["index_url"].as_table().unwrap().clone()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            index_url: IndexUrl::default(),
        }
    }
}

impl Into<toml::Value> for Config {
    fn into(self) -> toml::Value {
        let mut table = toml::value::Table::new();
        table.insert("index_url".to_string(), self.index_url.into());
        toml::Value::Table(table)
    }
}

impl Config {
    fn to_pretty_string() -> String {
        format!(
            r#"# vcmp_browser config
# This is a config file for vcmp_browser.
# You can edit this file to change the config of vcmp_browser.
# If you don't know what you are doing, please don't edit this file.

{}"#,
            toml::to_string_pretty(&CONFIG.get().unwrap().into()).unwrap()
        )
    }

    fn load_from_path(path: PathBuf) -> Self {
        event!(Level::INFO, "Load config from {}", path.display());
        if !path.exists() {
            event!(Level::INFO, "Not found config. Use default config.");
            return Self::default();
        } 
        let config_content = std::fs::read_to_string(path).unwrap();
        Self::from(toml::from_str::<toml::Table>(&config_content).unwrap())
    }

    fn save_to_path(&self, path: PathBuf) {
        event!(Level::INFO, "Save config to {}", path.display());
        std::fs::write(path, Config::to_pretty_string()).unwrap();
    }
}



pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init() {
    let config = &CONFIG;
    let config_path = get_home_dir().join("config.toml");
    config.set(Config::load_from_path(config_path.clone())).unwrap();

    get_config().save_to_path(config_path.clone());
}

pub fn get_config() -> Config {
    CONFIG.get().unwrap().clone()
}
