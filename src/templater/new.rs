use std::path::PathBuf;
use anyhow::{anyhow, Result};
use crate::config::global::{Template, Config};

pub async fn create_new(path: PathBuf, name: String) -> Result<()> {
    let template = Template {path: std::fs::canonicalize(&path)?, name: name.clone()};
    
    let mut config = Config::load();
    config.add_template(template);
    
    if config.save().is_err() {
        return Err(anyhow!("Failed to save config file"));
    }

    println!("Successfully added new template \"{}\"", name);

    Ok(())
}