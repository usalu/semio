//! ⏱️ Drives one effect-shaped receipt step under a real original action; the harness asserts that the physical receipt equals the observed heap.
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::{OriginalRetirementFailure, RetainedAllocationEffect, RetainedCloneGrant, RetainedWorkOutcome, RetainedWorkPoll, ValueError};

/// 🧵️ Runs one receipt step with the effect of a fresh original turn and returns its recorded reply.
/// @see <https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html>
pub(crate) fn driven(grant: RetainedCloneGrant, mut call: impl FnMut(&mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError>) -> Result<RetainedCloneStep, OriginalRetirementFailure> {
    semio_framework_value_clone_source_testing::drive::warm();
    semio_framework_value_clone_source_testing::drive::drive(grant, 1 << 10, |effect| {
        let result = call(effect);
        let work = effect.performed_work();
        RetainedWorkPoll::Complete(RetainedWorkOutcome { result: result.map_err(Into::into), work })
    }).result
}

/// 🧮️ Funds the copy axis of a fixture grant with the original shared author admission quote on top of the step's own copy policy.
/// @see <https://doc.rust-lang.org/std/cmp/trait.Ord.html#method.max>
pub(crate) fn funded_copy(policy: usize) -> usize { policy.max(semio_framework_value::SharedUtf8::admission_demand().copy_bytes) }
