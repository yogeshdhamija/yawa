use crate::services::ports::PersistanceAdapter;
use crate::services::service;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start a new weightlifting routine! Let's GOOOoOoOo!!!!!1
    Start {
        /// The reference weight to start with (in lbs). 45 is a good number to start with if it's
        /// your first time.
        #[clap(short)]
        reference_weight: u64,
    },

    /// Display current status of your lifting program.
    Status {},

    /// About your next workout.
    Next {
        #[clap(subcommand)]
        command: NextCommands,
    },
}

#[derive(Debug, Subcommand)]
enum NextCommands {
    /// Show the next workout you need to do
    Show {},
}

pub fn start_program_with_args(persistance_adapter: &impl PersistanceAdapter) -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Commands::Status {} => {
            println!("{}", service::status(persistance_adapter)?);
        }
        Commands::Start { reference_weight } => {
            persistance_adapter.persist(*reference_weight)?;
        }
        Commands::Next {
            command: _next_subcommand,
        } => {
            println!("{}", service::next_show(persistance_adapter)?);
        }
    }

    Ok(())
}
