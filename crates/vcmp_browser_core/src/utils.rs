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

pub fn move_files(src: &std::path::Path, dst: &std::path::Path) -> Result<(), String> {
    if src.is_file() && dst.is_file() {
        std::fs::rename(src, dst).expect("Failed to move file");
    } else if src.is_dir() && dst.is_dir() {
        let files = std::fs::read_dir(src).expect("Failed to read directory");
        for file in files {
            let file = file.expect("Failed to read file");
            let file_path = file.path();
            let dst_path = dst.join(file_path.file_name().expect("Failed to get file name"));

            move_files(&file_path, &dst_path).expect("Failed to move file");
        }
    }
    Ok(())
}
