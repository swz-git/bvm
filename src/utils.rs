use std::{cmp::min, path::Path};

use bytes::Bytes;
use directories::BaseDirs;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;

pub fn get_data_dir() -> String {
    if let Some(base_dirs) = BaseDirs::new() {
        // TODO swz: Fix this ugly mess
        return String::from(base_dirs.data_dir().join("bvm").to_str().unwrap());
    } else {
        panic!("Couldn't get base directories")
    };
}

pub fn semver_sort(version: Vec<String>) -> Vec<String> {
    return version; // BUG: CRITICAL This should sort using semver
                    // Should probably be implemented using https://discord.com/channels/273534239310479360/273541522815713281/1009949089217134733
                    // https://docs.rs/semver/latest/semver/struct.Version.html#impl-Ord-for-Version
}

// takes "bun-v1.2", "v1.12.0" and "6" and returns "1.2.0", "1.12.0" and "6.0.0"
pub fn parse_version_string(version: &str, fill_empty: bool) -> String {
    let re = Regex::new(r"^(bun-v|v)?(?P<version>\d(\.\d+)*)$").unwrap();
    let version_matches = re
        .captures(version)
        .expect(&*format!("Invalid version string: {}", version));

    let versionstr = &version_matches["version"];
    let mut versionsplit: Vec<&str> = versionstr.split(".").collect();

    if fill_empty {
        for _ in 0..3 - versionsplit.len().min(3) {
            versionsplit.push("0");
        }
    }

    versionsplit.join(".")
}

// TODO: Messy function, also sort by version
pub fn get_version_bin(version: &str, only_bvm: bool) -> Option<String> {
    let bvm_version_list = get_available_versions(true);
    let bvm_version: String = semver_sort(bvm_version_list)
        .iter()
        .find(|x| x.starts_with(&parse_version_string(version, false)))
        .unwrap_or(&String::from("NOTFOUND"))
        .to_string();
    if bvm_version == String::from("NOTFOUND") {
        if only_bvm {
            return None;
        }
        let bun_bin_path = match version {
            "package-manager" => Path::new("/usr/bin/bun"),
            "system" => Path::new("$HOME/.bun/bin/bun"),
            _ => return None,
        };
        if bun_bin_path.exists() && bun_bin_path.is_file() {
            return Some(String::from(bun_bin_path.to_str().unwrap()));
        } else {
            return None;
        };
    } else {
        let data_dir = get_data_dir();
        let bun_bin_path_string = String::from(
            Path::new(Path::new(&*data_dir))
                .join(format!("versions/bun-v{}/bun", bvm_version))
                .to_str()
                .unwrap(),
        );
        let bun_bin_path = Path::new(&*bun_bin_path_string);
        if bun_bin_path.exists() && bun_bin_path.is_file() {
            return Some(String::from(bun_bin_path.to_str().unwrap()));
        } else {
            return None;
        };
    }
}

// TODO swz: Should probably clean this function up, its pretty messy
pub fn get_available_versions(only_bvm: bool) -> Vec<String> {
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
    if !only_bvm {
        to_check.push(Version {
            name: "package-manager".to_owned(),
            path: String::from(Path::new("/usr/bin/bun").to_str().unwrap()),
        });
        to_check.push(Version {
            name: "system".to_owned(),
            path: String::from(Path::new("$HOME/.bun/bin/bun").to_str().unwrap()),
        });
    }

    for path in to_check.iter() {
        if Path::new(&path.path).is_file() {
            versions.push(path.name.clone())
        }
    }

    versions
        .iter()
        .map(|x| {
            if x == "package-manager" || x == "system" {
                x.clone()
            } else {
                parse_version_string(x, true)
            }
        })
        .collect()
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

pub fn get_current_version(symlink_path: &Path) -> Option<String> {
    if !symlink_path.exists() {
        return None;
    } else {
        let actual = symlink_path
            .read_link()
            .expect("Symlink path didn't contain symlink");
        let version = actual
            .parent()
            .unwrap()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        return Some(parse_version_string(version, true));
    }
}
