use anyhow::Result;
use std::{
    fs::{self, File},
    path,
};
use toml;

use serde::{Deserialize, Serialize};

fn def_dir() -> String {
    "/".to_string()
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct Config {
    #[serde(default = "def_dir")]
    pub def_dir: String,
    #[serde(default)]
    pub hidden_files: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            def_dir: def_dir(),
            hidden_files: false,
        }
    }
}

impl Config {
    pub fn load(&mut self, path_str: String) -> Result<()> {
        let s = fs::read_to_string(&path_str)?;
        let conf: Config = toml::from_str(&s)?;
        *self = conf;
        Ok(())
    }

    pub fn save(&self, path_str: Option<String>) -> Result<()> {
        let f_path = match path_str {
            Some(path) => path,
            _ => "config.toml".to_string(),
        };

        let file_path = path::Path::new(&f_path);
        let conf = toml::to_string_pretty(self)?;

        if !file_path.exists() {
            File::create_new(file_path)?;
        }

        fs::write(file_path, conf)?;

        Ok(())
    }
}
