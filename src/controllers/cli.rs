use crate::services::ports::{PersistenceAdapter, UserInputAdapter};
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

    /// Display current status of your lifting lifting.
    Status {},

    /// Show the next workout in your program.
    Next {},

    /// Complete the next workout of your program! (Run 'next' to see it first!)
    Complete {},
}

pub fn start_program_with_args(
    persistence_adapter: &impl PersistenceAdapter,
    tui_adapter: &impl UserInputAdapter,
) -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Commands::Status {} => {
            let program = service::status(persistence_adapter)?;
            println!(
                "Current program: {}\nCurrent reference weight: {}",
                program.name, program.reference_weight
            );
        }
        Commands::Start { reference_weight } => {
            let program = service::new_program(persistence_adapter, *reference_weight)?;
            println!("Started program: {}", program.name);
        }
        Commands::Next {} => {
            let res = service::next_show(persistence_adapter)?;
            let lifts = res
                .1
                .iter()
                .map(|lift| format!("{lift}"))
                .collect::<Vec<String>>()
                .join("\n");
            let string = format!("=== Day: {} ===\n{}", res.0, lifts);
            println!("{}", string);
        }
        Commands::Complete {} => {
            service::complete(persistence_adapter, tui_adapter)?;
            println!("Well done!");
        }
    }

    Ok(())
}
