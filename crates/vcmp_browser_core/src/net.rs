use std::net::SocketAddr;

use crate::error::Error;

// 将地址解析为 IPv4 地址，如果无法解析则 panic
pub fn resolve_to_ipv4(addr: &str) -> Result<SocketAddr, Error> {
    use std::net::ToSocketAddrs;

    let addrs = addr.to_socket_addrs().expect("Failed to resolve address");

    // 过滤出 IPv4 地址
    let ipv4_addrs: Vec<_> = addrs
        .filter(|addr| matches!(addr, SocketAddr::V4(_)))
        .collect();

    // 如果没有找到 IPv4 地址，panic!
    if ipv4_addrs.is_empty() {
        return Err(Error::FailedToResloveAddress(addr.to_string()));
    }

    // 返回第一个 IPv4 地址
    Ok(ipv4_addrs[0])
}
