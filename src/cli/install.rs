use crate::{
    local_version,
    node_version::{self, NodeVersion},
};

use anyhow::anyhow;
use clap::{Parser, ValueEnum};
use console::{Emoji, style};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{Read, Seek, Write};

const OS: &str = "win"; // This tool is only designed for Windows, there are better choices on other OS
const EXT: &str = ".zip";

/// Install the specified version
#[derive(Parser, Debug)]
pub struct InstallArgs {
    /// The node version to install, such as: "latest", "lts", "22", "16.20.2"
    #[arg(required = true)]
    version: String,

    /// Select the node architecture to install, the default matches the current system
    #[arg(default_value = "default")]
    arch: InstallArch,

    /// Set the node download mirror
    #[arg(
        short,
        long,
        env = "RIGN_NODE_MIRROR",
        default_value = "https://nodejs.org/dist"
    )]
    node_mirror: String,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
pub enum InstallArch {
    Default,
    X86,
    X64,
    Arm64,
}

pub async fn handle(args: InstallArgs) -> anyhow::Result<()> {
    install_version(&args.version, args.arch, &args.node_mirror).await?;
    Ok(())
}

/// Build download link
fn build_download_url(
    mirror: &str,
    version: &str,
    os: &str,
    arch: &str,
    ext: &str,
) -> (String, String) {
    let filename = format!("node-v{version}-{os}-{arch}{ext}");
    let url = format!("{mirror}/v{version}/{filename}",);
    (url, filename)
}

/// Download file and return byte data
async fn download_bytes(url: &str) -> anyhow::Result<Vec<u8>> {
    let mut res = reqwest::get(url).await?.error_for_status()?;
    let total_size: u64 = res
        .content_length()
        .ok_or(anyhow::anyhow!("Download failed: file size not available"))?;

    println!(
        "{}{}{}",
        Emoji("⏳ ", ""),
        style("Start downloading: ").cyan().bold(),
        style(url).blue()
    );

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")?
        .progress_chars("#>-")
    );

    let mut data: Vec<u8> = Vec::with_capacity(total_size as usize);

    while let Some(chunk) = res.chunk().await? {
        data.extend_from_slice(&chunk);
        pb.set_position(data.len() as u64);
    }

    pb.finish();
    println!(
        "{}{}",
        Emoji("✔️ ", ""),
        style("Download complete").cyan().bold()
    );

    Ok(data)
}

/// Decompress the ZIP file to the specified directory
fn decompress_zip<R: Read + Seek>(reader: R, target_dir: &std::path::Path) -> anyhow::Result<()> {
    let mut archive = zip::ZipArchive::new(reader)?;
    archive.extract_unwrapped_root_dir(target_dir, zip::read::root_dir_common_filter)?;
    Ok(())
}

/// Get the string representation of the current architecture
/// If not specified, returns the default value based on the current system architecture
fn get_arch(arch: InstallArch) -> &'static str {
    match arch {
        InstallArch::X64 => "x64",
        InstallArch::X86 => "x86",
        InstallArch::Arm64 => "arm64",
        InstallArch::Default => {
            let os_arch = std::env::consts::ARCH.to_lowercase();
            if os_arch.contains("64") {
                if os_arch.contains("arm") || os_arch.contains("aarch") {
                    "arm64"
                } else {
                    "x64"
                }
            } else {
                "x86"
            }
        }
    }
}

/// Install the specified version of Node.js
/// If the version already exists, it returns success directly
pub async fn install_version(
    version_req: &str,
    arch: InstallArch,
    mirror: &str,
) -> anyhow::Result<()> {
    // Find matching Node.js version information
    let NodeVersion { files, version, .. } = node_version::match_node_version(version_req)
        .await?
        .unwrap_or_else(|| panic!("Version not found {}",
            style(version_req).yellow()));

    // The version field of NodeVersion starts with v, intercept the version number
    let version = &version[1..];

    // Check if this version is already installed
    let install_versions = local_version::get_installation_versions().await?;
    if install_versions.iter().any(|v| v == version) {
        println!("Version {} is already installed", style(version).yellow());
        return Ok(());
    }

    let arch = get_arch(arch);

    // Check if the corresponding installation file exists
    let filename = format!("{OS}-{arch}-{ext}", ext = &EXT[1..]);
    if !files.iter().any(|x| x == &filename) {
        return Err(anyhow!(
            "Version {} not found installation file: {}",
            style(version).yellow(),
            style(&arch).cyan()
        ));
    }

    // Build download link
    let (download_url, file_name) = build_download_url(mirror, version, OS, arch, EXT);
    let install_dir = local_version::get_install_version_path(version)?;

    // Check if the installation file for this version is already cached
    let cache_file_path = std::env::temp_dir().join(file_name);
    if cache_file_path.is_file() {
        if let Ok(cache_file) = std::fs::File::open(&cache_file_path) {
            if decompress_zip(cache_file, install_dir.as_path()).is_ok() {
                println!(
                    "{} has been saved to {:?}",
                    style(format!("Node v{version}")).yellow(),
                    style(install_dir).blue()
                );
                return Ok(());
            }
        }
    }

    // Download and decompress
    let zip_data = download_bytes(&download_url).await?;
    let reader = std::io::Cursor::new(&zip_data);
    decompress_zip(reader, install_dir.as_path())?;

    println!(
        "{} has been saved to {:?}",
        style(format!("Node v{version}")).yellow(),
        style(install_dir).blue()
    );

    // Cache the downloaded file to the temporary directory
    if let Ok(mut cache_file) = std::fs::File::create(cache_file_path) {
        if cache_file.write(&zip_data).is_err() {
            println!("Failed to cache this version of the file")
        }
    }

    Ok(())
}
