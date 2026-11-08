//! 📬️ Original job outcome records and their bounded payload retirement.
use semio_framework_job_payload::RetainedJobPayload;
use semio_framework_value_retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind};
use std::mem::MaybeUninit;
//#region 🚦️StepOutcome
/// 📸️ A pause point where work is resumable but not yet committed — `state` is opaque, pack-encoded
/// (or, for a dependency-free job like [`TortureJob`], hand-rolled little-endian) bytes the job alone
/// interprets; `applied_progress` is the Puzzle 3D `FillBuilder.applied_count` pattern generalized: how
/// much of `state` is COMMITTED versus merely planned, so a caller can show "these N are done" without
/// decoding `state` itself.
#[derive(Debug, PartialEq, Eq)]
pub struct Checkpoint {
    pub state: RetainedJobPayload,
    pub applied_progress: u64,
}

/// 🖥️ Terminal success payload: the job's final persisted `state` plus its `output` — both opaque
/// bytes, so the runtime stays completely job-agnostic (design doc Decision 2).
#[derive(Debug, PartialEq, Eq)]
pub struct CommitCandidate {
    pub state: RetainedJobPayload,
    pub output: RetainedJobPayload,
}

/// 💥️ Opaque, job-specific error payload — never interpreted by the runtime, same reasoning as
/// [`CommitCandidate`]'s fields.
#[derive(Debug, PartialEq, Eq)]
pub struct JobFault {
    pub detail: RetainedJobPayload,
}

/// ☎️ What one [`InteractiveJob::step`] call reports. [`StepOutcome::Yield`]/[`StepOutcome::PreviewReady`]/
/// [`StepOutcome::CheckpointReady`] all mean "call `step` again"; [`StepOutcome::is_terminal`] marks
/// the other three.
#[derive(Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "Step outcomes move preadmitted payload pages inline; boxing would allocate outside the step grant.")]
pub enum StepOutcome {
    Yield,
    PreviewReady(RetainedJobPayload),
    CheckpointReady(Checkpoint),
    Complete(CommitCandidate),
    Cancelled,
    Fault(JobFault),
}

impl StepOutcome {
    pub fn is_terminal(&self) -> bool {
        matches!(self, StepOutcome::Complete(_) | StepOutcome::Cancelled | StepOutcome::Fault(_))
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            StepOutcome::Yield | StepOutcome::Cancelled => true,
            StepOutcome::PreviewReady(payload) => payload.terminal_is_empty(),
            StepOutcome::CheckpointReady(checkpoint) => checkpoint.state.terminal_is_empty(),
            StepOutcome::Complete(candidate) => candidate.state.terminal_is_empty() && candidate.output.terminal_is_empty(),
            StepOutcome::Fault(fault) => fault.detail.terminal_is_empty(),
        }
    }
}
//#endregion 🚦️StepOutcome

//#region 🧩️InteractiveJob
#[derive(Clone, Copy, Debug, PartialEq, Eq,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
pub enum InteractiveJobCloseStep {
    Pending { progress: RetainedCloneProgress },
    Blocked,
    Complete { progress: RetainedCloneProgress },
}

impl InteractiveJobCloseStep {
    /// 📊️ Keeps every admitted turn's physical and logical receipt, including terminal release.
    pub fn progress(self)->RetainedCloneProgress {match self{Self::Pending{progress}|Self::Complete{progress}=>progress,Self::Blocked=>RetainedCloneProgress::default()}}
    /// 🛡️ Checks independent authority and the original owner's terminal witness before publication.
    pub fn admit(self,grant:RetainedCloneGrant,terminal_is_empty:bool)->Result<Self,ValueError>{
        if !self.progress().fits(grant){Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job close receipt exceeds its grant").with_retained_progress(self.progress()))}else if matches!(self,Self::Complete{..})&&!terminal_is_empty{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job close completed with a non-empty terminal witness").with_retained_progress(self.progress()))}else{Ok(self)}
    }
}


#[path="../../♻️retirement/📄️payload/🦀️.rs"]
mod payload_retirement;
pub use payload_retirement::{close_step_outcome_slot,step_outcome_slot_retirement_demands,JobOutcomeSlot,JobPayloadSlot};
