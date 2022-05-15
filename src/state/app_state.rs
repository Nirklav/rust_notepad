use druid::{Data, Lens};
use druid::im::Vector;
use crate::backups::Backup;
use crate::error::AppError;
use crate::state;
use crate::state::config::Config;
use crate::state::new_tab::NewTab;
use crate::state::tabs::Tabs;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub config: Config,
    pub new_tab: NewTab,
    pub tabs: Tabs,
    pub to_remove: Vector<String>
}

impl AppState {
    pub fn load() -> Result<Self, AppError> {
        Ok(AppState {
            config: Config::load()?,
            new_tab: NewTab::new(),
            tabs: Tabs::load()?,
            to_remove: Vector::new()
        })
    }

    pub fn add_new_clear_tab(&mut self) -> Result<(), AppError> {
        let (name, _) = self.new_tab.take();
        self.tabs.add(name, None)
    }

    pub fn add_new_protected_tab(&mut self) -> Result<(), AppError> {
        let (name, password) = self.new_tab.take();
        self.tabs.add(name, Some(password))
    }

    pub fn save(&mut self) -> Result<(), AppError> {
        if self.config.auto_backup {
            self.made_backup()?;
        } else {
            self.tabs.save()?;
        }

        Ok(())
    }

    pub fn made_backup(&mut self) -> Result<(), AppError> {
        self.tabs.save()?;

        let mut backup = Backup::google_drive()?;
        let names = (&self.config.backup_folder, &self.config.backup_file);
        let docs_path = state::docs_path()?;
        let docs = state::docs(&docs_path)?;
        backup.backup(names, &docs)?;

        Ok(())
    }

    pub fn restore_backup(&mut self) -> Result<(), AppError> {
        self.tabs.save()?;

        let mut backup = Backup::google_drive()?;
        let names = (&self.config.backup_folder, &self.config.backup_file);
        let docs_path = state::docs_path()?;
        backup.restore(names, &docs_path)?;

        self.tabs.reload()?;
        Ok(())
    }
}