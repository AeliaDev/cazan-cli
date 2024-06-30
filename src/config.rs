use std::path::Path;
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config<'a> {
    pub name: &'a str,
    pub version: Version,
    pub authors: Vec<&'a str>,
    #[serde(rename(serialize = "fileHashSalt", deserialize = "fileHashSalt"))]
    pub file_hash_salt: Option<&'a str>,
    #[serde(skip_serializing, rename(serialize = "useAutoplayForMultimedia", deserialize = "useAutoplayForMultimedia"))]
    pub use_autoplay_for_multimedia: Option<bool>,
    pub plugins: Option<Vec<PluginConfig<'a>>>
}

#[derive(Serialize, Deserialize)]
pub struct PluginConfig<'a> {
    pub name: &'a str,
    pub version: Option<Version>,
    pub path: Option<&'a Path>
}