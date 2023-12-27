mod config;

use anyhow::Result;
use clap::{command, Parser};
use config::Config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to a config file.
    #[arg(short, long)]
    config: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cfg = Config::parse(cli.config)?;

    Ok(())
}
