use std::path::PathBuf;
use std::io::Read;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Template {
    pub path: PathBuf,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub templates: Vec<Template>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            templates: Vec::new(),
        }
    }

    pub fn add_template(&mut self, template: Template) {
        self.templates.push(template);
    }

    pub fn remove_template(&mut self, name: &str) {
        self.templates.retain(|t| t.name != name);
    }

    pub fn get_template(&self, name: &str) -> Option<&Template> {
        self.templates.iter().find(|t| t.name == name)
    }

    pub fn get_templates(&self) -> &Vec<Template> {
        &self.templates
    }

    pub fn save(&self) -> Result<()> {
        let path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("lotus")
            .join("lotus.toml");

        let dir = path.parent().unwrap();

        if !dir.exists() {
            std::fs::create_dir_all(dir)?;
        }

        let file = std::fs::File::create(path)?;

        let content = toml::to_string_pretty(self)?;

        std::io::Write::write_all(&mut &file, content.as_bytes())?;

        Ok(())
    }

    pub fn load() -> Self {
        let path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("lotus")
            .join("lotus.toml");

        if !path.exists() {
            Self {
                templates: Vec::new(),
            }
        } else {
            let mut file = std::fs::File::open(path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            toml::from_str(&content).unwrap()
        }
    }
}