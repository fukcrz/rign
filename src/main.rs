use std::process;

use clap::Parser;
use console::style;

mod cli;
mod env;
mod local_version;
mod node_version;
mod version;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    if let Err(e) = cli::handle(cli).await {
        println!("Operation failed:\n{:?}", style(e).red());
        process::exit(1);
    }
}
