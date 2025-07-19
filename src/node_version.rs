use std::{io::Write, path::PathBuf};

use crate::version::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum LtsInfo {
    Name(String),
    No(bool),
}

/// Node version information
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NodeVersion {
    pub version: String,
    pub date: String,
    pub files: Vec<String>,
    pub v8: String,
    pub lts: LtsInfo,
    pub security: bool,

    pub npm: Option<String>,
    pub uv: Option<String>,
    pub zlib: Option<String>,
    pub openssl: Option<String>,
    pub modules: Option<String>,
}

impl NodeVersion {
    pub fn as_version(&self) -> Version {
        Version::parse(&self.version).unwrap()
    }

    pub fn is_lts(&self) -> bool {
        match self.lts {
            LtsInfo::Name(_) => true,
            LtsInfo::No(lts) => lts,
        }
    }
}

impl Eq for NodeVersion {}

impl PartialEq for NodeVersion {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version
    }
}

impl PartialOrd for NodeVersion {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NodeVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_version().cmp(&other.as_version())
    }
}

/// URL to get Node version information
const FETCH_VERSIONS_URL: &str = "https://nodejs.org/dist/index.json";

/// Get the online Node version list
pub async fn fetch_versions() -> anyhow::Result<Vec<NodeVersion>> {
    let versions = reqwest::get(FETCH_VERSIONS_URL)
        .await?
        .json::<Vec<NodeVersion>>()
        .await?;
    Ok(versions)
}

/// Version information cache file name
const VERSIONS_CACHE_FILE_NAME: &str = "rign-node-dist.json";

fn get_cached_versions(cache_file_path: &PathBuf) -> anyhow::Result<Option<Vec<NodeVersion>>> {
    // Try to read version information from the cache file
    if !cache_file_path.is_file() {
        return Ok(None);
    }
    let create_tiem = std::fs::metadata(cache_file_path)?.modified()?;
    if let Ok(dur) = create_tiem.elapsed() {
        // 12-hour validity
        if dur.as_secs() > 3600 * 12 {
            return Ok(None);
        }
    }
    let json_str = std::fs::read_to_string(cache_file_path)?;
    let versions = serde_json::from_str::<Vec<NodeVersion>>(&json_str)?;
    Ok(Some(versions))
}

/// Get the Node version list
/// The first call will get the version information from the network and cache it in a file in the temporary directory
/// Subsequent calls will read the cache file first
pub async fn get_versions() -> anyhow::Result<Vec<NodeVersion>> {
    let cache_file_path = std::env::temp_dir().join(VERSIONS_CACHE_FILE_NAME);

    if let Ok(Some(versions)) = get_cached_versions(&cache_file_path) {
        return Ok(versions);
    }

    // If the cache file does not exist or fails to be read, get the version information from the network
    let versions = fetch_versions().await?;

    // Write the obtained version information to the cache file
    if let Ok(mut cache_file) = std::fs::File::create(&cache_file_path) {
        if let Ok(json_data) = &serde_json::to_vec(&versions) {
            if let Err(err) = cache_file.write(json_data) {
                println!("Failed to write cache file:\n{err:?}");
            }
        }
    }

    Ok(versions)
}

/// Get the latest version of node
pub async fn get_latest_version() -> anyhow::Result<NodeVersion> {
    let versions = get_versions().await?;
    let latest = versions.into_iter().max();
    Ok(latest.unwrap())
}

/// Get the latest LTS version of node
pub async fn get_lts_version() -> anyhow::Result<NodeVersion> {
    let versions = get_versions().await?;
    let lts = versions.into_iter().filter(|x| x.is_lts()).max();
    Ok(lts.unwrap())
}

/// Match Node version by version number, support "lts", "latest" and version number
/// If multiple versions meet the requirements, the latest version is returned
pub async fn match_node_version(version: &str) -> anyhow::Result<Option<NodeVersion>> {
    match version {
        "lts" => Ok(Some(get_lts_version().await?)),
        "latest" => Ok(Some(get_latest_version().await?)),
        version => {
            let node_versions = get_versions().await?;

            if let Some(lts_name) = version.strip_prefix("lts@") {
                let lts_version = node_versions
                    .into_iter()
                    .filter(|x| {
                        if let LtsInfo::Name(name) = &x.lts {
                            name == lts_name
                        } else {
                            false
                        }
                    })
                    .max();
                return Ok(lts_version);
            }

            let version_req = VersionReq::parse(version)?;
            let matched = node_versions
                .into_iter()
                .filter(|x| version_req.matches(&x.as_version()))
                .max();
            Ok(matched)
        }
    }
}
