use crate::lifting::*;

#[derive(Clone, Debug)]
pub struct Gzcl4Day {
    pub days: Vec<Day>,
    pub reference_weight: u64,
}

impl Gzcl4Day {
    pub fn start(reference_weight: u64) -> Self {
        Gzcl4Day {
            reference_weight,
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

impl Program for Gzcl4Day {
    fn days(&self) -> &Vec<Day> {
        &self.days
    }

    fn next_workout(&self) -> Vec<LiftAttempt> {
        let day = self.days.first().unwrap();
        day.lifts
            .iter()
            .map(|lift| LiftAttempt {
                lift: lift.clone(),
                weight: match lift.weight {
                    WeightScheme::BasedOnReference { .. } => Some(self.reference_weight),
                    WeightScheme::Any => Some(30),
                    WeightScheme::None => None,
                    WeightScheme::LinearBasedOnPrevious { .. } => Some(30),
                },
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use crate::programs::*;

    #[test]
    fn can_create_program() {
        Gzcl4Day::start(100);
    }

    #[test]
    fn stores_weights() {
        assert_eq!(
            format!("{}", Gzcl4Day::start(100).next_workout()[0]),
            "Weighted Pullup -> 4x3,1x3+ @ 20"
        );
        assert_eq!(
            format!("{}", Gzcl4Day::start(100).next_workout()[1]),
            "Pullup -> 3x7+"
        );
    }

    #[test]
    fn all_non_reference_weights_initialized_at_certain_value() {
        assert_eq!(
            format!("{}", Gzcl4Day::start(100).next_workout()[3]),
            "Face Pull -> 2x15,1x15-25 @ 30"
        );
    }
}
