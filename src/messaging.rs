use std::{env::temp_dir, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::cli::Action;

#[must_use]
pub fn get_socket_path() -> PathBuf {
    temp_dir().join(concat!(env!("CARGO_PKG_NAME"), "-socket"))
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestMessage {
    Command(Action),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseMessage {
    None,
    // TODO send errors?
}
