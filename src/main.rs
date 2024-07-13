use std::os::unix::net::UnixStream;

use anyhow::Result;
use clap::Parser;
use classicube_command_control_plugin::{
    cli::Cli,
    logger,
    messaging::{get_socket_path, RequestMessage, ResponseMessage},
    traits::{UnixStreamRecvMessageTrait, UnixStreamSendMessageTrait},
};
use tracing::debug;

fn main() -> Result<()> {
    let cli = Cli::parse();

    logger::initialize(true, false, module_path!());

    let socket_path = get_socket_path();
    let mut socket = UnixStream::connect(socket_path)?;
    socket.send_message(&RequestMessage::Command(cli.action))?;

    let response: ResponseMessage = socket.recv_message()?;
    debug!("{:?}", response);

    Ok(())
}
