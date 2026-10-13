//! 📤️ One job lends every semantic outcome from payload pages it still owns until a paid physical closure.
use crate::tool_contract::ReservedOutcomeKind;
use crate::tool_contract::receive_step;
use crate::tool_contract::close_result;
use crate::tool_contract::BoundedFaultText;
use semio_framework_job::{InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPublicationKind, RetainedJobPublication, StepContext};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError};

/// 🚦️ What one cleanup turn decided, before its outcome is lent from the pages the job owns.
pub(crate) enum CleanupTurn {
    Yield,
    Cancelled,
    Complete,
    Checkpoint(u64),
    Fault(&'static [u8]),
    FaultText(String),
}

/// 🪶️ The checkpoint and fault publication of a cleanup job without the commit builder a reserved job carries: a cleanup never commits output, and each inline page table is 6 KiB that the admission quote charges three times.
struct CleanupOutcomes {
    publication: RetainedJobPublication,
    delivered: bool,
}

impl CleanupOutcomes {
    fn new() -> Self {
        Self { publication: RetainedJobPublication::new(), delivered: false }
    }

    fn is_delivered(&self) -> bool {
        self.delivered
    }

    fn terminal_is_empty(&self) -> bool {
        self.publication.terminal_is_empty()
    }

    fn retire(&mut self, cx: &mut StepContext<'_>) -> Result<(), ValueError> {
        if !self.publication.terminal_is_empty() {
            let step = self.publication.close_step(cx.retained_grant());
            receive_step(cx, step)?;
        }
        if self.publication.terminal_is_empty() {
            self.delivered = false;
        }
        Ok(())
    }

    fn advance<'a>(&'a mut self, kind: ReservedOutcomeKind, source: &[u8], cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        let publication_kind = match kind {
            ReservedOutcomeKind::Preview => JobPublicationKind::Preview,
            ReservedOutcomeKind::Checkpoint { applied_progress } => JobPublicationKind::Checkpoint { applied_progress },
            ReservedOutcomeKind::Fault => JobPublicationKind::Fault,
            ReservedOutcomeKind::Commit => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "a cleanup job publishes no commit output")),
        };
        let original = self.publication.advance_from_source(publication_kind, source, cx)?;
        self.delivered |= original.is_some();
        Ok(original)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, None),
            JobOutcomeKind::PreviewReady | JobOutcomeKind::CheckpointReady { .. } | JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
        }
    }

    fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if !self.publication.terminal_is_empty() {
            return self.publication.retirement_demands();
        }
        Ok(RetirementDemand::default())
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if !self.publication.terminal_is_empty() {
            return self.publication.close_step(grant);
        }
        self.delivered = false;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    }
}

/// 🧳️ The publication custody of one cleanup job: its sealed pages and the bytes they are authored from.
pub(crate) struct CleanupPublication {
    outcomes: CleanupOutcomes,
    pending: Option<ReservedOutcomeKind>,
    state: [u8; 4],
    fault: BoundedFaultText,
}

impl CleanupPublication {
    pub(crate) fn new() -> Self {
        Self { outcomes: CleanupOutcomes::new(), pending: None, state: [0; 4], fault: BoundedFaultText::new() }
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
