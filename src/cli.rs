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
}

#[derive(Debug, clap::Args, Serialize, Deserialize)]
pub struct ChatArgs {
    pub message: String,
}
