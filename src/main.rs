mod banner;
mod cli;
mod commands;
mod node;

use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;

fn main() {
    let args = Cli::parse();

    if args.no_color {
        colored::control::set_override(false);
    }

    let result = match args.command {
        None => {
            // Bare `ghostnet` — greet and point at the help.
            banner::print_banner();
            banner::welcome();
            println!("  Run {} to see everything this CLI can do.", "ghostnet --help".bold());
            Ok(())
        }
        Some(Commands::Info) => commands::info::run(),
        Some(Commands::Setup) => commands::setup::run(),
        Some(Commands::Identity { action }) => commands::identity::run(action),
        Some(Commands::Send { peer, message, seed, endpoint }) => {
            commands::send::run(peer, message, seed, endpoint)
        }
        Some(Commands::Listen { seed, endpoint }) => commands::listen::run(seed, endpoint),
    };

    if let Err(err) = result {
        eprintln!("{} {}", "error:".red().bold(), err);
        std::process::exit(1);
    }
}
