//! 🤝️ Shared numerical job protocol: outcome desk, close ladder, batch steps and owner close helpers.

use super::*;

pub const SPARSE_INDEX_SPACE: usize = usize::MAX;
pub const SPARSE_PAGE_BYTES: usize = 4096;

/// 🧭️ What one internal step of a numerical job decided before any outcome is lent to the caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericalRun {
    Yield,
    Cancelled,
    Fault,
    Preview,
    Checkpoint(u64),
    Complete,
}

/// 🎁️ Owns the one finished payload a numerical job lends at a time, retries a refused admission and pays the lent payload back from the next call's wallet.
pub struct NumericalOutcomeDesk {
    pub lent: Option<RetainedJobPayload>,
    pub delivered: bool,
    pub pending: Option<NumericalRun>,
    pub state_marker: RetainedJobPayload,
    pub output_marker: RetainedJobPayload,
    pub fault_marker: RetainedJobPayload,
}

impl Default for NumericalOutcomeDesk {
    fn default() -> Self {
        Self {
            lent: None,
            delivered: false,
            pending: None,
            state_marker: RetainedJobPayload::empty(JobPayloadStream::CommitState),
            output_marker: RetainedJobPayload::empty(JobPayloadStream::CommitOutput),
            fault_marker: RetainedJobPayload::empty(JobPayloadStream::Fault),
        }
    }
}

/// 🎟️ Whether one grant covers one quoted demand with at least one item.
pub fn numerical_grant_covers(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items != 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth
}

/// 🚪️ Whether a grant admits one close turn: a payload or writer frontier must be covered by its own quote, an owner rung is bounded by its own release limit.
pub fn numerical_close_gate(grant: RetainedCloneGrant, frontier: Option<RetirementDemand>) -> bool {
    match frontier {
        Some(demand) => numerical_grant_covers(grant, demand),
        None => grant.maximum_items != 0 && grant.maximum_depth != 0,
    }
}

/// 📏️ The largest single owner release one numerical close turn performs: every admitted owner is at most one owner page.
pub const NUMERICAL_CLOSE_RELEASE_BYTES: usize = NUMERICAL_OWNER_PAGE_BYTES;

/// 🪜️ Quotes one ladder rung of a numerical job's retained owners.
pub fn numerical_rung_demand(terminal: bool) -> RetirementDemand {
    if terminal { RetirementDemand::default() } else { RetirementDemand { release_bytes: NUMERICAL_CLOSE_RELEASE_BYTES, depth: 1, ..Default::default() } }
}

/// 🧯️ Maps one payload or writer close turn onto the job close vocabulary.
pub fn numerical_close_result(step: Result<RetainedCloneStep, ValueError>) -> Result<InteractiveJobCloseStep, ValueError> {
    let (RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) = step?;
    Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } })
}

/// 📏️ A job's own quote for its next close turn, read from its four declared demand currencies.
pub fn numerical_close_demand(job: &impl InteractiveJob) -> Result<RetirementDemand, ValueError> {
    let copy_bytes = job.next_close_copy_byte_demand()?;
    Ok(RetirementDemand { copy_bytes, capacity_bytes: job.next_close_capacity_byte_demand(copy_bytes)?, release_bytes: job.next_close_release_byte_demand()?, depth: job.next_close_depth_demand()?.max(1) })
}

/// 🪙️ Runs one grant-shaped numerical close turn of `owner` under the caller's original wallet: the turn only runs
/// when that wallet covers the job's quote, its effects are debited once, and a terminal owner reports `Complete`.
pub fn numerical_close_original<J>(cx: &mut StepContext<'_>, owner: &mut J, demand: RetirementDemand, turn: impl FnOnce(&mut J, RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError>, terminal: impl FnOnce(&J) -> bool) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
    let step = close_original_retained_step(cx, demand, |grant| {
        Ok(match turn(owner, grant)? {
            InteractiveJobCloseStep::Complete { progress } => RetainedCloneStep::Complete(progress),
            InteractiveJobCloseStep::Pending { progress } => RetainedCloneStep::Progress(progress),
            InteractiveJobCloseStep::Blocked => RetainedCloneStep::Progress(RetainedCloneProgress::default()),
        })
    })?;
    Ok(if terminal(owner) { InteractiveJobCloseStep::Complete { progress: step.progress() } } else { step })
}

/// 🪜️ Maps one tuple-shaped owner ladder rung onto the job close vocabulary; `None` once the ladder is exhausted.
pub fn numerical_owner_turn(rung: Result<Option<(usize, usize)>, ()>) -> Option<InteractiveJobCloseStep> {
    match rung {
        Ok(Some((items, bytes))) => Some(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: items, released_bytes: bytes, ..Default::default() } }),
        Err(()) => Some(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }),
        Ok(None) => None,
    }
}

/// 🪜️ Maps a `(complete, items, bytes)` ladder turn onto the job close vocabulary.
pub fn numerical_ladder_turn(turn: (bool, usize, usize)) -> InteractiveJobCloseStep {
    let (complete, items, bytes) = turn;
    let progress = RetainedCloneProgress { copied_items: items, released_bytes: bytes, ..Default::default() };
    if complete { InteractiveJobCloseStep::Complete { progress } } else { InteractiveJobCloseStep::Pending { progress } }
}

/// 🧭️ What one batch-adapter step learned from its borrowed outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumericalBatchEnd {
    Continue,
    Complete,
    Fault,
    Cancelled,
}

/// ▶️ Advances one native numerical batch under its persistent caller's actual original context.
pub fn numerical_batch_step<J: InteractiveJob>(job: &mut J, context: &mut StepContext<'_>) -> Result<NumericalBatchEnd, OriginalTurnFailure> {
    Ok(match job.step(context)? {
        None | Some(JobOutcomeBorrow::Yield { .. } | JobOutcomeBorrow::PreviewReady { .. } | JobOutcomeBorrow::CheckpointReady { .. }) => NumericalBatchEnd::Continue,
        Some(JobOutcomeBorrow::Complete { .. }) => NumericalBatchEnd::Complete,
        Some(JobOutcomeBorrow::Fault { .. }) => NumericalBatchEnd::Fault,
        Some(JobOutcomeBorrow::Cancelled { .. }) => NumericalBatchEnd::Cancelled,
    })
}

/// 🧹️ Advances one native numerical close under the same original caller without renewing stock or cancellation.
pub fn numerical_batch_close<J: InteractiveJob>(job: &mut J, context: &mut StepContext<'_>) -> Result<InteractiveJobCloseStep, OriginalTurnFailure> {
    job.begin_close();
    job.close_step(context)
}

/// 🪺️ Maps a child job's close turn onto the parent's; a terminal child is removed by the caller after this turn.
pub fn numerical_child_close(step: InteractiveJobCloseStep, terminal: bool) -> InteractiveJobCloseStep {
    let progress = step.progress();
    match step {
        InteractiveJobCloseStep::Blocked => step,
        _ if terminal => InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress } },
        _ => InteractiveJobCloseStep::Pending { progress },
    }
}

/// 🧯️ Closes one retained payload under a grant that covers its own quote.
pub fn numerical_payload_close(payload: &mut RetainedJobPayload, grant: RetainedCloneGrant) -> Result<InteractiveJobCloseStep, ValueError> {
    let demand = payload.retirement_demands()?;
    if !numerical_grant_covers(grant, demand) {
        return Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() });
    }
    numerical_close_result(payload.close_step(grant))
}

/// 🧯️ Closes one half-built writer slot under a grant that covers its own quote; `None` when the slot is empty.
pub fn numerical_writer_close(slot: &mut Option<RetainedJobPayloadWriter>, grant: RetainedCloneGrant) -> Option<Result<InteractiveJobCloseStep, ValueError>> {
    let writer = slot.as_mut()?;
    writer.begin_close();
    let demand = match writer.retirement_demands() {
        Ok(demand) => demand,
        Err(error) => return Some(Err(error)),
    };
    if !numerical_grant_covers(grant, demand) {
        return Some(Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress::default() }));
    }
    let step = numerical_close_result(writer.close_step(grant));
    if writer.terminal_is_empty() {
        *slot = None;
    }
    Some(step)
}

/// 📏️ Quotes a half-built writer slot's next close turn; `None` when the slot is empty.
pub fn numerical_writer_demand(slot: &Option<RetainedJobPayloadWriter>) -> Option<Result<RetirementDemand, ValueError>> {
    slot.as_ref().map(RetainedJobPayloadWriter::retirement_demands)
}

/// 🔁️ Closes one restored checkpoint page, then the operation ledger, from the calling step's own wallet; true once the payload is terminal-empty.
pub fn numerical_restore_close_due(payload: &mut RetainedJobPayload, context: &mut StepContext<'_>, page_slot: &mut usize, page_entry: &mut usize, close_due: &mut bool, total_pages: usize) -> Result<bool, NumericalCheckpointFault> {
    let demand = payload.retirement_demands().map_err(|_| NumericalCheckpointFault::Admission)?;
    let grant = context.retained_grant();
    if !numerical_grant_covers(grant, demand) {
        return Ok(false);
    }
    let pages = payload.page_count();
    let step = payload.close_step(grant).map_err(|_| NumericalCheckpointFault::Admission)?;
    context.consume_retained(step.progress()).map_err(|_| NumericalCheckpointFault::Admission)?;
    if payload.page_count() < pages {
        *page_slot += 1;
        *page_entry = 0;
        if *page_slot < total_pages {
            *close_due = false;
            return Ok(false);
        }
    }
    Ok(payload.terminal_is_empty())
}

/// 🧯️ Closes a restore cursor's raw payload under a grant that covers its own quote; `None` once the payload is gone.
pub fn numerical_restore_payload_close(slot: &mut Option<RetainedJobPayload>, grant: RetainedCloneGrant) -> Option<Result<InteractiveJobCloseStep, ValueError>> {
    let payload = slot.as_mut()?;
    if payload.terminal_is_empty() {
        *slot = None;
        return Some(Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }));
    }
    Some(numerical_payload_close(payload, grant))
}

/// 📏️ Quotes a restore cursor's raw payload frontier; `None` once only owner rungs remain.
pub fn numerical_restore_frontier(payload: &Option<RetainedJobPayload>) -> Result<Option<RetirementDemand>, ValueError> {
    match payload {
        Some(payload) if !payload.terminal_is_empty() => payload.retirement_demands().map(Some),
        Some(_) => Ok(Some(RetirementDemand { depth: 1, ..Default::default() })),
        None => Ok(None),
    }
}

impl NumericalOutcomeDesk {
    pub fn lend_preview(&mut self, payload: RetainedJobPayload) -> NumericalRun {
        self.lent = Some(payload);
        NumericalRun::Preview
    }

    pub fn lend_checkpoint(&mut self, payload: RetainedJobPayload, applied_progress: u64) -> NumericalRun {
        self.lent = Some(payload);
        NumericalRun::Checkpoint(applied_progress)
    }

    pub fn lend_complete(&mut self, output: RetainedJobPayload) -> NumericalRun {
        self.lent = Some(output);
        NumericalRun::Complete
    }

    /// 🤝️ True when the previous call delivered a payload that this call must pay back before the job resumes.
    pub fn retire_due(&self, cx: &StepContext<'_>, operation: Operation) -> bool {
        self.delivered && !cx.is_cancelled() && cx.operation() == operation.operation && cx.generation() == operation.generation
    }

    pub fn take_pending(&mut self) -> Option<NumericalRun> {
        self.pending.take()
    }

    /// 💸️ Pays the delivered payload back turn by turn from the calling step's own wallet.
    pub fn retire_delivered(&mut self, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        let Some(payload) = self.lent.as_mut() else {
            self.delivered = false;
            return Ok(());
        };
        let demand = payload.retirement_demands()?;
        let grant = cx.retained_grant();
        if numerical_grant_covers(grant, demand) {
            let progress = match payload.close_step(grant)? {
                RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => progress,
            };
            cx.consume_retained(progress)?;
        }
        if payload.terminal_is_empty() {
            self.lent = None;
            self.delivered = false;
        }
        Ok(())
    }

    /// 📬️ Lends the decided outcome, or keeps it pending when the caller's grant cannot admit it yet.
    pub fn admit<'a>(&'a mut self, cx: &mut StepContext<'_>, run: NumericalRun) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let admitted = match run {
            NumericalRun::Yield => return Ok(None),
            NumericalRun::Cancelled => JobOutcomeBorrow::admit_cancelled(cx)?,
            NumericalRun::Fault => JobOutcomeBorrow::admit_fault(cx, &self.fault_marker)?,
            NumericalRun::Preview => JobOutcomeBorrow::admit_preview(cx, self.lent.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "numerical preview has no staged payload"))?)?,
            NumericalRun::Checkpoint(applied_progress) => JobOutcomeBorrow::admit_checkpoint(cx, self.lent.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "numerical checkpoint has no staged payload"))?, applied_progress)?,
            NumericalRun::Complete => JobOutcomeBorrow::admit_complete(cx, Some(&self.state_marker), Some(self.lent.as_ref().unwrap_or(&self.output_marker)))?,
        };
        if admitted.is_none() {
            self.pending = Some(run);
        } else if matches!(run, NumericalRun::Preview | NumericalRun::Checkpoint(_) | NumericalRun::Complete) {
            self.delivered = true;
        }
        Ok(admitted)
    }

    pub fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        let lent = || self.lent.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "numerical outcome has no staged payload"));
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Fault => descriptor.fault(&self.fault_marker),
            JobOutcomeKind::PreviewReady => descriptor.preview(lent()?),
            JobOutcomeKind::CheckpointReady { .. } => descriptor.checkpoint(lent()?),
            JobOutcomeKind::Complete => descriptor.complete(Some(&self.state_marker), Some(self.lent.as_ref().unwrap_or(&self.output_marker))),
        }
    }

    pub fn retirement_demands(&self) -> Option<Result<RetirementDemand, ValueError>> {
        self.lent.as_ref().map(RetainedJobPayload::retirement_demands)
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Option<Result<InteractiveJobCloseStep, ValueError>> {
        let payload = self.lent.as_mut()?;
        if payload.terminal_is_empty() {
            self.lent = None;
            self.delivered = false;
            return Some(Ok(InteractiveJobCloseStep::Pending { progress: RetainedCloneProgress { copied_items: 1, ..Default::default() } }));
        }
        let step = numerical_payload_close(payload, grant);
        if payload.terminal_is_empty() {
            self.lent = None;
            self.delivered = false;
        }
        Some(step)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.lent.is_none() && self.pending.is_none()
    }

    /// 🧹️ Pays back the lent payload through its own quoted grants; for owners that keep their job after completion.
    pub fn retire_all(&mut self) {
        while let Some(demand) = self.retirement_demands() {
            let demand = demand.expect("a locally owned lent payload quotes its close demand");
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
            let step = self.close_step(grant);
            assert!(matches!(step, Some(Ok(InteractiveJobCloseStep::Pending { .. } | InteractiveJobCloseStep::Complete { .. }))), "a locally owned lent payload close has no external owner and no refusal");
        }
        self.pending = None;
    }
}

pub fn close_vec_owner_step<T>(owner: &mut Vec<T>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if owner.pop().is_some() {
        return Ok(Some((1, 0)));
    }
    let bytes = owner.capacity().checked_mul(size_of::<T>()).ok_or(())?;
    if bytes == 0 {
        return Ok(None);
    }
    if bytes > maximum_bytes {
        return Err(());
    }
    *owner = Vec::new();
    Ok(Some((1, bytes)))
}

pub fn close_paged_owner_step<T, const N: usize>(owner: &mut PagedList<T, N>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if owner.pop().is_some() {
        return Ok(Some((1, 0)));
    }
    let progress = owner.release_empty_page(maximum_bytes).map_err(|_| ())?;
    if progress.progressed {
        return Ok(Some((0, progress.released_allocation_bytes)));
    }
    if owner.terminal_is_empty() {
        Ok(None)
    } else {
        Err(())
    }
}
pub fn paged_owner_retirement_demand<T,const N:usize>(owner:&PagedList<T,N>)->Result<RetirementDemand,ValueError>{let failure=|error:semio_framework_value_persistent::list::PagedListError|ValueError::literal(ValueRefusalKind::InvariantViolated,error.reason);if owner.is_empty(){Ok(RetirementDemand{release_bytes:owner.next_release_allocation_bytes().map_err(failure)?,depth:owner.next_release_depth_demand().map_err(failure)?,..Default::default()})}else{Ok(RetirementDemand{depth:owner.next_pop_depth_demand().map_err(failure)?,..Default::default()})}}

pub fn paged_from_values<T, I: IntoIterator<Item = T>>(values: I) -> PagedList<T, SPARSE_INDEX_SPACE> where I::IntoIter: ExactSizeIterator {
    let values = values.into_iter();
    let target = values.len();
    let mut owner = PagedList::default();
    while owner.capacity() < target {
        owner.reserve_capacity_one(target, SPARSE_PAGE_BYTES).expect("cold CSR page allocation");
    }
    for value in values {
        owner.push_reserved(value).unwrap_or_else(|_| unreachable!("cold CSR reserved exact slot"));
    }
    owner
}

pub fn close_nested_vec_owner_step<T>(owner: &mut Vec<Vec<T>>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if let Some(child) = owner.last_mut() {
        if let Some(step) = close_vec_owner_step(child, maximum_bytes)? {
            return Ok(Some(step));
        }
        owner.pop();
        return Ok(Some((1, 0)));
    }
    close_vec_owner_step(owner, maximum_bytes)
}
