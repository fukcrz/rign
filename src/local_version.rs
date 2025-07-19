use std::{env, fs, path::PathBuf};

use anyhow::Context;

/// Directory name for saving node files
const NODE_INSTALLATION_DIR_NAME: &str = "nodejs";

/// Directory name of the 'symbolic link' directory linked to the currently active version
const ACTIVED_VERSION_SYMLINK_DIR_NAME: &str = "actived";

const VERSION_REGEX: &str = r"(\d+)\.(\d+)\.(\d+)";

/// Get the directory path to save Node files
pub fn get_installation_path() -> anyhow::Result<PathBuf> {
    let path = env::current_exe()?
        .parent()
        .unwrap()
        .join(NODE_INSTALLATION_DIR_NAME);
    Ok(path)
}

/// Get the path of the specified version
pub fn get_install_version_path(version: &str) -> anyhow::Result<PathBuf> {
    let path = get_installation_path()?.join(version);
    Ok(path)
}

/// Get the path of the symbolic link for the currently active version
pub fn get_actived_version_symlink_path() -> anyhow::Result<PathBuf> {
    Ok(get_installation_path()?.join(ACTIVED_VERSION_SYMLINK_DIR_NAME))
}

/// Get all installed versions
pub async fn get_installation_versions() -> anyhow::Result<Vec<String>> {
    let versions_path = get_installation_path()?;

    if !versions_path.is_dir() {
        return Ok(Vec::new());
    }

    let dirs = fs::read_dir(&versions_path)?;
    let version_re = regex::Regex::new(VERSION_REGEX).unwrap();

    // Filter out all directories whose names are valid version numbers
    let versions: Vec<String> = dirs
        .filter_map(|v| {
            let Ok(v) = v else {
                return None;
            };
            let Ok(ft) = &v.file_type() else {
                return None;
            };
            if !ft.is_dir() {
                return None;
            }
            let name = v.file_name();
            let name = name.to_str()?;
            if !version_re.is_match(name) {
                return None;
            }
            Some(name.to_string())
        })
        .collect();

    Ok(versions)
}

/// Get the currently active version
pub fn get_actived_version() -> anyhow::Result<Option<String>> {
    let symlink_path = get_actived_version_symlink_path()?;
    let Ok(link_path) = fs::read_link(symlink_path) else {
        return Ok(None);
    };
    if !link_path.is_dir() {
        return Ok(None);
    }
    // Get the name of the directory pointed to by the symbolic link
    let version_str = link_path
        .file_name()
        .context("Failed to get the name of the directory pointed to by the symbolic link")?
        .to_str()
        .context("The name of the directory pointed to by the obtained symbolic link is not a valid UTF-8 character encoding")?;

    let version_re = regex::Regex::new(VERSION_REGEX).unwrap();
    if !version_re.is_match(version_str) {
        return Err(anyhow::anyhow!(
            "The directory name '{}' pointed to by the symbolic link is not a valid version number",
            version_str
        ));
    }

    Ok(Some(version_str.into()))
}
