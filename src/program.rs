use anyhow::{anyhow, Result};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum WeightScheme {
    BasedOnReference { multiplier: f64, offset: i64 },
    Any,
    None,
    LinearBasedOnPrevious { amount_to_increase: u64 },
}

#[allow(dead_code)]
impl WeightScheme {
    fn parse(notation: &str) -> Result<Self> {
        let error = "Invalid notation";
        if notation == "any" {
            return Ok(WeightScheme::Any);
        } else if notation.contains('r') {
            // 3.14r+12
            let mut split = notation.split('r');
            return Ok(WeightScheme::BasedOnReference {
                multiplier: split.next().ok_or(anyhow!(error))?.parse()?,
                offset: split.next().unwrap_or("0").parse().unwrap_or(0),
            });
        } else {
            // add20
            let add_str = notation.split("add").skip(1).next().ok_or(anyhow!(error))?;
            return Ok(WeightScheme::LinearBasedOnPrevious {
                amount_to_increase: add_str.parse()?,
            });
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
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
}

#[allow(dead_code)]
impl Set {
    fn parse(notation: &str) -> Result<Self> {
        let error = "Invalid notation";
        if notation.contains('-') {
            let mut split = notation.split('-');
            return Ok(Set::Range {
                minimum_reps: split.next().ok_or(anyhow!(error))?.parse()?,
                maximum_reps: split.next().ok_or(anyhow!(error))?.parse()?,
            });
        } else if notation.contains('+') {
            let rep_string = notation.split('+').next().ok_or(anyhow!(error))?;
            return Ok(Set::Amrap {
                minimum_reps: rep_string.parse()?,
            });
        } else {
            return Ok(Set::Defined {
                reps: notation.parse()?,
            });
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub struct Lift {
    name: String,
    sets: Vec<Set>,
    weight: WeightScheme,
}

#[allow(dead_code)]
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
}

mod private {
    use super::*;
    impl Lift {
        /// notation is like '2x3,1x3+'
        pub(super) fn parse_sets(notation: &str) -> Result<Vec<Set>> {
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
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Day {
    name: String,
    lifts: Vec<Lift>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Program {
    days: Vec<Day>,
}

#[allow(dead_code)]
impl Program {
    fn new() -> Self {
        Program {
            days: vec![Day {
                name: "Pull".to_string(),
                lifts: vec![
                    Lift::parse("Weighted Pullup -> 4x3,1x3+ @ 0.5r-30").unwrap(),
                    Lift::parse("Weighted Pullup -> 4x3,1x3+ @ 0.5r-30").unwrap(),
                    Lift::parse("Weighted Pullup -> 4x3,1x3+ @ 0.5r-30").unwrap(),
                    Lift::parse("Weighted Pullup -> 4x3,1x3+ @ 0.5r-30").unwrap(),
                    Lift::parse("Weighted Pullup -> 4x3,1x3+ @ 0.5r-30").unwrap(),
                ],
            }],
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::program::*;

    #[test]
    fn can_create_lift_schemes() {
        assert!(Lift::parse(":(").is_err());
        let a = Lift::parse("Pullups -> 2x5,1x5-7,1x5+ @ 0.2r").unwrap();
        println!("{a:#?}");
        assert!(
            a == Lift {
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
    fn can_create_rep_schemes() {
        assert!(Set::parse(":(").is_err());
        assert!(
            Set::parse("2-3").unwrap()
                == Set::Range {
                    maximum_reps: 3,
                    minimum_reps: 2
                }
        );
        assert!(Set::parse("3+").unwrap() == Set::Amrap { minimum_reps: 3 });
        assert!(Set::parse("3").unwrap() == Set::Defined { reps: 3 });
    }

    #[test]
    fn can_create_weight_schemes() {
        assert!(WeightScheme::parse(":(").is_err());
        assert!(
            WeightScheme::parse("3.14r-12").unwrap()
                == WeightScheme::BasedOnReference {
                    multiplier: 3.14,
                    offset: -12
                }
        );
        assert!(
            WeightScheme::parse("3.14r+12").unwrap()
                == WeightScheme::BasedOnReference {
                    multiplier: 3.14,
                    offset: 12
                }
        );
        assert!(
            WeightScheme::parse("3.14r").unwrap()
                == WeightScheme::BasedOnReference {
                    multiplier: 3.14,
                    offset: 0
                }
        );
        assert!(WeightScheme::parse("any").unwrap() == WeightScheme::Any);
        assert!(
            WeightScheme::parse("add20").unwrap()
                == WeightScheme::LinearBasedOnPrevious {
                    amount_to_increase: 20
                }
        );
    }
}
