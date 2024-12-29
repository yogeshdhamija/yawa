use std::fmt::{Display, Formatter};
use std::time::Duration;
use anyhow::anyhow;

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
    /// # use yawa::domain::set::Set;
    /// Set::parse("8-12").unwrap();
    /// Set::parse("3+").unwrap();
    /// Set::parse("Any").unwrap();
    /// Set::parse("10").unwrap();
    /// Set::parse("5s").unwrap();
    /// ```
    pub fn parse(notation: &str) -> anyhow::Result<Self> {
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

#[cfg(test)]
mod tests {
    use std::time::Duration;
    use crate::domain::set::Set;

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
}