use serde::{Deserialize, Serialize};
use tauri::{Emitter, Window};
use vcmp_browser_core::{
    handshake::{handshake, IntoAddr}, internet
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InternetServerResponse {
    pub ip: String,
    pub port: u16,
    pub servername: String,
    pub gamemode: String,
    pub info_players: usize,
    pub info_maxplayers: usize,
    pub players: Vec<String>,
    pub version: String,
    pub ping: usize,
}

#[tauri::command]
pub async fn fetch_internet(window: Window) -> Result<(), String> {
    let fetch_servers = match internet::fetch_internet_servers().await {
        Ok(servers) => servers,
        Err(e) => {
            return Err(format!("{e}"));
        }
    };
    
    let mut tasks = vec![];

    for server in fetch_servers {
        let window = window.clone();
        tasks.push(
            tokio::spawn(async move {
                let info = match handshake(&server.addr.into_addr()).await {
                    Ok(info) => info,
                    Err(_) => {
                        return;
                    }
                };
                let _ = window.emit("fetch_internet", info);
            })
        )
    }

    // set timeout for 5s
    tokio::select! {
        _ = futures::future::join_all(tasks) => {}
        _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {}
    }
    
    Ok(())

}