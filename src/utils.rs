use std::{cmp::min, path::Path};

use bytes::Bytes;
use directories::BaseDirs;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

pub fn get_data_dir() -> String {
    if let Some(base_dirs) = BaseDirs::new() {
        // TODO swz: Fix this ugly mess
        return String::from(base_dirs.data_dir().join("bvm").to_str().unwrap());
    } else {
        panic!("Couldn't get base directories")
    };
}

pub fn get_version_bin(version: &str, only_bvm: bool) -> Option<String> {
    let mut bvm_version_list = Path::new(&get_data_dir())
        .join("versions")
        .read_dir()
        .expect("Failed to read installed versions")
        .map(|x| x.unwrap().path());
    let bvm_version_path = bvm_version_list
        .find(|x| {
            x.as_path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .contains(version)
        })
        .unwrap_or(Path::new("FAILED").to_path_buf());
    let bvm_version_file = bvm_version_path.as_path().join("bun");
    let bin_path: &Path = match version {
        "package-manager" => Path::new("/usr/bin/bun"),
        "system" => Path::new("$HOME/.bun/bin/bun"),
        _ => bvm_version_file.as_path(),
    };
    if only_bvm && bvm_version_file.as_path() != bin_path {
        return None;
    }
    if bin_path.exists() && bin_path.is_file() {
        return Some(String::from(bin_path.to_str().unwrap()));
    } else {
        None
    }
}

// TODO swz: Should probably clean this function up, its pretty messy
pub fn get_available_versions() -> Vec<String> {
    let mut versions: Vec<String> = vec![];
    let mut to_check: Vec<Version> = vec![];

    struct Version {
        name: String,
        path: String,
    }

    for path in Path::new(&get_data_dir())
        .join("versions")
        .read_dir()
        .expect("Error reading versions")
    {
        let bunpath = path.as_ref().unwrap().path().join("bun");
        let dirname = path.as_ref().unwrap().file_name();
        to_check.push(Version {
            name: String::from(dirname.to_str().unwrap()),
            path: String::from(bunpath.to_str().unwrap()),
        })
    }
    to_check.push(Version {
        name: "package-manager".to_owned(),
        path: String::from(Path::new("/usr/bin/bun").to_str().unwrap()),
    });
    to_check.push(Version {
        name: "system".to_owned(),
        path: String::from(Path::new("$HOME/.bun/bin/bun").to_str().unwrap()),
    });

    for path in to_check.iter() {
        if Path::new(&path.path).is_file() {
            versions.push(path.name.clone())
        }
    }
    versions
}

pub async fn download_with_progress(url: &str) -> bytes::Bytes {
    let client = reqwest::Client::new();
    // Reqwest setup
    let res = client
        .get(url)
        .send()
        .await
        .unwrap_or_else(|_| panic!("Failed to GET from '{}'", &url));
    let total_size = res
        .content_length()
        .unwrap_or_else(|| panic!("Failed to get content length from '{}'", &url));

    // Indicatif setup
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .progress_chars("#>-"));
    pb.set_message(&format!("Downloading {}", url));

    // download chunks
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();
    let mut resulting: Bytes = Bytes::new();

    while let Some(item) = stream.next().await {
        let chunk = item.expect("Error while downloading file");
        resulting = Bytes::from([resulting, chunk.clone()].concat());
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(&format!("Downloaded {}", url));
    resulting
}
