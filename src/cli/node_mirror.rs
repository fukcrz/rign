use clap::Parser;

/// Set the node download mirror to env.
/// like: https://npmmirror.com/mirrors/node
#[derive(Parser, Debug)]
pub struct NodeMirrorArgs {
    #[arg()]
    mirror: Option<String>,
}

static ENV_NAME: &str = "RIGN_NODE_MIRROR";

pub async fn handle(args: NodeMirrorArgs) -> anyhow::Result<()> {
    match args.mirror {
        Some(mirror) => {
            windows_env::set(ENV_NAME, mirror)?;
            println!(
                "Setup successful, the environment variables will take effect in a new terminal session."
            );
        }
        None => {
            if let Some(v) = windows_env::get(ENV_NAME)? {
                println!("node_mirror: {v}");
            } else {
                println!("node_mirror is not set")
            }
        }
    }

    Ok(())
}
