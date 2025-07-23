use std::time::Duration;

use arc_bytes::ArcBytes;
use tokio::net::UdpSocket;

use crate::types::{AddressInfo, ServerInfo};
use crate::util::{
    decode_gbk, format_addr, resolve_address
};
use crate::error::{
    VCMPError,
    VCMPResult
};

pub trait IntoAddr {
    fn into_addr(self) -> String;
}

impl IntoAddr for AddressInfo {
    fn into_addr(self) -> String {
        format_addr(self.ip, self.port)
    }
}

impl From<(AddressInfo, Vec<u8>, Vec<u8>, Duration)> for ServerInfo {
    fn from((addr, resp_i, resp_c, elapsed): (AddressInfo, Vec<u8>, Vec<u8>, Duration)) -> Self {
        // first 4 bytes is "VCMP"
        let mut resp_i = ArcBytes::from(resp_i[11..].to_vec());
        let mut resp_c = ArcBytes::from(resp_c[11..].to_vec());

        // version
        let version = {
            decode_gbk(resp_i.read_bytes(12).unwrap().as_slice()).trim_end_matches('\0').to_string()
        };

        let password = { resp_i.read_bytes(1).unwrap()[0] == 1 };

        let online = {
            let res = resp_i.read_bytes(2).unwrap();
            u16::from_le_bytes([res[0], res[1]])
        };

        let maxplayers = {
            let res = resp_i.read_bytes(2).unwrap();
            u16::from_le_bytes([res[0], res[1]])
        };

        // servername
        let servername = {
            let length = {
                let res = resp_i.read_bytes(4).unwrap();
                u32::from_le_bytes([res[0], res[1], res[2], res[3]]) as usize
            };
            decode_gbk(resp_i.read_bytes(length).unwrap().as_slice())
        };

        let gamemode = {
            let length = {
                let res = resp_i.read_bytes(4).unwrap();
                u32::from_le_bytes([res[0], res[1], res[2], res[3]]) as usize
            };
            decode_gbk(resp_i.read_bytes(length).unwrap().as_slice())
        };

        let mapname = {
            let length = {
                let res = resp_i.read_bytes(4).unwrap();
                u32::from_le_bytes([res[0], res[1], res[2], res[3]]) as usize
            };
            decode_gbk(resp_i.read_bytes(length).unwrap().as_slice())
        };
        
        // c

        let players = {
            let mut res: Vec<String> = vec![];
            let count = {
                let res = resp_c.read_bytes(2).unwrap();
                u16::from_le_bytes([res[0], res[1]])
            };
            for _ in 0..count {
                let length = resp_c.read_bytes(1).unwrap()[0];
                let name = decode_gbk(resp_c.read_bytes(length as usize).unwrap().as_slice());
                res.push(name);
            }
            res
        };

        Self {
            addr,
            servername,
            gamemode,
            mapname,
            version,
            online,
            maxplayers,
            password,
            players,
            elapsed,
        }
    }
}

pub async fn handshake(address: &String) -> VCMPResult<ServerInfo> {
    let addr = match resolve_address(address.as_str()) {
        Ok(addr) => addr,
        Err(e) => return Err(e),
    };

    let socket = UdpSocket::bind("0.0.0.0:0").await.expect("Failed to bind socket"); // 按道理不会 panic 

    if let Err(_) = socket.connect(addr).await {
        return Err(VCMPError::UnknownHost(addr));
    }

    // build packet
    let packet = { 
        let mut packet: Vec<u8> = Vec::new();
        packet.extend_from_slice(b"VCMP");
        // if not ipv4
        if addr.ip().is_ipv6() {
            // default 127.0.0.1
            packet.extend_from_slice(&[127, 0, 0, 1]);
        } else {
            packet.extend_from_slice(&addr.ip().to_string().parse::<std::net::Ipv4Addr>().unwrap().octets());
        }
        packet.extend_from_slice(&addr.port().to_le_bytes());
        packet
    };

    let packet_i = {
        let mut packet = packet.clone();
        packet.push(b'i');
        packet
    };
    let packet_c = {
        let mut packet = packet.clone();
        packet.push(b'c');
        packet
    };

    let start = std::time::Instant::now();

    let resp_packet_i = {
        let send_res = socket.send(&packet_i).await;
        if let Err(e) = send_res {
            return Err(VCMPError::SocketSendError(e.to_string()));
        };
        let mut buf = [0; 1024];
        let recv_res = socket.recv(&mut buf).await;
        if let Err(e) = recv_res {
            return Err(VCMPError::SocketRecvError(e.to_string()));
        }
        buf[0..recv_res.unwrap()].to_vec()
    };

    let elapsed = start.elapsed();

    let resp_packet_c = {
        let send_res = socket.send(&packet_c).await;
        if let Err(e) = send_res {
            return Err(VCMPError::SocketSendError(e.to_string()));
        };
        let mut buf = [0; 1024];
        let recv_res = socket.recv(&mut buf).await;
        if let Err(e) = recv_res {
            return Err(VCMPError::SocketRecvError(e.to_string()));
        }
        buf[0..recv_res.unwrap()].to_vec()
    };

    // parse
    Ok(ServerInfo::from((
        AddressInfo {
            ip: addr.ip().to_string(),
            port: addr.port(),
        },
        resp_packet_i,
        resp_packet_c,
        elapsed
    )))
}