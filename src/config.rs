use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::Path;

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
