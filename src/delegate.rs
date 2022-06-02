use druid::{AppDelegate, Command, commands, DelegateCtx, Env, Handled, Target, WindowHandle, WindowId};
use crate::{AppState, windows};

pub struct Delegate {
    main: Option<Window>
}

pub struct Window {
    id: WindowId,
    handle: WindowHandle
}

impl Delegate {
    pub fn new() -> Self {
        Delegate {
            main: None
        }
    }
}

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        state: &mut AppState,
        _env: &Env) -> Handled {

        match  &cmd {
            c if c.is(commands::NEW_FILE) => {
                ctx.new_window(windows::new_file_window::new());
                Handled::Yes
            },
            c if c.is(commands::SAVE_FILE) => {
                if let Err(e) = state.save() {
                    ctx.new_window(windows::information_window::new(format!("Error: {}", e)));
                }
                Handled::Yes
            },
            c if c.is(crate::commands::SHOW_BACKUPS) => {
                ctx.new_window(windows::backup_window::new());
                Handled::Yes
            },
            c if c.is(crate::commands::SHOW_MAIN_WINDOW) => {
                if let Some(ref main) = self.main {
                    main.handle.bring_to_front_and_focus();
                    Handled::Yes
                } else {
                    Handled::No
                }
            },
            c if c.is(commands::SHOW_ABOUT) => {
                ctx.new_window(windows::about_window::new());
                Handled::Yes
            },
            _ => {
                Handled::No
            }
        }
    }

    fn window_added(
        &mut self,
        id: WindowId,
        handle: WindowHandle,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx) {

        if self.main.is_none() {
            self.main = Some(Window {
                id,
                handle: handle.clone()
            });
        }
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx) {
        if let Some(ref main) = self.main {
            if id == main.id {
                data.save().expect("Cannot save file");
            }
        }
    }
}