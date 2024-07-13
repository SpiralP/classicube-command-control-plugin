use std::{
    fs,
    os::unix::net::{UnixListener, UnixStream},
    thread,
};

use anyhow::Result;
use tracing::{debug, warn};

use crate::{
    messaging::{get_socket_path, RequestMessage, ResponseMessage},
    plugin::commands::queue_cli_action,
    traits::{UnixStreamRecvMessageTrait, UnixStreamSendMessageTrait},
};

pub fn start() -> Result<()> {
    let socket_path = get_socket_path();

    if socket_path.exists() {
        warn!("socket_path already exists, replacing");
        fs::remove_file(&socket_path)?;
    }

    debug!(?socket_path, "binding to unix socket");
    let socket = UnixListener::bind(socket_path)?;

    thread::spawn(move || {
        for result in socket.incoming() {
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

fn handle_client(mut stream: UnixStream) -> Result<()> {
    let message: RequestMessage = stream.recv_message()?;
    debug!("{message:?}");

    match message {
        RequestMessage::Command(action) => queue_cli_action(action)?,
    }

    stream.send_message(&ResponseMessage::None)?;

    Ok(())
}
