use anyhow::Result;
use crate::config::global:: Config;

pub async fn deploy_name(name: String) -> Result<()> {
    let config = Config::load();
    let template = config.get_template(&name).unwrap();

    println!("Deploying template \"{}\" from \"{}\"", template.name, template.path.display());

    println!("not implemented yet!");

    Ok(())
}