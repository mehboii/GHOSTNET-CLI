use anyhow::Result;
use colored::Colorize;

use crate::{banner, node};

pub fn run(
    peer: String,
    message: String,
    seed: Option<String>,
    endpoint: Option<String>,
) -> Result<()> {
    banner::print_banner();
    banner::welcome();

    let mut args = vec!["send".to_string(), peer.clone(), message];
    if let Some(seed) = seed {
        args.push("--seed".to_string());
        args.push(seed);
    }
    if let Some(endpoint) = endpoint {
        args.push("--endpoint".to_string());
        args.push(endpoint);
    }

    println!("{} {}", "→ Connecting to the mesh and sending to".cyan(), peer.bright_cyan());

    node::run_bridge_json(&args)?;

    println!("{}", "✓ Message sent.".green().bold());
    Ok(())
}
