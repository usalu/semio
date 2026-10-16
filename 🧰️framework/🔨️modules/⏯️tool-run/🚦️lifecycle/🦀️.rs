//! 🧩️ Tool run lifecycle: slot state, events, effects, rejections and the pure reducer machine that folds them.

use super::*;

//#region 🔖️Lifecycle
/// 🚦️ Lifecycle state of a non-empty ledger slot; "no run" is the absence of a slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_value::RetireOwned)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ToolRunState {
    Starting,
    Running,
    Paused,
    Complete,
    Finalizing,
    Finalized,
    Aborting,
    Aborted,
    Faulted,
}

impl ToolRunState {
    pub const ALL: [Self; 9] = [Self::Starting, Self::Running, Self::Paused, Self::Complete, Self::Finalizing, Self::Finalized, Self::Aborting, Self::Aborted, Self::Faulted];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::Running => "running",
            Self::Paused => "paused",
            Self::Complete => "complete",
            Self::Finalizing => "finalizing",
            Self::Finalized => "finalized",
            Self::Aborting => "aborting",
            Self::Aborted => "aborted",
            Self::Faulted => "faulted",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|state| state.as_str() == text)
    }

    /// 🔢️ Wire ordinal (declaration order).
    pub fn ordinal(self) -> u8 {
        self as u8
    }

    pub fn from_ordinal(ordinal: u64) -> Option<Self> {
        Self::ALL.get(usize::try_from(ordinal).ok()?).copied()
    }

    /// 🏁️ `finalized`, `aborted` and `faulted`.
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Finalized | Self::Aborted | Self::Faulted)
    }

    /// 🗣️ Framework status label of this state.
    pub fn label(self) -> ToolRunLabel {
        match self {
            Self::Starting => ToolRunLabel::StateStarting,
            Self::Running => ToolRunLabel::StateRunning,
            Self::Paused => ToolRunLabel::StatePaused,
            Self::Complete => ToolRunLabel::StateComplete,
            Self::Finalizing => ToolRunLabel::StateFinalizing,
            Self::Finalized => ToolRunLabel::StateFinalized,
            Self::Aborting => ToolRunLabel::StateAborting,
            Self::Aborted => ToolRunLabel::StateAborted,
            Self::Faulted => ToolRunLabel::StateFaulted,
        }
    }
}

/// 🎰️ The reducer's view of the single per-instance ledger slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ToolRunSlot {
    pub run: u64,
    pub generation: u32,
    pub state: ToolRunState,
}

/// 📨️ Everything that can move a slot: user actions (§2.5) and driver observations (§2.2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRunEvent {
    Start { run: u64 },
    JobAdmitted { run: u64, generation: u32 },
    Pause { run: u64, generation: u32 },
    Resume { run: u64, generation: u32 },
    Step { run: u64, generation: u32 },
    JobComplete { run: u64, generation: u32 },
    JobFault { run: u64, generation: u32 },
    SettingsChanged { run: u64 },
    BaseChanged { run: u64 },
    Finalize { run: u64, generation: u32 },
    PublicationComplete { run: u64, generation: u32 },
    RevalidationConflicts { run: u64, generation: u32 },
    StoreRejected { run: u64, generation: u32 },
    Abort { run: u64, generation: u32, publishing: bool },
    AbortComplete { run: u64, generation: u32 },
    Dismiss { run: u64 },
    Closed,
}

impl ToolRunEvent {
    /// 🗝️ Lifecycle-law matrix column of this event.
    pub fn key(self) -> ToolRunEventKey {
        use ToolRunEventKey as K;
        match self {
            Self::Start { .. } => K::Start,
            Self::JobAdmitted { .. } => K::JobAdmitted,
            Self::Pause { .. } => K::Pause,
            Self::Resume { .. } => K::Resume,
            Self::Step { .. } => K::Step,
            Self::JobComplete { .. } => K::JobComplete,
            Self::JobFault { .. } => K::JobFault,
            Self::SettingsChanged { .. } => K::SettingsChanged,
            Self::BaseChanged { .. } => K::BaseChanged,
            Self::Finalize { .. } => K::Finalize,
            Self::PublicationComplete { .. } => K::PublicationComplete,
            Self::RevalidationConflicts { .. } => K::RevalidationConflicts,
            Self::StoreRejected { .. } => K::StoreRejected,
            Self::Abort { publishing: false, .. } => K::Abort,
            Self::Abort { publishing: true, .. } => K::AbortWhilePublishing,
            Self::AbortComplete { .. } => K::AbortComplete,
            Self::Dismiss { .. } => K::Dismiss,
            Self::Closed => K::Closed,
        }
    }

    /// 🎯️ `(run, generation)` the event is addressed to; `None` where it carries none.
    pub fn target(self) -> (Option<u64>, Option<u32>) {
        match self {
            Self::Start { run } | Self::SettingsChanged { run } | Self::BaseChanged { run } | Self::Dismiss { run } => (Some(run), None),
            Self::JobAdmitted { run, generation }
            | Self::Pause { run, generation }
            | Self::Resume { run, generation }
            | Self::Step { run, generation }
            | Self::JobComplete { run, generation }
            | Self::JobFault { run, generation }
            | Self::Finalize { run, generation }
            | Self::PublicationComplete { run, generation }
            | Self::RevalidationConflicts { run, generation }
            | Self::StoreRejected { run, generation }
            | Self::Abort { run, generation, .. }
            | Self::AbortComplete { run, generation } => (Some(run), Some(generation)),
            Self::Closed => (None, None),
        }
    }
}

/// 🗺️ Matrix column of an event; `abort` with `publishing = true` is `AbortWhilePublishing`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRunEventKey {
    Start,
    JobAdmitted,
    Pause,
    Resume,
    Step,
    JobComplete,
    JobFault,
    SettingsChanged,
    BaseChanged,
    Finalize,
    PublicationComplete,
    RevalidationConflicts,
    StoreRejected,
    Abort,
    AbortWhilePublishing,
    AbortComplete,
    Dismiss,
    Closed,
}

impl ToolRunEventKey {
    pub const ALL: [Self; 18] = [
        Self::Start,
        Self::JobAdmitted,
        Self::Pause,
        Self::Resume,
        Self::Step,
        Self::JobComplete,
        Self::JobFault,
        Self::SettingsChanged,
        Self::BaseChanged,
        Self::Finalize,
        Self::PublicationComplete,
        Self::RevalidationConflicts,
        Self::StoreRejected,
        Self::Abort,
        Self::AbortWhilePublishing,
        Self::AbortComplete,
        Self::Dismiss,
        Self::Closed,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::JobAdmitted => "jobAdmitted",
            Self::Pause => "pause",
            Self::Resume => "resume",
            Self::Step => "step",
            Self::JobComplete => "jobComplete",
            Self::JobFault => "jobFault",
            Self::SettingsChanged => "settingsChanged",
            Self::BaseChanged => "baseChanged",
            Self::Finalize => "finalize",
            Self::PublicationComplete => "publicationComplete",
            Self::RevalidationConflicts => "revalidationConflicts",
            Self::StoreRejected => "storeRejected",
            Self::Abort => "abort",
            Self::AbortWhilePublishing => "abortWhilePublishing",
            Self::AbortComplete => "abortComplete",
            Self::Dismiss => "dismiss",
            Self::Closed => "closed",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|key| key.as_str() == text)
    }
}

/// 🛠️ What the driver must do for an accepted transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRunEffect {
    SpawnJob,
    Schedule,
    StopScheduling,
    DriveOneUnit,
    HoldResult,
    Reconfigure,
    Refold,
    BeginFinalize,
    ReleaseProvisional,
    RetractConflicts,
    KeepProvisional,
    CloseJob,
    CancelBatch,
    RetireProvisional,
    DiscardProvisional,
    ClearTrace,
    RetireAll,
}

impl ToolRunEffect {
    pub const ALL: [Self; 17] = [
        Self::SpawnJob,
        Self::Schedule,
        Self::StopScheduling,
        Self::DriveOneUnit,
        Self::HoldResult,
        Self::Reconfigure,
        Self::Refold,
        Self::BeginFinalize,
        Self::ReleaseProvisional,
        Self::RetractConflicts,
        Self::KeepProvisional,
        Self::CloseJob,
        Self::CancelBatch,
        Self::RetireProvisional,
        Self::DiscardProvisional,
        Self::ClearTrace,
        Self::RetireAll,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::SpawnJob => "spawnJob",
            Self::Schedule => "schedule",
            Self::StopScheduling => "stopScheduling",
            Self::DriveOneUnit => "driveOneUnit",
            Self::HoldResult => "holdResult",
            Self::Reconfigure => "reconfigure",
            Self::Refold => "refold",
            Self::BeginFinalize => "beginFinalize",
            Self::ReleaseProvisional => "releaseProvisional",
            Self::RetractConflicts => "retractConflicts",
            Self::KeepProvisional => "keepProvisional",
            Self::CloseJob => "closeJob",
            Self::CancelBatch => "cancelBatch",
            Self::RetireProvisional => "retireProvisional",
            Self::DiscardProvisional => "discardProvisional",
            Self::ClearTrace => "clearTrace",
            Self::RetireAll => "retireAll",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|effect| effect.as_str() == text)
    }

    /// 💾️ True only for the one transition that changes the store generation.
    pub fn commits(self) -> bool {
        self == Self::ReleaseProvisional
    }
}

/// 🙅️ Why an event was not applied; every rejection is a silent no-op for the user.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRunRejection {
    Stale,
    Busy,
    Illegal,
}

impl ToolRunRejection {
    pub const ALL: [Self; 3] = [Self::Stale, Self::Busy, Self::Illegal];

    /// 🔖️ Fault code, e.g. `toolRun.stale`.
    pub fn code(self) -> &'static str {
        match self {
            Self::Stale => "toolRun.stale",
            Self::Busy => "toolRun.busy",
            Self::Illegal => "toolRun.illegal",
        }
    }

    pub fn parse(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|rejection| rejection.code() == code)
    }
}

impl std::fmt::Display for ToolRunRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for ToolRunRejection {}

/// ➡️ Next slot (`None` = no run) and the effect the driver performs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ToolRunTransition {
    pub slot: Option<ToolRunSlot>,
    pub effect: ToolRunEffect,
}

/// 🛣️ What a run occupies on its document instance for its local actor (§2.2 invariant 5): every mutating run shares
/// the one mutating lane, and a read-only run owns the lane of its tool on its window instance.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunLane {
    pub mutating: bool,
    pub tool_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

impl ToolRunLane {
    /// 🤝️ Two runs on the same lane cannot both be non-terminal: two mutating runs always share it, two read-only runs
    /// share it on the same tool and window, and a mutating and a read-only run never do.
    pub fn shares(&self, other: &ToolRunLane) -> bool {
        match (self.mutating, other.mutating) {
            (true, true) => true,
            (false, false) => self.tool_id == other.tool_id && self.window_id == other.window_id,
            _ => false,
        }
    }

    /// 🚦️ Admits a start on this lane among the instance's `live` runs: `toolRun.busy` while a run on the same lane is
    /// non-terminal, else the indices of the terminal runs on the lane the new run replaces.
    pub fn admit(&self, live: &[(ToolRunLane, ToolRunState)]) -> Result<Vec<usize>, ToolRunRejection> {
        let shared: Vec<usize> = live.iter().enumerate().filter(|(_, (lane, _))| lane.shares(self)).map(|(index, _)| index).collect();
        if shared.iter().any(|index| !live[*index].1.is_terminal()) {
            return Err(ToolRunRejection::Busy);
        }
        Ok(shared)
    }
}

/// ⚙️ The pure §2.2 reducer of one run slot; [`ToolRunLane::admit`] decides which slot a start may take.
pub struct ToolRunMachine;

impl ToolRunMachine {
    /// ⚖️ Applies `event` to `slot`: staleness guard first, then the transition table.
    pub fn apply(slot: Option<ToolRunSlot>, event: ToolRunEvent) -> Result<ToolRunTransition, ToolRunRejection> {
        use ToolRunEffect as F;
        use ToolRunEventKey as K;
        use ToolRunState as S;
        if let ToolRunEvent::Start { run } = event {
            return match slot {
                Some(current) if !current.state.is_terminal() => Err(ToolRunRejection::Busy),
                Some(current) if run <= current.run => Err(ToolRunRejection::Illegal),
                _ => Ok(ToolRunTransition { slot: Some(ToolRunSlot { run, generation: 0, state: S::Starting }), effect: F::SpawnJob }),
            };
        }
        if event == ToolRunEvent::Closed {
            return Ok(ToolRunTransition { slot: None, effect: F::RetireAll });
        }
        let current = slot.ok_or(ToolRunRejection::Stale)?;
        let (run, generation) = event.target();
        if run != Some(current.run) || generation.is_some_and(|generation| generation != current.generation) {
            return Err(ToolRunRejection::Stale);
        }
        let (to, effect, increment) = match (current.state, event.key()) {
            (S::Starting, K::JobAdmitted) => (Some(S::Running), F::Schedule, false),
            (S::Running, K::Pause) => (Some(S::Paused), F::StopScheduling, false),
            (S::Paused, K::Resume) => (Some(S::Running), F::Schedule, false),
            (S::Paused, K::Step) => (Some(S::Paused), F::DriveOneUnit, false),
            (S::Running | S::Paused, K::JobComplete) => (Some(S::Complete), F::HoldResult, false),
            (S::Running | S::Paused, K::SettingsChanged) => (Some(current.state), F::Reconfigure, true),
            (S::Complete, K::SettingsChanged) => (Some(S::Running), F::Reconfigure, true),
            (S::Running | S::Paused | S::Complete, K::BaseChanged) => (Some(current.state), F::Refold, true),
            // 🏁️ A running or paused run finalizes the partial result it holds: the driver closes the run job at its tick
            // boundary (placements are applied whole per tick) and publishes exactly the provisional edits computed so far.
            (S::Running | S::Paused | S::Complete, K::Finalize) => (Some(S::Finalizing), F::BeginFinalize, false),
            (S::Finalizing, K::PublicationComplete) => (Some(S::Finalized), F::ReleaseProvisional, false),
            (S::Finalizing, K::RevalidationConflicts) => (Some(S::Complete), F::RetractConflicts, true),
            (S::Finalizing, K::StoreRejected) => (Some(S::Complete), F::KeepProvisional, true),
            (S::Starting | S::Running | S::Paused | S::Complete, K::Abort | K::AbortWhilePublishing) => (Some(S::Aborting), F::CloseJob, false),
            (S::Finalizing, K::Abort) => (Some(S::Aborting), F::CancelBatch, false),
            (S::Finalizing, K::AbortWhilePublishing) => return Err(ToolRunRejection::Stale),
            (S::Aborting, K::AbortComplete) => (Some(S::Aborted), F::RetireProvisional, false),
            (S::Starting | S::Running | S::Paused, K::JobFault) => (Some(S::Faulted), F::DiscardProvisional, false),
            (S::Finalized | S::Aborted | S::Faulted, K::Dismiss) => (None, F::ClearTrace, false),
            _ => return Err(ToolRunRejection::Illegal),
        };
        let generation = if increment { current.generation.wrapping_add(1) } else { current.generation };
        Ok(ToolRunTransition { slot: to.map(|state| ToolRunSlot { run: current.run, generation, state }), effect })
    }
}
//#endregion 🔖️Lifecycle
