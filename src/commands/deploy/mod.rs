use anyhow::Result;
use clap::Parser;
use crate::templater::deploy::deploy_name;

#[derive(Debug, Parser)]
#[clap(about = "Deploy a template")]
pub struct Options {
    pub name: String,
}

pub async fn run(options: Options) -> Result<()> {
    let name = options.name;

    deploy_name(name).await?;

    Ok(())
}