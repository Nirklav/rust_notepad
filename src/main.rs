#![windows_subsystem = "windows"]

extern crate core;
extern crate crypto;
extern crate getrandom;

mod state;
mod ui;
mod error;
mod windows;
mod delegate;
mod aes;
mod hash;
mod gen;

use druid::{AppLauncher, PlatformError};
use crate::delegate::Delegate;
use crate::state::app_state::AppState;

fn main() -> Result<(), PlatformError> {
    let state = match AppState::load() {
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    };

    AppLauncher::with_window(windows::main_window::new())
        .delegate(Delegate::new())
        .launch(state)?;

    Ok(())
}