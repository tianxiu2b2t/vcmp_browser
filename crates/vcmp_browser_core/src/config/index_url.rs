use serde_derive::Deserialize;
use std::default::Default;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct IndexUrl {
    #[serde(default = "default_master_url")]
    master: String,
    #[serde(default = "default_update_url")]
    update: String,
}

impl IndexUrl {
    pub fn get_master(&self) -> String {
        if self.master.is_empty() {
            // set to master
            return default_master_url();
        }

        self.master.clone()
    }

    pub fn get_update(&self) -> String {
        if self.update.is_empty() {
            // set to update
            return default_update_url();
        }

        self.update.clone()
    }
}

impl From<IndexUrl> for toml::Value {
    fn from(url: IndexUrl) -> Self {
        let mut root_table = toml::Table::new();
        root_table.insert("master".to_string(), toml::Value::String(url.get_master()));
        root_table.insert("update".to_string(), toml::Value::String(url.get_update()));

        toml::Value::Table(root_table)
    }
}

fn default_master_url() -> String {
    "https://txit.top/vcmp".to_string()
}
fn default_update_url() -> String {
    "https://txit.top/vcmp".to_string()
}
