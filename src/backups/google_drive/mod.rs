pub mod oauth;

mod credentials;
mod token;
mod api;
mod id;

use std::path::Path;
use reqwest::blocking::Client;
use crate::AppError;
use crate::backups::{BackupStorage, Names};
use crate::backups::google_drive::credentials::Credentials;
use crate::backups::google_drive::id::Id;

pub struct GoogleDrive {
    credentials: Credentials,
    client: Client
}

impl GoogleDrive {
    fn backup_folder(&mut self, name: &str) -> Result<Option<Id>, AppError> {
        let mut folders = self.list(&name, None, true)?;
        Ok(folders.pop())
    }

    fn backup_file(&mut self, backup_folder: &Id, name: &str) -> Result<Option<Id>, AppError> {
        let mut files = self.list(&name, Some(&backup_folder), false)?;
        Ok(files.pop())
    }
}

impl BackupStorage for GoogleDrive {
    fn save<'a, 'b>(&mut self, names: Names<'a, 'b>, file_path: &Path) -> Result<(), AppError> {
        if let Some(backup_folder) = self.backup_folder(&names.folder)? {
            if let Some(backup_file) = self.backup_file(&backup_folder, &names.file)? {
                self.update_file(&backup_file, file_path)?;
            } else {
                let _file_id = self.create_file(&names.file, Some(&backup_folder), file_path)?;
            }
        } else {
            let backup_folder = self.create_folder(&names.folder)?;
            let _file_id = self.create_file(&names.file, Some(&backup_folder), file_path)?;
        }

        Ok(())
    }

    fn restore<'a, 'b>(&mut self, names: Names<'a, 'b>, file_path: &Path) -> Result<(), AppError> {
        if let Some(backup_folder) = self.backup_folder(&names.folder)? {
            if let Some(backup_file) = self.backup_file(&backup_folder, &names.file)? {
                self.download_file(&backup_file, file_path)?;
                return Ok(());
            }
        }

        Err(AppError::BackupNotFound)
    }
}