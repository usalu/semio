use semio_framework_value::{
    retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep},
    retirement::{controlled::ControlledRetirement, RetireOwned},
    OriginalRetirementFailure, RetirementDemand,
};
use semio_framework_value_clone_source_testing::drive::drive;

pub(super) fn grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

pub(super) fn receipt(result: &Result<RetainedCloneStep, OriginalRetirementFailure>) -> RetainedCloneProgress {
    match result {
        Ok(step) => step.progress(),
        Err(error) => error.retained_progress(),
    }
}

pub(super) fn retire<T: RetireOwned>(value: T) {
    let mut owner = ControlledRetirement::new(value).unwrap_or_else(|(error, _)| panic!("{error}"));
    for _ in 0..4096 {
        if owner.terminal_is_empty() {
            return;
        }
        let grant = grant(owner.next_demand(4096).unwrap());
        let work = owner.next_computational_work(4096).unwrap();
        let step = drive(grant, work, |effect| owner.step(4096, effect)).result.unwrap_or_else(|_| panic!("fixture retirement refused"));
        assert!(step.progress().fits(grant));
    }
    panic!("fixture retirement did not settle");
}
