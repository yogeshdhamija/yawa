use crate::lifting::{Day, Lift, LiftAttemptResult};
use crate::programs::Program;
use crate::services::ports::PersistenceAdapter;
use anyhow::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use serde_json::to_string_pretty;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::create_dir_all;
use std::fs::File;
use std::io::Read;
use std::io::Write;

pub struct FileSystem {}
pub fn new() -> FileSystem {
    FileSystem {}
}

#[derive(Serialize, Deserialize)]
struct SerializableProgram {
    name: String,
    reference_weight: usize,
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
            reference_weight: program.reference_weight,
            days_in_notation: program.days.iter().map(|it| format!("{it}")).collect(),
            weights,
            current_day: program.current_day,
            past_attempt_results_in_notation: program
                .past_attempt_results
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
            name: serializable_program.name.clone(),
            current_day: serializable_program.current_day,
            past_attempt_results: past_attempts,
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
    fn persist(&self, program: &Program) -> Result<()> {
        create_dir_all("/tmp/yawa")?;
        let mut file = File::create("/tmp/yawa/saved.json")?;
        write!(
            file,
            "{}",
            format!("{}", SerializableProgram::from(program))
        )?;
        Ok(())
    }
    fn summon(&self) -> Result<Program> {
        let program_string = read_file_to_string("/tmp/yawa/saved.json")?;
        let serializable_program: SerializableProgram =
            SerializableProgram::parse(&program_string)?;
        Ok(Program::from(&serializable_program)?)
    }
}

fn read_file_to_string(path: &str) -> Result<String> {
    let mut file = File::open(path)?;
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
        let serializable_program: SerializableProgram = SerializableProgram::from(&program);
        let string: String = serializable_program.to_string();
        let after_round_trip =
            Program::from(&SerializableProgram::parse(&string).unwrap()).unwrap();
        assert_eq!(after_round_trip, program);
    }
}
