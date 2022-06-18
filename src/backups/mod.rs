use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::{Path, PathBuf};
use zip::{ZipArchive, ZipWriter};
use crate::{AppError, exe_directory, GoogleDrive};

pub mod google_drive;

pub trait BackupStorage {
    fn save<'a, 'b>(&mut self, names: Names<'a, 'b>, file: &Path) -> Result<(), AppError>;
    fn restore<'a, 'b>(&mut self, names: Names<'a, 'b>, file: &Path) -> Result<(), AppError>;
}

pub struct Backup<S> {
    storage: S
}

impl Backup<GoogleDrive> {
    pub fn google_drive() -> Result<Self, AppError> {
        Ok(Backup {
            storage: GoogleDrive::new()?
        })
    }
}

impl <S: BackupStorage> Backup<S> {
    pub fn backup<'a, 'b>(&mut self, names: impl Into<Names<'a, 'b>>, files: &[impl AsRef<Path>]) -> Result<(), AppError> {
        let names = names.into();
        let archive_file_path = self.temp_file_path()?;
        {
            let archive_file = File::create(&archive_file_path)?;
            let mut archive = ZipWriter::new(archive_file);

            for path in files {
                let path = path.as_ref();
                let file_name = Backup::<S>::extract_file_name(path)?;
                archive.start_file(file_name, Default::default())?;

                let mut file = File::open(path)?;
                copy(&mut file, &mut archive)?;
            }
        }

        self.storage.save(names, &archive_file_path)?;
        fs::remove_file(&archive_file_path)?;

        Ok(())
    }

    pub fn restore<'a, 'b>(&mut self, names: impl Into<Names<'a, 'b>>, folder: impl AsRef<Path>) -> Result<(), AppError> {
        let names = names.into();
        let folder = folder.as_ref();
        let archive_file_path = self.temp_file_path()?;
        {
            self.storage.restore(names, &archive_file_path)?;

            let archive_file = File::open(&archive_file_path)?;
            let mut archive = ZipArchive::new(archive_file)?;

            for i in 0..archive.len() {
                let mut source_file = archive.by_index(i)?;
                let destination_file_path = folder.join(source_file.name());
                let mut destination_file = File::create(&destination_file_path)?;
                copy(&mut source_file, &mut destination_file)?;
            }
        }

        fs::remove_file(&archive_file_path)?;
        Ok(())
    }

    fn extract_file_name(path: &Path) -> Result<&str, AppError> {
        if let Some(os_file_name) = path.file_name() {
            if let Some(file_name) = os_file_name.to_str() {
                return Ok(file_name);
            }
        }

        Err(AppError::Internal("It is not a file"))
    }

    fn temp_file_path(&self) -> Result<PathBuf, AppError> {
        Ok(exe_directory()?.join("temp.zip"))
    }
}

pub struct Names<'a, 'b> {
    pub folder: &'a str,
    pub file: &'b str
}

impl<'a, 'b, T: AsRef<str>> From<(&'a T, &'b T)> for Names<'a, 'b> {
    fn from(arguments: (&'a T, &'b T)) -> Self {
        Names {
            folder: arguments.0.as_ref(),
            file: arguments.1.as_ref()
        }
    }
}