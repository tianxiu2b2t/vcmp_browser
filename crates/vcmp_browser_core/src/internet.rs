/*use std::fmt::Display;

use reqwest::Url;

use tracing::{Level, event};

use crate::config::get_config;

#[derive(Debug, Clone)]
pub struct Server {
    ip: String,
    port: u16,
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

#[derive(Debug, Clone)]
pub struct InternetServer {
    pub server: Server,
    pub official: bool,
}

impl InternetServer {
    pub fn ip(&self) -> String {
        self.server.ip.clone()
    }
    pub fn port(&self) -> u16 {
        self.server.port
    }
    pub fn is_official(&self) -> bool {
        self.official
    }
}

impl Display for InternetServer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.server)
    }
}

pub fn fetch_internet_servers() -> Vec<InternetServer> {
    let url = get_config().index_url.get_master();
    let uri = Url::parse(
        ({
            if url.ends_with("/") {
                format!("{url}servers/")
            } else {
                format!("{url}/servers/")
            }
        })
        .as_str(),
    )
    .expect("Failed to parse URL");
    event!(Level::INFO, "Fetching servers from {}", uri);
    let response = reqwest::blocking::get(uri);
    if let Err(e) = response {
        event!(Level::ERROR, "Failed to fetch servers: {}", e);
        return vec![];
    }

    let response = response.unwrap();
    let json = response.json();
    if let Err(e) = json {
        event!(Level::ERROR, "Failed to parse servers: {}", e);
        return vec![];
    }

    let json_data: serde_json::Value = json.unwrap();
    let mut servers = vec![];
    for server in json_data.get("servers").unwrap().as_array().unwrap() {
        servers.push(InternetServer {
            server: Server {
                ip: server.get("ip").unwrap().as_str().unwrap().to_string(),
                port: server.get("port").unwrap().as_u64().unwrap() as u16,
            },
            official: server.get("is_official").unwrap().as_bool().unwrap(),
        })
    }

    event!(Level::INFO, "Fetched {} servers", servers.len());

    servers
}
*/

use reqwest::Url;
use tracing::{
    event, Level
};

use crate::{error::{
    VCMPResult,
    VCMPError
}, types::InternetServer};
use crate::config::get_config;

pub async fn fetch_internet_servers() -> VCMPResult<Vec<InternetServer>> {
    let url = get_config().index_url.get_master();
    let uri = match Url::parse(
        ({
            if url.ends_with("/") {
                format!("{url}servers/")
            } else {
                format!("{url}/servers/")
            }
        })
        .as_str(),
    ) {
        Ok(uri) => uri,
        Err(e) => return Err(VCMPError::Error(e.to_string())),
    };

    let response = reqwest::get(uri).await;
    if let Err(e) = response {
        return Err(VCMPError::Error(e.to_string()));
    }

    let response = response.unwrap();
    let json = response.json::<serde_json::Value>().await;
    if let Err(e) = json {
        return Err(VCMPError::Error(e.to_string()));
    }

    let json_data = json.unwrap();
    let mut servers = vec![];
    for server in json_data.get("servers").unwrap().as_array().unwrap() {
        servers.push(InternetServer {
            addr: crate::types::AddressInfo {
                ip: server.get("ip").unwrap().as_str().unwrap().to_string(),
                port: server.get("port").unwrap().as_u64().unwrap() as u16,
            },
            official: server.get("is_official").unwrap().as_bool().unwrap(),
        })
    }

    event!(Level::INFO, "Fetched {} servers", servers.len());

    Ok(servers)
}