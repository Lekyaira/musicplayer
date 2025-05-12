mod gui;
mod player;
mod cli;

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Run in CLI mode
    #[arg(short, long)]
    cli: bool,

    /// Path to music file or directory
    // #[arg(short, long)]
    path: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.cli {
        cli::run(args.path)?;
    } else if args.path.is_some() {
        gui::run(args.path)?;
    } else {
        gui::run(None)?;
    }

    Ok(())
}
