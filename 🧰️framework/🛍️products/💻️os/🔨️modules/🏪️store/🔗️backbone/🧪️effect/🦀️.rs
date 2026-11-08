//! 🧪️ Test fixture that funds one backbone retirement turn through an authoring wallet.
use semio_framework_value::{AuthoringWallet, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneStep, RetainedWorkPoll};

pub(crate) fn step(poll: OriginalRetirementPoll) -> RetainedCloneStep {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn.result.unwrap(),
        RetainedWorkPoll::Pending { .. } => panic!("fixture turn stayed pending"),
    }
}

pub(crate) fn funded<R>(grant: RetainedCloneGrant, operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> R {
    AuthoringWallet::new(grant, 1 << 20).run(grant, 64, operation).unwrap().expect("quote admitted").output
}
