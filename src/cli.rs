mod term;
mod social_event;
mod stat;
mod data;
mod region;
mod indicator;

use anyhow::Result;
use clap::{Parser, Subcommand};
use crate::cli::data::DataArgs;
use crate::cli::indicator::IndicatorArgs;
use crate::cli::region::RegionArgs;
use crate::cli::social_event::SocialEventArgs;
use crate::cli::stat::StatArgs;
use crate::cli::term::TermArgs;

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
    Region(RegionArgs),
    Indicator(IndicatorArgs),
    Term(TermArgs),
    SocialEvent(SocialEventArgs),
    Stat(StatArgs),
    Data(DataArgs),
}

pub async fn start() -> Result<()> {
    let command = Cli::parse();

    match command.command {
        Command::Indicator(args) => {
            indicator::handle(args).await
        }
        Command::Region(args) => {
            region::handle(args).await
        }
        Command::Term(args) => {
            term::handle(args).await
        }
        Command::SocialEvent(args) => {
            social_event::handle(args).await
        }
        Command::Stat(args) => {
            stat::handle(args).await
        }
        Command::Data(args) => {
            data::handle(args).await
        }
    }
}