//! Local audit metadata only: no message body or seed phrase is written to disk.
use crate::config;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub timestamp: u64,
    pub direction: String,
    pub peer: String,
    pub bytes: usize,
    pub outcome: String,
}

fn path() -> Result<std::path::PathBuf> {
    Ok(config::data_dir()?.join("history.ndjson"))
}

pub fn record(direction: &str, peer: &str, bytes: usize, outcome: &str) -> Result<()> {
    std::fs::create_dir_all(config::data_dir()?)?;
    let entry = Entry {
        timestamp: config::unix_time(),
        direction: direction.into(),
        peer: peer.into(),
        bytes,
        outcome: outcome.into(),
    };
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path()?)?;
    writeln!(file, "{}", serde_json::to_string(&entry)?)?;
    Ok(())
}

pub fn recent(limit: usize) -> Result<Vec<Entry>> {
    let path = path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut entries: Vec<Entry> = std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    entries.reverse();
    entries.truncate(limit);
    Ok(entries)
}
