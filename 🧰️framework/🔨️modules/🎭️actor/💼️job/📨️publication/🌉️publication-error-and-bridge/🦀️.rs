//! 🌉️ Publication error and bridge: the publication error and the test-only outcome projection and turn bridge. See [`super`].

use super::*;

/// 🚫️ Publication validation failure detected before actor-visible state can change.
#[derive(Debug)]
pub enum JobPublicationError {
    OperationMismatch { active: u64, published: u64 },
    Stale { live_revision: u64, live_generation: u64 },
    StepSequence { expected: u64, published: u64 },
    PreviewSequence { before: u64, after: u64 },
    Terminal,
    Retained(semio_framework_value::ValueError),
    Original(semio_framework_job::OriginalTurnFailure),
}

impl std::fmt::Display for JobPublicationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OperationMismatch { active, published } => write!(formatter, "job bridge operation mismatch: active={active}, published={published}"),
            Self::Stale { live_revision, live_generation } => write!(formatter, "job bridge stale revision/generation: live revision={live_revision}, generation={live_generation}"),
            Self::StepSequence { expected, published } => write!(formatter, "job bridge step sequence mismatch: expected={expected}, published={published}"),
            Self::PreviewSequence { before, after } => write!(formatter, "job bridge preview cursor mismatch: before={before}, after={after}"),
            Self::Terminal => formatter.write_str("job bridge received a turn after a terminal publication"),
            Self::Retained(error) => std::fmt::Display::fmt(error, formatter),
            Self::Original(error)=>formatter.write_str(error.primary.message()),
        }
    }
}

impl std::error::Error for JobPublicationError {}

#[cfg(test)]
struct JobOutcomeProjection {
    pub(super) descriptor: semio_framework_job::JobOutcomeDescriptor,
    pub(super) bytes: [Vec<u8>; 2],
    pub(super) phase: usize,
    pub(super) page: usize,
}

#[cfg(test)]
impl JobOutcomeProjection {
    pub(super) fn new(descriptor: semio_framework_job::JobOutcomeDescriptor) -> Self {
        Self { descriptor, bytes: [Vec::new(), Vec::new()], phase: 0, page: 0 }
    }

    pub(super) fn step<J: semio_framework_job::InteractiveJob + ?Sized>(&mut self, producer: &J, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<JobStepOutcome>, semio_framework_value::ValueError> {
        if !matches!(self.descriptor.kind(), semio_framework_job::JobOutcomeKind::Cancelled) && (cx.is_cancelled() || cx.fuel_remaining() == 0 || cx.latest_us().map(|now| now >= cx.deadline_us()).unwrap_or_else(|| cx.deadline_exceeded())) { return Ok(None); }
        let view = producer.borrow_outcome(&self.descriptor)?;
        let originals = match view {
            semio_framework_job::JobOutcomeView::PreviewReady { payload, .. } => [Some(payload), None],
            semio_framework_job::JobOutcomeView::CheckpointReady { state, .. } => [Some(state), None],
            semio_framework_job::JobOutcomeView::Complete { state, output, .. } => [state, output],
            semio_framework_job::JobOutcomeView::Fault { detail, .. } => [Some(detail), None],
            semio_framework_job::JobOutcomeView::Yield { .. } | semio_framework_job::JobOutcomeView::Cancelled { .. } => [None, None],
        };
        while self.phase < originals.len() {
            if let Some(page) = originals[self.phase].and_then(|original| original.page(self.page)) {
                let destination = &mut self.bytes[self.phase];
                let extent = destination.len().checked_add(page.len()).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original actor projection extent overflow"))?;
                let original_extent = originals[self.phase].unwrap().len();
                if extent > original_extent { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original actor projection exceeded admitted payload extent")); }
                let required = if destination.capacity() == 0 { original_extent } else { 0 };
                let grant = cx.retained_grant();
                if grant.maximum_items == 0 || grant.maximum_copy_bytes < page.len() || grant.maximum_capacity_bytes < required || grant.maximum_depth < 1 { return Ok(None); }
                if required != 0 {
                    let layout = std::alloc::Layout::array::<u8>(required).map_err(|_| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original actor projection layout overflow"))?;
                    let pointer = std::ptr::NonNull::new(unsafe { std::alloc::alloc(layout) }).ok_or_else(|| semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "original actor projection allocation failed"))?;
                    *destination = unsafe { Vec::from_raw_parts(pointer.as_ptr(), 0, required) };
                }
                let progress = semio_framework_job::RetainedCloneProgress { copied_items: 1, copied_bytes: page.len(), retained_capacity_bytes: required, ..Default::default() };
                cx.consume_retained(progress).map_err(|error| error.with_retained_progress(progress))?;
                destination.extend_from_slice(page);
                self.page += 1;
                return Ok(None);
            }
            self.phase += 1;
            self.page = 0;
        }
        let acknowledged = self.descriptor.acknowledge(cx.retained_grant());
        cx.consume_retained(acknowledged.progress())?;
        if !self.descriptor.is_acknowledged() { return Ok(None); }
        let first = std::mem::take(&mut self.bytes[0]);
        let second = std::mem::take(&mut self.bytes[1]);
        Ok(Some(match self.descriptor.kind() {
            semio_framework_job::JobOutcomeKind::Yield => JobStepOutcome::Yield,
            semio_framework_job::JobOutcomeKind::PreviewReady => JobStepOutcome::PreviewReady { preview: first },
            semio_framework_job::JobOutcomeKind::CheckpointReady { applied_progress } => JobStepOutcome::CheckpointReady { checkpoint: JobCheckpoint { state: first, applied_progress } },
            semio_framework_job::JobOutcomeKind::Complete => JobStepOutcome::Complete { candidate: JobCommitCandidate { state: first, output: second } },
            semio_framework_job::JobOutcomeKind::Cancelled => JobStepOutcome::Cancelled,
            semio_framework_job::JobOutcomeKind::Fault => JobStepOutcome::Fault { detail: first },
        }))
    }
}

/// 🌉️ Stateful actor bridge that advances at most one interactive job step or one retained payload page per turn.
#[cfg(test)]
pub struct JobTurnBridge {
    pub(super) operation: semio_framework_job::Operation,
    next_step_sequence: u64,
    pub(super) terminal: bool,
    pub(super) pending: Option<JobOutcomeProjection>,
    pub(super) original: semio_framework_job::RetainedCloneGrant,
    pub(super) progress: semio_framework_job::RetainedCloneProgress,
    authority: Option<semio_framework_job_payload::JobPayloadAuthority>,
}

#[cfg(test)]
impl JobTurnBridge {
    pub fn new(operation: semio_framework_job::Operation, original: semio_framework_job::RetainedCloneGrant) -> Result<Self, semio_framework_value::ValueError> {
        let Some((authority, progress)) = semio_framework_job_payload::JobPayloadAuthority::admit(operation.operation, operation.generation, original)? else {
            return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "actor bridge requires original payload authority admission"));
        };
        Ok(Self { operation, next_step_sequence: 0, terminal: false, pending: None, original, progress, authority: Some(authority) })
    }

    pub fn close_step(&mut self, grant: semio_framework_job::RetainedCloneGrant) -> Result<semio_framework_job::RetainedCloneStep, semio_framework_value::ValueError> {
        self.terminal = true;
        if let Some(pending) = self.pending.as_mut() {
            if !pending.descriptor.is_acknowledged() { return Ok(semio_framework_job::RetainedCloneStep::Progress(pending.descriptor.acknowledge(grant).progress())); }
            if let Some(bytes) = pending.bytes.iter_mut().find(|bytes| bytes.capacity() != 0) {
                let released_bytes = bytes.capacity();
                if grant.maximum_items == 0 || grant.maximum_release_bytes < released_bytes || grant.maximum_depth == 0 { return Ok(semio_framework_job::RetainedCloneStep::Progress(Default::default())); }
                drop(std::mem::take(bytes));
                return Ok(semio_framework_job::RetainedCloneStep::Progress(semio_framework_job::RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }));
            }
            if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(semio_framework_job::RetainedCloneStep::Progress(Default::default())); }
            self.pending = None;
            return Ok(semio_framework_job::RetainedCloneStep::Progress(semio_framework_job::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if let Some(authority) = self.authority.as_mut() {
            if !authority.terminal_is_empty() { return authority.close_step(grant).map(|step| semio_framework_job::RetainedCloneStep::Progress(step.progress())); }
            if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(semio_framework_job::RetainedCloneStep::Progress(Default::default())); }
            self.authority = None;
            return Ok(semio_framework_job::RetainedCloneStep::Progress(semio_framework_job::RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        Ok(semio_framework_job::RetainedCloneStep::Complete(Default::default()))
    }

    pub fn terminal_is_empty(&self) -> bool { self.pending.is_none() && self.authority.is_none() }

    #[allow(clippy::too_many_arguments)]
    pub fn step<J: semio_framework_job::InteractiveJob + ?Sized>(
        &mut self,
        job: &mut J,
        turn: JobTurn,
        active_operation: semio_framework_job::OperationId,
        live_revision: semio_framework_job::RevisionId,
        live_generation: semio_framework_job::Generation,
        site: &'static str,
        stage: semio_framework_job::InteractiveStage,
        budget: semio_framework_job::StepBudget,
        cancel: semio_framework_job::CancelToken,
        now_us: fn() -> Option<u64>,
    ) -> Result<JobPublication, JobPublicationError> {
        let next_step_sequence = self.next_step_sequence.checked_add(1).ok_or_else(|| JobPublicationError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original actor bridge sequence exhausted")))?;
        if self.terminal {
            return Err(JobPublicationError::Terminal);
        }
        if turn.operation.operation != active_operation.0 || self.operation.operation != active_operation {
            return Err(JobPublicationError::OperationMismatch { active: active_operation.0, published: turn.operation.operation });
        }
        if turn.step_sequence != self.next_step_sequence {
            return Err(JobPublicationError::StepSequence { expected: self.next_step_sequence, published: turn.step_sequence });
        }
        if !matches!(semio_framework_job::validate_commit(&self.operation, live_revision, live_generation), semio_framework_job::CommitValidation::Accepted)
            || turn.operation.base_revision != live_revision.0
            || turn.operation.generation != live_generation.0
            || turn.operation.base_revision != self.operation.base_revision.0
            || turn.operation.generation != self.operation.generation.0
            || turn.operation.preview_sequence != self.operation.preview_sequence
            || turn.operation.seed != self.operation.seed
        {
            return Err(JobPublicationError::Stale { live_revision: live_revision.0, live_generation: live_generation.0 });
        }
        if budget.retained != self.original {
            return Err(JobPublicationError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "actor bridge requires its original cumulative grant")));
        }
        let authority = self.authority.as_ref().ok_or_else(|| JobPublicationError::Retained(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "original actor bridge authority has closed")))?;
        let mut context = semio_framework_job::StepContext::with_payload_authority(self.operation.operation, self.operation.generation, budget, &cancel, now_us, &mut self.operation.preview_sequence, &mut self.progress, authority).map_err(JobPublicationError::Retained)?;
        let outcome = if let Some(projection) = self.pending.as_mut() {
            match projection.step(job, &mut context).map_err(JobPublicationError::Retained)? {
                Some(outcome) => { self.pending = None; outcome }
                None => JobStepOutcome::Yield,
            }
        } else {
            let before = context.preview_sequence();
            let outcome = semio_framework_job::drive_step(job, &mut context, site, stage, &mut None).map_err(JobPublicationError::Original)?;
            let after = context.preview_sequence();
            let Some(outcome) = outcome else {
                self.next_step_sequence = next_step_sequence;
                drop(context);
                return Ok(JobPublication { turn: JobTurn { operation: JobOperation::from_job(self.operation), ..turn }, outcome: JobStepOutcome::Yield });
            };
            let mut descriptor = outcome.into_descriptor();
            let preview = matches!(descriptor.kind(), semio_framework_job::JobOutcomeKind::PreviewReady);
            if (preview && after != before.saturating_add(1)) || (!preview && after != before) {
                let acknowledged = descriptor.acknowledge(context.retained_grant());
                context.consume_retained(acknowledged.progress()).map_err(JobPublicationError::Retained)?;
                self.pending = (!descriptor.is_acknowledged()).then(|| JobOutcomeProjection::new(descriptor));
                return Err(JobPublicationError::PreviewSequence { before, after });
            }
            self.pending = Some(JobOutcomeProjection::new(descriptor));
            if matches!(self.pending.as_ref().unwrap().descriptor.kind(), semio_framework_job::JobOutcomeKind::Yield | semio_framework_job::JobOutcomeKind::Cancelled) {
                let projected = self.pending.as_mut().unwrap().step(job, &mut context).map_err(JobPublicationError::Retained)?;
                if let Some(projected) = projected { self.pending = None; projected } else { JobStepOutcome::Yield }
            } else { JobStepOutcome::Yield }
        };
        let terminal = matches!(outcome, JobStepOutcome::Complete { .. } | JobStepOutcome::Cancelled | JobStepOutcome::Fault { .. });
        drop(context);
        let publication = JobPublication { turn: JobTurn { operation: JobOperation::from_job(self.operation), ..turn }, outcome };
        self.next_step_sequence = next_step_sequence;
        self.terminal = terminal;
        Ok(publication)
    }
}
