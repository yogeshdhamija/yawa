use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    start: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start,
}

pub fn parse_arguments_and_handle_help_and_version() {
    Args::parse();
}
