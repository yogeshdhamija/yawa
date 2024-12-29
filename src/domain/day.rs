use std::fmt::{Display, Formatter};
use anyhow::anyhow;
use crate::domain::lifting::Lift;

#[derive(Clone, Debug, PartialEq)]
pub struct Day {
    pub name: String,
    pub lifts: Vec<Lift>,
}

impl Day {
    /// ```
    /// # use yawa::domain::day::Day;
    /// Day::parse("Day Name | Bench press -> 3x5,1x5-6,1x6+ @ 2r | Pullup -> 3x5,1x5-6,1x6+").unwrap();
    /// Day::parse("Something else").is_err();
    /// ```
    pub fn parse(notation: &str) -> anyhow::Result<Day> {
        let error = "Cannot parse notation.";
        let mut lines = notation.split(" | ");
        let name = lines.next().ok_or(anyhow!(error))?.trim().to_string();
        let mut lifts = Vec::new();
        lines.try_for_each(|line| {
            return anyhow::Ok(if line.trim() != "" {
                lifts.push(Lift::parse(line)?)
            });
        })?;
        Ok(Day { name, lifts })
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lifts = self
            .lifts
            .iter()
            .map(|lift| format!("{lift}"))
            .collect::<Vec<String>>()
            .join(" | ");
        write!(f, "{} | {}", self.name, lifts)
    }
}