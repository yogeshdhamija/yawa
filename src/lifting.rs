use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub enum WeightScheme {
    BasedOnReference { multiplier: f64, offset: i64 },
    Any,
    None,
    LinearBasedOnPrevious { amount_to_increase: usize },
}

impl Display for WeightScheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            WeightScheme::BasedOnReference { multiplier, offset } => {
                if offset > &0 {
                    write!(f, "{multiplier}r+{offset}")
                } else if offset == &0 {
                    write!(f, "{multiplier}r")
                } else {
                    write!(f, "{multiplier}r{offset}")
                }
            }
            WeightScheme::Any => write!(f, "any"),
            WeightScheme::None => write!(f, ""),
            WeightScheme::LinearBasedOnPrevious { amount_to_increase } => {
                write!(f, "add{amount_to_increase}")
            }
        };
    }
}

impl WeightScheme {
    /// notation options:
    /// ```
    /// parse("any")
    /// parse("3.14r+12")
    /// parse("3.14r-12")
    /// parse("add10")
    /// ```
    pub fn parse(notation: &str) -> Result<Self> {
        let error = "Invalid notation";
        return if notation == "any" {
            Ok(WeightScheme::Any)
        } else if notation.contains('r') {
            // 3.14r+12
            let mut split = notation.split('r');
            Ok(WeightScheme::BasedOnReference {
                multiplier: split.next().ok_or(anyhow!(error))?.parse()?,
                offset: split.next().unwrap_or("0").parse().unwrap_or(0),
            })
        } else {
            // add20
            let add_str = notation.split("add").skip(1).next().ok_or(anyhow!(error))?;
            Ok(WeightScheme::LinearBasedOnPrevious {
                amount_to_increase: add_str.parse()?,
            })
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Set {
    Amrap {
        minimum_reps: usize,
    },
    Range {
        maximum_reps: usize,
        minimum_reps: usize,
    },
    Any,
    Defined {
        reps: usize,
    },
    Time {
        duration: Duration,
    },
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return match self {
            Set::Amrap { minimum_reps } => write!(f, "{minimum_reps}+"),
            Set::Range {
                maximum_reps,
                minimum_reps,
            } => write!(f, "{minimum_reps}-{maximum_reps}"),
            Set::Any => write!(f, "Any"),
            Set::Defined { reps } => write!(f, "{reps}"),
            Set::Time { duration } => write!(f, "{}s", duration.as_secs()),
        };
    }
}

impl Set {
    /// notation options:
    /// ```
    /// parse("8-12")
    /// parse("3+")
    /// parse("Any")
    /// parse("10")
    /// parse("5s")
    /// ```
    pub fn parse(notation: &str) -> Result<Self> {
        let error = "Invalid notation";
        return if notation == "Any" {
            Ok(Set::Any)
        } else if notation.contains('-') {
            let mut split = notation.split('-');
            Ok(Set::Range {
                minimum_reps: split.next().ok_or(anyhow!(error))?.parse()?,
                maximum_reps: split.next().ok_or(anyhow!(error))?.parse()?,
            })
        } else if notation.contains('+') {
            let rep_string = notation.split('+').next().ok_or(anyhow!(error))?;
            Ok(Set::Amrap {
                minimum_reps: rep_string.parse()?,
            })
        } else if notation.contains('s') {
            let seconds_str = notation.split('s').next().ok_or(anyhow!(error))?;
            let duration = Duration::from_secs(seconds_str.parse()?);
            Ok(Set::Time { duration })
        } else {
            Ok(Set::Defined {
                reps: notation.parse()?,
            })
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Lift {
    pub name: String,
    pub sets: Vec<Set>,
    pub weight: WeightScheme,
}

impl Eq for Lift {}

impl Hash for Lift {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

fn format(sets: &Vec<Set>) -> String {
    struct Accum<'a> {
        count: i64,
        set: &'a Set,
    }
    let mut accumulated_sets: Vec<Accum> = Vec::new();
    for set in sets {
        if accumulated_sets.last().filter(|x| x.set == set).is_some() {
            accumulated_sets.last_mut().unwrap().count += 1;
        } else {
            accumulated_sets.push(Accum {
                count: 1,
                set: &set,
            });
        }
    }
    accumulated_sets
        .iter()
        .map(|x| format!("{}x{}", x.count, x.set))
        .collect::<Vec<String>>()
        .join(",")
}

impl Display for Lift {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if format!("{}", self.weight) != "" {
            write!(
                f,
                "{} -> {} @ {}",
                self.name,
                format(&self.sets),
                self.weight
            )
        } else {
            write!(f, "{} -> {}", self.name, format(&self.sets))
        }
    }
}

impl Lift {
    /// notation options:
    /// ```
    /// parse("name -> 3xReps,1xReps @ weight")
    /// ```
    /// See Set::parse() and WeightScheme::parse() to
    /// see how the `Reps` and `weight` portion above
    /// should be structured.  
    /// Example:
    /// ```
    /// parse("Barbell bench press -> 3x5,1x5-6,1x6+ @ 0.8r-10")
    /// ```
    pub fn parse(notation: &str) -> Result<Self> {
        let error = "Cannot parse notation";
        let name = notation.split("->").next().ok_or(anyhow!(error))?.trim();
        let rest = notation.split("->").skip(1).next().ok_or(anyhow!(error))?;
        let reps = if !rest.contains('@') {
            rest.trim()
        } else {
            rest.split("@").next().ok_or(anyhow!(error))?.trim()
        };
        let weight = if rest.contains('@') {
            let a = rest.split("@").skip(1).next().ok_or(anyhow!(error))?.trim();
            WeightScheme::parse(a)?
        } else {
            WeightScheme::None
        };
        Ok(Lift {
            name: name.to_string(),
            sets: Self::parse_sets(reps)?,
            weight,
        })
    }

    /// notation is like '2x3,1x3+'
    fn parse_sets(notation: &str) -> Result<Vec<Set>> {
        let error = "Cannot parse sets notation";
        let mut vec = Vec::new();
        notation
            .split(',')
            .try_for_each(|sets_n_reps| -> Result<()> {
                // sets_n_reps looks like '2x3+'
                let mut split = sets_n_reps.split('x');
                let times: i64 = split.next().ok_or(anyhow!(error))?.parse()?;
                let reps = split.next().ok_or(anyhow!(error))?;
                for _ in 0..times {
                    let b = Set::parse(reps)?;
                    vec.push(b);
                }
                Ok(())
            })?;
        Ok(vec)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LiftAttempt {
    pub lift: Lift,
    pub weight: Option<usize>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LiftAttemptResult {
    NotCompleted,
    Completed { completed_maximum_reps: bool },
}

impl Display for LiftAttemptResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LiftAttemptResult::NotCompleted => write!(f, "NotCompleted"),
            LiftAttemptResult::Completed {
                completed_maximum_reps,
            } => match completed_maximum_reps {
                true => write!(f, "Completed+MaxReps"),
                false => write!(f, "Completed"),
            },
        }
    }
}

impl LiftAttemptResult {
    pub fn parse(notation: &str) -> Result<Self> {
        match notation {
            "NotCompleted" => Ok(LiftAttemptResult::NotCompleted),
            "Completed" => Ok(LiftAttemptResult::Completed {
                completed_maximum_reps: false,
            }),
            "Completed+MaxReps" => Ok(LiftAttemptResult::Completed {
                completed_maximum_reps: true,
            }),
            _ => Err(anyhow!("Cannot parse notation")),
        }
    }
}

impl Display for LiftAttempt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.lift.weight {
            WeightScheme::None => {
                write!(f, "{} -> {}", self.lift.name, format(&self.lift.sets))
            }
            WeightScheme::BasedOnReference { multiplier, offset } => {
                let res = (multiplier * (self.weight.unwrap() as f64)) + (offset as f64);
                write!(
                    f,
                    "{} -> {} @ {}",
                    self.lift.name,
                    format(&self.lift.sets),
                    res
                )
            }
            WeightScheme::Any => {
                write!(f, "{} -> {} @ any", self.lift.name, format(&self.lift.sets))
            }
            WeightScheme::LinearBasedOnPrevious { .. } => write!(
                f,
                "{} -> {} @ {}",
                self.lift.name,
                format(&self.lift.sets),
                if self.weight.is_some() {
                    self.weight.unwrap().to_string()
                } else {
                    "any".to_string()
                }
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Day {
    pub name: String,
    pub lifts: Vec<Lift>,
}

impl Day {
    pub fn parse(notation: &str) -> Result<Day> {
        let error = "Cannot parse notation.";
        let mut lines = notation.split("\n");
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
            .join("\n");
        write!(f, "{}\n{}", self.name, lifts)
    }
}

#[cfg(test)]
mod tests {
    use crate::lifting::*;
    use std::time::Duration;

    #[test]
    fn can_serialize_lift_attempt_result() {
        assert_eq!("NotCompleted", LiftAttemptResult::NotCompleted.to_string());
        assert_eq!(
            "Completed",
            LiftAttemptResult::Completed {
                completed_maximum_reps: false
            }
            .to_string()
        );
        assert_eq!(
            "Completed+MaxReps",
            LiftAttemptResult::Completed {
                completed_maximum_reps: true
            }
            .to_string()
        );
    }

    #[test]
    fn can_parse_lift_attempt_result() {
        assert_eq!(
            LiftAttemptResult::parse("NotCompleted").unwrap(),
            LiftAttemptResult::NotCompleted
        );
        assert_eq!(
            LiftAttemptResult::parse("Completed").unwrap(),
            LiftAttemptResult::Completed {
                completed_maximum_reps: false
            }
        );
        assert_eq!(
            LiftAttemptResult::parse("Completed+MaxReps").unwrap(),
            LiftAttemptResult::Completed {
                completed_maximum_reps: true
            }
        );
    }

    #[test]
    fn can_parse_day() {
        assert_eq!(
            Day {
                name: "Day Name".to_string(),
                lifts: vec![
                    Lift::parse("Bench press -> 3x5,1x5-6,1x6+ @ 2r").unwrap(),
                    Lift::parse("Pullup -> 3x5,1x5-6,1x6+").unwrap()
                ],
            },
            Day::parse("Day Name\nBench press -> 3x5,1x5-6,1x6+ @ 2r\nPullup -> 3x5,1x5-6,1x6+")
                .unwrap()
        );
    }

    #[test]
    fn can_create_day() {
        assert_eq!(
            format!(
                "{}",
                Day {
                    name: "Day Name".to_string(),
                    lifts: vec![
                        Lift::parse("Bench press -> 3x5,1x5-6,1x6+ @ 2r").unwrap(),
                        Lift::parse("Pullup -> 3x5,1x5-6,1x6+").unwrap()
                    ],
                }
            ),
            "Day Name\nBench press -> 3x5,1x5-6,1x6+ @ 2r\nPullup -> 3x5,1x5-6,1x6+"
        );
    }

    #[test]
    fn can_make_lift_attempt() {
        assert_eq!(
            format!(
                "{}",
                LiftAttempt {
                    lift: Lift::parse("Pullups -> 3x5").unwrap(),
                    weight: None
                }
            ),
            "Pullups -> 3x5"
        );
        assert_eq!(
            format!(
                "{}",
                LiftAttempt {
                    lift: Lift::parse("Pullups -> 3x5 @ 0.5r+10").unwrap(),
                    weight: Some(100),
                }
            ),
            "Pullups -> 3x5 @ 60"
        );
        assert_eq!(
            format!(
                "{}",
                LiftAttempt {
                    lift: Lift::parse("Pullups -> 3x5 @ any").unwrap(),
                    weight: None,
                }
            ),
            "Pullups -> 3x5 @ any"
        );
        assert_eq!(
            format!(
                "{}",
                LiftAttempt {
                    lift: Lift::parse("Pullups -> 3x5 @ add10").unwrap(),
                    weight: None,
                }
            ),
            "Pullups -> 3x5 @ any"
        );
        assert_eq!(
            format!(
                "{}",
                LiftAttempt {
                    lift: Lift::parse("Pullups -> 3x5 @ add10").unwrap(),
                    weight: Some(25),
                }
            ),
            "Pullups -> 3x5 @ 25"
        );
    }

    #[test]
    fn can_create_lift_schemes() {
        assert!(Lift::parse(":(").is_err());
        assert_eq!(
            Lift::parse("Pullups -> 2x5,1x5-7,1x5+ @ 0.2r").unwrap(),
            Lift {
                name: "Pullups".to_string(),
                sets: vec![
                    Set::parse("5").unwrap(),
                    Set::parse("5").unwrap(),
                    Set::parse("5-7").unwrap(),
                    Set::parse("5+").unwrap(),
                ],
                weight: WeightScheme::parse("0.2r").unwrap()
            }
        );
    }

    #[test]
    fn can_display_lift_schemes() {
        assert_eq!(
            format!("{}", Lift::parse("Pullups -> 2x5,1x5-7,1x5+").unwrap()),
            "Pullups -> 2x5,1x5-7,1x5+"
        );
        assert_eq!(
            format!(
                "{}",
                Lift::parse("Some really long name -> 3xAny,2x5,1x5-7,1x5+ @ any").unwrap()
            ),
            "Some really long name -> 3xAny,2x5,1x5-7,1x5+ @ any"
        );
        assert_eq!(
            format!(
                "{}",
                Lift::parse("Pullups -> 2x5,1x5-7,1x5+ @ 0.2r").unwrap()
            ),
            "Pullups -> 2x5,1x5-7,1x5+ @ 0.2r"
        );
    }

    #[test]
    fn can_create_rep_schemes() {
        assert!(Set::parse(":(").is_err());
        assert_eq!(
            Set::parse("2-3").unwrap(),
            Set::Range {
                maximum_reps: 3,
                minimum_reps: 2
            }
        );
        assert_eq!(Set::parse("3+").unwrap(), Set::Amrap { minimum_reps: 3 });
        assert_eq!(Set::parse("3").unwrap(), Set::Defined { reps: 3 });
        assert_eq!(Set::parse("Any").unwrap(), Set::Any);
        assert_eq!(
            Set::parse("2s").unwrap(),
            Set::Time {
                duration: Duration::new(2, 0)
            }
        );
    }

    #[test]
    fn can_display_rep_schemes() {
        assert_eq!(format!("{}", Set::parse("2-3").unwrap()), "2-3");
        assert_eq!(format!("{}", Set::parse("3+").unwrap()), "3+");
        assert_eq!(format!("{}", Set::parse("3").unwrap()), "3");
        assert_eq!(format!("{}", Set::parse("Any").unwrap()), "Any");
        assert_eq!(format!("{}", Set::parse("2s").unwrap()), "2s");
    }

    #[test]
    fn can_create_weight_schemes() {
        assert!(WeightScheme::parse(":(").is_err());
        assert_eq!(
            WeightScheme::parse("3.14r-12").unwrap(),
            WeightScheme::BasedOnReference {
                multiplier: 3.14,
                offset: -12
            }
        );
        assert_eq!(
            WeightScheme::parse("3.14r+12").unwrap(),
            WeightScheme::BasedOnReference {
                multiplier: 3.14,
                offset: 12
            }
        );
        assert_eq!(
            WeightScheme::parse("3.14r").unwrap(),
            WeightScheme::BasedOnReference {
                multiplier: 3.14,
                offset: 0
            }
        );
        assert_eq!(WeightScheme::parse("any").unwrap(), WeightScheme::Any);
        assert_eq!(
            WeightScheme::parse("add20").unwrap(),
            WeightScheme::LinearBasedOnPrevious {
                amount_to_increase: 20
            }
        );
    }

    #[test]
    fn can_display_weight_schemes() {
        assert_eq!(
            format!("{}", WeightScheme::parse("3.14r-12").unwrap()),
            "3.14r-12"
        );
        assert_eq!(
            format!("{}", WeightScheme::parse("3.14r+12").unwrap()),
            "3.14r+12"
        );
        assert_eq!(
            format!("{}", WeightScheme::parse("3.14r").unwrap()),
            "3.14r"
        );
        assert_eq!(format!("{}", WeightScheme::parse("any").unwrap()), "any");
        assert_eq!(
            format!("{}", WeightScheme::parse("add20").unwrap()),
            "add20"
        );
    }
}
