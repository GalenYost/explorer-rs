use std::{fs, path};
use anyhow::{bail, Result};

use crate::config_parse::Config;

const DEFAULT_CONFIG_PATH: &str = "config.toml";

#[derive(Debug,PartialEq,Eq,Clone,Default)]
pub enum EntryType {
    #[default]
    Dir,
    File
}

pub struct App {
    pub current_dir: String,
    pub config: Config,
    pub entries: Option<Vec<Entry>>,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Entry {
    pub entry_type: EntryType,
    pub name: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_dir: "/".to_string(),
            config: Config::default(),
            entries: None,
        }
    }
}

impl App {
    pub fn render(&mut self, initial_path: Option<String>, config_path: Option<String>) -> Result<()> {
        self.config.load(match config_path {
            Some(p) => p,
            _ => DEFAULT_CONFIG_PATH.to_string(),
        })?;

        self.current_dir = match initial_path {
            Some(p) => p,
            _ => self.config.def_dir.clone()
        };

        self.query_dir()?;
    
        // todo: init ui

        Ok(())
    }

    pub fn close(&self) -> Result<()> {
        // todo: terminate ui runtime

        self.config.save(Some(DEFAULT_CONFIG_PATH.to_string()))?;

        Ok(())
    }

    pub fn query_dir(&mut self) -> Result<()> {
        let mut entries: Vec<Entry> = Vec::new();

        for entry_result in fs::read_dir(self.current_dir.clone())? {
            let entry = entry_result?;
            entries.push(Entry {
                entry_type: match entry.file_type()?.is_file() {
                    true => EntryType::File,
                    false => EntryType::Dir,
                },
                name: entry.file_name().into_string().unwrap(),
            });
        }

        self.entries = Some(entries);
        Ok(())
    }

    pub fn change_dir(&mut self, path: String) -> Result<()> {
        let p = path::Path::new(&path);
        if p.exists() {
            self.current_dir = path;
            self.query_dir()?;
            return Ok(());
        } else {
            bail!("Invalid path: {}", path);
        }
    } 
}
