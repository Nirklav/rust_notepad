use druid::{AppDelegate, Command, commands, DelegateCtx, Env, Handled, Target, WindowHandle, WindowId};
use crate::{AppState, windows};

pub struct Delegate {
    main_id: Option<WindowId>
}

impl Delegate {
    pub fn new() -> Self {
        Delegate {
            main_id: None
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
                if let Err(e) = state.tabs.save() {
                    ctx.new_window(windows::information_window::new(format!("Error: {}", e)));
                }
                Handled::Yes
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
        _handle: WindowHandle,
        _data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx) {
        if self.main_id.is_none() {
            self.main_id = Some(id.clone());
        }
    }

    fn window_removed(
        &mut self,
        id: WindowId,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx) {
        if let Some(main_id) = self.main_id {
            if id == main_id {
                data.tabs.save().expect("Cannot save file");
            }
        }
    }
}