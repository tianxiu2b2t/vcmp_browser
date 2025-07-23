use std::io::Cursor;

use reqwest::Url;
use tracing::{Level, event};

use crate::{config::get_config, util::get_home_dir};

#[derive(Clone, Debug)]
pub struct Resource {
    pub version: String,
    pub size: usize,
}

pub fn download_resource(version: &str) -> Result<Resource, String> {
    let download_buffer = {
        let url = &get_config().index_url.get_update();
        let uri = Url::parse(
            ({
                if url.ends_with("/") {
                    format!("{url}download/")
                } else {
                    format!("{url}/download/")
                }
            })
            .as_str(),
        )
        .expect("Failed to parse URL");
        let session = reqwest::blocking::Client::new();
        let form_data = reqwest::blocking::multipart::Form::new().text(
            "json",
            format!(r#"{{"password": "", "version": "{version}"}}"#),
        );
        let response = session
            .post(uri)
            .multipart(form_data)
            .send()
            .expect("Failed to send request");
        response.bytes().expect("Failed to read response")
    };

    event!(
        Level::INFO,
        "Resource total size: {} bytes",
        download_buffer.len()
    );

    // unpack for 7z
    let dest = get_home_dir().join("versions").join(version);
    event!(Level::INFO, "Unpacking resource to {dest:?}");

    let seek = Cursor::new(download_buffer.clone().to_vec());
    sevenz_rust::decompress::<Cursor<Vec<u8>>>(seek, dest.clone())
        .expect("Failed to unpack resource");
    Ok(Resource {
        version: version.to_string(),
        size: std::fs::read_dir(dest.clone())
            .expect("Failed to read directory")
            .count(),
    })
}
