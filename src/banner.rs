use colored::Colorize;

/// The N11X Collective branding banner, printed at the top of CLI output.
pub fn print_banner() {
    let lines = [
        "╔═══════════════════════════════════════════════╗",
        "║                                               ║",
        "║        N 1 1 X   C O L L E C T I V E           ║",
        "║         · G H O S T N E T   C L I ·            ║",
        "║                                               ║",
        "║      encrypted · decentralized · private       ║",
        "║                                               ║",
        "╚═══════════════════════════════════════════════╝",
    ];
    println!();
    for line in lines {
        println!("{}", line.bright_magenta().bold());
    }
    println!();
}

/// Greets the current user by name.
pub fn welcome() {
    let user = current_user();
    println!(
        "{} {}{}",
        "👻 Welcome,".bright_green().bold(),
        user.bright_cyan().bold(),
        "!".bright_green().bold()
    );
    println!(
        "{}",
        "   You're plugged into the GhostNet mesh — N11X Collective.".bright_black()
    );
    println!();
}

fn current_user() -> String {
    std::env::var("USERNAME")
        .or_else(|_| std::env::var("USER"))
        .unwrap_or_else(|_| "ghost".to_string())
}
