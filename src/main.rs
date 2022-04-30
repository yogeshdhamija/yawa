mod controllers;

fn main() {
    controllers::cli::parse_arguments_and_handle_help_and_version();
}
