use anyhow::Result;
use colored::Colorize;

use crate::{config, history, node};

pub fn run(
    peer: String,
    message: String,
    seed: Option<String>,
    endpoint: Option<String>,
) -> Result<()> {
    if message.len() > 64 * 1024 {
        anyhow::bail!("message exceeds the 64 KiB UTF-8 limit");
    }
    if peer.trim().is_empty() {
        anyhow::bail!("peer node ID cannot be empty");
    }
    let seed = node::resolve_seed(seed);
    if seed.is_none() {
        anyhow::bail!("an identity is required: set GHOSTNET_SEED or pass --seed; refusing to create an unreachable temporary identity");
    }
    let endpoint = config::endpoint(endpoint)?;
    let mut args = vec!["send".to_string(), peer.clone(), message];
    if let Some(endpoint) = endpoint {
        args.push("--endpoint".to_string());
        args.push(endpoint);
    }

    println!(
        "{} {}",
        "→ Connecting to the mesh and sending to".cyan(),
        peer.bright_cyan()
    );

    node::run_bridge_json(&args, seed.as_deref())?;
    history::record("sent", &peer, args[2].len(), "sent")?;

    println!("{}", "Message sent.".green().bold());
    Ok(())
}
