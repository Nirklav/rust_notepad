use std::{fs, collections};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use druid::Data;
use druid::im::{HashMap, Vector};
use crate::state;
use crate::error::AppError;
use crate::state::tab_state::TabState;

#[derive(Clone, Data)]
pub struct TabsState {
    tabs: HashMap<u64, TabState>,
    to_remove: Vector<String>,
    rev: u64
}

impl TabsState {
    pub fn load() -> Result<Self, AppError> {
        let mut tabs = collections::HashMap::<u64, TabState>::new();
        let mut rev = 0;
        let dir = match fs::read_dir(state::DIR) {
            Ok(p) => p,
            Err(_) => {
                fs::create_dir(state::DIR)?;
                let mut file = File::create(Path::new(state::DIR).join("text.txt"))?;
                write!(file, "New text")?;
                fs::read_dir(state::DIR)?
            }
        };

        for entity_result in dir {
            if let Ok(entry) = entity_result {
                let path = &entry.path();
                if path.is_file() {
                    let name = path
                        .file_stem().ok_or(AppError::internal("Not a file"))?
                        .to_str().ok_or(AppError::internal("Invalid file name"))?;

                    rev += 1;
                    let tab = TabState::load(rev, &name)?;
                    tabs.insert(tab.id, tab);
                }
            }
        }

        Ok(TabsState {
            tabs: HashMap::from(tabs),
            to_remove: Vector::new(),
            rev
        })
    }

    pub fn keys(&self) -> Vec<u64> {
        let mut keys = Vec::<u64>::new();
        for (key, _) in &self.tabs {
            keys.push(*key);
        }
        keys
    }

    pub fn rev(&self) -> u64 {
        self.rev
    }

    pub fn get(&self, key: u64) -> &TabState {
        self.tabs.get(&key).expect("Index error")
    }

    pub fn get_mut(&mut self, key: u64) -> &mut TabState {
        self.tabs.get_mut(&key).expect("Index error")
    }

    pub fn add(&mut self, name: String, password: Option<String>) -> Result<(), AppError> {
        for (_, tab) in &self.tabs {
            if tab.name == name {
                return Err(AppError::FileAlreadyExist)
            }
        }

        self.rev += 1;
        let tab = TabState::new(self.rev, name, password)?;
        self.tabs.insert(tab.id, tab);

        Ok(())
    }

    pub fn remove(&mut self, key: u64) -> Result<(), AppError> {
        let tab = self.tabs.remove(&key).ok_or(AppError::internal("Invalid key"))?;

        let path = tab.remove()?;
        let file_name = path
            .to_str().ok_or(AppError::internal("Invalid file name"))?
            .to_string();

        self.to_remove.push_back(file_name);
        self.rev += 1;
        Ok(())
    }

    pub fn save(&mut self) -> Result<(), AppError> {
        for (_, tab) in &self.tabs {
            tab.save()?;
        }

        for file_name in &self.to_remove {
            fs::remove_file(&file_name)?;
        }
        self.to_remove.clear();

        Ok(())
    }
}