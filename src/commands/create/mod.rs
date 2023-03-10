use anyhow::Result;
use std::path::PathBuf;
use clap::Parser;
use crate::templater::new::create_new;

#[derive(Debug, Parser)]
#[clap(about = "Create a template")]
pub struct Options {
    pub path: String,
    pub name: String,
}

pub async fn run(options: Options) -> Result<()> {
    let path = PathBuf::from(options.path);
    let name = options.name;

    create_new(path, name).await?;

    Ok(())
}