use crate::{
    local_version,
    version::{Version, VersionReq},
};

use clap::Parser;
use console::style;
use std::fs;

/// Delete the specified version
#[derive(Parser, Debug)]
pub struct UninstallArgs {
    /// The node version to delete, such as: "22", "22.17", "16.20.2"
    #[arg(required = true)]
    version: String,

    /// Agree to all operations without confirmation
    #[arg(short = 'y', long)]
    allow: bool,
}

pub async fn handle(args: UninstallArgs) -> anyhow::Result<()> {
    let install_versions = local_version::get_installation_versions().await?;
    let version_req = VersionReq::parse(&args.version)?;

    // Filter out the versions that meet the requirements
    let matched_versions: Vec<String> = install_versions
        .into_iter()
        .filter(|v| {
            let ver = Version::parse(v);
            match ver {
                Ok(v) => version_req.matches(&v),
                Err(_) => false,
            }
        })
        .collect();

    if matched_versions.is_empty() {
        println!(
            "This version is not installed: {}",
            style(args.version).yellow()
        )
    } else if matched_versions.len() == 1 {
        uninstall_versions(&matched_versions, args.allow)?;
    } else {
        let versions: Vec<String> = if args.allow {
            matched_versions
        } else {
            let selections =
                dialoguer::MultiSelect::with_theme(&dialoguer::theme::ColorfulTheme::default())
                    .with_prompt("Please select the version to delete")
                    .items(&matched_versions)
                    .interact()
                    .unwrap();

            selections
                .iter()
                .map(|i| matched_versions[*i].clone())
                .collect()
        };

        uninstall_versions(&versions, args.allow)?;
    }

    Ok(())
}

/// Delete all specified versions
pub fn uninstall_versions(versions: &Vec<String>, allow: bool) -> anyhow::Result<()> {
    let cur = local_version::get_actived_version()?.unwrap_or("".into());

    for v in versions {
        // Detect and handle the currently active version
        if v == &cur {
            let confirmation = allow
                || dialoguer::Confirm::new()
                    .with_prompt(format!(
                        "{} is the currently used version, are you sure you want to delete it?",
                        style(v).yellow()
                    ))
                    .default(false)
                    .interact()?;
            if confirmation {
                fs::remove_dir(local_version::get_actived_version_symlink_path()?)?;
            } else {
                println!("{} {}", style("Skip:").cyan(), style(v).yellow());
            }
        }

        // Delete the version folder
        fs::remove_dir_all(local_version::get_install_version_path(v)?)?;
        println!("Version {} has been deleted", style(v).yellow());
    }

    Ok(())
}
