use anyhow::{anyhow, Result};
use crate::config::global::Config;
use ignore::{DirEntry, Walk};
use std::fs::{copy, create_dir_all, set_permissions, OpenOptions, File};
use std::path::{Path};

fn process_file(entry: &DirEntry) -> Result<()> {
    let path = entry.path();
    println!("processing file: {}", path.display());

    let target_path = std::fs::canonicalize(Path::new("."))?;
    println!("target path: {}", target_path.display());

    set_permissions(&path, entry.metadata()?.permissions())
        .map_err(|e| anyhow!("failed to set permissions: {}", e))?;
    
    if let Some(parent_dir) = target_path.parent() {
        create_dir_all(parent_dir)
            .map_err(|e| anyhow!("failed to create directory: {}", e))?;
    }

    // copy the file or directory to the target path
    if entry.path().is_file() {
        // use std::io::copy to copy the file
        let mut from = OpenOptions::new().write(true).read(true).open(&path)
            .map_err(|e| anyhow!("failed to open file from: {}", e))?;

        let mut to = File::create(&target_path)
            .map_err(|e| anyhow!("failed to create file to: {}", e))?;
        std::io::copy(&mut from, &mut to)
            .map_err(|e| anyhow!("failed to copy file: {}", e))?;
    } else if entry.path().is_dir() {
        create_dir_all(&target_path)
            .map_err(|e| anyhow!("failed to create directory: {}", e))?;
    }

    Ok(())
}

pub async fn deploy_name(name: String) -> Result<()> {
    let config = Config::load();
    let template = config.get_template(&name).unwrap();

    println!("Deploying template \"{}\" from \"{}\"", template.name, template.path.display());

    // iterate through every file and directory in the template
    for result in Walk::new(&template.path) {
        let entry = match result {
            Ok(entry) => entry,
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        process_file(&entry)?;
    }
    



    Ok(())
}