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
mod ipc;

use std::env;
use std::path::PathBuf;
use druid::{AppLauncher, PlatformError};
use named_lock::NamedLock;
use crate::backups::google_drive::GoogleDrive;
use crate::delegate::Delegate;
use crate::error::AppError;
use crate::ipc::Ipc;
use crate::ipc::ipc_command::IpcCommand;
use crate::state::app_state::AppState;

fn main() -> Result<(), PlatformError> {
    let named_lock = convert(NamedLock::create("notepad"))?;

    let _guard = match named_lock.try_lock() {
        Ok(g) => g,
        Err(_) => {
            Ipc::send(IpcCommand::ShowWindow).unwrap();
            return Ok(())
        }
    };

    let state = match AppState::load() {
        Ok(s) => s,
        Err(e) => panic!("{}", e)
    };

    let launcher = AppLauncher::with_window(windows::main_window::new());
    let ipc = Ipc::start(launcher.get_external_handle());

    launcher
        .delegate(Delegate::new())
        .launch(state)?;

    drop(ipc);
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