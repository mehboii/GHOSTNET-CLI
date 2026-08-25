use anyhow::Result;
use colored::Colorize;

use crate::node;

pub fn run(
    peer: String,
    message: String,
    seed: Option<String>,
    endpoint: Option<String>,
) -> Result<()> {
    let mut args = vec!["send".to_string(), peer.clone(), message];
    if let Some(endpoint) = endpoint {
        args.push("--endpoint".to_string());
        args.push(endpoint);
    }

    // Seed (key material) is passed to the child via env, never argv.
    let seed = node::resolve_seed(seed);

    println!("{} {}", "→ Connecting to the mesh and sending to".cyan(), peer.bright_cyan());

    node::run_bridge_json(&args, seed.as_deref())?;

    println!("{}", "Message sent.".green().bold());
    Ok(())
}
