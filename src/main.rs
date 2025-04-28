mod api;
mod game;
mod celestia;
mod avail;
mod constants;
mod cli;
mod commands;

use clap::Parser;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { da } => {
            commands::start::run(da).await;
        }
        Commands::Address { da } => {
            commands::address::run(da).await;
        }
    }
}
