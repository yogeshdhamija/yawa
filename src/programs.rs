use crate::lifting::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Program {
    pub days: Vec<Day>,
    pub reference_weight: usize,
    pub starting_reference_weight: usize,
    pub workouts_completed: usize,
    pub name: String,
    pub weights: HashMap<Lift, usize>,
    pub current_day: usize,
    pub current_cycle_attempt_results: Vec<Vec<LiftAttemptResult>>,
}

impl Program {
    pub fn complete_workout(self, results: &[LiftAttemptResult]) -> Program {
        self.save_results(results)
            .increment_non_reference_weights()
            .increment_reference()
            .increment_count()
            .increment_day()
    }

    pub fn next_workout(&self) -> Vec<LiftAttempt> {
        self.days[self.current_day]
            .lifts
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

    fn increment_reference(mut self) -> Self {
        if !self.is_last_day() {
            return self;
        }
        if self.are_all_completed(self.indexes_of_past_attempts_that_are_reference_lifts()) {
            self.reference_weight = self.reference_weight + 5;
        }
        self
    }

    fn are_all_completed(&self, past_attempt_indexes: Vec<(usize, usize)>) -> bool {
        past_attempt_indexes.iter().all(|(day_index, lift_index)| {
            match self.current_cycle_attempt_results[*day_index][*lift_index] {
                LiftAttemptResult::Completed {
                    completed_maximum_reps,
                } => completed_maximum_reps,
                _ => false,
            }
        })
    }

    fn indexes_of_past_attempts_that_are_reference_lifts(&self) -> Vec<(usize, usize)> {
        let mut past_attempt_indexes: Vec<(usize, usize)> = Vec::new();
        self.days.iter().enumerate().for_each(|(day_index, day)| {
            day.lifts.iter().enumerate().for_each(|(lift_index, lift)| {
                if matches!(lift.weight, WeightScheme::BasedOnReference { .. }) {
                    past_attempt_indexes.push((day_index, lift_index))
                }
            })
        });
        past_attempt_indexes
    }

    fn is_last_day(&self) -> bool {
        self.current_day == self.days.len() - 1
    }

    fn save_results(mut self, results: &[LiftAttemptResult]) -> Self {
        self.ensure_past_attempts_match_day_length();
        self.current_cycle_attempt_results[self.current_day] = Vec::from(results);
        self
    }

    fn ensure_past_attempts_match_day_length(&mut self) {
        while self.current_cycle_attempt_results.len() <= self.current_day {
            self.current_cycle_attempt_results.push(Vec::new());
        }
    }

    fn increment_day(mut self) -> Self {
        self.current_day += 1;
        if self.current_day >= self.days.len() {
            self.current_day = 0;
        }
        self
    }

    fn increment_non_reference_weights(mut self) -> Self {
        self.days[self.current_day]
            .lifts
            .iter()
            .enumerate()
            .for_each(|(index, lift)| {
                if let WeightScheme::LinearBasedOnPrevious { amount_to_increase } = lift.weight {
                    if self.current_cycle_attempt_results[self.current_day][index]
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
    fn increment_count(mut self) -> Self {
        self.workouts_completed += 1;
        self
    }
}

pub fn start_gzcl_4day(reference_weight: usize) -> Program {
    fn lift(str: &str) -> Lift {
        Lift::parse(str).unwrap()
    }
    Program {
        name: "GZCL-based 4-day cycle (Pull, Push, Legs, Core)".to_string(),
        reference_weight,
        starting_reference_weight: reference_weight,
        weights: HashMap::from([
            (lift("Face Pull -> 2x15,1x15-25 @ add20"), 30),
            (lift("Cable Curl -> 2x15,1x15-25 @ add20"), 20),
            (lift("Tricep Cable Pressdown -> 2x15,1x15-25 @ add20"), 20),
            (lift("Leg press -> 2x15,1x15-25 @ add30"), 45),
            (
                lift("Standing dumbbell calf raise -> 2x15,1x15-25 @ add20"),
                45,
            ),
        ]),
        days: vec![
            Day {
                name: "Pull".to_string(),
                lifts: vec![
                    lift("Weighted Pullup -> 4x3,1x3+ @ 0.5r-30"),
                    lift("Pullup -> 3x7+"),
                    lift("Barbell Row -> 3x10 @ 0.65r"),
                    lift("Face Pull -> 2x15,1x15-25 @ add20"),
                    lift("Cable Curl -> 2x15,1x15-25 @ add20"),
                ],
            },
            Day {
                name: "Push".to_string(),
                lifts: vec![
                    lift("Bench press -> 4x3,1x3+ @ 1r"),
                    lift("Overhead press -> 3x10 @ 0.5r"),
                    lift("Incline bench press -> 3x10 @ 0.6r"),
                    lift("Pushup -> 3x15+"),
                    lift("Tricep Cable Pressdown -> 2x15,1x15-25 @ add20"),
                ],
            },
            Day {
                name: "Legs".to_string(),
                lifts: vec![
                    lift("Squat -> 4x3,1x3+ @ 1.35r"),
                    lift("Deadlift -> 3x8 @ 1.25r"),
                    lift("Romanian Deadlift -> 3x10 @ 0.675r"),
                    lift("Leg press -> 2x15,1x15-25 @ add30"),
                    lift("Standing dumbbell calf raise -> 2x15,1x15-25 @ add20"),
                ],
            },
            Day {
                name: "Core".to_string(),
                lifts: vec![
                    lift("Plank -> 1x30s @ any"),
                    lift("Ab Rollout -> 3xAny"),
                    lift("Cable Core Press -> 3xAny @ any"),
                    lift("Bent-knee reverse hyperextension -> 3xAny @ any"),
                    lift("Knee raises -> 3xAny"),
                    lift("Leg extensions -> 3xAny @ any"),
                ],
            },
        ],
        current_day: 0,
        current_cycle_attempt_results: vec![],
        workouts_completed: 0,
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
                    .complete_workout(&one_incomplete)
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
            assert_eq!(start_gzcl_4day(100).reference_weight, 100);
            assert_eq!(start_gzcl_4day(100).starting_reference_weight, 100);
            let all_lifts_completed = [Completed {
                completed_maximum_reps: true,
            }; 5];

            let before_cycle_completed = start_gzcl_4day(100)
                .complete_workout(&all_lifts_completed)
                .complete_workout(&all_lifts_completed)
                .complete_workout(&all_lifts_completed);
            assert_eq!(before_cycle_completed.reference_weight, 100);

            let after_cycle_completed = before_cycle_completed
                .complete_workout(&all_lifts_completed);
            assert_eq!(after_cycle_completed.reference_weight, 105);
            assert_eq!(after_cycle_completed.starting_reference_weight, 100);
        }
        #[test]
        fn is_not_affected_by_failures_in_previous_cycles() {
            let incremented_weight = 105;
            let not_incremented_weight = 100;
            let all_lifts_completed = [Completed { completed_maximum_reps: true }; 5];
            let all_lifts_not_completed = [NotCompleted; 5];

            assert_eq!(start_gzcl_4day(not_incremented_weight).reference_weight, not_incremented_weight);
            assert_eq!(start_gzcl_4day(not_incremented_weight).starting_reference_weight, not_incremented_weight);

            let after_failed_cycle_completed = start_gzcl_4day(not_incremented_weight)
                .complete_workout(&all_lifts_completed)
                .complete_workout(&all_lifts_not_completed)
                .complete_workout(&all_lifts_completed)
                .complete_workout(&all_lifts_completed);
            assert_eq!(after_failed_cycle_completed.reference_weight, not_incremented_weight);

            let after_successful_cycle_completed = after_failed_cycle_completed
                .complete_workout(&all_lifts_completed)
                .complete_workout(&all_lifts_completed)
                .complete_workout(&all_lifts_completed)
                .complete_workout(&all_lifts_completed);
            assert_eq!(after_successful_cycle_completed.starting_reference_weight, not_incremented_weight);

            assert_eq!(after_successful_cycle_completed.reference_weight, incremented_weight);
        }
        #[test]
        fn increments_each_day_and_rolls_over() {
            assert_eq!(start_gzcl_4day(100).current_day, 0);

            let before_completing_cycle = start_gzcl_4day(100)
                .complete_workout(&[NotCompleted; 5])
                .complete_workout(&[NotCompleted; 5]);
            assert_eq!(before_completing_cycle.current_day, 2);
            assert_eq!(before_completing_cycle.workouts_completed, 2);

            let after_completing_cycle = start_gzcl_4day(100)
                .complete_workout(&[NotCompleted; 5])
                .complete_workout(&[NotCompleted; 5])
                .complete_workout(&[NotCompleted; 5])
                .complete_workout(&[NotCompleted; 5]);
            assert_eq!(after_completing_cycle.current_day, 0);
            assert_eq!(after_completing_cycle.workouts_completed, 4);
        }
    }
}
