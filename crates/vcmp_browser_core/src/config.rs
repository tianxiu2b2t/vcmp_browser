/*use std::{default::Default, path::PathBuf, sync::OnceLock};

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

impl Into<toml::Table> for Config {
    fn into(self) -> toml::Table {
        let mut table = toml::Table::new();
        table.insert("index_url".to_string(), self.index_url.into());
        toml::Value(table).into()
    }
}

impl Config {
    fn to_pretty_string() -> String {
        use toml::to_string_pretty;
        format!(
            r#"# vcmp_browser config
# This is a config file for vcmp_browser.
# You can edit this file to change the config of vcmp_browser.
# If you don't know what you are doing, please don't edit this file.

{}"#,
            to_string_pretty::<toml::Table>(get_config().into()).unwrap()
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

pub fn get_config_file() -> PathBuf {
    get_home_dir().join("config.toml")
}

pub fn init() {
    let config = &CONFIG;
    let config_path = get_config_file();
    config.set(Config::load_from_path(config_path.clone())).unwrap();

    get_config().save_to_path(config_path.clone());
}

pub fn get_config() -> Config {
    CONFIG.get().unwrap().clone()
}
*/

//use serde_derive::Deserialize;

pub mod index_url;

use index_url::IndexUrl;
use serde_derive::Deserialize;

use std::sync::OnceLock;
use std::default::Default;

use crate::utils::get_home_dir;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MainConfig {
    pub index_url: IndexUrl,
}


impl MainConfig {
    pub fn load_from_path() -> Self {
        let path = get_home_dir().join("config.toml");
        if !path.exists() {
            tracing::event!(tracing::Level::INFO, "Config file not found, using default config.");
            Self::default().save_to_path();
        }
        let config_content = std::fs::read_to_string(path).unwrap();
        toml::from_str::<Self>(&config_content).unwrap()
    }

    pub fn save_to_path(&self) {
        let path = get_home_dir().join("config.toml");

        use toml::to_string_pretty;

        let mut root_table = toml::Table::new();
        root_table.insert("index_url".to_string(), self.index_url.clone().into());
        let config_content = to_string_pretty(&root_table).unwrap();
        
        std::fs::write(path, config_content).unwrap();
        

    }
}

pub static CONFIG: OnceLock<MainConfig> = OnceLock::new();

pub fn get_config() -> MainConfig {
    CONFIG.get().unwrap().clone()
}

pub fn init() {
    let config = &CONFIG;
    config.set(MainConfig::load_from_path()).unwrap();
}