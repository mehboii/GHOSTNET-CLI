use anyhow::Result;
use colored::Colorize;
use serde_json::Value;

use crate::cli::IdentityAction;
use crate::node;

pub fn run(action: IdentityAction) -> Result<()> {
    // For `load`, the seed is passed to the child via env (not argv).
    let (args, seed): (Vec<String>, Option<String>) = match &action {
        IdentityAction::Create => (vec!["identity-create".to_string()], None),
        IdentityAction::Load { seed } => (
            vec!["identity-load".to_string()],
            node::resolve_seed(seed.clone()),
        ),
    };

    if matches!(action, IdentityAction::Load { .. }) && seed.is_none() {
        return Err(anyhow::anyhow!(
            "a 12-word seed phrase is required (pass it as an argument or set GHOSTNET_SEED)"
        ));
    }

    let resp = node::run_bridge_json(&args, seed.as_deref())?;
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
