use crate::local_version;

/// Get the path where the current executable file is located
pub fn get_exe_dir() -> anyhow::Result<String> {
    let dir = std::env::current_exe()?
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .into();
    Ok(dir)
}

/// Get the path of the symbolic link directory linked to the activated version
pub fn get_node_dir() -> anyhow::Result<String> {
    let dir = local_version::get_actived_version_symlink_path()?
        .to_str()
        .unwrap()
        .into();
    Ok(dir)
}

/// Get all paths in the current environment variable PATH
pub fn get_env_path_paths() -> anyhow::Result<Vec<String>> {
    let path_value = windows_env::get("path")?;
    let paths: Vec<String> = match path_value {
        Some(path_value) => path_value
            .split(";")
            .filter(|x| !x.is_empty())
            .map(|x| x.into())
            .collect(),
        None => Vec::new(),
    };
    Ok(paths)
}
