use clap::Parser;
use console::style;

use crate::local_version;

/// Check the currently active node version
#[derive(Parser, Debug)]
pub struct ActivedArgs {}

pub async fn handle(_args: ActivedArgs) -> anyhow::Result<()> {
    match local_version::get_actived_version()? {
        Some(v) => println!("{}", style(&v).yellow()),
        None => println!("none"),
    }
    Ok(())
}
