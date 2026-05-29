pub mod identity;
pub mod info;
pub mod listen;
pub mod send;
pub mod setup;

use crate::cli::Commands;
use anyhow::Result;

/// Run a single parsed command. Shared by one-shot mode and the interactive shell.
pub fn dispatch(command: Commands) -> Result<()> {
    match command {
        Commands::Setup => setup::run(),
        Commands::Info => info::run(),
        Commands::Identity { action } => identity::run(action),
        Commands::Send { peer, message, seed, endpoint } => {
            send::run(peer, message, seed, endpoint)
        }
        Commands::Listen { seed, endpoint } => listen::run(seed, endpoint),
    }
}
