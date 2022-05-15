use druid::Data;
use std::fs::File;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use crate::{AppError, exe_directory};

#[derive(Serialize, Deserialize, Debug, Clone, Data)]
pub struct Config {
    pub auto_backup: bool,
    pub backup_folder: String,
    pub backup_file: String,
}

impl Config {
    pub fn load() -> Result<Self, AppError> {
        let config_path = Config::config_path()?;
        if config_path.exists() {
            let file = File::open(config_path)?;
            Ok(serde_json::from_reader(file)?)
        } else {
            let empty = Config {
                auto_backup: false,
                backup_folder: "Backups".to_string(),
                backup_file: "notepad_backup.zip".to_string()
            };
            let file = File::create(config_path)?;
            serde_json::to_writer_pretty(file, &empty)?;
            Ok(empty)
        }
    }

    pub fn save(&self) -> Result<(), AppError> {
        let config_path = Config::config_path()?;
        let file = File::create(config_path)?;
        serde_json::to_writer_pretty(file, self)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf, AppError> {
        Ok(exe_directory()?.join("config.json"))
    }
}