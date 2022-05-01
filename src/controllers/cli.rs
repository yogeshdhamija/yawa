use crate::controllers::ports::PersistanceAdapter;
use anyhow::{anyhow, Result};
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
}

pub fn start_program_with_args(persistance_adapter: &impl PersistanceAdapter) -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Commands::Status {} => {
            if let Some(reference_weight) = persistance_adapter.summon() {
                println!("Current reference weight: {reference_weight}");
            } else {
                return Err(anyhow!("No status. Start a program first!"));
            }
        }
        Commands::Start { reference_weight } => {
            persistance_adapter.persist(*reference_weight)?;
        }
    }

    Ok(())
}
