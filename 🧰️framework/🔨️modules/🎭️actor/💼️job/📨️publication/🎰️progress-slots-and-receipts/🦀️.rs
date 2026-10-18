//! 🎰️ Progress slots and receipts: the publication owner, the progress and retirement slots, the admission, the receipt, the close step and the overlay store record. See [`super`].

use super::*;

//#region 🎨️JobProgressOverlay
#[derive(Debug)]
pub(crate) struct JobPublicationOwner {
    pub(crate) publication: JobPublication,
    pub(crate) phase: u8,
    pub(super) total_phases: u8,
    pub(super) owner_bytes: usize,
}

impl JobPublicationOwner {
    pub(crate) fn try_new(publication: JobPublication) -> Result<Self, (JobProgressFault, JobPublication)> {
        match publication_owner_shape(&publication, false) {
            Ok((_, owner_bytes, total_phases)) => Ok(Self { publication, phase: 0, total_phases, owner_bytes }),
            Err(fault) => Err((fault, publication)),
        }
    }

    pub(crate) fn close_one(&mut self) -> (bool, usize, usize) {
        let released_bytes = match (&mut self.publication.outcome, self.phase) {
            (JobStepOutcome::PreviewReady { preview }, 0) => std::mem::take(preview).capacity(),
            (JobStepOutcome::CheckpointReady { checkpoint }, 0) => std::mem::take(&mut checkpoint.state).capacity(),
            (JobStepOutcome::Complete { candidate }, 0) => std::mem::take(&mut candidate.state).capacity(),
            (JobStepOutcome::Complete { candidate }, 1) => std::mem::take(&mut candidate.output).capacity(),
            (JobStepOutcome::Fault { detail }, 0) => std::mem::take(detail).capacity(),
            _ => 0,
        };
        self.phase = self.phase.saturating_add(1);
        self.owner_bytes = self.owner_bytes.saturating_sub(released_bytes);
        (self.phase >= self.total_phases, 1, released_bytes)
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.phase >= self.total_phases
            && self.owner_bytes == 0
            && match &self.publication.outcome {
                JobStepOutcome::PreviewReady { preview } => preview.capacity() == 0,
                JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.state.capacity() == 0,
                JobStepOutcome::Complete { candidate } => candidate.state.capacity() == 0 && candidate.output.capacity() == 0,
                JobStepOutcome::Fault { detail } => detail.capacity() == 0,
                JobStepOutcome::Yield | JobStepOutcome::Cancelled => true,
            }
    }
}

pub(super) fn publication_owner_shape(publication: &JobPublication, enforce_page_limit: bool) -> Result<(usize, usize, u8), JobProgressFault> {
    let capacities = match &publication.outcome {
        JobStepOutcome::Yield | JobStepOutcome::Cancelled => ([0usize, 0usize], 0usize),
        JobStepOutcome::PreviewReady { preview } => ([preview.capacity(), 0], 1),
        JobStepOutcome::CheckpointReady { checkpoint } => ([checkpoint.state.capacity(), 0], 1),
        JobStepOutcome::Complete { candidate } => ([candidate.state.capacity(), candidate.output.capacity()], 2),
        JobStepOutcome::Fault { detail } => ([detail.capacity(), 0], 1),
    };
    if enforce_page_limit && capacities.0[..capacities.1].iter().any(|capacity| *capacity > JOB_PROGRESS_PAGE_MAXIMUM_BYTES) {
        return Err(JobProgressFault::Oversized);
    }
    let owner_bytes = capacities.0[..capacities.1].iter().try_fold(0usize, |total, capacity| total.checked_add(*capacity)).ok_or(JobProgressFault::Capacity)?;
    let total_phases = u8::try_from(capacities.1.checked_add(1).ok_or(JobProgressFault::Capacity)?).map_err(|_| JobProgressFault::Capacity)?;
    Ok((capacities.1 + 1, owner_bytes, total_phases))
}

#[derive(Debug)]
pub(super) struct JobProgressSlot {
    pub(super) epoch: u64,
    pub(super) occupied: bool,
    pub(super) published: bool,
    pub(super) awaiting_ack: bool,
    pub(super) checked_out: bool,
    pub(super) terminal: bool,
    pub(super) reservation: Option<u64>,
    pub(super) identity: JobProgressIdentity,
    pub(super) kind: JobProgressKind,
    pub(super) applied_progress: u64,
    pub(super) preview: Option<JobPublicationOwner>,
}

impl JobProgressSlot {
    pub(super) fn vacant() -> Self {
        Self {
            epoch: 0,
            occupied: false,
            published: false,
            awaiting_ack: false,
            checked_out: false,
            terminal: false,
            reservation: None,
            identity: JobProgressIdentity { actor: ActorId(0), job: 0, operation: 0, base_revision: 0, generation: 0, step_sequence: 0, preview_sequence: 0 },
            kind: JobProgressKind::Yield,
            applied_progress: 0,
            preview: None,
        }
    }

    pub(super) fn clear(&mut self) {
        let epoch = self.epoch;
        *self = Self::vacant();
        self.epoch = epoch;
    }
}

#[derive(Debug)]
pub(super) struct JobProgressRetirementSlot {
    pub(super) reservation: Option<u64>,
    pub(super) owner: Option<JobPublicationOwner>,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct JobProgressPriorState {
    pub(super) published: bool,
    pub(super) terminal: bool,
    pub(super) identity: JobProgressIdentity,
    pub(super) kind: JobProgressKind,
    pub(super) applied_progress: u64,
}

impl JobProgressRetirementSlot {
    pub(super) fn vacant() -> Self {
        Self { reservation: None, owner: None }
    }
}

/// 🎟️ Linear reservation made before the publication owner crosses into the overlay.
#[must_use = "job progress admission must be published or cancelled"]
#[derive(Debug)]
pub struct JobProgressAdmission {
    pub(super) token: u64,
    pub(super) active_index: usize,
    pub(super) active_epoch: u64,
    pub(super) identity: JobProgressIdentity,
    pub(super) live: JobProgressLiveAuthority,
    pub(super) retirement_indices: [usize; 2],
    pub(super) abort_retirement: Option<usize>,
    pub(super) owner_items: usize,
    pub(super) owner_bytes: usize,
}

/// ✅️ Linear ACK/abort authority for one adopted publication.
#[must_use = "an adopted job publication must be acknowledged or aborted"]
#[derive(Debug)]
pub struct JobProgressReceipt {
    pub(super) identity: JobProgressIdentity,
    pub(super) kind: JobProgressKind,
    pub(super) applied_progress: u64,
    pub(super) owner_bytes: usize,
    pub(super) active_index: usize,
    pub(super) active_epoch: u64,
    pub(super) prior: JobProgressPriorState,
    pub(super) displaced_retirement: Option<usize>,
    pub(super) abort_retirement: Option<(usize, u64)>,
}

impl JobProgressReceipt {
    pub fn identity(&self) -> JobProgressIdentity {
        self.identity
    }

    pub fn kind(&self) -> JobProgressKind {
        self.kind
    }

    pub fn applied_progress(&self) -> u64 {
        self.applied_progress
    }

    pub fn owner_bytes(&self) -> usize {
        self.owner_bytes
    }
}

/// 🧹️ One truthful cleanup opportunity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobProgressCloseStep {
    Pending { released_items: usize, released_bytes: usize },
    Blocked,
    Complete,
}

/// 🎨️ Fixed actor-owned preview/progress authority; committed scenes are never stored here.
pub struct JobProgressOverlayStore {
    pub(super) active: [JobProgressSlot; JOB_PROGRESS_ACTIVE_CAPACITY],
    pub(super) retirements: [JobProgressRetirementSlot; JOB_PROGRESS_RETIREMENT_CAPACITY],
    pub(super) closing_actors: [Option<ActorId>; JOB_PROGRESS_ACTIVE_CAPACITY],
    pub(super) close_all: bool,
    pub(super) next_token: u64,
    pub(super) owned_items: usize,
    pub(super) owned_bytes: usize,
    pub(super) reserved_items: usize,
    pub(super) reserved_bytes: usize,
}

impl Default for JobProgressOverlayStore {
    fn default() -> Self {
        Self::new()
    }
}
//#endregion 🎨️JobProgressOverlay
