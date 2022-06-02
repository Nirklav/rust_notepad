pub mod ipc_command;

use std::io::{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use druid::ExtEventSink;
use crate::AppError;
use crate::ipc::ipc_command::IpcCommand;

const IPC_ADDRESS : &str = "127.0.0.1:12863";

pub struct Ipc {
    handle: Option<JoinHandle<()>>,
    stop: Arc<AtomicBool>
}

pub struct IpcServices {
    pub sink: ExtEventSink,
    pub stop: Arc<AtomicBool>
}

impl Ipc {
    pub fn start(sink: ExtEventSink) -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let services = IpcServices {
            sink,
            stop: stop.clone()
        };

        let handle = thread::spawn(move || {
            if let Err(e) = Ipc::receive_loop(services) {
                println!("Error on receiving ipc command: {}", e);
            };
        });

        Ipc {
            handle: Some(handle),
            stop: stop.clone()
        }
    }

    fn receive_loop(services: IpcServices) -> Result<(), AppError> {
        let listener = TcpListener::bind(IPC_ADDRESS)?;
        listener.set_nonblocking(true)?;

        loop {
            if services.stop.load(Ordering::SeqCst) {
                break;
            }

            let command = match Ipc::receive(&listener)? {
                Some(c) => c,
                None => {
                    thread::sleep(Duration::from_millis(200));
                    continue;
                }
            };

            command.execute(&services);
        }

        Ok(())
    }

    pub fn send(command: IpcCommand) -> Result<(), AppError> {
        let mut stream = TcpStream::connect(IPC_ADDRESS)?;
        let buf = serde_json::to_vec(&command)?;

        stream.write_u32::<LittleEndian>(buf.len() as u32)?;
        stream.write(&buf)?;

        Ok(())
    }

    fn receive(listener: &TcpListener) -> Result<Option<IpcCommand>, AppError> {
        let mut stream = match listener.accept() {
            Ok((s, _)) => s,
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => return Ok(None),
            Err(e) => return Err(From::from(e))
        };

        let size = stream.read_u32::<LittleEndian>()? as usize;
        let mut buf = vec![0u8; size];
        stream.read_exact(&mut buf)?;

        Ok(serde_json::from_slice(&buf)?)
    }
}

impl Drop for Ipc {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);

        if let Some(handle) = self.handle.take() {
            handle.join().expect("Cannot join thread.")
        }
    }
}