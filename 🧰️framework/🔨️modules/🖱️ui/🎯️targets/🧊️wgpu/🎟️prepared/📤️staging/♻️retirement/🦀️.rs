//! ♻️ Shared bounded close receipt and original backing retirement contracts.
use semio_framework_job::{InteractiveJobCloseStep as CloseStep, OriginalTurnFailure, RetainedCloneGrant, RetainedCloneProgress, StepContext};
use semio_framework_value::{AuthoringWallet, OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneStep, RetainedWorkPoll, RetirementDemand, RetirementPollExit, ValueError, ValueRefusalKind};
use std::mem::size_of;

/// 🧾️ One prepared close ladder step under the effect of the turn that funds it.
pub type PreparedClose = Result<CloseStep, RetirementPollExit>;

pub fn prepared_close_gate<E: From<ValueError>>(grant: RetainedCloneGrant, terminal: bool, release: Result<usize, ValueError>) -> Option<Result<CloseStep,E>> {
    if terminal { return Some(Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() })); }
    let bytes = match release { Ok(bytes) => bytes, Err(error) => return Some(Err(error.into())) };
    if grant.maximum_items == 0 || bytes > grant.maximum_release_bytes { return Some(Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() })); }
    if grant.maximum_depth == 0 { return Some(Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original prepared close requires depth").into())); }
    None
}

pub fn prepared_close_progress<E>(released_bytes: usize) -> Result<CloseStep,E> {
    Ok(CloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes } })
}

pub fn prepared_close_child<A, E: From<A>>(step: Result<CloseStep,A>) -> Result<CloseStep,E> {
    step.map(|step|match step { CloseStep::Complete { progress } => CloseStep::Pending { progress }, step => step }).map_err(Into::into)
}

/// 🪆️ Forwards a child effect poll into the ladder: a pending child stays pending and a settled failure is kept whole.
pub fn prepared_close_poll(poll: OriginalRetirementPoll) -> PreparedClose {
    RetirementPollExit::resume(poll).map(|step| match step { RetainedCloneStep::Complete(progress) => CloseStep::Complete { progress }, RetainedCloneStep::Progress(progress) => CloseStep::Pending { progress } })
}

/// 🧾️ Settles one ladder step against its effect once; `blocked` and `complete` keep the two receipts a retained step cannot carry.
fn prepared_close_receipt(effect: &mut RetainedAllocationEffect<'_>, close: PreparedClose, blocked: &mut bool, complete: &mut bool) -> OriginalRetirementPoll {
    let result = close.map(|step| match step {
        CloseStep::Pending { progress } => RetainedCloneStep::Progress(progress),
        CloseStep::Complete { progress } => { *complete = true; RetainedCloneStep::Complete(progress) }
        CloseStep::Blocked => { *blocked = true; RetainedCloneStep::Progress(RetainedCloneProgress::default()) }
    });
    RetirementPollExit::settle(effect, result)
}

fn prepared_close_shape(step: CloseStep, blocked: bool, complete: bool) -> CloseStep {
    if blocked { CloseStep::Blocked } else if complete { CloseStep::Complete { progress: step.progress() } } else { step }
}

/// 🪙️ Runs one prepared close ladder step inside the context's own performed-work effect: the single debit is the job's receipt of the settled poll.
pub fn prepared_close_turn(cx: &mut StepContext<'_>, quote: RetainedCloneGrant, close: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> PreparedClose) -> Result<CloseStep,OriginalTurnFailure> {
    let demand = RetirementDemand { copy_bytes: quote.maximum_copy_bytes, capacity_bytes: quote.maximum_capacity_bytes, release_bytes: quote.maximum_release_bytes, depth: quote.maximum_depth };
    let (mut blocked, mut complete) = (false, false);
    let step = semio_framework_job::close_original_effect_step(cx, demand, |effect| { let result = close(effect); prepared_close_receipt(effect, result, &mut blocked, &mut complete) })?;
    Ok(prepared_close_shape(step, blocked, complete))
}

/// 🏦️ Runs one prepared close ladder step of a turn-less owner (fixtures, standalone hosts) under an explicit finite stock that is the grant itself.
pub fn prepared_close_authored(grant: RetainedCloneGrant, close: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> PreparedClose) -> Result<CloseStep, OriginalRetirementFailure> {
    let (mut blocked, mut complete) = (false, false);
    let authored = AuthoringWallet::new(grant, 64).run(grant, 64, |effect| { let result = close(effect); prepared_close_receipt(effect, result, &mut blocked, &mut complete) })?;
    let Some(authored) = authored else { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "original prepared close work quote exceeds its authored stock").into()) };
    let step = match authored.output {
        RetainedWorkPoll::Pending { .. } => CloseStep::Pending { progress: RetainedCloneProgress::default() },
        RetainedWorkPoll::Complete(turn) => CloseStep::Pending { progress: turn.result?.progress() },
    };
    Ok(prepared_close_shape(step, blocked, complete))
}

pub fn prepared_vec_bytes<T>(value: &Vec<T>) -> Result<usize, ValueError> {
    value.capacity().checked_mul(size_of::<T>()).ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "prepared backing extent overflow"))
}

pub fn prepared_owner_demands(terminal: bool, release: Result<usize, ValueError>) -> Result<RetainedCloneGrant, ValueError> {
    Ok(RetainedCloneGrant { maximum_items: usize::from(!terminal), maximum_copy_bytes: 0, maximum_capacity_bytes: 0, maximum_release_bytes: release?, maximum_depth: usize::from(!terminal) })
}

pub fn prepared_close_text(key: &mut String) -> usize {
    if key.pop().is_some() { return 0; }
    let bytes = key.capacity();
    *key = String::new();
    bytes
}

pub fn prepared_close_bytes(bytes: &mut Vec<u8>, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
    if !bytes.is_empty() {
        if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original prepared bytes close requires depth")); }
        let processed = bytes.len().min(grant.maximum_copy_bytes);
        bytes.truncate(bytes.len() - processed);
        return Ok(CloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, copied_bytes: processed, ..Default::default() } });
    }
    let release = bytes.capacity();
    if grant.maximum_items == 0 || release > grant.maximum_release_bytes { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
    if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original prepared byte backing close requires depth")); }
    drop(std::mem::take(bytes));
    prepared_close_progress(release)
}

