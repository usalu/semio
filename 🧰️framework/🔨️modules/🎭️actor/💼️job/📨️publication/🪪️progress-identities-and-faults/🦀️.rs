//! ⏳️ Progress identities and faults: the progress capacities, the progress identity, kind and live authority, the progress faults and the rejected publication. See [`super`].

use super::*;

//#region 🎨️JobProgressOverlay
pub const JOB_PROGRESS_ACTIVE_CAPACITY: usize = 64;

pub const JOB_PROGRESS_RETIREMENT_CAPACITY: usize = 128;

pub const JOB_PROGRESS_PAGE_MAXIMUM_BYTES: usize = 16 * 1024;

pub const JOB_PROGRESS_TOTAL_MAXIMUM_BYTES: usize = 4 * 1024 * 1024;

pub const JOB_PROGRESS_TOTAL_MAXIMUM_ITEMS: usize = 512;

/// 🪪️ Exact identity of one actor-visible job progress publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobProgressIdentity {
    pub actor: ActorId,
    pub job: u64,
    pub operation: u64,
    pub base_revision: u64,
    pub generation: u64,
    pub step_sequence: u64,
    pub preview_sequence: u64,
}

impl JobProgressIdentity {
    pub fn from_publication(actor: ActorId, publication: &JobPublication) -> Self {
        Self {
            actor,
            job: publication.turn.job,
            operation: publication.turn.operation.operation,
            base_revision: publication.turn.operation.base_revision,
            generation: publication.turn.operation.generation,
            step_sequence: publication.turn.step_sequence,
            preview_sequence: publication.turn.operation.preview_sequence,
        }
    }

    fn same_job(self, other: Self) -> bool {
        self.actor == other.actor && self.job == other.job
    }

    pub(super) fn same_operation(self, other: Self) -> bool {
        self.same_job(other) && self.operation == other.operation && self.base_revision == other.base_revision && self.generation == other.generation
    }
}

/// 📡️ Scalar, coalescible progress kind retained separately from committed actor scenes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobProgressKind {
    Yield,
    Preview,
    Checkpoint,
    CommitValidated,
    Cancelled,
    Fault,
}

impl JobProgressKind {
    pub(super) fn of(outcome: &JobStepOutcome) -> Self {
        match outcome {
            JobStepOutcome::Yield => Self::Yield,
            JobStepOutcome::PreviewReady { .. } => Self::Preview,
            JobStepOutcome::CheckpointReady { .. } => Self::Checkpoint,
            JobStepOutcome::Complete { .. } => Self::CommitValidated,
            JobStepOutcome::Cancelled => Self::Cancelled,
            JobStepOutcome::Fault { .. } => Self::Fault,
        }
    }

    pub(super) fn terminal(self) -> bool {
        matches!(self, Self::CommitValidated | Self::Cancelled | Self::Fault)
    }
}

/// 🔐️ Independent revision/generation authority supplied immediately before publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JobProgressLiveAuthority {
    pub operation: u64,
    pub base_revision: u64,
    pub generation: u64,
}

impl JobProgressLiveAuthority {
    pub fn new(operation: u64, base_revision: u64, generation: u64) -> Self {
        Self { operation, base_revision, generation }
    }

    pub(super) fn accepts(self, identity: JobProgressIdentity) -> bool {
        self.operation == identity.operation && self.base_revision == identity.base_revision && self.generation == identity.generation
    }
}

/// 🚫️ Fail-closed reason returned without consuming a rejected publication owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobProgressFault {
    Cancelled,
    Budget,
    Capacity,
    Oversized,
    Stale,
    StepSequence,
    PreviewSequence,
    Busy,
    Token,
    Missing,
    CheckedOut,
    Exhausted,
}

impl std::fmt::Display for JobProgressFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "job progress overlay fault: {self:?}")
    }
}

impl std::error::Error for JobProgressFault {}

/// 📮️ Exact rejected publication owner; callers must return or close it rather than recreate input.
#[must_use = "a rejected job publication retains the exact payload owner"]
#[derive(Debug)]
pub struct JobProgressRejected {
    pub(super) fault: JobProgressFault,
    pub(super) publication: Option<JobPublication>,
}

impl JobProgressRejected {
    pub fn new(fault: JobProgressFault, publication: JobPublication) -> Self {
        Self { fault, publication: Some(publication) }
    }

    pub fn fault(&self) -> JobProgressFault {
        self.fault
    }

    pub fn publication(&self) -> &JobPublication {
        self.publication.as_ref().expect("rejected job publication owner already returned")
    }

    pub fn into_publication(mut self) -> JobPublication {
        self.publication.take().expect("rejected job publication owner already returned")
    }
}
//#endregion 🎨️JobProgressOverlay
