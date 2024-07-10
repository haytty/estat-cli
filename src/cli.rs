mod term;
mod social_event;
mod stat;
mod data;
mod region;
mod indicator;

use anyhow::Result;
use clap::{Parser, Subcommand};
use crate::cli::indicator::IndicatorArgs;

#[derive(Parser)]
#[clap(
    name = "estat-cli",
    version = "1.0",
    author = "haytty",
    about = "e-stat cli tool")
]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Indicator(IndicatorArgs),
}

pub async fn start() -> Result<()> {
    let command = Cli::parse();

    match command.command {
        Command::Indicator(args) => {
            indicator::handle(args).await
        }
    }
}