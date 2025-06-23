use std::path::PathBuf;

pub fn get_home_dir() -> PathBuf {
    #[cfg(debug_assertions)]
    let path = PathBuf::from("./appdata");
    #[cfg(not(debug_assertions))]
    let path = std::env::home_dir()
        .unwrap_or(PathBuf::from("appdata"))
        .join("vcmp_browser");
    std::fs::create_dir_all(&path).unwrap();
    path
}
