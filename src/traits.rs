use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

use crate::ipc::IpcConnection;

pub trait IpcStreamSendMessageTrait<T: ?Sized> {
    fn send_message(&mut self, message: &T) -> Result<()>;
}

impl<T> IpcStreamSendMessageTrait<T> for IpcConnection
where
    T: ?Sized + Serialize,
{
    fn send_message(&mut self, message: &T) -> Result<()> {
        bincode::serialize_into(self, message)?;

        Ok(())
    }
}

pub trait IpcStreamRecvMessageTrait<T> {
    fn recv_message(&mut self) -> Result<T>;
}

impl<T> IpcStreamRecvMessageTrait<T> for IpcConnection
where
    T: DeserializeOwned,
{
    fn recv_message(&mut self) -> Result<T> {
        let message: T = bincode::deserialize_from(self)?;
        Ok(message)
    }
}
