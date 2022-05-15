use std::{fs, collections};
use std::fs::File;
use std::io::Write;
use druid::Data;
use druid::im::{HashMap, Vector};
use crate::state;
use crate::error::AppError;
use crate::state::tab::Tab;

#[derive(Clone, Data)]
pub struct Tabs {
    tabs: HashMap<u64, Tab>,
    to_remove: Vector<String>,
    rev: u64
}

impl Tabs {
    pub fn load() -> Result<Self, AppError> {
        let (rev, tabs) = Self::load_tabs()?;
        Ok(Tabs {
            tabs: HashMap::from(tabs),
            to_remove: Vector::new(),
            rev
        })
    }

    pub fn reload(&mut self) -> Result<(), AppError> {
        let (_, tabs) = Self::load_tabs()?;

        for (_, loaded_tab) in tabs {
            let pair = self.tabs
                .iter_mut()
                .find(|p| p.1.name == loaded_tab.name);

            if let Some((_, tab)) = pair {
                tab.update(&loaded_tab);
            } else {
                self.rev += 1;
                self.tabs.insert(self.rev, loaded_tab);
            }
        }

        Ok(())
    }

    fn load_tabs() -> Result<(u64, collections::HashMap::<u64, Tab>), AppError> {
        let mut tabs = collections::HashMap::<u64, Tab>::new();
        let mut rev = 0;
        let docs_path = state::docs_path()?;
        let docs = match state::docs(&docs_path) {
            Ok(p) => p,
            Err(_) => {
                fs::create_dir(&docs_path)?;
                let mut file = File::create(docs_path.join("text.txt"))?;
                write!(file, "New text")?;
                state::docs(&docs_path)?
            }
        };

        for path in docs {
            if path.is_file() {
                let name = path
                    .file_stem().ok_or(AppError::internal("Not a file"))?
                    .to_str().ok_or(AppError::internal("Invalid file name"))?;

                rev += 1;
                let tab = Tab::load(rev, &name)?;
                tabs.insert(tab.id, tab);
            }
        }

        Ok((rev, tabs))
    }

    pub fn keys(&self) -> Vec<u64> {
        let mut keys = Vec::<u64>::new();
        for (key, _) in &self.tabs {
            keys.push(*key);
        }
        keys.sort();
        keys
    }

    pub fn rev(&self) -> u64 {
        self.rev
    }

    pub fn get(&self, key: u64) -> &Tab {
        self.tabs.get(&key).expect("Index error")
    }

    pub fn get_mut(&mut self, key: u64) -> &mut Tab {
        self.tabs.get_mut(&key).expect("Index error")
    }

    pub fn add(&mut self, name: String, password: Option<String>) -> Result<(), AppError> {
        for (_, tab) in &self.tabs {
            if tab.name == name {
                return Err(AppError::FileAlreadyExist)
            }
        }

        self.rev += 1;
        let tab = Tab::new(self.rev, name, password)?;
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

    pub(super) fn save(&mut self) -> Result<(), AppError> {
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