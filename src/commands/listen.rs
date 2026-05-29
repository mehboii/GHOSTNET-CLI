use anyhow::{anyhow, Context, Result};
use colored::Colorize;
use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::process::Stdio;

use crate::node;

pub fn run(seed: Option<String>, endpoint: Option<String>) -> Result<()> {
    let mut args = vec!["listen".to_string()];
    if let Some(endpoint) = endpoint {
        args.push("--endpoint".to_string());
        args.push(endpoint);
    }

    // Seed (key material) is passed to the child via env, never argv.
    let seed = node::resolve_seed(seed);

    let mut cmd = node::build_command(&args, seed.as_deref())?;
    cmd.stdout(Stdio::piped());

    let mut child = cmd
        .spawn()
        .context("failed to launch Node.js — is it installed and on your PATH?")?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("could not capture bridge output"))?;

    println!("{}", "Listening for messages…  (Ctrl-C to stop)".bold());
    println!();

    for line in BufReader::new(stdout).lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<Value>(&line) {
            Ok(value) => render_event(&value),
            Err(_) => println!("{line}"),
        }
    }

    let _ = child.wait();
    Ok(())
}

fn render_event(value: &Value) {
    let event = value.get("event").and_then(Value::as_str).unwrap_or("");
    match event {
        "identity" => {
            let id = value.get("nodeId").and_then(Value::as_str).unwrap_or("(unknown)");
            println!("  {} {}", "your node id:".bright_black(), id.bright_cyan());
        }
        "connect" => println!("  {}", "● connected to the mesh".green()),
        "disconnect" => {
            let reason = value.get("reason").and_then(Value::as_str).unwrap_or("");
            println!("  {} {}", "○ disconnected".yellow(), reason.bright_black());
        }
        "message" => {
            let from = value.get("from").and_then(Value::as_str).unwrap_or("?");
            let data = value.get("data").and_then(Value::as_str).unwrap_or("");
            println!("  {} {}", format!("{from}:").bright_magenta().bold(), data);
        }
        "error" => {
            let err = value.get("error").and_then(Value::as_str).unwrap_or("unknown error");
            eprintln!("  {} {}", "error:".red().bold(), err);
        }
        _ => println!("  {value}"),
    }
}
