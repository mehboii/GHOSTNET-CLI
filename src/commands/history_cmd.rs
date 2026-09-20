use crate::history;
use anyhow::Result;

pub fn run(limit: usize) -> Result<()> {
    for entry in history::recent(limit)? {
        println!(
            "{}  {} {} ({} bytes, {})",
            entry.timestamp, entry.direction, entry.peer, entry.bytes, entry.outcome
        );
    }
    Ok(())
}
