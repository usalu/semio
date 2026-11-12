//! ⏱️ Retained work outcomes cross owned cursors with their independent actual work.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::RetirementStep;
use semio_framework_value::{RetainedWorkOutcome, ValueError, ValueRefusalKind};
/// 🧭️ A retirement owner that was expected to exist but was omitted becomes an invariant refusal.
pub fn owned_step(turn: RetainedWorkOutcome<Option<RetainedCloneStep>>, message: &'static str) -> RetainedWorkOutcome {
    RetainedWorkOutcome { work: turn.work, result: turn.result.and_then(|step| step.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, message))) }
}
/// 🧭️ One owned close turn reports the cursor step for the work it spent.
pub fn cursor_step(turn: RetainedWorkOutcome, empty: bool) -> RetirementStep {
    match turn.result {
        Ok(RetainedCloneStep::Complete(progress)) if progress == RetainedCloneProgress::default() && empty => RetirementStep::Complete(turn.work),
        Ok(step) => RetirementStep::Progress(step.progress(), turn.work),
        Err(error) => RetirementStep::Failure(error.into(), turn.work),
    }
}
/// 🧭️ A completed refusal-free turn without custody work.
pub fn idle_step(step: RetainedCloneStep) -> RetainedWorkOutcome {
    RetainedWorkOutcome { result: Ok(step), work: 0 }
}
/// 🧭️ A refusal that spent no work.
pub fn refused(error: ValueError) -> RetainedWorkOutcome {
    RetainedWorkOutcome { result: Err(error), work: 0 }
}
/// 🧭️ A retirement begin that found no owner lets the next owner try.
pub fn begun(turn: RetainedWorkOutcome<Option<RetainedCloneStep>>) -> Option<RetainedWorkOutcome> {
    match turn.result {
        Ok(Some(step)) => Some(RetainedWorkOutcome { result: Ok(step), work: turn.work }),
        Ok(None) => None,
        Err(error) => Some(RetainedWorkOutcome { result: Err(error), work: turn.work }),
    }
}
/// 🧭️ A single bounded construction action spends one unit of work.
pub fn costed<T, E>(result: Result<T, E>) -> RetainedWorkOutcome<T, E> {
    RetainedWorkOutcome { result, work: 1 }
}
/// 🧭️ A retirement owner that was expected to exist but was omitted becomes the caller's refusal.
pub fn own_with(turn: RetainedWorkOutcome<Option<RetainedCloneStep>>, missing: impl FnOnce() -> ValueError) -> RetainedWorkOutcome {
    RetainedWorkOutcome { work: turn.work, result: turn.result.and_then(|step| step.ok_or_else(missing)) }
}
/// 🧭️ A polled close keeps its state: a suspended child retries next turn as zero progress and only a settled failure is a refusal.
pub fn settled(poll: semio_framework_value::OriginalRetirementPoll) -> RetainedWorkOutcome {
    match poll {
        semio_framework_value::RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work },
        semio_framework_value::RetainedWorkPoll::Complete(turn) => RetainedWorkOutcome { result: turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error), work: turn.work },
    }
}
/// 🧭️ One local event pays one unit to the effect; `false` means the quote was not funded and nothing was charged.
pub fn paid(effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Result<bool, ValueError> {
    if effect.remaining_work_units()? < 1 {
        return Ok(false);
    }
    if effect.performed(1)? { Ok(true) } else { Err(ValueError::literal(ValueRefusalKind::WorkLimit, "IFC local event exceeded its quoted work")) }
}
/// 🧭️ Runs one quoted step inside the turn's own original effect and charges the turn exactly the work it performed; `None` means the quote was not admitted and nothing ran.
pub fn effected<R>(cx: &mut semio_framework_job::StepContext<'_>, run: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> R) -> Result<Option<R>, semio_framework_job::OriginalTurnFailure> {
    let grant = cx.retained_grant();
    let Some((output, performed)) = cx.with_original_effect(grant, |_, effect| run(effect))? else { return Ok(None) };
    cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
    Ok(Some(output))
}
/// 🧭️ Retires one resident owner through the shared close field; a vacant begin lets the next owner try.
pub fn begun_in<T: semio_framework_value::retirement::RetireOwned>(retirement: &mut semio_framework_value::retained_clone::RetainedCloneClose, slot: &mut Option<T>, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> Option<RetainedWorkOutcome> {
    begun(retirement.begin_granted(slot, effect))
}
/// 🧭️ A grant-shaped leaf close reports its work, which the effect then pays once; an unfunded unit is a typed work-limit refusal.
pub fn charged(effect: &mut semio_framework_value::RetainedAllocationEffect<'_>, turn: RetainedWorkOutcome) -> RetainedWorkOutcome {
    let paid = match u64::try_from(turn.work) {
        Ok(0) => Ok(true),
        Ok(work) => effect.performed(work),
        Err(_) => Ok(false),
    };
    let result = match paid {
        Ok(true) => turn.result,
        Ok(false) => Err(ValueError::literal(ValueRefusalKind::WorkLimit, "IFC leaf work exceeds its effect quote")),
        Err(error) => Err(error),
    };
    RetainedWorkOutcome { result, work: effect.performed_work() }
}
/// 🧭️ Quotes the control one settled construction adds around its constructor so a grant admits it in a single turn.
pub fn settled_construction(demand: semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    let construction = semio_framework_job::OriginalConstructionDemand { items: 1, copy_bytes: demand.copy_bytes, capacity_bytes: demand.capacity_bytes, release_bytes: demand.release_bytes, depth: demand.depth.max(1) };
    let (grant, _) = semio_framework_job::original_construction_settled_turn_quote::<Option<semio_framework_value::DslValue>>(construction, 1)?;
    Ok(semio_framework_value::RetirementDemand { copy_bytes: grant.maximum_copy_bytes, capacity_bytes: grant.maximum_capacity_bytes, release_bytes: grant.maximum_release_bytes, depth: grant.maximum_depth })
}
/// 🧾️ Folds a typed native receiving failure into its cause with the receipt it retained.
pub fn receiving_fault<E: semio_framework_value::native_decoding::NativeReceivingEffectError>(error: E) -> ValueError {
    let progress = error.receiving_progress();
    error.cause().cloned().unwrap_or_else(|| ValueError::invariant("native receiving failed without a typed cause")).with_retained_progress(progress)
}
/// 🫴️ One native receiving turn of an effect-shaped cursor; an unstarted frame is cancelled so the turn retries intact.
pub fn receive_effect<E: semio_framework_value::native_decoding::NativeReceivingEffectError>(
    cx: &mut semio_framework_job::StepContext<'_>,
    demand: semio_framework_value::RetirementDemand,
    run: impl FnOnce(&mut semio_framework_value::NativeDecodeControl<'_>, &mut semio_framework_value::RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<semio_framework_value::RetainedCloneStep, E>,
) -> Result<(), semio_framework_job::OriginalTurnFailure> {
    use semio_framework_value::native_decoding::{NativeReceivingEffect, NativeReceivingEffectPhase, NativeReceivingEffectPoll};
    if cx.should_yield() || !semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(), demand) {
        return Ok(());
    }
    let maximum = cx.fuel_remaining();
    let mut out = None;
    let mut frame = NativeReceivingEffect::<E>::default();
    let mut done = None;
    cx.with_native_receiving(&mut out, |native, _| {
        let polled = native.receive_effect(demand, maximum, &mut frame, run)?;
        if matches!(polled, NativeReceivingEffectPoll::Pending { work: 0 }) && frame.phase() == NativeReceivingEffectPhase::Granted {
            frame.cancel_unstarted(native)?
        }
        done = Some(polled);
        Ok(())
    })?;
    match done {
        Some(NativeReceivingEffectPoll::Complete(turn)) => {
            semio_framework_job::receive_original_receiving_work(cx, RetainedWorkOutcome { result: turn.result.map_err(receiving_fault), work: turn.work })?;
            Ok(())
        }
        Some(NativeReceivingEffectPoll::Pending { work: 0 }) | None => Ok(()),
        Some(NativeReceivingEffectPoll::Pending { .. }) => Err(ValueError::invariant("native receiving effect suspended mid-turn").into()),
    }
}
/// 🪙️ Runs one quoted owner close inside the context's own effect and settles the outcome's performed work once.
pub fn closed_in(cx: &mut semio_framework_job::StepContext<'_>, demand: semio_framework_value::RetirementDemand, run: impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>) -> RetainedWorkOutcome) -> Result<semio_framework_job::InteractiveJobCloseStep, semio_framework_job::OriginalTurnFailure> {
    semio_framework_job::close_original_effect_step(cx, demand, |effect| semio_framework_value::RetainedWorkPoll::Complete(run(effect)))
}
/// ♻️ Seeds a self-closing owner whose inherent `begin_close`, `close_demands(body)`, `close_owned(body, effect)` and `empty` drive its reply-shaped retirement.
#[macro_export]
macro_rules! retire_self {
    ([$($generics:tt)*] $type:ty) => {
        impl<$($generics)*> $crate::retained::__value::retirement::RetireOwned for $type {
            type RetirementCursor = Self;
            fn retirement_seed(mut self) -> Self {
                self.begin_close();
                self
            }
            fn retirement_birth_bytes(&self) -> Option<usize> {
                Some(::std::mem::size_of::<Self>())
            }
            fn controlled_retirement_supported() -> bool {
                true
            }
        }
        impl<$($generics)*> $crate::retained::__value::retirement::RetirementCursor for $type {
            fn close_into<'reply>(&mut self, body: usize, effect: &mut $crate::retained::__value::RetainedAllocationEffect<'_>, reply: $crate::retained::__value::retirement::__control::OriginalEffectReply<'reply, $crate::retained::__value::retirement::RetirementStep, $crate::retained::__value::OriginalRetirementFailure>) -> Result<$crate::retained::__value::retirement::__control::OriginalEffectReplyComplete<'reply, $crate::retained::__value::retirement::RetirementStep, $crate::retained::__value::OriginalRetirementFailure>, $crate::retained::__value::ValueError> {
                if effect.invocation().is_none() {
                    return Err($crate::retained::__value::ValueError::invariant("guidance retirement requires its original producer invocation"));
                }
                let turn = self.close_owned(body, effect);
                let empty = self.empty();
                Ok(match $crate::cursor_step(turn, empty) {
                    $crate::retained::__value::retirement::RetirementStep::Failure(error, _) => reply.complete(Err(error)),
                    step => reply.complete(Ok(step)),
                })
            }
            fn next_computational_work(&self, _: usize) -> Result<u64, $crate::retained::__value::ValueError> {
                Ok(1)
            }
            fn next_demand(&self, body: usize) -> Result<$crate::retained::__value::RetirementDemand, $crate::retained::__value::ValueError> {
                self.close_demands(body)
            }
            fn terminal_is_empty(&self) -> bool {
                self.empty()
            }
            fn terminal_release_bytes(&self) -> Option<usize> {
                self.empty().then_some(::std::mem::size_of::<Self>())
            }
        }
    };
}
#[doc(hidden)]
pub use semio_framework_value as __value;
