use clap::Parser;
use dialoguer::console::style;

use crate::local_version;

/// List all installed versions
#[derive(Parser, Debug)]
pub struct ListArgs {}

pub async fn handle(_args: ListArgs) -> anyhow::Result<()> {
    let versions = local_version::get_installation_versions().await?;

    if versions.is_empty() {
        println!("No version installed");
        return Ok(());
    }

    let current_version = local_version::get_actived_version()?.unwrap_or("".into());

    // Output all installed versions
    for version in &versions {
        if version == &current_version {
            println!(
                "{:<10}{}",
                style(&version).yellow(),
                style(" # current").green().bold(),
            );
        } else {
            println!("{}", style(&version).yellow());
        };
    }

    Ok(())
}
