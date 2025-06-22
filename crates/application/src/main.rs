use vcmp_browser_core::config::init as init_config;

pub fn init_logger() {
    tracing_subscriber::fmt().with_max_level(
        tracing::Level::TRACE,
    ).init();
}

pub fn main() {
    init_logger();
    init_config();
    tracing::event!(tracing::Level::INFO, "{:?}", vcmp_browser_core::resources::download_resource("https://txit.top/vcmp", "0.4.7.1"));
}