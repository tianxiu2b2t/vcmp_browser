use serde::{
    Deserialize,
    Serialize,
};
use std::time::Duration;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddressInfo {
    pub ip: String,
    pub port: u16,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Server {
    pub addr: AddressInfo
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InternetServer {
    pub addr: AddressInfo,
    pub official: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub addr: AddressInfo,
    pub servername: String,
    pub gamemode: String,
    pub mapname: String,
    pub version: String,
    pub online: u16,
    pub maxplayers: u16,
    pub password: bool,
    pub players: Vec<String>,
    pub elapsed: Duration,
}