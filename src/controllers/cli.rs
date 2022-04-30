use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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

pub fn start_program_with_args() -> Result<()> {
    let args = Args::parse();
    match &args.command {
        Commands::Status {} => {
            if !Path::new("/tmp/yawa/saved.json").is_file() {
                return Err(anyhow!("No status. Start a program first!"));
            }
        }
        Commands::Start {
            reference_weight: _,
        } => {
            create_dir_all("/tmp/yawa")?;
            let mut file = File::create("/tmp/yawa/saved.json")?;
            file.write_all(b"Hello, world!")?;
        }
    }

    Ok(())
}
