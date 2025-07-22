pub mod game;
pub mod index_url;
pub mod profile;

use serde_derive::Deserialize;

use std::default::Default;
use std::sync::OnceLock;

use crate::config::{game::GameSettings, index_url::IndexUrl, profile::Profile};
use crate::utils::get_home_dir;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct MainConfig {
    pub index_url: IndexUrl,
    pub profile: Profile,
    pub game: GameSettings,
}

impl MainConfig {
    pub fn load_from_path() -> Self {
        let path = get_home_dir().join("config.toml");
        if !path.exists() {
            tracing::event!(
                tracing::Level::INFO,
                "Config file not found, using default config."
            );
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
        root_table.insert("profile".to_string(), self.profile.clone().into());
        root_table.insert("game".to_string(), self.game.clone().into());
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
