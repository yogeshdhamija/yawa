use crate::lifting::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub days: Vec<Day>,
    pub reference_weight: u64,
    pub name: String,
    pub weights: HashMap<Lift, u64>,
    pub current_day: u64,
}

impl Program {
    pub fn increment_day(self, _results: &[LiftAttemptResult]) -> Program {
        Program {
            current_day: if self.current_day + 1 < self.days.len() as u64 {
                self.current_day + 1
            } else {
                0
            },
            ..self
        }
    }
    pub fn next_workout(&self) -> Vec<LiftAttempt> {
        let day = &self.days[self.current_day as usize];
        day.lifts
            .iter()
            .map(|lift| LiftAttempt {
                lift: lift.clone(),
                weight: match lift.weight {
                    WeightScheme::BasedOnReference { .. } => Some(self.reference_weight),
                    WeightScheme::Any => None,
                    WeightScheme::None => None,
                    WeightScheme::LinearBasedOnPrevious { .. } => {
                        self.weights.get(lift).map(|it| *it)
                    }
                },
            })
            .collect()
    }
}

pub fn start_gzcl_4day(reference_weight: u64) -> Program {
    Program {
        name: "GZCL-based 4-day cycle".to_string(),
        reference_weight,
        weights: HashMap::from([
            (
                Lift::parse("Face Pull -> 2x15,1x15-25 @ add20").unwrap(),
                30,
            ),
            (
                Lift::parse("Cable Curl -> 2x15,1x15-25 @ add20").unwrap(),
                20,
            ),
        ]),
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
                    Lift::parse("Standing dumbbell calf raise -> 2x15,1x15-25 @ add20").unwrap(),
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
        current_day: 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::programs::*;

    #[test]
    fn stores_weights() {
        assert_eq!(
            format!("{}", start_gzcl_4day(100).next_workout()[0]),
            "Weighted Pullup -> 4x3,1x3+ @ 20"
        );
        assert_eq!(
            format!("{}", start_gzcl_4day(100).next_workout()[1]),
            "Pullup -> 3x7+"
        );
    }

    #[test]
    fn all_non_reference_weights_initialized() {
        assert_eq!(
            format!("{}", start_gzcl_4day(100).next_workout()[3]),
            "Face Pull -> 2x15,1x15-25 @ 30"
        );
        assert_eq!(
            format!("{}", start_gzcl_4day(100).next_workout()[4]),
            "Cable Curl -> 2x15,1x15-25 @ 20"
        );
    }

    mod incrementing {
        use super::*;
        use crate::lifting::LiftAttemptResult::Completed;

        #[test]
        #[ignore]
        fn increments_weights() {
            let program = start_gzcl_4day(100);
            assert_eq!(program.weights.len(), 0);
            program.increment_day(&[
                Completed {
                    completed_maximum_reps: true,
                },
                Completed {
                    completed_maximum_reps: true,
                },
                Completed {
                    completed_maximum_reps: true,
                },
                Completed {
                    completed_maximum_reps: true,
                },
                Completed {
                    completed_maximum_reps: true,
                },
            ]);
        }

        #[test]
        fn increments_each_day_and_rolls_over() {
            assert_eq!(start_gzcl_4day(100).current_day, 0);
            assert_eq!(start_gzcl_4day(100).increment_day(&[]).current_day, 1);
            assert_eq!(
                start_gzcl_4day(100)
                    .increment_day(&[])
                    .increment_day(&[])
                    .current_day,
                2
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .increment_day(&[])
                    .increment_day(&[])
                    .increment_day(&[])
                    .current_day,
                3
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .increment_day(&[])
                    .increment_day(&[])
                    .increment_day(&[])
                    .increment_day(&[])
                    .current_day,
                0
            );
        }
    }
}
