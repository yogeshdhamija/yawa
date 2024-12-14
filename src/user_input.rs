pub enum Action {
    StartProgram {
        reference_weight: usize
    },
    SeeStatus,
    SeeNextDay,
    CompleteDay
}
