use crate::lifting::{Day, Lift};
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
struct State {
    name: String,
    reference_weight: u64,
    days_in_notation: Vec<String>,
    weights: HashMap<String, u64>,
    current_day: u64,
    past_attempt_results_in_notation: Vec<Vec<String>>,
}

impl Program {
    fn parse(notation: &str) -> Result<Self> {
        let state: State = from_str(notation)?;
        let mut days = Vec::new();
        state
            .days_in_notation
            .iter()
            .try_for_each(|it| anyhow::Ok(days.push(Day::parse(it)?)))?;
        let mut weights = HashMap::new();
        state.weights.iter().try_for_each(|it| {
            weights.insert(Lift::parse(it.0)?, *it.1);
            anyhow::Ok(())
        })?;
        Ok(Program {
            days,
            weights,
            reference_weight: state.reference_weight,
            name: state.name,
            current_day: state.current_day,
            past_attempt_results: vec![],
        })
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut weights = HashMap::new();
        self.weights.iter().for_each(|it| {
            weights.insert(it.0.to_string(), *it.1);
        });
        write!(
            f,
            "{}",
            to_string_pretty(&State {
                name: self.name.clone(),
                reference_weight: self.reference_weight,
                days_in_notation: self.days.iter().map(|it| format!("{it}")).collect(),
                weights,
                current_day: self.current_day,
                past_attempt_results_in_notation: vec![]
            })
            .unwrap()
        )
    }
}

impl PersistenceAdapter for FileSystem {
    fn persist(&self, program: &Program) -> Result<()> {
        create_dir_all("/tmp/yawa")?;
        let mut file = File::create("/tmp/yawa/saved.json")?;
        let program_string = format!("{program}");
        write!(file, "{program_string}")?;
        Ok(())
    }
    fn summon(&self) -> Result<Program> {
        let mut file = File::open("/tmp/yawa/saved.json")?;
        let mut program_string = String::new();
        file.read_to_string(&mut program_string)?;
        Ok(Program::parse(&program_string)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::programs::start_gzcl_4day;

    #[test]
    fn can_create_and_save_program() {
        let program = start_gzcl_4day(100);
        let string = format!("{}", program);
        let after_round_trip = Program::parse(&string).unwrap();
        assert_eq!(after_round_trip, program);
    }
}
