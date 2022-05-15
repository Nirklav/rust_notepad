use druid::{commands, Env, lens, Menu, MenuItem, Widget, WidgetExt, WindowDesc, WindowId};
use druid::widget::Tabs;
use druid_shell::RawMods;
use crate::*;
use crate::ui::tabs_dynamic_policy::TabsDynamicPolicy;

pub fn new() -> WindowDesc<AppState> {
    let size = (1400.0, 800.0);
    WindowDesc::new(ui())
        .title("Notepad")
        .window_size(size)
        .set_position(windows::primary_screen_center(size))
        .menu(menu)
}

fn ui() -> impl Widget<AppState> {
    Tabs::for_policy(TabsDynamicPolicy)
        .lens(lens!(AppState, tabs))
}

fn menu(_id: Option<WindowId>, _state: &AppState, _env: &Env) -> Menu<AppState> {
    Menu::new("Menu")
        .entry(Menu::new("File")
            .entry(MenuItem::new("New file")
                .command(commands::NEW_FILE)
                .hotkey(RawMods::Ctrl, "n"))
            .entry(MenuItem::new("Save all")
                .command(commands::SAVE_FILE)
                .hotkey(RawMods::Ctrl, "s"))
            .entry(MenuItem::new("Backups")
                .command(crate::commands::SHOW_BACKUPS))
            .entry(MenuItem::new("Exit")
                .command(commands::CLOSE_ALL_WINDOWS)))
        .entry(MenuItem::new("About")
            .command(commands::SHOW_ABOUT))
}