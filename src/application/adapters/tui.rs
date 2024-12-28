use crate::domain::lifting::{LiftAttempt, LiftAttemptResult, Set};
use crate::application::services::ports::UserInputAdapter;
use crate::domain::user_input::Action;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::io;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,

    /// The directory used by yawa to save data. Default: current directory.
    #[clap(short, long)]
    save_directory: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Start a new weightlifting routine! Let's GOOOoOoOo!!!!!1
    Start {
        /// The reference weight to start with (in lbs). 45 is a good number to start with if it's
        /// your first time.
        #[clap(short)]
        reference_weight: usize,
    },

    /// Display current status of your lifting lifting.
    Status {},

    /// Show the next workout in your program.
    Next {},

    /// Complete the next workout of your program! (Run 'next' to see it first!)
    Complete {},
}

pub struct Tui {}
pub fn new() -> Tui {
    Tui {}
}

impl UserInputAdapter for Tui {
    fn check_complete(&self, attempts: &[LiftAttempt]) -> Result<Vec<LiftAttemptResult>> {
        attempts.iter().map(ask_user_for_attempt_result).collect()
    }

    fn ask_what_to_do(&self) -> Result<(Action, Option<PathBuf>)> {
        let args = Args::parse();
        Ok((
            match args.command {
                Commands::Start { reference_weight } => Action::StartProgram { reference_weight: reference_weight },
                Commands::Status {} => Action::SeeStatus,
                Commands::Next {} => Action::SeeNextDay,
                Commands::Complete {} => Action::CompleteDay,
            },
            args.save_directory,
        ))
    }
}

fn ask_user_for_attempt_result(attempt: &LiftAttempt) -> Result<LiftAttemptResult, anyhow::Error> {
    if did_complete_lift(attempt)? {
        return Ok(LiftAttemptResult::Completed {
            completed_maximum_reps: did_complete_maximum_reps(attempt)?,
        });
    } else {
        return Ok(LiftAttemptResult::NotCompleted);
    }
}

fn did_complete_maximum_reps(attempt: &LiftAttempt) -> Result<bool, anyhow::Error> {
    if attempt.has_rep_range() {
        get_user_confirmation(&"        ... were you able to achieve the maximum rep range?")
    } else {
        Ok(true)
    }
}

fn did_complete_lift(attempt: &LiftAttempt) -> Result<bool, anyhow::Error> {
    get_user_confirmation(&format!("Did you complete: {}?", attempt))
}

fn get_user_confirmation(prompt: &str) -> Result<bool> {
    loop {
        print_with_yes_or_no(prompt)?;
        let string = read_string_from_stdin()?;
        if string.trim() == "y" {
            return Ok(true);
        } else if string.trim() == "n" {
            return Ok(false);
        }
    }
}

fn print_with_yes_or_no(prompt: &str) -> Result<(), anyhow::Error> {
    print!("{} [y/n] ", prompt);
    io::stdout().flush()?;
    Ok(())
}

fn read_string_from_stdin() -> Result<String, std::io::Error> {
    let mut string = String::new();
    io::stdin().read_line(&mut string)?;
    return Ok(string);
}

impl LiftAttempt {
    fn has_rep_range(&self) -> bool {
        self.lift
            .sets
            .iter()
            .any(|it| matches!(it, Set::Range { .. }))
    }
}
