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
    pub fn increment_day(mut self, results: &[LiftAttemptResult]) -> Program {
        self.days[self.current_day as usize]
            .lifts
            .iter()
            .enumerate()
            .for_each(|(index, lift)| {
                if let WeightScheme::LinearBasedOnPrevious { amount_to_increase } = lift.weight {
                    if results[index]
                        == (LiftAttemptResult::Completed {
                            completed_maximum_reps: true,
                        })
                    {
                        self.weights.insert(
                            lift.clone(),
                            self.weights
                                .get(lift)
                                .map(|it| it + amount_to_increase)
                                .or(Some(amount_to_increase))
                                .unwrap(),
                        );
                    }
                }
            });
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
    fn parse(str: &str) -> Lift {
        Lift::parse(str).unwrap()
    }
    Program {
        name: "GZCL-based 4-day cycle".to_string(),
        reference_weight,
        weights: HashMap::from([
            (parse("Face Pull -> 2x15,1x15-25 @ add20"), 30),
            (parse("Cable Curl -> 2x15,1x15-25 @ add20"), 20),
            (parse("Tricep Cable Pressdown -> 2x15,1x15-25 @ add20"), 20),
            (parse("Leg press -> 2x15,1x15-25 @ add30"), 45),
            (
                parse("Standing dumbbell calf raise -> 2x15,1x15-25 @ add20"),
                45,
            ),
        ]),
        days: vec![
            Day {
                name: "Pull".to_string(),
                lifts: vec![
                    parse("Weighted Pullup -> 4x3,1x3+ @ 0.5r-30"),
                    parse("Pullup -> 3x7+"),
                    parse("Barbell Row -> 3x10 @ 0.65r"),
                    parse("Face Pull -> 2x15,1x15-25 @ add20"),
                    parse("Cable Curl -> 2x15,1x15-25 @ add20"),
                ],
            },
            Day {
                name: "Push".to_string(),
                lifts: vec![
                    parse("Bench press -> 4x3,1x3+ @ 1r"),
                    parse("Overhead press -> 3x10 @ 0.5r"),
                    parse("Incline bench press -> 3x10 @ 0.6r"),
                    parse("Pushup -> 3x15+"),
                    parse("Tricep Cable Pressdown -> 2x15,1x15-25 @ add20"),
                ],
            },
            Day {
                name: "Legs".to_string(),
                lifts: vec![
                    parse("Squat -> 4x3,1x3+ @ 1.35r"),
                    parse("Deadlift -> 3x8 @ 1.25r"),
                    parse("Romanian Deadlift -> 3x10 @ 0.675r"),
                    parse("Leg press -> 2x15,1x15-25 @ add30"),
                    parse("Standing dumbbell calf raise -> 2x15,1x15-25 @ add20"),
                ],
            },
            Day {
                name: "Core".to_string(),
                lifts: vec![
                    parse("Plank -> 1x30s @ any"),
                    parse("Ab Rollout -> 3xAny"),
                    parse("Cable Core Press -> 3xAny @ any"),
                    parse("Bent-knee reverse hyperextension -> 3xAny @ any"),
                    parse("Knee raises -> 3xAny"),
                    parse("Leg extensions -> 3xAny @ any"),
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
        fn increments_weights() {
            let before = start_gzcl_4day(100);
            let lift_incremented = before.days[0].lifts[3].clone();
            assert_eq!(
                lift_incremented.to_string(),
                "Face Pull -> 2x15,1x15-25 @ add20"
            );
            let lift_not_incremented = before.days[0].lifts[4].clone();
            assert_eq!(
                lift_not_incremented.to_string(),
                "Cable Curl -> 2x15,1x15-25 @ add20"
            );
            assert_eq!(before.weights[&lift_incremented], 30);
            assert_eq!(before.weights[&lift_not_incremented], 20);
            let after = before.increment_day(&[
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
                    completed_maximum_reps: false,
                },
            ]);
            assert_eq!(after.weights[&lift_incremented], 50);
            assert_eq!(after.weights[&lift_not_incremented], 20);
        }

        #[test]
        fn increments_each_day_and_rolls_over() {
            assert_eq!(start_gzcl_4day(100).current_day, 0);
            assert_eq!(
                start_gzcl_4day(100)
                    .increment_day(&[LiftAttemptResult::NotCompleted; 5])
                    .current_day,
                1
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .increment_day(&[LiftAttemptResult::NotCompleted; 5])
                    .increment_day(&[LiftAttemptResult::NotCompleted; 5])
                    .current_day,
                2
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .increment_day(&[LiftAttemptResult::NotCompleted; 5])
                    .increment_day(&[LiftAttemptResult::NotCompleted; 5])
                    .increment_day(&[LiftAttemptResult::NotCompleted; 5])
                    .current_day,
                3
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .increment_day(&[LiftAttemptResult::NotCompleted; 5])
                    .increment_day(&[LiftAttemptResult::NotCompleted; 5])
                    .increment_day(&[LiftAttemptResult::NotCompleted; 5])
                    .increment_day(&[LiftAttemptResult::NotCompleted; 5])
                    .current_day,
                0
            );
        }
    }
}
