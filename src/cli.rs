use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum Action {
    Chat(ChatArgs),
    Wait(WaitArgs),
}

#[derive(Debug, clap::Args, Serialize, Deserialize)]
pub struct ChatArgs {
    pub message: String,
}

#[derive(Debug, clap::Args, Serialize, Deserialize)]
pub struct WaitArgs {
    #[clap(subcommand)]
    pub event: WaitEvent,
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum WaitEvent {
    MapLoaded,
}
