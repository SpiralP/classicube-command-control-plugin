use std::thread;

use anyhow::Result;
use tracing::{debug, warn};

use crate::{
    ipc::{IpcConnection, IpcServer},
    messaging::{RequestMessage, ResponseMessage},
    plugin::commands::queue_cli_action,
    traits::{IpcStreamRecvMessageTrait, IpcStreamSendMessageTrait},
};

pub fn start() -> Result<()> {
    let ipc = IpcServer::new()?;

    thread::spawn(move || {
        for result in ipc.incoming() {
            let stream = result.unwrap();

            thread::spawn(move || {
                debug!("new client");
                if let Err(e) = handle_client(stream) {
                    warn!("handle_client: {e:?}");
                }
            });
        }
    });

    Ok(())
}

fn handle_client(mut stream: IpcConnection) -> Result<()> {
    let message: RequestMessage = stream.recv_message()?;
    debug!("{message:?}");

    match message {
        RequestMessage::Command(action) => queue_cli_action(action)?,
    }

    stream.send_message(&ResponseMessage::None)?;

    Ok(())
}
