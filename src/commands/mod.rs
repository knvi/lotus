pub mod create;
pub mod deploy;

use anyhow::Result;
use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Create(create::Options),
    Deploy(deploy::Options),
}

pub async fn handle(command: Commands) -> Result<()> {
    match command {
        Commands::Create(options) => create::run(options).await,
        Commands::Deploy(options) => deploy::run(options).await,
    }
}