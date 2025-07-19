use clap::Parser;
use dialoguer::console::style;

/// Add the tool and node directories to the user's environment variables
#[derive(Parser, Debug)]
pub struct AddPathArgs {
    /// Ignore the directory where the current tool executable is located
    #[arg(long)]
    no_exe: bool,

    /// Ignore the node directory
    #[arg(long)]
    no_node: bool,
}

pub async fn handle(args: AddPathArgs) -> anyhow::Result<()> {
    let paths = crate::env::get_env_path_paths()?;
    let exe_dir = crate::env::get_exe_dir()?;
    let node_dir = crate::env::get_node_dir()?;

    // Check if the directory of the current tool's executable and the node directory are already in the PATH
    let mut exe_dir_added = false;
    let mut node_dir_added = false;

    println!("{}", style("path env paths:").blue());

    // Iterate through and print all paths currently in the PATH
    // and check if the directory of the current tool's executable and the node directory are already included
    paths.iter().for_each(|x| {
        if x == &exe_dir {
            exe_dir_added = true;
            println!(
                "{} {}",
                style(x).yellow(),
                style("# The directory where the current tool executable is located").green()
            );
            return;
        }

        if x == &node_dir {
            node_dir_added = true;
            println!(
                "{} {}",
                style(x).yellow(),
                style("# node directory").green()
            );
            return;
        }

        println!("{}", style(x).yellow());
    });

    if (exe_dir_added || args.no_exe) && (node_dir_added || args.no_node) {
        // No action required
        return Ok(());
    }

    if !exe_dir_added && !args.no_exe {
        println!("{}", style(format!("+ {}", &exe_dir)).green());
        windows_env::append("path", &exe_dir)?;
    }

    if !node_dir_added && !args.no_node {
        println!("{}", style(format!("+ {}", &node_dir)).green());
        windows_env::append("path", &node_dir)?;
    }

    println!("{}", style("Added successfully").green());

    Ok(())
}
