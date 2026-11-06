//! 🚦️ Typed job outcomes retain their original payload owners.
use crate::{RetainedJobPayload,RetainedCloneProgress,RetainedCloneGrant,ValueError,ValueRefusalKind};
//#region 🚦️StepOutcome
/// 📸️ See `README.md`.
#[derive(Debug, PartialEq, Eq)]
pub struct Checkpoint {
    pub state: RetainedJobPayload,
    pub applied_progress: u64,
}

/// 🖥️ See `README.md`.
#[derive(Debug, PartialEq, Eq)]
pub struct CommitCandidate {
    pub state: RetainedJobPayload,
    pub output: RetainedJobPayload,
}

/// 💥️ See `README.md`.
#[derive(Debug, PartialEq, Eq)]
pub struct JobFault {
    pub detail: RetainedJobPayload,
}

/// ☎️ See `README.md`.
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

/// ♻️ One bounded original job-close receipt.
#[derive(Clone, Copy, Debug, PartialEq, Eq,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
pub enum InteractiveJobCloseStep {
    Pending { progress: RetainedCloneProgress },
    Blocked,
    Complete { progress: RetainedCloneProgress },
}

impl InteractiveJobCloseStep {
    /// 📊️ See `README.md`.
    pub fn progress(self)->RetainedCloneProgress {match self{Self::Pending{progress}|Self::Complete{progress}=>progress,Self::Blocked=>RetainedCloneProgress::default()}}
    /// 🛡️ See `README.md`.
    pub fn admit(self,grant:RetainedCloneGrant,terminal_is_empty:bool)->Result<Self,ValueError>{
        if !self.progress().fits(grant)||matches!(self,Self::Complete{..})&&!terminal_is_empty{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original job close receipt or terminal witness refused").with_retained_progress(self.progress()))}else{Ok(self)}
    }
}

