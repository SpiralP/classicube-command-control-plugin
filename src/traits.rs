use std::os::unix::net::UnixStream;

use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};

pub trait UnixStreamSendMessageTrait<T: ?Sized> {
    fn send_message(&mut self, message: &T) -> Result<()>;
}

impl<T> UnixStreamSendMessageTrait<T> for UnixStream
where
    T: ?Sized + Serialize,
{
    fn send_message(&mut self, message: &T) -> Result<()> {
        bincode::serialize_into(self, message)?;

        Ok(())
    }
}

pub trait UnixStreamRecvMessageTrait<T> {
    fn recv_message(&mut self) -> Result<T>;
}

impl<T> UnixStreamRecvMessageTrait<T> for UnixStream
where
    T: DeserializeOwned,
{
    fn recv_message(&mut self) -> Result<T> {
        let message: T = bincode::deserialize_from(self)?;
        Ok(message)
    }
}
