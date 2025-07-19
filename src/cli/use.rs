use crate::{cli::install, local_version, node_version};

use clap::Parser;
use console::style;

/// Switch the activated node version
#[derive(Parser, Debug)]
pub struct UseArgs {
    /// The version to switch to, such as: "latest", "lts", "22", "16.20.2"
    #[arg(required = true)]
    version: String,

    /// Agree to all operations without confirmation
    #[arg(short = 'y', long)]
    allow: bool,

    /// Set the node download mirror
    #[arg(
        short,
        long,
        env = "RIGN_NODE_MIRROR",
        default_value = "https://nodejs.org/dist"
    )]
    node_mirror: String,
}

pub async fn handle(args: UseArgs) -> anyhow::Result<()> {
    let version = node_version::match_node_version(&args.version).await?;

    match version {
        Some(version) => {
            // NodeVersion.version is a version number starting with v, intercept the number version number
            let version = &version.version[1..];
            let version_path = local_version::get_install_version_path(version)?;
            if !version_path.is_dir() {
                let confirmation = args.allow
                    || dialoguer::Confirm::new()
                        .with_prompt(format!(
                            "The target version {} does not exist, do you want to install it?",
                            style(&version).yellow()
                        ))
                        .default(true)
                        .interact()?;
                if confirmation {
                    install::install_version(
                        version,
                        install::InstallArch::Default,
                        &args.node_mirror,
                    )
                    .await?
                } else {
                    return Ok(());
                }
            }

            let current_path = local_version::get_actived_version_symlink_path()?;
            if current_path.exists() {
                std::fs::remove_dir(&current_path)?;
            }

            // Use directory junctions to create symbolic links without administrator privileges
            junction::create(version_path, current_path)?;

            println!(
                "{}Switched to {}",
                console::Emoji("✅ ", ""),
                style(&version).yellow()
            );
        }
        None => {
            println!(
                "The target version {} does not exist",
                style(&args.version).yellow()
            )
        }
    }

    Ok(())
}
