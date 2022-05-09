use crate::lifting::{Day, Lift, LiftAttempt, LiftAttemptResult};
use crate::programs::Program;
use crate::services::ports::PersistenceAdapter;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::env::current_dir;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};

const HISTORY_SAVE_FILE_NAME: &'static str = "lift_history.txt";
const INFO_SAVE_FILE_NAME: &'static str = "info.txt";
const PROGRAM_SAVE_FILE_NAME: &'static str = "program.json";
const SAVE_DIRECTORY_NAME: &'static str = "yawa_save_data";

pub struct FileSystem {
    save_dir: PathBuf,
}

pub fn new() -> Result<FileSystem> {
    Ok(FileSystem {
        save_dir: {
            let mut dir = current_dir()?;
            dir.push(&Path::new(SAVE_DIRECTORY_NAME));
            dir
        },
    })
}

#[derive(Serialize, Deserialize)]
struct SerializableProgram {
    name: String,
    workouts_completed: usize,
    reference_weight: usize,
    starting_reference_weight: usize,
    days_in_notation: Vec<String>,
    weights: HashMap<String, usize>,
    current_day: usize,
    past_attempt_results_in_notation: Vec<Vec<String>>,
}

impl SerializableProgram {
    fn from(program: &Program) -> Self {
        let mut weights = HashMap::new();
        program.weights.iter().for_each(|it| {
            weights.insert(it.0.to_string(), *it.1);
        });
        SerializableProgram {
            name: program.name.clone(),
            workouts_completed: program.workouts_completed,
            reference_weight: program.reference_weight,
            starting_reference_weight: program.starting_reference_weight,
            days_in_notation: program.days.iter().map(|it| format!("{it}")).collect(),
            weights,
            current_day: program.current_day,
            past_attempt_results_in_notation: program
                .current_cycle_attempt_results
                .iter()
                .map(|day| day.iter().map(|it| it.to_string()).collect())
                .collect(),
        }
    }
    fn parse(program_string: &String) -> Result<SerializableProgram> {
        Ok(from_str(&program_string)?)
    }
}

impl Program {
    fn from(serializable_program: &SerializableProgram) -> Result<Self> {
        let days = Self::read_days(&serializable_program)?;
        let weights = Self::read_weights(&serializable_program)?;
        let past_attempts = Self::read_past_attempts(&serializable_program)?;
        Ok(Self {
            days,
            weights,
            reference_weight: serializable_program.reference_weight,
            starting_reference_weight: serializable_program.starting_reference_weight,
            workouts_completed: serializable_program.workouts_completed,
            name: serializable_program.name.clone(),
            current_day: serializable_program.current_day,
            current_cycle_attempt_results: past_attempts,
        })
    }

    fn read_past_attempts(
        serializable_program: &SerializableProgram,
    ) -> Result<Vec<Vec<LiftAttemptResult>>> {
        let mut past_attempts: Vec<Vec<LiftAttemptResult>> = Vec::new();
        serializable_program
            .past_attempt_results_in_notation
            .iter()
            .try_for_each(|day| {
                let mut attempts: Vec<LiftAttemptResult> = Vec::new();
                day.iter().try_for_each(|attempt_notation| {
                    anyhow::Ok(attempts.push(LiftAttemptResult::parse(attempt_notation)?))
                })?;
                past_attempts.push(attempts);
                anyhow::Ok(())
            })?;
        Ok(past_attempts)
    }

    fn read_weights(serializable_program: &SerializableProgram) -> Result<HashMap<Lift, usize>> {
        let mut weights = HashMap::new();
        serializable_program.weights.iter().try_for_each(|it| {
            weights.insert(Lift::parse(it.0)?, *it.1);
            anyhow::Ok(())
        })?;
        Ok(weights)
    }

    fn read_days(serializable_program: &SerializableProgram) -> Result<Vec<Day>> {
        let mut days = Vec::new();
        serializable_program
            .days_in_notation
            .iter()
            .try_for_each(|it| anyhow::Ok(days.push(Day::parse(it)?)))?;
        Ok(days)
    }
}

impl Display for SerializableProgram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", to_string_pretty(&self).unwrap())
    }
}

impl PersistenceAdapter for FileSystem {
    fn set_save_dir(self, dir: &Path) -> Self {
        FileSystem {
            save_dir: {
                let mut new_dir = dir.to_path_buf().clone();
                new_dir.push(&Path::new(SAVE_DIRECTORY_NAME));
                new_dir
            },
        }
    }

    fn persist(&self, program: &Program) -> Result<()> {
        write_string_to_file(
            &self.save_dir.display().to_string(),
            PROGRAM_SAVE_FILE_NAME,
            &format!("{}", SerializableProgram::from(program)),
        )?;
        self.save_info_file()?;
        Ok(())
    }

    fn save_history(&self, attempt: &LiftAttempt, result: &LiftAttemptResult) -> Result<()> {
        append_string_to_file(
            &self.save_dir.display().to_string(),
            HISTORY_SAVE_FILE_NAME,
            &format!("{}: {} | {}\n", chrono::Utc::now(), attempt, result),
        )
    }

    fn summon(&self) -> Result<Program> {
        let program_string =
            read_file_to_string(&self.save_dir.display().to_string(), PROGRAM_SAVE_FILE_NAME)?;
        let serializable_program: SerializableProgram =
            SerializableProgram::parse(&program_string)?;
        Ok(Program::from(&serializable_program)?)
    }
}

impl FileSystem {
    fn save_info_file(&self) -> Result<()> {
        write_string_to_file(
            &self.save_dir.display().to_string(),
            INFO_SAVE_FILE_NAME,
            &format!(
                "Data in this folder was saved by the program yawa, version {}. See: https://github.com/yogeshdhamija/yawa",
                option_env!("CARGO_PKG_VERSION")
                    .or(Some("UNKNOWN"))
                    .unwrap()
            ),
        )?;
        Ok(())
    }
}

fn append_string_to_file(directory: &str, file_name: &str, string: &str) -> Result<()> {
    create_dir_all(directory)?;
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(format!("{}/{}", directory, file_name))?;
    write!(file, "{}", string)?;
    Ok(())
}

fn write_string_to_file(directory: &str, file_name: &str, string: &str) -> Result<()> {
    create_dir_all(directory)?;
    let mut file = File::create(format!("{}/{}", directory, file_name))?;
    write!(file, "{}", string)?;
    Ok(())
}

fn read_file_to_string(directory: &str, file_name: &str) -> Result<String> {
    let mut file = File::open(format!("{}/{}", directory, file_name))?;
    let mut program_string = String::new();
    file.read_to_string(&mut program_string)?;
    Ok(program_string)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::programs::start_gzcl_4day;

    #[test]
    fn can_create_and_save_program() {
        let program = start_gzcl_4day(100);
        let string: String = SerializableProgram::from(&program).to_string();
        let after_round_trip =
            Program::from(&SerializableProgram::parse(&string).unwrap()).unwrap();
        assert_eq!(after_round_trip, program);
    }
}
