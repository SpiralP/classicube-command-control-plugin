use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    Server(ServerArgs),
    Client(ClientArgs),
}

#[derive(Debug, clap::Args)]
pub struct ServerArgs {}

#[derive(Debug, clap::Args)]
pub struct ClientArgs {}
