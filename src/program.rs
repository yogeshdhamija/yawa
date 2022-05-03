use anyhow::{anyhow, Result};
use std::fmt::{Display, Formatter};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub enum WeightScheme {
    BasedOnReference { multiplier: f64, offset: i64 },
    Any,
    None,
    LinearBasedOnPrevious { amount_to_increase: u64 },
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
    fn parse(notation: &str) -> Result<Self> {
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
        minimum_reps: u64,
    },
    Range {
        maximum_reps: u64,
        minimum_reps: u64,
    },
    Any,
    Defined {
        reps: u64,
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
    fn parse(notation: &str) -> Result<Self> {
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
    name: String,
    sets: Vec<Set>,
    weight: WeightScheme,
}

impl Display for Lift {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        struct Accum<'a> {
            count: i64,
            set: &'a Set,
        }
        let mut accumulated_sets: Vec<Accum> = Vec::new();
        for set in &self.sets {
            if accumulated_sets.last().filter(|x| x.set == set).is_some() {
                accumulated_sets.last_mut().unwrap().count += 1;
            } else {
                accumulated_sets.push(Accum {
                    count: 1,
                    set: &set,
                });
            }
        }
        let result = accumulated_sets
            .iter()
            .map(|x| format!("{}x{}", x.count, x.set))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{} -> {} @ {}", self.name, result, self.weight)
    }
}

impl Lift {
    fn parse(notation: &str) -> Result<Self> {
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

#[derive(Clone, Debug)]
pub struct Day {
    pub name: String,
    pub lifts: Vec<Lift>,
}

#[derive(Clone, Debug)]
pub struct Program {
    pub days: Vec<Day>,
}

impl Program {
    pub fn gzcl_4day() -> Self {
        Program {
            days: vec![
                Day {
                    name: "Pull".to_string(),
                    lifts: vec![
                        Lift::parse("Weighted Pullup -> 4x3,1x3+ @ 0.5r-30").unwrap(),
                        Lift::parse("Pullup -> 3x7+").unwrap(),
                        Lift::parse("Barbell Row -> 3x10 @ 0.65r").unwrap(),
                        Lift::parse("Face Pull -> 2x15,1x15-25 @ add20").unwrap(),
                        Lift::parse("Cable Curl -> 2x15,1x15-25 @ add20").unwrap(),
                    ],
                },
                Day {
                    name: "Push".to_string(),
                    lifts: vec![
                        Lift::parse("Bench press -> 4x3,1x3+ @ 1r").unwrap(),
                        Lift::parse("Overhead press -> 3x10 @ 0.5r").unwrap(),
                        Lift::parse("Incline bench press -> 3x10 @ 0.6r").unwrap(),
                        Lift::parse("Pushup -> 3x15+").unwrap(),
                        Lift::parse("Tricep Cable Pressdown -> 2x15,1x15-25 @ add20").unwrap(),
                    ],
                },
                Day {
                    name: "Legs".to_string(),
                    lifts: vec![
                        Lift::parse("Squat -> 4x3,1x3+ @ 1.35r").unwrap(),
                        Lift::parse("Deadlift -> 3x8 @ 1.25r").unwrap(),
                        Lift::parse("Romanian Deadlift -> 3x10 @ 0.675r").unwrap(),
                        Lift::parse("Leg press -> 2x15,1x15-25 @ add30").unwrap(),
                        Lift::parse("Standing dumbbell calf raise -> 2x15,1x15-25 @ add20")
                            .unwrap(),
                    ],
                },
                Day {
                    name: "Core".to_string(),
                    lifts: vec![
                        Lift::parse("Plank -> 1x30s @ any").unwrap(),
                        Lift::parse("Ab Rollout -> 3xAny").unwrap(),
                        Lift::parse("Cable Core Press -> 3xAny @ any").unwrap(),
                        Lift::parse("Bent-knee reverse hyperextension -> 3xAny @ any").unwrap(),
                        Lift::parse("Knee raises -> 3xAny").unwrap(),
                        Lift::parse("Leg extensions -> 3xAny @ any").unwrap(),
                    ],
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::program::*;
    use std::time::Duration;

    #[test]
    fn can_create_program() {
        Program::gzcl_4day();
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
