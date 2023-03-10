#![warn(clippy::pedantic)]

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    lotus::run().await?;

    Ok(())
}