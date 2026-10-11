//! 📤️ One job lends every semantic outcome from payload pages it still owns until a paid physical closure.
use semio_framework_job::{InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, JobPublicationKind, RetainedJobPublication, RetainedPayloadBuilder, StepContext};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError};

/// 🚦️ The semantic outcome whose original source bytes the job keeps stable until it is delivered.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ReservedOutcomeKind {
    Preview,
    Checkpoint { applied_progress: u64 },
    Fault,
    Commit,
}

/// 🧳️ Enclosing job owns this inline storage; each turn funds one original publication frontier.
pub(crate) struct ReservedJobOutcomes {
    publication: RetainedJobPublication,
    commit: RetainedPayloadBuilder,
    commit_cursor: usize,
    delivered: bool,
}

/// 🧾️ Admits one received step receipt into the caller's wallet, refused or not.
pub(crate) fn receive_step(cx: &mut StepContext<'_>, step: Result<RetainedCloneStep, ValueError>) -> Result<RetainedCloneStep, ValueError> {
    match step {
        Ok(step) => {
            cx.consume_retained(step.progress())?;
            Ok(step)
        }
        Err(error) => {
            cx.consume_retained(error.retained_progress())?;
            Err(error)
        }
    }
}

/// 🔁️ Maps one paid close receipt onto the job close protocol; completion is the job's own terminal witness.
pub(crate) fn close_result(step: Result<RetainedCloneStep, ValueError>) -> InteractiveJobCloseStep {
    match step {
        Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
        Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
    }
}

impl ReservedJobOutcomes {
    pub(crate) fn new() -> Self {
        Self { publication: RetainedJobPublication::new(), commit: RetainedPayloadBuilder::new(JobPayloadStream::CommitOutput), commit_cursor: 0, delivered: false }
    }

    /// 📬️ Whether a lent outcome still owns its payload pages and must be retired before the next one.
    pub(crate) fn is_delivered(&self) -> bool {
        self.delivered
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.publication.terminal_is_empty() && self.commit.terminal_is_empty()
    }

    /// ♻️ Retires the previous publication one paid turn at a time inside the wallet of a running step.
    pub(crate) fn retire(&mut self, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        if !self.publication.terminal_is_empty() {
            let step = self.publication.close_step(cx.retained_grant());
            receive_step(cx, step)?;
        }
        if self.publication.terminal_is_empty() {
            self.delivered = false;
        }
        Ok(())
    }

    /// ✍️ Advances one paid frontier of the pending outcome and lends it once its original pages are sealed.
    pub(crate) fn advance<'a>(&'a mut self, kind: ReservedOutcomeKind, source: &[u8], cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let publication_kind = match kind {
            ReservedOutcomeKind::Preview => JobPublicationKind::Preview,
            ReservedOutcomeKind::Checkpoint { applied_progress } => JobPublicationKind::Checkpoint { applied_progress },
            ReservedOutcomeKind::Fault => JobPublicationKind::Fault,
            ReservedOutcomeKind::Commit => {
                if !self.commit.is_initialized() {
                    self.commit.advance_initialization(cx)?;
                    return Ok(None);
                }
                if self.commit_cursor < source.len() {
                    self.commit.append_original(cx, source, &mut self.commit_cursor)?;
                    return Ok(None);
                }
                if self.commit.published().is_none() {
                    self.commit.seal(cx)?;
                    return Ok(None);
                }
                let original = JobOutcomeBorrow::admit_complete(cx, None, self.commit.published())?;
                self.delivered |= original.is_some();
                return Ok(original);
            }
        };
        let original = self.publication.advance_from_source(publication_kind, source, cx)?;
        self.delivered |= original.is_some();
        Ok(original)
    }

    /// 🤝️ Resolves a descriptor against the same original pages this job lent.
    pub(crate) fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, self.commit.published()),
            JobOutcomeKind::PreviewReady | JobOutcomeKind::CheckpointReady { .. } | JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
        }
    }

    /// 📏️ Quotes the next original page, builder or metadata turn without granting it.
    pub(crate) fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        if !self.commit.terminal_is_empty() {
            return self.commit.retirement_demands();
        }
        Ok(RetirementDemand::default())
    }

    /// ♻️ Closes the publication before the commit pages, never both in one turn.
    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(grant);
        }
        if !self.commit.terminal_is_empty() {
            let step = self.commit.close_step_granted(grant)?;
            if self.commit.terminal_is_empty() {
                self.commit_cursor = 0;
                self.delivered = false;
            }
            return Ok(step);
        }
        self.delivered = false;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    }
}

impl Default for ReservedJobOutcomes {
    fn default() -> Self {
        Self::new()
    }
}

/// 🧯️ One bounded fault text kept inline so its publication source never moves or allocates.
pub(crate) struct BoundedFaultText {
    bytes: [u8; Self::MAXIMUM_BYTES],
    length: usize,
}

impl BoundedFaultText {
    pub(crate) const MAXIMUM_BYTES: usize = 480;

    pub(crate) fn new() -> Self {
        Self { bytes: [0; Self::MAXIMUM_BYTES], length: 0 }
    }

    /// ✍️ Stores the longest whole-character prefix of `text` that fits.
    pub(crate) fn set(&mut self, text: &str) {
        let mut length = text.len().min(Self::MAXIMUM_BYTES);
        while !text.is_char_boundary(length) {
            length -= 1;
        }
        self.bytes[..length].copy_from_slice(&text.as_bytes()[..length]);
        self.length = length;
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.length]
    }
}

impl Default for BoundedFaultText {
    fn default() -> Self {
        Self::new()
    }
}

/// 🚦️ What one cleanup turn decided, before its outcome is lent from the pages the job owns.
pub(crate) enum CleanupTurn {
    Yield,
    Cancelled,
    Complete,
    Checkpoint(u64),
    Fault(&'static [u8]),
    FaultText(String),
}

/// 🧳️ The publication custody of one cleanup job: its sealed pages and the bytes they are authored from.
pub(crate) struct CleanupPublication {
    outcomes: ReservedJobOutcomes,
    pending: Option<ReservedOutcomeKind>,
    state: [u8; 4],
    fault: BoundedFaultText,
}

impl CleanupPublication {
    pub(crate) fn new() -> Self {
        Self { outcomes: ReservedJobOutcomes::new(), pending: None, state: [0; 4], fault: BoundedFaultText::new() }
    }

    fn queue_checkpoint(&mut self, instance_id: u32, applied_progress: u64) {
        self.state = instance_id.to_le_bytes();
        self.pending = Some(ReservedOutcomeKind::Checkpoint { applied_progress });
    }

    fn queue_fault(&mut self, detail: &[u8]) {
        self.fault.set(&String::from_utf8_lossy(detail));
        self.pending = Some(ReservedOutcomeKind::Fault);
    }

    fn advance_pending<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let Some(kind) = self.pending else { return Ok(None) };
        let source: &[u8] = if kind == ReservedOutcomeKind::Fault { self.fault.as_bytes() } else { &self.state };
        let outcome = self.outcomes.advance(kind, source, cx)?;
        if outcome.is_some() {
            self.pending = None;
        }
        Ok(outcome)
    }

    pub(crate) fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        self.outcomes.borrow_outcome(descriptor)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.outcomes.terminal_is_empty()
    }

    pub(crate) fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        self.outcomes.close_demands()
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        self.pending = None;
        close_result(self.outcomes.close_step(grant))
    }
}

impl Default for CleanupPublication {
    fn default() -> Self {
        Self::new()
    }
}

/// 🧹️ A cleanup job decides one turn; the shared driver publishes its outcome page by page.
pub(crate) trait CleanupJob {
    fn advance(&mut self, cx: &mut StepContext<'_>) -> CleanupTurn;
    fn instance_id(&self) -> u32;
    fn publication_mut(&mut self) -> &mut CleanupPublication;
}

/// ▶️ One bounded step: retire the previous page, else author the pending page, else decide a new turn.
pub(crate) fn drive_turn<'a, J: CleanupJob>(job: &'a mut J, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
    if cx.is_cancelled() {
        return JobOutcomeBorrow::admit_cancelled(cx);
    }
    if job.publication_mut().outcomes.is_delivered() {
        job.publication_mut().outcomes.retire(cx)?;
        return Ok(None);
    }
    if job.publication_mut().pending.is_none() {
        match job.advance(cx) {
            CleanupTurn::Yield => return JobOutcomeBorrow::admit_yield(cx),
            CleanupTurn::Cancelled => return JobOutcomeBorrow::admit_cancelled(cx),
            CleanupTurn::Complete => return JobOutcomeBorrow::admit_complete(cx, None, None),
            CleanupTurn::Checkpoint(applied_progress) => {
                let instance_id = job.instance_id();
                job.publication_mut().queue_checkpoint(instance_id, applied_progress);
            }
            CleanupTurn::Fault(detail) => job.publication_mut().queue_fault(detail),
            CleanupTurn::FaultText(detail) => job.publication_mut().queue_fault(detail.as_bytes()),
        }
        return Ok(None);
    }
    job.publication_mut().advance_pending(cx)
}
