use serde_derive::Deserialize;
use std::default::Default;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Profile {
    #[serde(default = "default_empty")]
    pub username: String,
}

impl From<Profile> for toml::Value {
    fn from(url: Profile) -> Self {
        let mut root_table = toml::Table::new();
        root_table.insert("username".to_string(), toml::Value::String(url.username));
        
        toml::Value::Table(root_table)
    }
}


fn default_empty() -> String { "".to_string() }