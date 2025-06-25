use serde_derive::Deserialize;
use std::default::Default;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct GameSettings {
    #[serde(default = "default_empty")]
    pub application: String,
}

impl From<GameSettings> for toml::Value {
    fn from(url: GameSettings) -> Self {
        let mut root_table = toml::Table::new();
        root_table.insert("application".to_string(), toml::Value::String(url.application));
        
        toml::Value::Table(root_table)
    }
}


fn default_empty() -> String { "".to_string() }