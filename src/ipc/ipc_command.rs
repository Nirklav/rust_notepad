use druid::Target;
use serde::{Serialize, Deserialize};
use crate::commands;
use crate::ipc::IpcServices;

#[derive(Serialize, Deserialize, Debug)]
pub enum IpcCommand {
    ShowWindow
}

impl IpcCommand {
    pub fn execute(&self, services: &IpcServices) {
        dbg!(&self);

        match self {
            IpcCommand::ShowWindow => IpcCommand::show_window(services)
        }
    }

    fn show_window(services: &IpcServices) {
        if let Err(e) = services.sink.submit_command(commands::SHOW_MAIN_WINDOW, (), Target::Auto) {
            println!("Cannot submit command:{}", e)
        }
    }
}