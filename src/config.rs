use semver::Version;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Config<'a> {
    pub name: &'a str,
    pub version: Version,
    pub authors: Vec<&'a str>,
    pub file_hash_salt: Option<&'a str>,
    #[serde(skip_serializing)]
    pub use_autoplay_for_multimedia: Option<bool>,
    pub plugins: Option<Vec<PluginConfig<'a>>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PluginConfig<'a> {
    pub name: &'a str,
    pub version: Option<Version>,
    pub path: Option<&'a Path>,
}

pub fn checksum(file: &PathBuf) -> Result<String, std::io::Error> {
    let mut file = fs::File::open(file)?;
    let mut sha256 = Sha256::new();
    std::io::copy(&mut file, &mut sha256)?;
    Ok(format!("{:x}", sha256.finalize()))
}
