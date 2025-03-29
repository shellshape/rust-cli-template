{% if subcommands %}mod commands;{% endif %}
{% if config %}mod config;{% endif %}

use anyhow::Result;
use clap::{command, Parser};
{% if subcommands %}use commands::*;{% endif %}
{% if config %}use config::Config;{% endif %}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    {% if config %}
    /// Path to a config file
    #[arg(short, long)]
    config: Option<String>,
    {% endif %}

    {% if subcommands %}
    #[command(subcommand)]
    commands: Commands,
    {% endif %}
}

{% if subcommands %}
// List the names of your sub commands here.
register_commands! {
    Hello
}
{% endif %}

fn main() -> Result<()> {
    let cli = Cli::parse();

    {% if config %}
    let cfg = Config::parse(cli.config)?;
    {% endif %}

    {% if subcommands %}
    cli.commands.run()?;
    {% endif %}

    Ok(())
}
