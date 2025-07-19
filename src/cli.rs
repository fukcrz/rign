use clap::{Parser, Subcommand};

mod actived;
mod add_path;
mod clean_path;
mod install;
mod list;
mod node_mirror;
mod show;
mod uninstall;
mod r#use;

/// A nodejs version management tool
#[derive(Parser, Debug)]
#[command(name = "rnm", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Supported subcommands
#[derive(Subcommand, Debug)]
enum Commands {
    #[command(visible_aliases = &["cur", "current"])]
    Actived(actived::ActivedArgs),

    #[command(visible_aliases = &["ls"])]
    List(list::ListArgs),

    #[command()]
    Show(show::ShowArgs),

    #[command(visible_aliases = &["i", "add"])]
    Install(install::InstallArgs),

    #[command(visible_aliases = &["rm", "remove", "delete"])]
    Uninstall(uninstall::UninstallArgs),

    #[command()]
    Use(r#use::UseArgs),

    #[command()]
    AddPath(add_path::AddPathArgs),

    #[command()]
    CleanPath(clean_path::CleanPathArgs),

    #[command()]
    NodeMirror(node_mirror::NodeMirrorArgs),
}

pub async fn handle(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Actived(args) => actived::handle(args).await,
        Commands::List(args) => list::handle(args).await,
        Commands::Show(args) => show::handle(args).await,
        Commands::Install(args) => install::handle(args).await,
        Commands::Uninstall(args) => uninstall::handle(args).await,
        Commands::Use(args) => r#use::handle(args).await,
        Commands::AddPath(args) => add_path::handle(args).await,
        Commands::CleanPath(args) => clean_path::handle(args).await,
        Commands::NodeMirror(args) => node_mirror::handle(args).await,
    }
}
