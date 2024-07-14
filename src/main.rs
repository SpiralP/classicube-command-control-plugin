use std::{thread, time::Duration};

use anyhow::{bail, Result};
use clap::Parser;
use classicube_command_control_plugin::{
    cli::Cli,
    ipc::IpcConnection,
    logger,
    messaging::{RequestMessage, ResponseMessage},
    traits::{IpcStreamRecvMessageTrait, IpcStreamSendMessageTrait},
};
use tracing::{debug, info, warn};

const RECONNECT_INTERVAL: Duration = Duration::from_secs(5);
const RECONNECT_FAIL_DURATION: Duration = Duration::from_secs(2 * 60);

fn main() -> Result<()> {
    let cli = Cli::parse();

    logger::initialize(true, false, module_path!());

    let mut failures = 0;
    loop {
        match IpcConnection::connect() {
            Ok(connection) => {
                return connected(connection, cli);
            }
            Err(e) => {
                warn!("error connecting: {e:?}");
                failures += 1;
            }
        }

        let failure_time = failures * RECONNECT_INTERVAL;
        if failure_time >= RECONNECT_FAIL_DURATION {
            bail!("too many connection failures");
        }

        info!("trying to reconnect in {RECONNECT_INTERVAL:?} (failed {failures:?} times)");
        thread::sleep(RECONNECT_INTERVAL);
    }
}

fn connected(mut client: IpcConnection, cli: Cli) -> Result<()> {
    client.send_message(&RequestMessage::Command(cli.action))?;

    let response: ResponseMessage = client.recv_message()?;
    debug!("{:?}", response);

    Ok(())
}
