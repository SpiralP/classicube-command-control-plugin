use std::{
    io::{self, Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use anyhow::Result;
use tracing::debug;

#[must_use]
fn get_socket_addr() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], 51234))
}

pub struct IpcServer {
    listener: TcpListener,
}

impl IpcServer {
    pub fn new() -> Result<Self> {
        let socket_addr = get_socket_addr();
        debug!(?socket_addr, "binding to tcp socket");
        let listener = TcpListener::bind(socket_addr)?;

        Ok(Self { listener })
    }

    #[must_use]
    pub fn incoming(&self) -> IpcConnectionIncoming {
        IpcConnectionIncoming {
            listener: &self.listener,
        }
    }
}

pub struct IpcConnectionIncoming<'a> {
    listener: &'a TcpListener,
}

impl<'a> Iterator for IpcConnectionIncoming<'a> {
    type Item = io::Result<IpcConnection>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(
            self.listener
                .accept()
                .map(|(stream, _)| IpcConnection { stream }),
        )
    }
}

pub struct IpcConnection {
    stream: TcpStream,
}

impl IpcConnection {
    pub fn connect() -> Result<Self> {
        let stream = TcpStream::connect(get_socket_addr())?;
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
