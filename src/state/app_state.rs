use druid::{Data, Lens};
use druid::im::Vector;
use crate::error::AppError;
use crate::state::new_tab::NewTab;
use crate::state::tabs_state::TabsState;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub new_tab: NewTab,
    pub tabs: TabsState,
    pub to_remove: Vector<String>
}

impl AppState {
    pub fn load() -> Result<Self, AppError> {
        Ok(AppState {
            new_tab: NewTab::new(),
            tabs: TabsState::load()?,
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
}