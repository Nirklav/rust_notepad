use druid::{commands, Menu, MenuItem};
use crate::AppState;
use crate::ui::context_menu_controller::MenuFactory;

pub struct TextBoxMenuFactory;

impl MenuFactory for TextBoxMenuFactory {
    fn make(&self) -> Menu<AppState> {
        Menu::empty()
            .entry(MenuItem::new("Copy")
                .command(commands::COPY))
            .entry(MenuItem::new("Cut")
                .command(commands::CUT))
            .entry(MenuItem::new("Paste")
                .command(commands::PASTE))
    }
}