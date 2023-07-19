use core::fmt;
use std::error::Error;

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BunVersion {
    major: i32,
    minor: i32,
    patch: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct VersionParseError;

impl fmt::Display for VersionParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParseError, Valid versions look like the following. 'bun-v1.0.0', 'v1.0.0', 'v0.4', '0.0.4', etc")
    }
}

impl Error for VersionParseError {
    fn description(&self) -> &str {
        "Invalid version, couldn't parse"
    }
}

impl std::str::FromStr for BunVersion {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^(bun-v|v)?(?P<version>\d+(\.\d+)*)$")
            .ok()
            .ok_or(VersionParseError)?;
        let version_matches = re.captures(s).ok_or(VersionParseError)?;

        let versionstr = &version_matches["version"];
        let versionsplit: Vec<&str> = versionstr.split(".").collect();

        Ok(BunVersion::new(
            versionsplit
                .get(0)
                .unwrap_or(&"0")
                .parse()
                .ok()
                .ok_or(VersionParseError)?,
            versionsplit
                .get(1)
                .unwrap_or(&"0")
                .parse()
                .ok()
                .ok_or(VersionParseError)?,
            versionsplit
                .get(2)
                .unwrap_or(&"0")
                .parse()
                .ok()
                .ok_or(VersionParseError)?,
        ))
    }
}

impl BunVersion {
    pub fn new(major: i32, minor: i32, patch: i32) -> Self {
        BunVersion {
            major,
            minor,
            patch,
        }
    }
}
