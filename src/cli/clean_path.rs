use clap::Parser;
use dialoguer::console::style;

/// Clean up the environment variables added by this tool
#[derive(Parser, Debug)]
pub struct CleanPathArgs {
    /// Ignore the directory where the current tool executable is located
    #[arg(long)]
    no_exe: bool,

    /// Ignore the node directory
    #[arg(long)]
    no_node: bool,
}

pub async fn handle(args: CleanPathArgs) -> anyhow::Result<()> {
    let paths = crate::env::get_env_path_paths()?;
    let exe_dir = crate::env::get_exe_dir()?;
    let node_dir = crate::env::get_node_dir()?;

    // Check if the directory of the current tool's executable and the node directory are already in the PATH
    let mut exe_dir_exists = false;
    let mut node_dir_exists = false;

    println!("{}", style("path env paths:").blue());

    // Iterate through and print all paths currently in the PATH
    // and check if the directory of the current tool's executable and the node directory are already included
    paths.iter().for_each(|x| {
        let (is_deleted, suffix) = if x == &exe_dir {
            exe_dir_exists = true;
            (
                !args.no_exe,
                style(" # The directory where the current tool executable is located").green(),
            )
        } else if x == &node_dir {
            node_dir_exists = true;
            (!args.no_node, style(" # node directory").green())
        } else {
            (false, style(""))
        };

        println!(
            "{}{}{}",
            if is_deleted {
                style("- ").red()
            } else {
                style("")
            },
            if is_deleted {
                style(x).red()
            } else {
                style(x).yellow()
            },
            suffix
        );
    });

    if !exe_dir_exists && !node_dir_exists {
        println!("{}", style("The environment variable is clean").green());
        return Ok(());
    }

    if !args.no_exe && exe_dir_exists {
        windows_env::remove_from_list("path", &exe_dir)?;
    }

    if !args.no_node && node_dir_exists {
        windows_env::remove_from_list("path", &node_dir)?;
    }

    println!("{}", style("Cleanup complete").green());

    Ok(())
}
