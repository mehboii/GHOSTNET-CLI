use anyhow::Result;
use colored::Colorize;
use serde_json::Value;

use crate::cli::IdentityAction;
use crate::{banner, node};

pub fn run(action: IdentityAction) -> Result<()> {
    banner::print_banner();
    banner::welcome();

    let args: Vec<String> = match &action {
        IdentityAction::Create => vec!["identity-create".to_string()],
        IdentityAction::Load { seed } => vec!["identity-load".to_string(), seed.clone()],
    };

    let resp = node::run_bridge_json(&args)?;
    let node_id = field(&resp, "nodeId");

    match action {
        IdentityAction::Create => {
            println!("{}", "New identity created 🔑".green().bold());
            kv("Node ID", &node_id.bright_cyan().to_string());
            kv("Seed phrase", &field(&resp, "seedPhrase").yellow().to_string());
            println!();
            println!(
                "{}",
                "  ⚠ Back up your seed phrase. Anyone who has it controls this identity."
                    .yellow()
            );
        }
        IdentityAction::Load { .. } => {
            println!("{}", "Identity restored ✓".green().bold());
            kv("Node ID", &node_id.bright_cyan().to_string());
        }
    }

    Ok(())
}

fn field(value: &Value, key: &str) -> String {
    value
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or("(unknown)")
        .to_string()
}

fn kv(label: &str, value: &str) {
    println!("  {:<13}{}", format!("{label}:").bright_black(), value);
}
