#[derive(Debug, Clone)]
pub struct IndexUrl {
    pub master: String,
    pub update: String,
}

impl Default for IndexUrl {
    fn default() -> Self {
        Self {
            master: "https://txit.top/vcmp".to_string(),
            update: "https://txit.top/vcmp".to_string(),
        }
    }
}

impl From<toml::Value> for IndexUrl {
    fn from(value: toml::Table) -> Self {
        Self {
            master: value["master"].as_str().unwrap().to_string(),
            update: value["update"].as_str().unwrap().to_string(),
        }
    }
}

impl Into<toml::Value> for IndexUrl {
    fn into(self) -> toml::Value {
        let mut value = toml::value::Table::new();
        value.insert("master".to_string(), toml::Value::String(self.master));
        value.insert("update".to_string(), toml::Value::String(self.update));
        toml::Value::Table(value)
    }
}