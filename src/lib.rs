pub(crate) mod commands;
pub(crate) mod config;
pub(crate) mod templater;

use anyhow::Result;
use clap::Parser;

use commands::{handle, Commands};

#[derive(Parser)]
#[clap(
    name = "lotus",
    about = "A future of templating",
    version = "0.1.0",
    author,
)]
pub struct CLI {
    #[clap(subcommand)]
    pub commands: Commands,
}

pub async fn run() -> Result<()> {
    let cli = CLI::parse();
    if let Err(error) = handle(cli.commands).await {
        eprintln!("{}", error);
    }

    Ok(())
}

