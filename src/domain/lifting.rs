use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use crate::domain::weight_scheme::*;
use crate::domain::set::*;

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
    /// General idea:
    /// ```
    /// # use yawa::domain::lifting::Lift;
    /// Lift::parse("name -> 3xReps,1xReps @ weight").is_err();
    /// ```
    /// See `Set::parse()` and `WeightScheme::parse()` to
    /// see how the `Reps` and `weight` portion above
    /// should be structured.
    ///
    /// Example:
    /// ```
    /// # use yawa::domain::lifting::Lift;
    /// Lift::parse("Barbell bench press -> 3x5,1x5-6,1x6+ @ 0.8r-10").unwrap();
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
    /// Notation options:
    /// ```
    /// # use yawa::domain::lifting::LiftAttemptResult;
    /// LiftAttemptResult::parse("NotCompleted").unwrap();
    /// LiftAttemptResult::parse("Completed").unwrap();
    /// LiftAttemptResult::parse("Completed+MaxReps").unwrap();
    /// LiftAttemptResult::parse("AnythingElse").is_err();
    /// ```
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

fn round_up_to_nearest5(input: usize) -> usize{
    let remainder = input % 5;
    return if remainder > 0 {
        input + (5 - remainder)
    } else {
        input
    }
}

impl Display for LiftAttempt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.lift.weight {
            WeightScheme::None => {
                write!(f, "{} -> {}", self.lift.name, format(&self.lift.sets))
            }
            WeightScheme::BasedOnReference { multiplier, offset } => {
                let weight = (multiplier * (self.weight.unwrap() as f64)) + (offset as f64);
                write!(
                    f,
                    "{} -> {} @ {}",
                    self.lift.name,
                    format(&self.lift.sets),
                    round_up_to_nearest5(weight.ceil() as usize)
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
                    round_up_to_nearest5(self.weight.unwrap()).to_string()
                } else {
                    "any".to_string()
                }
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::lifting::*;

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
    fn lift_attempts_round() {
        assert_eq!(
            LiftAttempt{
                lift: Lift::parse("Doobee doos -> 3x5 @ add20").unwrap(),
                weight: Some(22)
            }.to_string(),
            "Doobee doos -> 3x5 @ 25"
        );
        assert_eq!(
            LiftAttempt{
                    lift: Lift::parse("Doobee doos -> 3x5 @ 0.222r").unwrap(),
                    weight: Some(100)
            }.to_string(),
            "Doobee doos -> 3x5 @ 25"
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
}
