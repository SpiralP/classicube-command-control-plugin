use std::{
    env::temp_dir,
    fs,
    io::{self, Read, Write},
    os::unix::net::{Incoming, UnixListener, UnixStream},
    path::PathBuf,
};

use anyhow::Result;
use tracing::{debug, warn};

#[must_use]
fn get_socket_path() -> PathBuf {
    temp_dir().join(concat!(env!("CARGO_PKG_NAME"), "-socket"))
}

pub struct IpcServer {
    listener: UnixListener,
}

impl IpcServer {
    pub fn new() -> Result<Self> {
        let socket_path = get_socket_path();

        if socket_path.exists() {
            warn!("socket_path already exists, replacing");
            fs::remove_file(&socket_path)?;
        }

        debug!(?socket_path, "binding to unix socket");
        let listener = UnixListener::bind(socket_path)?;

        Ok(Self { listener })
    }

    #[must_use]
    pub fn incoming(&self) -> IpcConnectionIncoming {
        let incoming = self.listener.incoming();
        IpcConnectionIncoming { incoming }
    }
}

pub struct IpcConnectionIncoming<'a> {
    incoming: Incoming<'a>,
}

impl Iterator for IpcConnectionIncoming<'_> {
    type Item = io::Result<IpcConnection>;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.incoming.next()?;
        Some(result.map(|stream| IpcConnection { stream }))
    }
}

pub struct IpcConnection {
    stream: UnixStream,
}

impl IpcConnection {
    pub fn connect() -> Result<Self> {
        let socket_path = get_socket_path();
        let stream = UnixStream::connect(socket_path)?;
        Ok(Self { stream })
    }
}

impl Write for IpcConnection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}

impl Read for IpcConnection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}
