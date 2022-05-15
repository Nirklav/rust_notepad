use std::fs;
use std::fs::File;
use std::path::PathBuf;
use druid::{Data, Lens};
use crate::error::AppError;
use crate::state;
use crate::state::tab_content::TabContent;

#[derive(Clone, Debug, Data, Lens, PartialEq)]
pub struct Tab {
    pub id: u64,
    pub name: String,
    pub content: TabContent
}

impl Tab {
    pub fn new(id: u64, name: String, password: Option<String>) -> Result<Self, AppError> {
        let path = Tab::path(&name)?;
        if path.exists() {
            return Err(AppError::FileAlreadyExist);
        }

        File::create(path)?;

        Ok(Tab {
            id,
            name,
            content: TabContent::new(password)?
        })
    }

    pub fn load(id: u64, name: impl AsRef<str>) -> Result<Self, AppError> {
        let path = Tab::path(&name)?;
        let mut file = File::open(path)?;
        Ok(Tab {
            id,
            name: name.as_ref().to_string(),
            content: TabContent::read(&mut file)?
        })
    }

    pub fn update(&mut self, another: &Tab) {
        self.content = another.content.clone();
    }

    pub fn open(&mut self) -> Result<(), AppError> {
        let path = Tab::path(&self.name)?;
        let mut file = File::open(path)?;
        self.content = self.content.open(&mut file)?;
        Ok(())
    }

    pub(super) fn save(&self) -> Result<(), AppError> {
        let saving_path = Tab::saving_path(&self.name)?;
        let mut saving_file = File::create(&saving_path)?;
        if self.content.save(&mut saving_file)? {
            let path = Tab::path(&self.name)?;
            fs::remove_file(&path)?;
            fs::rename(&saving_path, &path)?;
        } else {
            fs::remove_file(&saving_path)?;
        }
        Ok(())
    }

    pub fn remove(&self) -> Result<PathBuf, AppError> {
        self.save()?;

        let path = Tab::path(&self.name)?;
        let del_path = Tab::del_path(&self.name)?;

        fs::rename(&path, &del_path)?;

        Ok(del_path)
    }

    pub fn opened(&self) -> bool {
        match &self.content {
            TabContent::Closed { .. } => false,
            _ => true
        }
    }

    fn path(name: impl AsRef<str>) -> Result<PathBuf, AppError> {
        Ok(state::docs_path()?
            .join(name.as_ref())
            .with_extension("txt"))
    }

    fn saving_path(name: impl AsRef<str>) -> Result<PathBuf, AppError> {
        Ok(state::docs_path()?
            .join(name.as_ref())
            .with_extension("saving.txt"))
    }

    fn del_path(name: impl AsRef<str>) -> Result<PathBuf, AppError> {
        let mut index = 0;
        loop {
            let path = state::docs_path()?
                .join(name.as_ref())
                .with_extension(format!("{}.del.txt", index));

            if !path.exists() {
                return Ok(path);
            }

            index += 1;
        }
    }
}