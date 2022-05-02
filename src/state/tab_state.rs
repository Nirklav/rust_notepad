use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use druid::{Data, Lens};
use crate::error::AppError;
use crate::state;
use crate::state::tab_content::TabContent;

#[derive(Clone, Debug, Data, Lens, PartialEq)]
pub struct TabState {
    pub id: u64,
    pub name: String,
    pub content: TabContent
}

impl TabState {
    pub fn new(id: u64, name: String, password: Option<String>) -> Result<Self, AppError> {
        let path = TabState::path(&name);
        if path.exists() {
            return Err(AppError::FileAlreadyExist);
        }

        File::create(path)?;

        Ok(TabState {
            id,
            name,
            content: TabContent::new(password)?
        })
    }

    pub fn load<S: AsRef<str>>(id: u64, name: S) -> Result<Self, AppError> {
        let path = TabState::path(&name);
        let mut file = File::open(path)?;
        Ok(TabState {
            id,
            name: name.as_ref().to_string(),
            content: TabContent::read(&mut file)?
        })
    }

    pub fn open(&mut self) -> Result<(), AppError> {
        let path = TabState::path(&self.name);
        let mut file = File::open(path)?;
        self.content = self.content.open(&mut file)?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), AppError> {
        let saving_path = TabState::saving_path(&self.name);
        let mut saving_file = File::create(&saving_path)?;
        if self.content.save(&mut saving_file)? {
            let path = TabState::path(&self.name);
            fs::remove_file(&path)?;
            fs::rename(&saving_path, &path)?;
        } else {
            fs::remove_file(&saving_path)?;
        }
        Ok(())
    }

    pub fn remove(&self) -> Result<PathBuf, AppError> {
        self.save()?;

        let path = TabState::path(&self.name);
        let del_path = TabState::del_path(&self.name);

        fs::rename(&path, &del_path)?;

        Ok(del_path)
    }

    pub fn opened(&self) -> bool {
        match &self.content {
            TabContent::Closed { .. } => false,
            _ => true
        }
    }

    fn path<S: AsRef<str>>(name: S) -> PathBuf {
        Path::new(state::DIR)
            .join(name.as_ref())
            .with_extension("txt")
    }

    fn saving_path<S: AsRef<str>>(name: S) -> PathBuf {
        Path::new(state::DIR)
            .join(name.as_ref())
            .with_extension("saving.txt")
    }

    fn del_path<S: AsRef<str>>(name: S) -> PathBuf {
        let mut index = 0;
        loop {
            let path = Path::new(state::DIR)
                .join(name.as_ref())
                .with_extension(format!("{}.del.txt", index));

            if !path.exists() {
                return path;
            }

            index += 1;
        }
    }
}