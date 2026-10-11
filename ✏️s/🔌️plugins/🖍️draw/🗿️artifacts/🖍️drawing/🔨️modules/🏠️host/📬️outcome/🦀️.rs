//! 📬️ Every Drawing job lends its yield, complete, cancelled and fault outcome from custody it owns until a paid close.

use semio_framework_job::{InteractiveJobCloseStep, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPublicationKind, RetainedJobPublication, StepContext};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind};

/// 🚦️ What one Drawing job turn decided before its outcome is lent.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrawingJobTurn {
    Yield,
    Complete,
    Cancelled,
    Fault,
}

/// 🧳️ The fault text and sealed pages of one Drawing job; the fault source stays inline so its pages never move.
pub struct DrawingJobOutcomes {
    publication: RetainedJobPublication,
    text: [u8; Self::MAXIMUM_FAULT_BYTES],
    text_length: usize,
    faulted: bool,
}

impl DrawingJobOutcomes {
    /// 📏️ The longest fault text one job publishes.
    pub const MAXIMUM_FAULT_BYTES: usize = 256;

    pub fn new() -> Self {
        let mut text = [0; Self::MAXIMUM_FAULT_BYTES];
        let detail = b"drawing.job-fault";
        text[..detail.len()].copy_from_slice(detail);
        Self { publication: RetainedJobPublication::new(), text, text_length: detail.len(), faulted: false }
    }

    /// 🧯️ Whether a fault turn already decided this job's terminal outcome.
    pub fn faulted(&self) -> bool {
        self.faulted
    }

    /// 🧯️ Keeps the longest whole-character prefix of `detail` as the fault every following turn publishes.
    pub fn fault(&mut self, detail: &str) -> DrawingJobTurn {
        let mut length = detail.len().min(Self::MAXIMUM_FAULT_BYTES);
        while !detail.is_char_boundary(length) {
            length -= 1;
        }
        self.text[..length].copy_from_slice(&detail.as_bytes()[..length]);
        self.text_length = length;
        self.faulted = true;
        DrawingJobTurn::Fault
    }

    /// ▶️ Lends the outcome the turn decided, paying one publication frontier for a fault.
    pub fn lend<'a>(&'a mut self, turn: DrawingJobTurn, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, ValueError> {
        match turn {
            DrawingJobTurn::Yield => JobOutcomeBorrow::admit_yield(cx),
            DrawingJobTurn::Complete => JobOutcomeBorrow::admit_complete(cx, None, None),
            DrawingJobTurn::Cancelled => JobOutcomeBorrow::admit_cancelled(cx),
            DrawingJobTurn::Fault => self.publication.advance_from_source(JobPublicationKind::Fault, &self.text[..self.text_length], cx),
        }
    }

    /// 🤝️ Resolves a descriptor against the pages this job lent.
    pub fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Complete => descriptor.complete(None, None),
            JobOutcomeKind::Fault => self.publication.borrow_outcome(descriptor),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "Drawing jobs lend only yield and terminal outcomes")),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.publication.terminal_is_empty()
    }

    /// 📏️ Quotes the next publication close turn without granting it.
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        self.publication.retirement_demands()
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.publication.close_step(grant)
    }

    /// 🔁️ Maps one paid publication close receipt onto the job close protocol; completion is the job's own terminal witness.
    pub fn close_job_step(&mut self, grant: RetainedCloneGrant) -> InteractiveJobCloseStep {
        match self.publication.close_step(grant) {
            Ok(step) => InteractiveJobCloseStep::Pending { progress: step.progress() },
            Err(error) => InteractiveJobCloseStep::Refused { kind: error.kind, progress: error.retained_progress() },
        }
    }
}

impl Default for DrawingJobOutcomes {
    fn default() -> Self {
        Self::new()
    }
}
