use crate::{encodes::decode_gbk, net::resolve_to_ipv4};
use arc_bytes::ArcBytes;
use core::time;
use std::{
    net::{Ipv4Addr, UdpSocket},
    time::Duration,
};

pub type AddressInfo = String;

#[derive(Debug)]
pub struct HandshakeInfo {
    pub address_info: AddressInfo,
    pub parse_address: String,
    pub servername: String,
    pub gamemode: String,
    pub mapname: String,
    pub maxplayers: u16,
    pub online: u16,
    pub players: Vec<String>,
    pub version: String,
    pub password: bool,
    pub elapsed: Duration,
}

impl<'a>
    From<(
        AddressInfo,
        ArcBytes<'a>,
        ArcBytes<'a>,
        ArcBytes<'a>,
        Duration,
    )> for HandshakeInfo
{
    fn from(
        (address_info, handshake_packet, mut resp_i, mut resp_c, elapsed): (
            AddressInfo,
            ArcBytes,
            ArcBytes,
            ArcBytes,
            Duration,
        ),
    ) -> Self {
        let handshake = ArcBytes::from(handshake_packet[4..8].to_vec());
        let addr =
            Ipv4Addr::new(handshake[0], handshake[1], handshake[2], handshake[3]).to_string();

        let version = {
            decode_gbk(resp_i.read_bytes(12).unwrap().as_slice())
                .trim_end_matches('\0')
                .to_string()
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
            address_info,
            parse_address: addr,
            servername,
            gamemode,
            mapname,
            maxplayers,
            online,
            players,
            version,
            password,
            elapsed,
        }
    }
}

pub fn handshake(address: &str) -> Result<HandshakeInfo, &str> {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
    let addr = resolve_to_ipv4(address).expect("Failed to resolve address");
    socket
        .set_read_timeout(Some(time::Duration::new(5, 0)))
        .expect("Failed to set timeout");
    socket.connect(addr).expect("Failed to connect to server");

    // Send handshake packet
    // VCMP
    // address bytes
    // port short 2
    // type i or c

    // build packet
    let mut packet: Vec<u8> = Vec::new();

    packet.extend_from_slice(b"VCMP");

    // address
    packet.extend_from_slice(&addr.ip().to_string().parse::<Ipv4Addr>().unwrap().octets());

    // port
    packet.extend_from_slice(&addr.port().to_le_bytes());

    // clone
    let mut packet_i = packet.clone();
    let mut packet_c = packet.clone();

    // type
    packet_i.push(b'i');
    packet_c.push(b'c');

    let handshake_length = packet.len() + 1;
    let start = std::time::Instant::now();

    let resp_packet_i = {
        socket.send(&packet_i).expect("Failed to send packet");
        let mut buf = [0; 1024];
        let res = socket.recv_from(&mut buf);
        if res.is_err() {
            return Err("Failed to receive response.");
        };
        let (size, _) = res.unwrap();
        ArcBytes::from(buf[handshake_length..size].to_vec())
    };
    let elapsed = start.elapsed();

    let resp_packet_c = {
        socket.send(&packet_c).expect("Failed to send packet");
        let mut buf = [0; 1024];
        let res = socket.recv_from(&mut buf);
        if res.is_err() {
            return Err("Failed to receive response.");
        };
        let (size, _) = res.unwrap();
        ArcBytes::from(buf[handshake_length..size].to_vec())
    };

    Ok(HandshakeInfo::from((
        address.to_string(),
        ArcBytes::from(packet),
        resp_packet_i,
        resp_packet_c,
        elapsed,
    )))
}
