use crate::lifting::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub days: Vec<Day>,
    pub reference_weight: usize,
    pub name: String,
    pub weights: HashMap<Lift, usize>,
    pub current_day: usize,
    pub past_attempt_results: Vec<Vec<LiftAttemptResult>>,
}

impl Program {
    pub fn complete_workout(self, results: &[LiftAttemptResult]) -> Program {
        self.save_results(results)
            .increment_non_reference_weights()
            .increment_reference()
            .increment_day()
    }

    fn increment_reference(mut self) -> Self {
        if self.current_day == self.days.len() - 1 {
            self.reference_weight = self.reference_weight + 5;
        }
        self
    }

    fn save_results(mut self, results: &[LiftAttemptResult]) -> Self {
        while self.past_attempt_results.len() <= self.current_day {
            self.past_attempt_results.push(Vec::new());
        }
        self.past_attempt_results[self.current_day] = Vec::from(results);
        self
    }

    fn increment_day(mut self) -> Self {
        self.current_day = if self.current_day + 1 < self.days.len() {
            self.current_day + 1
        } else {
            0
        };
        self
    }

    fn increment_non_reference_weights(mut self) -> Self {
        self.days[self.current_day]
            .lifts
            .iter()
            .enumerate()
            .for_each(|(index, lift)| {
                if let WeightScheme::LinearBasedOnPrevious { amount_to_increase } = lift.weight {
                    if self.past_attempt_results[self.current_day][index]
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
        self
    }

    pub fn next_workout(&self) -> Vec<LiftAttempt> {
        let day = &self.days[self.current_day];
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

pub fn start_gzcl_4day(reference_weight: usize) -> Program {
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
        past_attempt_results: vec![],
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
        use crate::lifting::LiftAttemptResult::{Completed, NotCompleted};

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
            let after = before.complete_workout(&[
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
        fn doesnt_increment_reference_weight_if_any_not_completed() {
            let completed_all = [Completed {
                completed_maximum_reps: true,
            }; 5];
            let one_incomplete = [
                NotCompleted,
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
            ];
            assert_eq!(start_gzcl_4day(100).reference_weight, 100);
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&completed_all)
                    .reference_weight,
                100
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&completed_all)
                    .complete_workout(&completed_all)
                    .reference_weight,
                100
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&one_incomplete)
                    .complete_workout(&completed_all)
                    .complete_workout(&completed_all)
                    .reference_weight,
                100
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&completed_all)
                    .complete_workout(&completed_all)
                    .complete_workout(&completed_all)
                    .complete_workout(&completed_all)
                    .reference_weight,
                100
            );
        }
        #[test]
        fn increments_reference_weight_if_all_completed() {
            let completed_all = [Completed {
                completed_maximum_reps: true,
            }; 5];
            assert_eq!(start_gzcl_4day(100).reference_weight, 100);
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&completed_all)
                    .reference_weight,
                100
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&completed_all)
                    .complete_workout(&completed_all)
                    .reference_weight,
                100
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&completed_all)
                    .complete_workout(&completed_all)
                    .complete_workout(&completed_all)
                    .reference_weight,
                100
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&completed_all)
                    .complete_workout(&completed_all)
                    .complete_workout(&completed_all)
                    .complete_workout(&completed_all)
                    .reference_weight,
                105
            );
        }
        #[test]
        fn increments_each_day_and_rolls_over() {
            assert_eq!(start_gzcl_4day(100).current_day, 0);
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&[LiftAttemptResult::NotCompleted; 5])
                    .current_day,
                1
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&[LiftAttemptResult::NotCompleted; 5])
                    .complete_workout(&[LiftAttemptResult::NotCompleted; 5])
                    .current_day,
                2
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&[LiftAttemptResult::NotCompleted; 5])
                    .complete_workout(&[LiftAttemptResult::NotCompleted; 5])
                    .complete_workout(&[LiftAttemptResult::NotCompleted; 5])
                    .current_day,
                3
            );
            assert_eq!(
                start_gzcl_4day(100)
                    .complete_workout(&[LiftAttemptResult::NotCompleted; 5])
                    .complete_workout(&[LiftAttemptResult::NotCompleted; 5])
                    .complete_workout(&[LiftAttemptResult::NotCompleted; 5])
                    .complete_workout(&[LiftAttemptResult::NotCompleted; 5])
                    .current_day,
                0
            );
        }
    }
}
