//! 🎟️ Runs host retirement laws under the original Work recipient and allocation marker.

use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant, RetainedWorkOutcome, ValueError};
use semio_framework_value_control_contract::with_original_invocation;
use semio_framework_value_testing_effect_bank::{EffectBank, Wallet};

pub fn effect_turn<R>(grant: RetainedCloneGrant, work: u64, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> R {
    let mut bank = EffectBank::new(Wallet::new(grant, work));
    effect_from_bank(&mut bank, grant, work, run)
}

pub fn effect_from_bank<R>(bank: &mut EffectBank, grant: RetainedCloneGrant, work: u64, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> R {
    let turn = bank.run_outcome(grant, work, |effect| {
        let source = effect.work_source_turn();
        let mut run = Some(run);
        let mut output = None;
        let mut invoke = |effect: &mut RetainedAllocationEffect<'_>| {
            output = run.take().map(|run| run(effect));
        };
        let result = with_original_invocation(source, effect, &mut invoke)
            .and_then(|()| output.ok_or_else(|| ValueError::invariant("Drawing fixture action did not run")));
        RetainedWorkOutcome { result, work: effect.performed_work() }
    });
    assert_ne!(bank.take_admitted(), Some(false));
    turn.result.unwrap()
}
