//! ⏱️ The interactive step ceiling as the runtime enforces it, for the engine's worker-step laws.
//!
//! @see 🧰️framework/🔨️modules/⏱️trace/🦀️.rs — `StepOverrunLedger`, `SUSTAINED_OVERRUN_QUARANTINE_STEPS`

use std::cell::Cell;

thread_local! {
    static CONSECUTIVE_OVERRUNS: Cell<u32> = const { Cell::new(0) };
}

/// ⏱️ Admits one measured worker step on this test's thread. A step within `INTERACTIVE_STEP_CEILING_US` resets the
/// run; an over-ceiling step extends it, and a run of `SUSTAINED_OVERRUN_QUARANTINE_STEPS` consecutive over-ceiling
/// steps fails — the run production's `StepOverrunLedger` quarantines, never a single descheduled reading.
#[track_caller]
pub(crate) fn admit_step(elapsed: std::time::Duration, what: std::fmt::Arguments<'_>) {
    let elapsed_us = u64::try_from(elapsed.as_micros()).unwrap_or(u64::MAX);
    let run = CONSECUTIVE_OVERRUNS.with(|overruns| {
        let next = if semio_framework_job::interactive_step_contract_violated(elapsed_us) { overruns.get() + 1 } else { 0 };
        overruns.set(next);
        next
    });
    assert!(run < semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS, "{what}: {run} consecutive steps exceeded the {} us interactive ceiling (last {elapsed_us} us)", semio_framework_job::INTERACTIVE_STEP_CEILING_US);
}

/// ⚖️ LAW: a single over-ceiling reading is recorded and forgotten once a step is admitted again, and the
/// `SUSTAINED_OVERRUN_QUARANTINE_STEPS`-th consecutive over-ceiling step fails — the ledger's own rule, not slack.
#[test]
fn a_descheduled_step_is_recorded_and_a_sustained_run_fails() {
    let over = std::time::Duration::from_micros(semio_framework_job::INTERACTIVE_STEP_CEILING_US + 1);
    let sustained = semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS;
    for _ in 1..sustained {
        admit_step(over, format_args!("descheduled step"));
    }
    admit_step(std::time::Duration::ZERO, format_args!("admitted step"));
    for _ in 1..sustained {
        admit_step(over, format_args!("descheduled step"));
    }
    assert!(std::panic::catch_unwind(|| admit_step(over, format_args!("sustained overrun"))).is_err(), "the sustained run fails");
}
