use arc_bytes::ArcBytes;
use reqwest::Url;

use crate::config::get_config;

pub fn download_resource<'a>(
    version: &str,
) -> Result<ArcBytes<'a>, String> {
    let url = get_config().update;
    let uri = Url::parse(({
        if url.ends_with("/") {
            format!("{url}download/")
        } else {
            format!("{url}/download/")
        }
    }).as_str()).expect("Failed to parse URL");
    let session = reqwest::blocking::Client::new();
    let form_data = reqwest::blocking::multipart::Form::new().text("json", format!(r#"{{"password": "", "version": "{version}"}}"#));
    let response = session.post(uri).multipart(form_data).send().expect("Failed to send request");
    let buffer = response.bytes().expect("Failed to read response");
    Ok(ArcBytes::from(buffer.to_vec()))
}
