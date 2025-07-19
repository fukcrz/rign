use std::collections::HashMap;

use anyhow::Ok;
use clap::Parser;
use console::{Emoji, style};

use crate::node_version::{self, LtsInfo, NodeVersion};

/// View released versions of node
#[derive(Parser, Debug)]
pub struct ShowArgs {
    /// View detailed information for a specified version
    /// Such as: "latest", "lts", "22", "16.20.2"
    #[arg()]
    version: Option<String>,
}

pub async fn handle(args: ShowArgs) -> anyhow::Result<()> {
    if let Some(v) = args.version {
        show_version(&v).await?;
    } else {
        show_latest_major_versions().await?;
    }
    Ok(())
}

/// Show the latest major versions
async fn show_latest_major_versions() -> anyhow::Result<()> {
    let versions = node_version::get_versions().await?;

    // Group by major version
    let mut group_by_major: HashMap<u64, Vec<NodeVersion>> = HashMap::new();
    versions.into_iter().for_each(|x| {
        let major = x.as_version().major();
        let v = group_by_major.entry(major).or_default();
        v.push(x);
    });

    let mut keys: Vec<u64> = group_by_major.keys().cloned().collect();
    keys.sort_by(|a, b| b.cmp(a));

    // Loop to output the latest version of the latest 10 major versions
    keys.iter().take(10).enumerate().for_each(|(i, key)| {
        let version = group_by_major.get(key).unwrap().iter().max().unwrap();
        let mut suffix = String::new();

        match &version.lts {
            LtsInfo::Name(name) => suffix.push_str(&format!(" # lts@{name}")),
            LtsInfo::No(is_lts) => {
                if *is_lts {
                    suffix.push_str(" # lts");
                }
            }
        };

        if i == 0 {
            suffix.push_str(" # latest");
        }

        println!(
            "{:<10}{}",
            style(&version.version).yellow(),
            style(suffix).green()
        )
    });

    Ok(())
}

/// Show detailed information for a specified version
async fn show_version(version: &str) -> anyhow::Result<()> {
    let match_version = node_version::match_node_version(version).await?;

    if let Some(nv) = match_version {
        print_node_version_details(&nv);
    } else {
        println!("No matching version found: {}", style(version).yellow())
    }

    Ok(())
}

/// Print detailed information for a Node.js version
fn print_node_version_details(nv: &NodeVersion) {
    // Basic information
    println!(
        "{} {} {} {}",
        Emoji("🚀", ""),
        style("Node.js Version").bold().green(),
        style(&nv.version).bold().green(),
        style("Details").bold().green()
    );
    println!("========================================");
    let lts_display = match &nv.lts {
        LtsInfo::Name(name) => style(format!("lts@{name}")).cyan(),
        LtsInfo::No(lts) => style((if *lts { "Yes" } else { "No" }).into()).dim(),
    };
    let security_display = if nv.security {
        style("Yes").green()
    } else {
        style("No").red()
    };
    let npm_display = match &nv.npm {
        Some(version) => style(&version[..]).yellow(),
        None => style("N/A").dim().italic(),
    };
    println!(
        "{:<20}{}",
        style("Release Date:").bold(),
        style(&nv.date).yellow()
    );
    println!("{:<20}{}", style("LTS Version:").bold(), lts_display);
    println!("{:<20}{}", style("NPM Version:").bold(), npm_display);
    println!(
        "{:<20}{}",
        style("Security Release:").bold(),
        security_display
    );

    // Dependencies
    println!();
    println!(
        "{} {}",
        Emoji("🔧", ""),
        style("Dependencies").bold().blue()
    );
    println!("----------------------------------------");
    let deps: [(&str, Option<String>); 5] = [
        ("V8", Some(nv.v8.clone())),
        ("OpenSSL", nv.openssl.clone()),
        ("libuv", nv.uv.clone()),
        ("Zlib", nv.zlib.clone()),
        ("Modules", nv.modules.clone()),
    ];
    for (key, val_opt) in &deps {
        let val_display = match val_opt {
            Some(v) => v,
            None => "N/A",
        };
        println!("- {:<8} {}", style(key).bold(), val_display);
    }

    // Available files
    println!();
    println!(
        "{} {}",
        Emoji("📦", ""),
        style("Available Files").bold().blue()
    );
    println!("----------------------------------------");
    for file in &nv.files {
        println!("- {file}")
    }
}
