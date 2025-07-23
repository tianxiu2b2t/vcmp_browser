use std::path::PathBuf;
use std::net::SocketAddr;
use encoding_rs;

use crate::error::{
    VCMPError,
    VCMPResult,
};

pub trait ProgressBar {
    /// Set the progress of the progress bar.
    /// The progress should be a value between 0 and 1.
    fn set_progress(&self, progress: f32);
}

pub struct EmptyProgressBar;

impl ProgressBar for EmptyProgressBar {
    fn set_progress(&self, _progress: f32) {
        // Do nothing
    }
}

// gbk to utf-8
pub fn decode_gbk(bytes: &[u8]) -> String {
    let (result, _, _) = encoding_rs::GBK.decode(bytes);
    result.to_string()
}

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

// 解析地址（域名或IP）为SocketAddr，优先返回IPv4地址
pub fn resolve_address(addr: &str) -> VCMPResult<SocketAddr> {
    use std::net::ToSocketAddrs;

    // 解析地址（可处理域名或直接IP）
    let addrs = addr.to_socket_addrs().map_err(|_| {
        VCMPError::FailedToResolveAddress(addr.to_string()) // 修正了拼写错误 Resolve
    })?;

    // 分离IPv4和IPv6地址
    let (ipv4_addrs, ipv6_addrs): (Vec<_>, Vec<_>) = addrs
        .partition(|addr| matches!(addr, SocketAddr::V4(_)));

    // 优先返回IPv4地址，如果有的话
    if let Some(ipv4_addr) = ipv4_addrs.first() {
        return Ok(*ipv4_addr);
    }

    // 如果没有IPv4地址，尝试返回IPv6地址
    if let Some(ipv6_addr) = ipv6_addrs.first() {
        return Ok(*ipv6_addr);
    }

    // 如果没有解析到任何地址，返回错误
    Err(VCMPError::FailedToResolveAddress(addr.to_string()))
}

pub fn format_addr(
    addr: String,
    port: u16,
) -> String {
    // try parse ipv6
    if let Ok(ipv6) = addr.parse::<std::net::Ipv6Addr>() {
        return format!("[{}]:{}", ipv6, port);
    }

    format!("{}:{}", addr, port)
}