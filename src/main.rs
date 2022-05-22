#![windows_subsystem = "windows"]

extern crate core;

mod state;
mod ui;
mod error;
mod windows;
mod delegate;
mod aes;
mod hash;
mod gen;
mod backups;
mod commands;
mod secure;

use std::env;
use std::path::PathBuf;
use druid::{AppLauncher, PlatformError};
use named_lock::NamedLock;
use crate::backups::google_drive::GoogleDrive;
use crate::delegate::Delegate;
use crate::error::AppError;
use crate::state::app_state::AppState;

fn main() -> Result<(), PlatformError> {
    let named_lock = convert(NamedLock::create("notepad"))?;
    let _guard = convert(named_lock.lock())?;

    let state = match AppState::load() {
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    };

    AppLauncher::with_window(windows::main_window::new())
        .delegate(Delegate::new())
        .launch(state)?;

    Ok(())
}

fn convert<T>(r: named_lock::Result<T>) -> Result<T, PlatformError> {
    match r {
        Ok(v) => Ok(v),
        Err(_) => Err(PlatformError::ApplicationDropped)
    }
}

pub fn exe_directory() -> Result<PathBuf, AppError> {
    let mut executable = env::current_exe()?;
    executable.pop();
    Ok(executable)
}