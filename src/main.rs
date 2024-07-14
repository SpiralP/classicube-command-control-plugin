use anyhow::Result;
use clap::Parser;
use classicube_command_control_plugin::{
    cli::Cli,
    ipc::IpcConnection,
    logger,
    messaging::{RequestMessage, ResponseMessage},
    traits::{IpcStreamRecvMessageTrait, IpcStreamSendMessageTrait},
};
use tracing::debug;

fn main() -> Result<()> {
    let cli = Cli::parse();

    logger::initialize(true, false, module_path!());

    let mut client = IpcConnection::connect()?;
    client.send_message(&RequestMessage::Command(cli.action))?;

    let response: ResponseMessage = client.recv_message()?;
    debug!("{:?}", response);

    Ok(())
}
