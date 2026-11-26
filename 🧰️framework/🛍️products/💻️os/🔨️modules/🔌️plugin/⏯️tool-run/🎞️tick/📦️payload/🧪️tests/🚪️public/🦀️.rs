//! 🚪️ The original shared tick payload helper and its original source fixture execute at the public package boundary.
#[path="../../🦀️.rs"]
mod original_payload;

/// 🏦️ Funded effect turns over one genuine fixture bank for the payload authority closure.
mod test_effect {
    use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant, RetainedWorkOutcome, RetainedWorkPoll};
    use semio_framework_value_testing_admission::{EffectBank, Wallet};
    pub fn bank() -> EffectBank { EffectBank::new(Wallet::new(RetainedCloneGrant { maximum_items: 1 << 20, maximum_copy_bytes: 1 << 30, maximum_capacity_bytes: 1 << 30, maximum_release_bytes: 1 << 30, maximum_depth: 1 << 20 }, 1 << 30)) }
    pub fn settled<T, E>(bank: &mut EffectBank, grant: RetainedCloneGrant, operation: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> RetainedWorkPoll<T, E>) -> RetainedWorkOutcome<T, E> {
        let poll = bank.run_with_source(grant, 16, |effect, source| {
            let mut operation = Some(operation);
            let mut result = None;
            semio_framework_value_testing_admission::with_original_invocation(Some(source.identity()), effect, &mut |effect| {
                result = Some(operation.take().expect("single original invocation")(effect));
            }).expect("complete the funded invocation");
            result.expect("original invocation ran")
        }).0;
        let turn = match poll { RetainedWorkPoll::Complete(turn) => turn, RetainedWorkPoll::Pending { .. } => panic!("a synchronous funded turn stayed pending") };
        let admitted = bank.take_admitted();
        assert!(admitted != Some(false) && (turn.work == 0 || admitted == Some(true)));
        turn
    }
}
