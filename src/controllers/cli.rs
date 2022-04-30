use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version,about)]
struct Args {
}

pub fn parse_arguments_and_handle_help_and_version() {
    Args::parse();
}
