pub mod tauri_command;

use vcmp_browser_core::config::init as init_config;

pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logger();
    init_config();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![tauri_command::fetch_internet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
