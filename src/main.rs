pub mod cli;

use std::{
    env::temp_dir,
    fs,
    os::unix::net::{UnixListener, UnixStream},
    thread,
};

use anyhow::Result;
use clap::Parser;
use classicube_command_control_plugin::{
    logger,
    traits::{UnixStreamRecvMessageTrait, UnixStreamSendMessageTrait},
};
use serde::{Deserialize, Serialize};
use tracing::{debug, warn};

use self::cli::{Action, Cli};

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestMessage {
    Thing,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseMessage {
    Thing,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    logger::initialize(true, false, module_path!());

    let socket_path = temp_dir().join(concat!(env!("CARGO_PKG_NAME"), "-socket"));

    match cli.action {
        Action::Server(args) => {
            if socket_path.exists() {
                warn!("socket_path already exists, replacing");
                fs::remove_file(&socket_path)?;
            }

            let socket = UnixListener::bind(socket_path)?;

            for result in socket.incoming() {
                let stream = result?;

                thread::spawn(|| {
                    debug!("new client");
                    if let Err(e) = handle_client(stream) {
                        warn!("handle_client: {e:?}");
                    }
                });
            }
        }

        Action::Client(args) => {
            let mut socket = UnixStream::connect(socket_path)?;
            socket.send_message(&RequestMessage::Thing)?;

            let response: ResponseMessage = socket.recv_message()?;
            debug!("{:?}", response);
        }
    }

    Ok(())
}

fn handle_client(mut stream: UnixStream) -> Result<()> {
    let message: RequestMessage = stream.recv_message()?;
    debug!("{message:?}");

    stream.send_message(&ResponseMessage::Thing)?;

    Ok(())
}
