use druid::{AppDelegate, Code, Command, commands, DelegateCtx, Env, Event, Handled, KeyEvent, Target, WindowHandle, WindowId};
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
    fn event(
        &mut self,
        _ctx: &mut DelegateCtx,
        _id: WindowId,
        event: Event,
        data: &mut AppState,
        _env: &Env) -> Option<Event> {

        match &event {
            Event::KeyDown(KeyEvent { code: Code::KeyS, mods, .. }) if mods.ctrl() => {
                let _ = data.tabs.save();
            },
            _ => { }
        };

        Some(event)
    }

    fn command(
        &mut self,
        ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _state: &mut AppState,
        _env: &Env) -> Handled {

        match  &cmd {
            c if c.is(commands::NEW_FILE) => {
                ctx.new_window(windows::new_file_window::new());
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
        if let None = self.main_id {
            self.main_id = Some(id.clone());
        }
    }

    fn window_removed(
        &mut self,
        _id: WindowId,
        data: &mut AppState,
        _env: &Env,
        _ctx: &mut DelegateCtx) {
        if let Some(id) = self.main_id {
            if id == id {
                data.tabs.save().expect("Cannot save file");
            }
        }
    }
}