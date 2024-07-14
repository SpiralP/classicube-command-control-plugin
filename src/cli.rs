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

#[derive(Debug, PartialEq, Eq, Hash, Subcommand, Serialize, Deserialize)]
pub enum WaitEvent {
    /// fired on `reset`
    Reset,
    /// fired on `on_new_map`
    MapLoading,
    /// fired on `on_new_map_loaded`
    MapLoaded,

    /// fired once a second, when loaded into a map and 0 chunks are loading
    ChunksLoaded,
    /// fired once a second, when loaded into a map
    Init,
}
