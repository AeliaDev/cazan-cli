use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BasicConfig<'a> {
    pub name: &'a str,
    pub version: Version,
    pub authors: Vec<&'a str>
}
