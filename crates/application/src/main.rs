// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    vcmp_launcher_lib::run()
}

/*use std::path::Path;

use tracing::event;
use vcmp_browser_core::{config::init as init_config, game_launcher::launcher_common_game};

pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
}

pub fn main() {
    init_logger();
    init_config();
    let res = launcher_common_game(
        Path::new("D:\\Files\\VC"),
        Path::new("C:\\Users\\2b2ttianxiu\\AppData\\Local\\Vice City Multiplayer\\0.4.7.1"),
        "-c -h 127.0.0.1 -c -p 8192 -n test1".to_string(),
    );
    if let Err(e) = res {
        event!(tracing::Level::ERROR, "Error: {}", e);
    }
}*/
