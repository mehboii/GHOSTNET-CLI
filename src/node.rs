//! Locating and invoking the bundled Node bridge that wraps @n11x/ghostnet-sdk.

use anyhow::{anyhow, Context, Result};
use serde_json::Value;
use std::path::PathBuf;
use std::process::{Command, Output};

/// Bridge sources are embedded into the binary and materialized on disk at setup.
pub const BRIDGE_JS: &str = include_str!("../bridge/ghostnet-bridge.mjs");
pub const BRIDGE_PKG: &str = include_str!("../bridge/package.json");

/// `node` executable (override with GHOSTNET_NODE).
pub fn node_bin() -> String {
    std::env::var("GHOSTNET_NODE").unwrap_or_else(|_| "node".to_string())
}

/// `npm` executable. On Windows the launcher is `npm.cmd`.
pub fn npm_bin() -> String {
    if let Ok(custom) = std::env::var("GHOSTNET_NPM") {
        return custom;
    }
    if cfg!(windows) {
        "npm.cmd".to_string()
    } else {
        "npm".to_string()
    }
}

fn home_dir() -> Result<PathBuf> {
    std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .map(PathBuf::from)
        .ok_or_else(|| anyhow!("could not determine your home directory"))
}

/// Where the bridge + its node_modules live: ~/.ghostnet-cli/bridge
pub fn bridge_dir() -> Result<PathBuf> {
    Ok(home_dir()?.join(".ghostnet-cli").join("bridge"))
}

pub fn bridge_script() -> Result<PathBuf> {
    Ok(bridge_dir()?.join("ghostnet-bridge.mjs"))
}

/// True once `npm install` has pulled the SDK into the bridge.
pub fn bridge_installed() -> bool {
    bridge_dir()
        .map(|d| d.join("node_modules").join("@n11x").join("ghostnet-sdk").exists())
        .unwrap_or(false)
}

/// Write the embedded bridge files to disk, creating the directory if needed.
pub fn ensure_bridge_files() -> Result<PathBuf> {
    let dir = bridge_dir()?;
    std::fs::create_dir_all(&dir)
        .with_context(|| format!("creating bridge directory {}", dir.display()))?;
    std::fs::write(dir.join("ghostnet-bridge.mjs"), BRIDGE_JS)?;
    std::fs::write(dir.join("package.json"), BRIDGE_PKG)?;
    Ok(dir)
}

fn require_ready() -> Result<PathBuf> {
    let script = bridge_script()?;
    if !script.exists() {
        return Err(anyhow!("the GhostNet SDK bridge isn't set up yet — run `ghostnet setup` first"));
    }
    if !bridge_installed() {
        return Err(anyhow!("the GhostNet SDK isn't installed yet — run `ghostnet setup` first"));
    }
    Ok(script)
}

/// Run a one-shot bridge command and return its raw process output.
pub fn run_bridge(args: &[String]) -> Result<Output> {
    let script = require_ready()?;
    let dir = bridge_dir()?;
    Command::new(node_bin())
        .arg(&script)
        .args(args)
        .current_dir(&dir)
        .output()
        .context("failed to launch Node.js — is it installed and on your PATH?")
}

/// Run a one-shot bridge command and parse the last NDJSON line it emitted.
pub fn run_bridge_json(args: &[String]) -> Result<Value> {
    let output = run_bridge(args)?;
    parse_last_json(&output)
}

/// Extract and parse the final non-empty JSON line from a process output.
pub fn parse_last_json(output: &Output) -> Result<Value> {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let line = stdout
        .lines()
        .rev()
        .find(|l| !l.trim().is_empty())
        .ok_or_else(|| {
            anyhow!(
                "the SDK bridge produced no output{}",
                if stderr.trim().is_empty() {
                    String::new()
                } else {
                    format!(":\n{}", stderr.trim())
                }
            )
        })?;

    let value: Value =
        serde_json::from_str(line).with_context(|| format!("parsing bridge output: {line}"))?;

    if value.get("ok").and_then(Value::as_bool) == Some(false) {
        let msg = value
            .get("error")
            .and_then(Value::as_str)
            .unwrap_or("bridge reported an error");
        return Err(anyhow!(msg.to_string()));
    }

    Ok(value)
}
