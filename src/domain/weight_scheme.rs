use std::fmt::{Display, Formatter};
use anyhow::anyhow;

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
    /// # use yawa::domain::weight_scheme::*;
    /// WeightScheme::parse("any").unwrap();
    /// WeightScheme::parse("3.14r+12").unwrap();
    /// WeightScheme::parse("3.14r-12").unwrap();
    /// WeightScheme::parse("add10").unwrap();
    /// ```
    pub fn parse(notation: &str) -> anyhow::Result<Self> {
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