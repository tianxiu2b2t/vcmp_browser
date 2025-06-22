use std::path::PathBuf;

pub fn get_home_dir() -> PathBuf {
    std::env::home_dir().unwrap_or(PathBuf::from("./appdata"))
}