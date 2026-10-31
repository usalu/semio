//! 📮️ Funded payload-builder turns shared by every fem numerical job and its contract tests: one boxed builder slot is opened, staged, finished and closed one granted turn at a time.

use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep, JobOutcomeBorrow, OriginalTurnFailure, OriginalTurnPrimary, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, StepContext, close_original_retained_step, receive_original_retained_work};
use semio_framework_job_payload::{JobPayloadStream, RetainedJobPayload, RetainedPayloadBuilder, RetainedWorkOutcome};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

/// 🎟️ Whether one grant covers one quoted demand with at least one item.
pub fn numerical_grant_covers(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

/// 🧯️ Maps one payload or writer close turn onto the job close vocabulary.
pub fn numerical_close_result(step: Result<RetainedCloneStep, ValueError>) -> Result<InteractiveJobCloseStep, ValueError> {
    let (RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) = step?;
    Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } })
}

/// 📦️ One boxed payload builder lives in a job slot until its funded close turns have released every page, ledger and the box itself.
pub type NumericalPayloadSlot = Option<Box<RetainedPayloadBuilder>>;

/// 🗣️ Projects every original turn refusal onto the numerical job's value vocabulary without losing its effect receipt.
pub fn numerical_turn_error(failure: OriginalTurnFailure) -> ValueError {
    let effects = failure.effects;
    let error = match failure.primary {
        OriginalTurnPrimary::Value(error) => error,
        OriginalTurnPrimary::Fault(fault) => ValueError::new(ValueRefusalKind::InvariantViolated, fault.message.to_string()),
        OriginalTurnPrimary::Domain(original) => ValueError::new(ValueRefusalKind::InvariantViolated, original.message().to_owned()),
        OriginalTurnPrimary::Receiving(original) => ValueError::new(ValueRefusalKind::InvariantViolated, original.message().to_owned()),
    };
    error.with_retained_progress(effects)
}

/// 📮️ Posts one funded builder turn to the step ledger; true once that turn completed its action, false when it must be re-entered after a yield.
pub fn numerical_post(context: &mut StepContext<'_>, turn: RetainedWorkOutcome) -> Result<bool, ValueError> {
    receive_original_retained_work(context, turn).map(|step| matches!(step, RetainedCloneStep::Complete(_))).map_err(numerical_turn_error)
}

/// 🪺️ Admits the boxed builder, then initializes its payload bindings one funded turn at a time; true once the writer is usable.
pub fn numerical_open_writer(slot: &mut NumericalPayloadSlot, stream: JobPayloadStream, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
    if slot.is_none() {
        let turn = RetainedPayloadBuilder::admit_boxed(stream, slot, &*context);
        numerical_post(context, turn)?;
        return Ok(false);
    }
    let builder = slot.as_mut().map(|builder| &mut **builder);
    let Some(builder) = builder else { return Ok(false) };
    if builder.is_initialized() {
        return Ok(true);
    }
    let turn = builder.advance_initialization(&*context);
    numerical_post(context, turn)
}

/// 🧱️ Begins the next canonical staged page of an initialized builder; true once the page is staged.
pub fn numerical_begin_page(writer: &mut RetainedPayloadBuilder, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
    let turn = writer.begin_staged_page_funded(context);
    numerical_post(context, turn)
}

/// 📤️ Commits the staged page into the builder's payload; true once the page is installed.
pub fn numerical_commit_page(writer: &mut RetainedPayloadBuilder, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
    let turn = writer.commit_staged_page_funded(context);
    numerical_post(context, turn)
}

/// 🏁️ Seals the finished payload, transfers it to the desk's lent place, then closes the empty builder and releases its box; true once the slot is empty.
pub fn numerical_finish_writer(slot: &mut NumericalPayloadSlot, lent: &mut Option<RetainedJobPayload>, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
    let Some(builder) = slot.as_mut().map(|builder| &mut **builder) else { return Ok(true) };
    if builder.terminal_is_empty() {
        let demand = RetainedPayloadBuilder::boxed_release_demands();
        let released = close_original_retained_step(context, demand, |_| {
            drop(slot.take());
            RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() })), work: 1 }
        });
        released.map_err(numerical_turn_error)?;
        return Ok(slot.is_none());
    }
    if lent.is_some() {
        let demand = builder.retirement_demands()?;
        close_original_retained_step(context, demand, |grant| builder.close_step_granted(grant)).map_err(numerical_turn_error)?;
        return Ok(false);
    }
    if builder.published().is_none() {
        let turn = builder.seal(&*context);
        numerical_post(context, turn)?;
        return Ok(false);
    }
    let grant = context.retained_grant();
    let turn = builder.take_finished(lent, grant);
    numerical_post(context, turn)?;
    Ok(false)
}

/// 📦️ One prepared byte string becomes one payload over funded turns: open the builder, append every byte, then seal, transfer and release.
pub struct NumericalBytesPublication {
    stream: JobPayloadStream,
    bytes: Vec<u8>,
    cursor: usize,
    appended: bool,
    writer: NumericalPayloadSlot,
}

impl NumericalBytesPublication {
    pub fn new(stream: JobPayloadStream, bytes: Vec<u8>) -> Self { Self { stream, bytes, cursor: 0, appended: false, writer: None } }

    /// ▶️ Advances exactly one funded builder turn; true once the finished payload sits in `lent`.
    pub fn advance(&mut self, lent: &mut Option<RetainedJobPayload>, context: &mut StepContext<'_>) -> Result<bool, ValueError> {
        if self.writer.as_ref().is_none_or(|writer| !writer.is_initialized()) {
            numerical_open_writer(&mut self.writer, self.stream, context)?;
            return Ok(false);
        }
        if !self.appended {
            let builder = self.writer.as_deref_mut().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "numerical bytes publication lost its builder"))?;
            let turn = builder.append_original(context, &self.bytes, &mut self.cursor);
            self.appended = numerical_post(context, turn)?;
            return Ok(false);
        }
        numerical_finish_writer(&mut self.writer, lent, context)
    }

    /// 📏️ Quotes the next close turn of the half-built builder; `None` once nothing remains.
    pub fn close_demand(&self) -> Option<Result<RetirementDemand, ValueError>> { numerical_writer_demand(&self.writer) }

    /// 🧹️ Closes the half-built builder under a covering grant; `None` once nothing remains.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Option<Result<InteractiveJobCloseStep, ValueError>> { numerical_writer_close(&mut self.writer, grant) }
}

/// 🧯️ Closes one half-built writer slot under a grant that covers its own quote; `None` when the slot is empty.
pub fn numerical_writer_close(slot: &mut NumericalPayloadSlot, grant: RetainedCloneGrant) -> Option<Result<InteractiveJobCloseStep, ValueError>> {
    let builder = slot.as_mut()?;
    if builder.terminal_is_empty() {
        let demand = RetainedPayloadBuilder::boxed_release_demands();
        if !numerical_grant_covers(grant, demand) {
            return Some(Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }));
        }
        *slot = None;
        return Some(Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() } }));
    }
    let demand = match builder.retirement_demands() {
        Ok(demand) => demand,
        Err(error) => return Some(Err(error)),
    };
    if !numerical_grant_covers(grant, demand) {
        return Some(Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }));
    }
    Some(numerical_close_result(builder.close_step_granted(grant).result))
}

/// 📏️ Quotes a half-built writer slot's next close turn; `None` when the slot is empty.
pub fn numerical_writer_demand(slot: &NumericalPayloadSlot) -> Option<Result<RetirementDemand, ValueError>> {
    slot.as_ref().map(|builder| if builder.terminal_is_empty() { Ok(RetainedPayloadBuilder::boxed_release_demands()) } else { builder.retirement_demands() })
}


/// 🦶️ A numerical job exposes its plain step outcome so contract tests and the claim-taking `InteractiveJob::step` share one body.
pub trait NumericalStep: InteractiveJob {
    /// ▶️ Advances one original step and lends its borrowed outcome to the caller.
    fn step_plain<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, OriginalTurnFailure>;
}
