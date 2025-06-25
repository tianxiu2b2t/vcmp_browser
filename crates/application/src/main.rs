use tracing::event;
use vcmp_browser_core::config::init as init_config;

pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
}

pub fn main() {
    init_logger();
    init_config();
    let resource = vcmp_browser_core::resources::download_resource("0.4.7.1").expect("Failed to download resource");
    event!(tracing::Level::INFO, "Downloaded resource: {:?}", resource);
}
