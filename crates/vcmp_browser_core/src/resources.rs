use std::io::Cursor;

use arc_bytes::ArcBytes;
use reqwest::Url;
use tempfile::{tempdir};

use crate::{config::get_config, utils::get_home_dir};

pub struct Resource {
    pub version: String,
    pub size: usize
}

pub fn download_resource<'a>(
    version: &str,
) -> Result<ArcBytes<'a>, String> {
    let url = &get_config().index_url.update;
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

/// unpack for 7z
pub fn unpack_resource(resource: ArcBytes<'_>) -> Result<Resource, String> {
    let temp_path = tempdir().expect("Failed to create temp directory");
    let temp_path = temp_path.path();
    
    let seek = Cursor::new({
        let buffer = resource.clone();
        buffer.to_vec()
    });

    sevenz_rust::decompress::<Cursor<Vec<u8>>>(seek, temp_path).expect("Failed to unpack resource");

    // read version.txt

    let version_path = std::fs::read_to_string(temp_path.join("version.txt")).expect("Failed to read version.txt");
    let version = version_path.trim();

    // count files in temp_path
    let files = std::fs::read_dir(temp_path).expect("Failed to read directory");
    let files = files.count();

    // move into resources
    let resource_path = get_home_dir().join("versions").join(version);

    move_files(temp_path, &resource_path).expect("Failed to move files");
    
    Ok(Resource {
        version: version.to_string(),
        size: files
    })
}

fn move_files(src: &std::path::Path, dst: &std::path::Path) -> Result<(), String> {
    if src.is_file() && dst.is_file() {
        std::fs::rename(src, dst).expect("Failed to move file");
    } else if src.is_dir() && dst.is_dir() {
        let files = std::fs::read_dir(src).expect("Failed to read directory");
        for file in files {
            let file = file.expect("Failed to read file");
            let file_path = file.path();
            let dst_path = dst.join(file_path.file_name().expect("Failed to get file name"));

            move_files(&file_path, &dst_path).expect("Failed to move file");
        }
    }
    Ok(())
}
