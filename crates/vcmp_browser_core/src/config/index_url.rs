use serde_derive::Deserialize;
use std::default::Default;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct IndexUrl {
    #[serde(default = "default_master_url")]
    pub master: String,
    #[serde(default = "default_update_url")]
    pub update: String,
}

impl From<IndexUrl> for toml::Value {
    fn from(url: IndexUrl) -> Self {
        let mut root_table = toml::Table::new();
        root_table.insert("master".to_string(), toml::Value::String(url.master));
        root_table.insert("update".to_string(), toml::Value::String(url.update));
        
        toml::Value::Table(root_table)
    }
}


fn default_master_url() -> String { "https://txit.top/vcmp".to_string() }
fn default_update_url() -> String { "https://txit.top/vcmp".to_string() }