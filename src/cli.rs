use clap::{Parser, Subcommand};

/// GhostNet CLI — encrypted mesh network client by the N11X Collective.
#[derive(Parser)]
#[command(
    name = "ghostnet",
    version,
    about = "GhostNet CLI — encrypted mesh client · N11X Collective",
    long_about = None,
    propagate_version = true
)]
pub struct Cli {
    /// Disable colored output.
    #[arg(long, global = true)]
    pub no_color: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Install the GhostNet SDK bridge (runs `npm install @n11x/ghostnet-sdk`).
    Setup,

    /// Show CLI / SDK status and N11X Collective info.
    Info,

    /// Create or restore a GhostNet identity.
    Identity {
        #[command(subcommand)]
        action: IdentityAction,
    },

    /// Send an end-to-end encrypted message to a peer.
    Send {
        /// Peer node ID (e.g. 0x...).
        peer: String,

        /// Message text (max 64 KB).
        message: String,

        /// Seed phrase to restore an identity (otherwise a fresh one is used).
        /// Prefer the GHOSTNET_SEED env var to keep it out of shell history.
        #[arg(long)]
        seed: Option<String>,

        /// Override the relay endpoint (wss:// only).
        #[arg(long)]
        endpoint: Option<String>,
    },

    /// Connect to the mesh and stream incoming messages.
    Listen {
        /// Seed phrase to restore an identity (otherwise a fresh one is used).
        /// Prefer the GHOSTNET_SEED env var to keep it out of shell history.
        #[arg(long)]
        seed: Option<String>,

        /// Override the relay endpoint (wss:// only).
        #[arg(long)]
        endpoint: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum IdentityAction {
    /// Generate a brand-new BIP-39 identity.
    Create,

    /// Restore an identity from a 12-word seed phrase.
    ///
    /// Tip: set the GHOSTNET_SEED environment variable instead of passing the
    /// phrase as an argument, to keep it out of shell history and process lists.
    Load {
        /// The 12-word seed phrase (quote it). Omit to read from GHOSTNET_SEED.
        seed: Option<String>,
    },
}
