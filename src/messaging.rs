use serde::{Deserialize, Serialize};

use crate::cli::Action;

#[derive(Debug, Serialize, Deserialize)]
pub enum RequestMessage {
    Command(Action),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ResponseMessage {
    None,
    // TODO send errors?
}
