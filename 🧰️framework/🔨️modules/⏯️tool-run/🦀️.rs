//! ⏯️ Domain-neutral tool run contract: identity, the pure lifecycle reducer, progress and step
//! ring, trace pages with their resident store, tick codec and writer, the static
//! `ToolRunDefinition`, the framework-reserved actions, chords and EN/DE labels.
//!
//! Pure: no async, no store. Schema of record: `🧬️schema/🔣️.json`. Contract:
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️13/INTERACTIVE-TOOLS-VISIBLE-PROCESS/📋️tool-run-contract.md` §2, §3.1, §3.2.

use dsl::os_dsl::schema::{FieldSpec, FieldValue, RecordLayout, RecordSpec, RecordValue, Shape};
use dsl::os_pack::{decode_record_body_exact, encode_record_body, DecodeOptions, EncodeOptions};
use dsl::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::OnceLock;
use ui::wgpu::{Locale, LocalizedLabel};

//#region 🔖️Limits
/// 💍️ Newest steps kept by a `ToolRunStepRing`.
pub const TOOL_RUN_STEP_RING_CAPACITY: usize = 64;
/// 🧷️ Arguments one step substitutes into its reason template.
pub const TOOL_RUN_STEP_ARGS_MAX: usize = 4;
/// 🧮️ Counters one progress snapshot carries.
pub const TOOL_RUN_COUNTERS_MAX: usize = 8;
/// 🧱️ Provisional document ops one run may hold (§2.8).
pub const TOOL_RUN_PROVISIONAL_OPS_MAX: u32 = 65_536;
/// 🗄️ Trace records a run keeps resident (§3.2).
pub const TOOL_RUN_TRACE_RESIDENT_RECORDS: usize = 1_048_576;
/// 📄️ Ops one trace page carries.
pub const TOOL_RUN_TRACE_PAGE_OPS_MAX: usize = 4_096;
/// 📦️ Encoded bytes one trace page may take (`LARGE_PREVIEW_PATCH_BYTES`).
pub const TOOL_RUN_TRACE_PAGE_BYTES_MAX: usize = 262_144;
/// 🎞️ Encoded bytes one tick may take (one `RetainedJobPayload` page).
pub const TOOL_RUN_TICK_BYTES_MAX: usize = 262_144;
/// 🗜️ Minimum logged ops before a trace store compacts its page log.
pub const TOOL_RUN_TRACE_LOG_COMPACT_FLOOR: usize = 4_096;
/// 📣️ Minimum interval between polite status announcements (§2.6).
pub const TOOL_RUN_STATUS_ANNOUNCE_INTERVAL_MS: u64 = 2_000;
/// 🚧️ First reason code reserved for framework-owned steps; plugin reasons stay below it.
pub const TOOL_RUN_RESERVED_REASON_FLOOR: u16 = 0xFF00;
/// 🔁️ Framework step: the document changed and the provisional result is being re-applied.
pub const TOOL_RUN_REASON_REBASING: u16 = 0xFF00;
/// ⚔️ Framework step: `{0}` provisional changes conflict with the current artifact.
pub const TOOL_RUN_REASON_CONFLICT: u16 = 0xFF01;
/// ✂️ Framework step: the oldest `{0}` rejected attempts were evicted from the trace.
pub const TOOL_RUN_REASON_TRACE_TRUNCATED: u16 = 0xFF02;
/// 🧯️ Framework step: the provisional op cap `{0}` was reached and the run completed.
pub const TOOL_RUN_REASON_PROVISIONAL_CAP: u16 = 0xFF03;
/// 🏷️ Prefix of the `group_id` stamped on the one `Edit` a finalized run publishes.
pub const TOOL_RUN_GROUP_ID_PREFIX: &str = "toolRun:";
//#endregion 🔖️Limits

//#region 🔖️Identity
/// 🪪️ Per-instance run id; `run` is monotone and never reused.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ToolRunId {
    pub app_instance_id: u32,
    pub run: u64,
}

impl ToolRunId {
    /// 🎫️ The `group_id` of the single `Edit` this run publishes at finalize.
    pub fn group_id(self) -> String {
        format!("{TOOL_RUN_GROUP_ID_PREFIX}{}", self.run)
    }
}

/// 🧿️ Run id plus the staleness generation and the committed revision the overlay was folded from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ToolRunIdentity {
    pub id: ToolRunId,
    pub generation: u32,
    pub base_revision: [u8; 32],
}

impl ToolRunIdentity {
    /// 🌱️ Generation 0 identity of a freshly started run.
    pub fn new(id: ToolRunId, base_revision: [u8; 32]) -> Self {
        Self { id, generation: 0, base_revision }
    }

    /// ⏫️ Same run, generation + 1 (reconfigure, rebase, conflict return).
    pub fn next_generation(self) -> Self {
        Self { generation: self.generation.wrapping_add(1), ..self }
    }

    /// 🥇️ Lexicographic freshness key for pages, progress and panels at `sequence`.
    pub fn freshness(self, sequence: u64) -> ToolRunFreshness {
        ToolRunFreshness { run: self.id.run, generation: self.generation, sequence }
    }
}

/// 🥈️ `(run, generation, sequence)`; renderers drop anything older than the newest seen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToolRunFreshness {
    pub run: u64,
    pub generation: u32,
    pub sequence: u64,
}
//#endregion 🔖️Identity

//#region 🔖️Lifecycle
/// 🚦️ Lifecycle state of a non-empty ledger slot; "no run" is the absence of a slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
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

//#region 🔖️Progress
/// 🚥️ Verdict of one traced attempt; colours are framework semantic tokens.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ToolRunVerdict {
    Testing,
    Success,
    Warning,
    Danger,
}

impl ToolRunVerdict {
    pub const ALL: [Self; 4] = [Self::Testing, Self::Success, Self::Warning, Self::Danger];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Testing => "testing",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|verdict| verdict.as_str() == text)
    }

    pub fn ordinal(self) -> u8 {
        self as u8
    }

    pub fn from_ordinal(ordinal: u8) -> Option<Self> {
        Self::ALL.get(usize::from(ordinal)).copied()
    }

    /// 🗑️ `warning` and `danger` records are the only ones residency may evict.
    pub fn is_rejected(self) -> bool {
        matches!(self, Self::Warning | Self::Danger)
    }
}

/// 🪧️ Severity of one step-log entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ToolRunStepKind {
    Info,
    Success,
    Warning,
    Danger,
}

impl ToolRunStepKind {
    pub const ALL: [Self; 4] = [Self::Info, Self::Success, Self::Warning, Self::Danger];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.as_str() == text)
    }

    pub fn ordinal(self) -> u8 {
        self as u8
    }

    pub fn from_ordinal(ordinal: u64) -> Option<Self> {
        Self::ALL.get(usize::try_from(ordinal).ok()?).copied()
    }
}

/// 🪝️ One positional template argument.
#[derive(Clone, Copy, Debug)]
pub enum ToolRunStepArg {
    Unsigned(u64),
    Float(f64),
}

impl PartialEq for ToolRunStepArg {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Unsigned(left), Self::Unsigned(right)) => left == right,
            (Self::Float(left), Self::Float(right)) => left.to_bits() == right.to_bits(),
            _ => false,
        }
    }
}

impl ToolRunStepArg {
    /// 🔤️ Locale-neutral text used for template substitution.
    pub fn to_plain_string(self) -> String {
        match self {
            Self::Unsigned(value) => value.to_string(),
            Self::Float(value) => value.to_string(),
        }
    }
}

/// 📝️ One step-log entry; plugins send reason codes and arguments, never prose.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolRunStep {
    pub sequence: u64,
    pub kind: ToolRunStepKind,
    pub stage: u16,
    pub reason: u16,
    pub subject: Option<u64>,
    pub repeat: u32,
    pub args: Vec<ToolRunStepArg>,
}

impl ToolRunStep {
    /// 🧩️ A single (repeat 1) step without subject or arguments.
    pub fn new(sequence: u64, kind: ToolRunStepKind, stage: u16, reason: u16) -> Self {
        Self { sequence, kind, stage, reason, subject: None, repeat: 1, args: Vec::new() }
    }

    /// 🧲️ Identical apart from `sequence` and `repeat`, so consecutive pushes coalesce.
    pub fn coalesces_with(&self, other: &Self) -> bool {
        self.kind == other.kind && self.stage == other.stage && self.reason == other.reason && self.subject == other.subject && self.args == other.args
    }
}

/// 🎡️ Newest ≤ 64 steps, overwrite-oldest, identical consecutive steps coalesced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ToolRunStepRing {
    steps: VecDeque<ToolRunStep>,
}

impl ToolRunStepRing {
    pub fn new() -> Self {
        Self { steps: VecDeque::with_capacity(TOOL_RUN_STEP_RING_CAPACITY) }
    }

    /// 📥️ Coalesces into the newest entry or appends, evicting the oldest when full.
    pub fn push(&mut self, step: ToolRunStep) {
        if let Some(newest) = self.steps.back_mut().filter(|newest| newest.coalesces_with(&step)) {
            newest.repeat = newest.repeat.saturating_add(step.repeat);
            newest.sequence = step.sequence;
            return;
        }
        if self.steps.len() == TOOL_RUN_STEP_RING_CAPACITY {
            self.steps.pop_front();
        }
        self.steps.push_back(step);
    }

    /// 🧾️ Rebuilds a ring from wire order without coalescing.
    pub fn from_steps(steps: Vec<ToolRunStep>) -> Result<Self, ToolRunCodecError> {
        if steps.len() > TOOL_RUN_STEP_RING_CAPACITY {
            return Err(ToolRunCodecError::Limit("step ring capacity"));
        }
        Ok(Self { steps: steps.into() })
    }

    pub fn len(&self) -> usize {
        self.steps.len()
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &ToolRunStep> {
        self.steps.iter()
    }

    pub fn oldest(&self) -> Option<&ToolRunStep> {
        self.steps.front()
    }

    pub fn newest(&self) -> Option<&ToolRunStep> {
        self.steps.back()
    }

    pub fn clear(&mut self) {
        self.steps.clear();
    }
}

/// 📟️ One counter value by index into `ToolRunDefinition.counters`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ToolRunCounter {
    pub counter: u16,
    pub value: u64,
}

/// 📊️ Progress snapshot; status text and percentage are derived by the renderer.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolRunProgress {
    pub identity: ToolRunIdentity,
    pub sequence: u64,
    pub state: ToolRunState,
    pub stage: u16,
    pub completed: u64,
    pub total: Option<u64>,
    pub counters: Vec<ToolRunCounter>,
    pub units_per_second: f32,
    pub conflicts: u32,
    pub steps: ToolRunStepRing,
}

impl ToolRunProgress {
    /// ➗️ `completed / total`; `None` while indeterminate.
    pub fn fraction(&self) -> Option<f64> {
        self.total.filter(|total| *total > 0).map(|total| self.completed as f64 / total as f64)
    }
}
//#endregion 🔖️Progress

//#region 🔖️Trace
/// 👻️ What one trace record shows; meshes and shapes index the plugin's existing lanes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolRunTraceSubject {
    Instance3d { mesh: u32, position: [f32; 3], rotation: [f32; 4], scale: f32 },
    Placement2d { shape: u32, position: [f32; 2], rotation: f32 },
    Entity { entity: u64 },
}

impl ToolRunTraceSubject {
    fn ordinal(self) -> u8 {
        match self {
            Self::Instance3d { .. } => 0,
            Self::Placement2d { .. } => 1,
            Self::Entity { .. } => 2,
        }
    }

    fn wire_bytes(self) -> usize {
        match self {
            Self::Instance3d { .. } => 36,
            Self::Placement2d { .. } => 16,
            Self::Entity { .. } => 8,
        }
    }
}

/// ✏️ One trace delta operation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToolRunTraceOp {
    Upsert { key: u64, verdict: ToolRunVerdict, reason: u16, subject: ToolRunTraceSubject },
    Retire { key: u64 },
    Clear,
}

impl ToolRunTraceOp {
    /// 📏️ Deterministic column-byte estimate used for page and delivery budgets.
    pub fn wire_bytes(self) -> usize {
        match self {
            Self::Upsert { subject, .. } => 13 + subject.wire_bytes(),
            Self::Retire { .. } => 9,
            Self::Clear => 1,
        }
    }
}

/// 📎️ Fixed per-page overhead of the delivery budget estimate.
pub const TOOL_RUN_TRACE_PAGE_OVERHEAD_BYTES: usize = 64;

/// 📃️ One columnar delta page.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolRunTracePage {
    pub identity: ToolRunIdentity,
    pub page: u32,
    pub ops: Vec<ToolRunTraceOp>,
}

impl ToolRunTracePage {
    /// 📐️ Delivery budget estimate: overhead plus every op's column bytes.
    pub fn wire_bytes_estimate(ops: &[ToolRunTraceOp]) -> usize {
        TOOL_RUN_TRACE_PAGE_OVERHEAD_BYTES + ops.iter().map(|op| op.wire_bytes()).sum::<usize>()
    }

    /// 🎒️ Pack record body; rejects pages over the op or byte cap.
    pub fn encode(&self) -> Result<Vec<u8>, ToolRunCodecError> {
        let bytes = encode_body(page_spec(), &page_record(self)?)?;
        if bytes.len() > TOOL_RUN_TRACE_PAGE_BYTES_MAX {
            return Err(ToolRunCodecError::Limit("trace page bytes"));
        }
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ToolRunCodecError> {
        if bytes.len() > TOOL_RUN_TRACE_PAGE_BYTES_MAX {
            return Err(ToolRunCodecError::Limit("trace page bytes"));
        }
        page_from_record(&decode_body(page_spec(), bytes)?)
    }
}

/// 📬️ One refresh of trace pages for a window; `clear` drops every record before `pages`.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolRunTraceDelta {
    pub identity: ToolRunIdentity,
    pub clear: bool,
    pub next: u32,
    pub pages: Vec<ToolRunTracePage>,
}

impl ToolRunTraceDelta {
    pub fn encode(&self) -> Result<Vec<u8>, ToolRunCodecError> {
        let mut record = RecordValue::default();
        record.fields.insert(1, FieldValue::Record(identity_record(&self.identity)));
        record.fields.insert(2, FieldValue::Bool(self.clear));
        record.fields.insert(3, FieldValue::UInt(u64::from(self.next)));
        if !self.pages.is_empty() {
            record.fields.insert(4, FieldValue::List(self.pages.iter().map(|page| page.encode().map(FieldValue::Bytes64)).collect::<Result<_, _>>()?));
        }
        encode_body(delta_spec(), &record)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ToolRunCodecError> {
        let record = decode_body(delta_spec(), bytes)?;
        let pages = list(&record, 4)?.iter().map(|item| ToolRunTracePage::decode(item_bytes(item)?)).collect::<Result<_, _>>()?;
        Ok(Self { identity: identity_from_field(&record, 1)?, clear: boolean(&record, 2)?, next: narrow(uint(&record, 3)?, "next")?, pages })
    }
}

/// 🧭️ Echoed by a renderer in its window instance view state; `page` is the next page it expects.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunTraceCursor {
    pub run: u64,
    pub generation: u32,
    pub page: u32,
}

/// 🗂️ One resident trace record.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToolRunTraceRecord {
    pub verdict: ToolRunVerdict,
    pub reason: u16,
    pub subject: ToolRunTraceSubject,
    stamp: u64,
}

/// 📋️ Outcome of applying ops to a `ToolRunTraceStore`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ToolRunTraceApply {
    pub logged: u32,
    pub evicted: u32,
    pub overflowed: u32,
    pub compacted: bool,
}

#[derive(Clone, Debug)]
struct ToolRunTraceLogPage {
    ops: Vec<ToolRunTraceOp>,
    bytes: usize,
}

/// 🗃️ Resident trace records plus the page log windows read through cursors (§3.2).
#[derive(Clone, Debug)]
pub struct ToolRunTraceStore {
    identity: ToolRunIdentity,
    capacity: usize,
    compact_floor: usize,
    records: HashMap<u64, ToolRunTraceRecord>,
    rejected: VecDeque<(u64, u64)>,
    next_stamp: u64,
    log: VecDeque<ToolRunTraceLogPage>,
    log_base: u32,
    next_page: u32,
    logged_ops: usize,
}

impl ToolRunTraceStore {
    pub fn new(identity: ToolRunIdentity) -> Self {
        Self::with_limits(identity, TOOL_RUN_TRACE_RESIDENT_RECORDS, TOOL_RUN_TRACE_LOG_COMPACT_FLOOR)
    }

    /// 🎛️ Store with an explicit residency capacity and log compaction floor.
    pub fn with_limits(identity: ToolRunIdentity, capacity: usize, compact_floor: usize) -> Self {
        Self { identity, capacity: capacity.max(1), compact_floor: compact_floor.max(1), records: HashMap::new(), rejected: VecDeque::new(), next_stamp: 0, log: VecDeque::new(), log_base: 0, next_page: 0, logged_ops: 0 }
    }

    pub fn identity(&self) -> ToolRunIdentity {
        self.identity
    }

    /// 🔗️ Adopts a new generation (records kept) or a new run (everything dropped).
    pub fn rebind(&mut self, identity: ToolRunIdentity) {
        if identity.id != self.identity.id {
            *self = Self::with_limits(identity, self.capacity, self.compact_floor);
        }
        self.identity = identity;
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn record(&self, key: u64) -> Option<&ToolRunTraceRecord> {
        self.records.get(&key)
    }

    pub fn records(&self) -> impl Iterator<Item = (u64, &ToolRunTraceRecord)> {
        self.records.iter().map(|(key, record)| (*key, record))
    }

    pub fn log_base(&self) -> u32 {
        self.log_base
    }

    pub fn next_page(&self) -> u32 {
        self.next_page
    }

    /// 📖️ Ops of logged page `page`, if still retained.
    pub fn log_page(&self, page: u32) -> Option<&[ToolRunTraceOp]> {
        let index = page.checked_sub(self.log_base)?;
        self.log.get(usize::try_from(index).ok()?).map(|logged| logged.ops.as_slice())
    }

    /// 🛂️ Applies a job page; pages of another run or generation are stale.
    pub fn apply_page(&mut self, page: &ToolRunTracePage) -> Result<ToolRunTraceApply, ToolRunRejection> {
        if page.identity.id != self.identity.id || page.identity.generation != self.identity.generation {
            return Err(ToolRunRejection::Stale);
        }
        Ok(self.apply_ops(&page.ops))
    }

    /// 🧺️ Applies ops with residency eviction and logs what renderers must replay.
    pub fn apply_ops(&mut self, ops: &[ToolRunTraceOp]) -> ToolRunTraceApply {
        let mut outcome = ToolRunTraceApply::default();
        let mut logged = Vec::with_capacity(ops.len());
        for op in ops {
            match *op {
                ToolRunTraceOp::Clear => {
                    self.records.clear();
                    self.rejected.clear();
                    logged.push(ToolRunTraceOp::Clear);
                }
                ToolRunTraceOp::Retire { key } => {
                    if self.records.remove(&key).is_some() {
                        logged.push(*op);
                    }
                }
                ToolRunTraceOp::Upsert { key, verdict, reason, subject } => {
                    if !self.records.contains_key(&key) && self.records.len() >= self.capacity {
                        if let Some(victim) = self.evict_oldest_rejected() {
                            logged.push(ToolRunTraceOp::Retire { key: victim });
                            outcome.evicted += 1;
                        } else if verdict.is_rejected() {
                            outcome.evicted += 1;
                            continue;
                        } else {
                            outcome.overflowed += 1;
                            continue;
                        }
                    }
                    let stamp = self.next_stamp;
                    self.next_stamp += 1;
                    self.records.insert(key, ToolRunTraceRecord { verdict, reason, subject, stamp });
                    if verdict.is_rejected() {
                        self.rejected.push_back((key, stamp));
                    }
                    logged.push(*op);
                }
            }
        }
        outcome.logged = self.log_ops(&logged);
        if self.rejected.len() > 2 * self.records.len() + TOOL_RUN_STEP_RING_CAPACITY {
            self.rebuild_rejected();
        }
        if self.logged_ops > 2 * self.records.len().max(self.compact_floor) {
            self.compact();
            outcome.compacted = true;
        }
        outcome
    }

    /// 📮️ Pages after `cursor` within `byte_budget` (always at least one pending page).
    pub fn delta_after(&self, cursor: Option<ToolRunTraceCursor>, byte_budget: usize) -> ToolRunTraceDelta {
        let resend = cursor.is_none_or(|cursor| cursor.run != self.identity.id.run || cursor.generation != self.identity.generation || cursor.page < self.log_base || cursor.page > self.next_page);
        let start = if resend { self.log_base } else { cursor.map_or(self.log_base, |cursor| cursor.page) };
        let mut pages = Vec::new();
        let mut bytes = 0usize;
        for (offset, logged) in self.log.iter().enumerate().skip((start - self.log_base) as usize) {
            if !pages.is_empty() && bytes + logged.bytes > byte_budget {
                break;
            }
            bytes += logged.bytes;
            pages.push(ToolRunTracePage { identity: self.identity, page: self.log_base + offset as u32, ops: logged.ops.clone() });
        }
        ToolRunTraceDelta { identity: self.identity, clear: resend, next: start + pages.len() as u32, pages }
    }

    fn log_ops(&mut self, ops: &[ToolRunTraceOp]) -> u32 {
        let mut pages = 0;
        for chunk in ops.chunks(TOOL_RUN_TRACE_PAGE_OPS_MAX) {
            self.logged_ops += chunk.len();
            self.log.push_back(ToolRunTraceLogPage { bytes: ToolRunTracePage::wire_bytes_estimate(chunk), ops: chunk.to_vec() });
            self.next_page += 1;
            pages += 1;
        }
        pages
    }

    fn evict_oldest_rejected(&mut self) -> Option<u64> {
        while let Some((key, stamp)) = self.rejected.pop_front() {
            if self.records.get(&key).is_some_and(|record| record.stamp == stamp && record.verdict.is_rejected()) {
                self.records.remove(&key);
                return Some(key);
            }
        }
        None
    }

    fn live_by_stamp(&self) -> Vec<(u64, ToolRunTraceRecord)> {
        let mut live: Vec<(u64, ToolRunTraceRecord)> = self.records.iter().map(|(key, record)| (*key, *record)).collect();
        live.sort_unstable_by_key(|(_, record)| record.stamp);
        live
    }

    fn rebuild_rejected(&mut self) {
        self.rejected = self.live_by_stamp().into_iter().filter(|(_, record)| record.verdict.is_rejected()).map(|(key, record)| (key, record.stamp)).collect();
    }

    fn compact(&mut self) {
        let live = self.live_by_stamp();
        let snapshot: Vec<ToolRunTraceOp> = std::iter::once(ToolRunTraceOp::Clear).chain(live.iter().map(|(key, record)| ToolRunTraceOp::Upsert { key: *key, verdict: record.verdict, reason: record.reason, subject: record.subject })).collect();
        self.rejected = live.iter().filter(|(_, record)| record.verdict.is_rejected()).map(|(key, record)| (*key, record.stamp)).collect();
        self.log.clear();
        self.logged_ops = 0;
        self.log_base = self.next_page;
        self.log_ops(&snapshot);
    }
}
//#endregion 🔖️Trace

//#region 🔖️Tick
/// 📽️ One job report: provisional ops, trace pages, steps and optional progress.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolRunTick {
    pub identity: ToolRunIdentity,
    pub sequence: u64,
    pub progress: Option<ToolRunProgress>,
    pub steps: Vec<ToolRunStep>,
    pub trace: Vec<ToolRunTracePage>,
    pub append_ops: Vec<Vec<u8>>,
    pub append_entities: Vec<u64>,
    pub retract_to: Option<u32>,
    /// 📨️ Opaque plugin bytes the run's windows read back through `ToolRunView::payload` — the latest intermediate
    /// result of the run (a live readout, a partial solution); the newest tick carrying one wins.
    pub payload: Option<Vec<u8>>,
}

impl ToolRunTick {
    /// 🧳️ Pack record body; rejects ticks over `TOOL_RUN_TICK_BYTES_MAX`.
    pub fn encode(&self) -> Result<Vec<u8>, ToolRunCodecError> {
        let mut record = RecordValue::default();
        record.fields.insert(1, FieldValue::Record(identity_record(&self.identity)));
        record.fields.insert(2, FieldValue::UInt(self.sequence));
        if let Some(progress) = &self.progress {
            record.fields.insert(3, FieldValue::Record(progress_record(progress)?));
        }
        if !self.steps.is_empty() {
            record.fields.insert(4, FieldValue::List(self.steps.iter().map(|step| step_record(step).map(FieldValue::Record)).collect::<Result<_, _>>()?));
        }
        if !self.trace.is_empty() {
            record.fields.insert(5, FieldValue::List(self.trace.iter().map(|page| page.encode().map(FieldValue::Bytes64)).collect::<Result<_, _>>()?));
        }
        if !self.append_ops.is_empty() {
            record.fields.insert(6, FieldValue::List(self.append_ops.iter().map(|op| FieldValue::Bytes64(op.clone())).collect()));
        }
        if !self.append_entities.is_empty() {
            record.fields.insert(7, FieldValue::Bytes64(self.append_entities.iter().flat_map(|entity| entity.to_le_bytes()).collect()));
        }
        if let Some(retract_to) = self.retract_to {
            record.fields.insert(8, FieldValue::UInt(u64::from(retract_to)));
        }
        if let Some(payload) = &self.payload {
            record.fields.insert(9, FieldValue::Bytes64(payload.clone()));
        }
        let bytes = encode_body(tick_spec(), &record)?;
        if bytes.len() > TOOL_RUN_TICK_BYTES_MAX {
            return Err(ToolRunCodecError::Limit("tick bytes"));
        }
        Ok(bytes)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, ToolRunCodecError> {
        if bytes.len() > TOOL_RUN_TICK_BYTES_MAX {
            return Err(ToolRunCodecError::Limit("tick bytes"));
        }
        let record = decode_body(tick_spec(), bytes)?;
        let progress = match record.get(3) {
            None | Some(FieldValue::Absent) => None,
            Some(FieldValue::Record(progress)) => Some(progress_from_record(progress)?),
            Some(_) => return Err(ToolRunCodecError::Malformed("progress")),
        };
        let steps = list(&record, 4)?.iter().map(|item| step_from_record(item_record(item)?)).collect::<Result<_, _>>()?;
        let trace = list(&record, 5)?.iter().map(|item| ToolRunTracePage::decode(item_bytes(item)?)).collect::<Result<_, _>>()?;
        let append_ops = list(&record, 6)?.iter().map(|item| item_bytes(item).map(<[u8]>::to_vec)).collect::<Result<_, _>>()?;
        let append_entities = u64_column(bytes_field(&record, 7)?, "appendEntities")?;
        let retract_to = optional_uint(&record, 8)?.map(|value| narrow(value, "retractTo")).transpose()?;
        let payload = match record.get(9) {
            None | Some(FieldValue::Absent) => None,
            Some(FieldValue::Bytes64(payload)) => Some(payload.clone()),
            Some(_) => return Err(ToolRunCodecError::Malformed("payload")),
        };
        Ok(Self { identity: identity_from_field(&record, 1)?, sequence: uint(&record, 2)?, progress, steps, trace, append_ops, append_entities, retract_to, payload })
    }
}

/// 🛑️ A writer call that would exceed a contract limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRunLimitError {
    ProvisionalOps,
    StepArgs,
}

impl std::fmt::Display for ToolRunLimitError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::ProvisionalOps => "toolRun.limit.provisionalOps",
            Self::StepArgs => "toolRun.limit.stepArgs",
        })
    }
}

impl std::error::Error for ToolRunLimitError {}

/// ✍️ Domain-neutral tick builder for run jobs (§3.7): bytes only, pages split at 4 096 ops.
#[derive(Clone, Debug)]
pub struct ToolRunTickWriter {
    identity: ToolRunIdentity,
    next_sequence: u64,
    next_page: u32,
    next_step: u64,
    provisional_base: u32,
    progress: Option<ToolRunProgress>,
    steps: Vec<ToolRunStep>,
    pages: Vec<ToolRunTracePage>,
    page_ops: Vec<ToolRunTraceOp>,
    append_ops: Vec<Vec<u8>>,
    append_entities: Vec<u64>,
    entity_marks: Vec<u32>,
    retract_to: Option<u32>,
    payload: Option<Vec<u8>>,
    pending_bytes: usize,
}

impl ToolRunTickWriter {
    pub fn new(identity: ToolRunIdentity) -> Self {
        Self::with_provisional_base(identity, 0)
    }

    /// ⏩️ A writer continuing a run whose provisional list already holds `provisional_len` ops — a job
    /// resumed from its checkpoint or a revalidate job — so `retract_to` below that length emits
    /// `retractTo` and `append_op` honours the provisional cap from there.
    pub fn with_provisional_base(identity: ToolRunIdentity, provisional_len: u32) -> Self {
        Self {
            identity,
            next_sequence: 0,
            next_page: 0,
            next_step: 0,
            provisional_base: provisional_len,
            progress: None,
            steps: Vec::new(),
            pages: Vec::new(),
            page_ops: Vec::new(),
            append_ops: Vec::new(),
            append_entities: Vec::new(),
            entity_marks: Vec::new(),
            retract_to: None,
            payload: None,
            pending_bytes: 0,
        }
    }

    pub fn identity(&self) -> ToolRunIdentity {
        self.identity
    }

    /// 🪢️ Stamps later ticks with a new identity (generation change).
    pub fn rebind(&mut self, identity: ToolRunIdentity) {
        self.identity = identity;
        for page in &mut self.pages {
            page.identity = identity;
        }
    }

    /// 🪙️ Provisional op count after the pending tick is applied.
    pub fn provisional_len(&self) -> u32 {
        self.provisional_base + self.append_ops.len() as u32
    }

    pub fn upsert(&mut self, key: u64, verdict: ToolRunVerdict, reason: u16, subject: ToolRunTraceSubject) {
        self.trace_op(ToolRunTraceOp::Upsert { key, verdict, reason, subject });
    }

    pub fn retire(&mut self, key: u64) {
        self.trace_op(ToolRunTraceOp::Retire { key });
    }

    pub fn clear_trace(&mut self) {
        self.trace_op(ToolRunTraceOp::Clear);
    }

    /// 🗒️ Appends a step with the writer's next step sequence.
    pub fn step(&mut self, kind: ToolRunStepKind, stage: u16, reason: u16, subject: Option<u64>, args: &[ToolRunStepArg]) -> Result<(), ToolRunLimitError> {
        if args.len() > TOOL_RUN_STEP_ARGS_MAX {
            return Err(ToolRunLimitError::StepArgs);
        }
        self.steps.push(ToolRunStep { sequence: self.next_step, kind, stage, reason, subject, repeat: 1, args: args.to_vec() });
        self.next_step += 1;
        self.pending_bytes += 40 + 9 * args.len();
        Ok(())
    }

    /// 🧬️ Appends one `OpBinary`-encoded provisional op; refuses past `TOOL_RUN_PROVISIONAL_OPS_MAX`.
    pub fn append_op(&mut self, op: Vec<u8>) -> Result<(), ToolRunLimitError> {
        if self.provisional_len() >= TOOL_RUN_PROVISIONAL_OPS_MAX {
            return Err(ToolRunLimitError::ProvisionalOps);
        }
        self.pending_bytes += op.len() + 6;
        self.append_ops.push(op);
        Ok(())
    }

    /// 🏷️ Tags `entity` with the provisional length reached so far; a later `retract_to` below that length
    /// drops it again.
    pub fn append_entity(&mut self, entity: u64) {
        self.pending_bytes += 8;
        self.entity_marks.push(self.provisional_len());
        self.append_entities.push(entity);
    }

    /// 🪚️ Truncates the provisional list to `len` ops; pending appends beyond it are dropped, and so is
    /// every pending entity appended once the list was longer than `len`.
    pub fn retract_to(&mut self, len: u32) {
        if len >= self.provisional_base {
            self.append_ops.truncate((len - self.provisional_base) as usize);
        } else {
            self.append_ops.clear();
            self.retract_to = Some(self.retract_to.map_or(len, |retract| retract.min(len)));
            self.provisional_base = len;
        }
        let kept = self.entity_marks.iter().take_while(|mark| **mark <= len).count();
        self.pending_bytes = self.pending_bytes.saturating_sub((self.append_entities.len() - kept) * 8);
        self.append_entities.truncate(kept);
        self.entity_marks.truncate(kept);
    }

    pub fn progress(&mut self, progress: ToolRunProgress) {
        self.progress = Some(progress);
    }

    /// 📨️ Sets the pending tick's plugin payload; a later call before `finish` replaces it.
    pub fn payload(&mut self, payload: Vec<u8>) {
        self.pending_bytes = self.pending_bytes.saturating_sub(self.payload.as_ref().map_or(0, |previous| previous.len() + 6)) + payload.len() + 6;
        self.payload = Some(payload);
    }

    pub fn is_empty(&self) -> bool {
        self.progress.is_none() && self.steps.is_empty() && self.pages.is_empty() && self.page_ops.is_empty() && self.append_ops.is_empty() && self.append_entities.is_empty() && self.retract_to.is_none() && self.payload.is_none()
    }

    /// 🌡️ Estimated encoded bytes of the pending tick.
    pub fn pending_bytes(&self) -> usize {
        self.pending_bytes
    }

    /// 🚰️ True once the pending tick should be finished to stay under the tick byte cap.
    pub fn should_flush(&self) -> bool {
        self.pending_bytes >= TOOL_RUN_TICK_BYTES_MAX / 2
    }

    /// 🎬️ Takes the pending tick with the next sequence, or `None` when nothing is pending.
    pub fn finish(&mut self) -> Option<ToolRunTick> {
        self.seal_page();
        if self.is_empty() {
            return None;
        }
        let tick = ToolRunTick {
            identity: self.identity,
            sequence: self.next_sequence,
            progress: self.progress.take(),
            steps: std::mem::take(&mut self.steps),
            trace: std::mem::take(&mut self.pages),
            append_ops: std::mem::take(&mut self.append_ops),
            append_entities: std::mem::take(&mut self.append_entities),
            retract_to: self.retract_to.take(),
            payload: self.payload.take(),
        };
        self.entity_marks.clear();
        self.provisional_base += tick.append_ops.len() as u32;
        self.next_sequence += 1;
        self.pending_bytes = 0;
        Some(tick)
    }

    fn trace_op(&mut self, op: ToolRunTraceOp) {
        self.pending_bytes += op.wire_bytes();
        self.page_ops.push(op);
        if self.page_ops.len() == TOOL_RUN_TRACE_PAGE_OPS_MAX {
            self.seal_page();
        }
    }

    fn seal_page(&mut self) {
        if self.page_ops.is_empty() {
            return;
        }
        self.pending_bytes += TOOL_RUN_TRACE_PAGE_OVERHEAD_BYTES;
        self.pages.push(ToolRunTracePage { identity: self.identity, page: self.next_page, ops: std::mem::take(&mut self.page_ops) });
        self.next_page += 1;
    }
}
//#endregion 🔖️Tick

//#region 🔖️Codec
/// 🧨️ Encode or decode failure of a tool run wire value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ToolRunCodecError {
    Pack(String),
    Malformed(&'static str),
    Limit(&'static str),
}

impl std::fmt::Display for ToolRunCodecError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pack(detail) => write!(formatter, "toolRun.codec.pack: {detail}"),
            Self::Malformed(what) => write!(formatter, "toolRun.codec.malformed: {what}"),
            Self::Limit(what) => write!(formatter, "toolRun.codec.limit: {what}"),
        }
    }
}

impl std::error::Error for ToolRunCodecError {}

fn field(id: u16, key: &str, shape: Shape) -> FieldSpec {
    FieldSpec::new(id, key, shape)
}

fn spec(fields: Vec<FieldSpec>) -> RecordSpec {
    RecordSpec::new(None, RecordLayout::Inline, fields)
}

fn identity_spec_owned() -> RecordSpec {
    spec(vec![field(1, "appInstanceId", Shape::UInt), field(2, "run", Shape::UInt), field(3, "generation", Shape::UInt), field(4, "baseRevision", Shape::Bytes64)])
}

fn step_spec_owned() -> RecordSpec {
    spec(vec![field(1, "sequence", Shape::UInt), field(2, "kind", Shape::UInt), field(3, "stage", Shape::UInt), field(4, "reason", Shape::UInt), field(5, "subject", Shape::UInt).optional(), field(6, "repeat", Shape::UInt), field(7, "args", Shape::Bytes64).optional()])
}

fn progress_spec_owned() -> RecordSpec {
    spec(vec![
        field(1, "identity", Shape::Record(identity_spec_owned)),
        field(2, "sequence", Shape::UInt),
        field(3, "state", Shape::UInt),
        field(4, "stage", Shape::UInt),
        field(5, "completed", Shape::UInt),
        field(6, "total", Shape::UInt).optional(),
        field(7, "counters", Shape::Bytes64).optional(),
        field(8, "unitsPerSecond", Shape::Float),
        field(9, "conflicts", Shape::UInt),
        field(10, "steps", Shape::List(Box::new(Shape::Record(step_spec_owned)))).optional(),
    ])
}

fn page_spec() -> &'static RecordSpec {
    static SPEC: OnceLock<RecordSpec> = OnceLock::new();
    SPEC.get_or_init(|| {
        let mut fields = vec![field(1, "identity", Shape::Record(identity_spec_owned)), field(2, "page", Shape::UInt)];
        for (id, key) in [(3, "opKinds"), (4, "keys"), (5, "verdicts"), (6, "reasons"), (7, "subjectKinds"), (8, "meshes"), (9, "shapes"), (10, "entities"), (11, "floats")] {
            fields.push(field(id, key, Shape::Bytes64).optional());
        }
        spec(fields)
    })
}

fn delta_spec() -> &'static RecordSpec {
    static SPEC: OnceLock<RecordSpec> = OnceLock::new();
    SPEC.get_or_init(|| spec(vec![field(1, "identity", Shape::Record(identity_spec_owned)), field(2, "clear", Shape::Bool), field(3, "next", Shape::UInt), field(4, "pages", Shape::List(Box::new(Shape::Bytes64))).optional()]))
}

fn tick_spec() -> &'static RecordSpec {
    static SPEC: OnceLock<RecordSpec> = OnceLock::new();
    SPEC.get_or_init(|| {
        spec(vec![
            field(1, "identity", Shape::Record(identity_spec_owned)),
            field(2, "sequence", Shape::UInt),
            field(3, "progress", Shape::Record(progress_spec_owned)).optional(),
            field(4, "steps", Shape::List(Box::new(Shape::Record(step_spec_owned)))).optional(),
            field(5, "trace", Shape::List(Box::new(Shape::Bytes64))).optional(),
            field(6, "appendOps", Shape::List(Box::new(Shape::Bytes64))).optional(),
            field(7, "appendEntities", Shape::Bytes64).optional(),
            field(8, "retractTo", Shape::UInt).optional(),
            field(9, "payload", Shape::Bytes64).optional(),
        ])
    })
}

fn encode_body(spec: &RecordSpec, record: &RecordValue) -> Result<Vec<u8>, ToolRunCodecError> {
    encode_record_body(spec, record, &EncodeOptions::default()).map_err(|error| ToolRunCodecError::Pack(error.to_string()))
}

fn decode_body(spec: &RecordSpec, bytes: &[u8]) -> Result<RecordValue, ToolRunCodecError> {
    decode_record_body_exact(bytes, spec, &DecodeOptions::default()).map_err(|error| ToolRunCodecError::Pack(error.to_string()))
}

fn identity_record(identity: &ToolRunIdentity) -> RecordValue {
    let mut record = RecordValue::default();
    record.fields.insert(1, FieldValue::UInt(u64::from(identity.id.app_instance_id)));
    record.fields.insert(2, FieldValue::UInt(identity.id.run));
    record.fields.insert(3, FieldValue::UInt(u64::from(identity.generation)));
    record.fields.insert(4, FieldValue::Bytes64(identity.base_revision.to_vec()));
    record
}

fn identity_from_record(record: &RecordValue) -> Result<ToolRunIdentity, ToolRunCodecError> {
    let base_revision = bytes_field(record, 4)?.try_into().map_err(|_| ToolRunCodecError::Malformed("baseRevision"))?;
    Ok(ToolRunIdentity { id: ToolRunId { app_instance_id: narrow(uint(record, 1)?, "appInstanceId")?, run: uint(record, 2)? }, generation: narrow(uint(record, 3)?, "generation")?, base_revision })
}

fn identity_from_field(record: &RecordValue, id: u16) -> Result<ToolRunIdentity, ToolRunCodecError> {
    match record.get(id) {
        Some(FieldValue::Record(identity)) => identity_from_record(identity),
        _ => Err(ToolRunCodecError::Malformed("identity")),
    }
}

fn step_record(step: &ToolRunStep) -> Result<RecordValue, ToolRunCodecError> {
    if step.args.len() > TOOL_RUN_STEP_ARGS_MAX {
        return Err(ToolRunCodecError::Limit("step args"));
    }
    if step.repeat == 0 {
        return Err(ToolRunCodecError::Malformed("repeat"));
    }
    let mut record = RecordValue::default();
    record.fields.insert(1, FieldValue::UInt(step.sequence));
    record.fields.insert(2, FieldValue::UInt(u64::from(step.kind.ordinal())));
    record.fields.insert(3, FieldValue::UInt(u64::from(step.stage)));
    record.fields.insert(4, FieldValue::UInt(u64::from(step.reason)));
    if let Some(subject) = step.subject {
        record.fields.insert(5, FieldValue::UInt(subject));
    }
    record.fields.insert(6, FieldValue::UInt(u64::from(step.repeat)));
    if !step.args.is_empty() {
        let mut args = Vec::with_capacity(9 * step.args.len());
        for arg in &step.args {
            match *arg {
                ToolRunStepArg::Unsigned(value) => {
                    args.push(0);
                    args.extend_from_slice(&value.to_le_bytes());
                }
                ToolRunStepArg::Float(value) => {
                    args.push(1);
                    args.extend_from_slice(&value.to_le_bytes());
                }
            }
        }
        record.fields.insert(7, FieldValue::Bytes64(args));
    }
    Ok(record)
}

fn step_from_record(record: &RecordValue) -> Result<ToolRunStep, ToolRunCodecError> {
    let raw_args = bytes_field(record, 7)?;
    let (args, rest) = raw_args.as_chunks::<9>();
    if !rest.is_empty() || args.len() > TOOL_RUN_STEP_ARGS_MAX {
        return Err(ToolRunCodecError::Malformed("step args"));
    }
    let args = args
        .iter()
        .map(|[kind, value @ ..]| match kind {
            0 => Ok(ToolRunStepArg::Unsigned(u64::from_le_bytes(*value))),
            1 => Ok(ToolRunStepArg::Float(f64::from_le_bytes(*value))),
            _ => Err(ToolRunCodecError::Malformed("step arg kind")),
        })
        .collect::<Result<_, _>>()?;
    let repeat = narrow(uint(record, 6)?, "repeat")?;
    if repeat == 0 {
        return Err(ToolRunCodecError::Malformed("repeat"));
    }
    Ok(ToolRunStep {
        sequence: uint(record, 1)?,
        kind: ToolRunStepKind::from_ordinal(uint(record, 2)?).ok_or(ToolRunCodecError::Malformed("step kind"))?,
        stage: narrow(uint(record, 3)?, "stage")?,
        reason: narrow(uint(record, 4)?, "reason")?,
        subject: optional_uint(record, 5)?,
        repeat,
        args,
    })
}

fn progress_record(progress: &ToolRunProgress) -> Result<RecordValue, ToolRunCodecError> {
    if progress.counters.len() > TOOL_RUN_COUNTERS_MAX {
        return Err(ToolRunCodecError::Limit("counters"));
    }
    let mut record = RecordValue::default();
    record.fields.insert(1, FieldValue::Record(identity_record(&progress.identity)));
    record.fields.insert(2, FieldValue::UInt(progress.sequence));
    record.fields.insert(3, FieldValue::UInt(u64::from(progress.state.ordinal())));
    record.fields.insert(4, FieldValue::UInt(u64::from(progress.stage)));
    record.fields.insert(5, FieldValue::UInt(progress.completed));
    if let Some(total) = progress.total {
        record.fields.insert(6, FieldValue::UInt(total));
    }
    if !progress.counters.is_empty() {
        record.fields.insert(7, FieldValue::Bytes64(progress.counters.iter().flat_map(|counter| counter.counter.to_le_bytes().into_iter().chain(counter.value.to_le_bytes())).collect()));
    }
    record.fields.insert(8, FieldValue::Float(f64::from(progress.units_per_second)));
    record.fields.insert(9, FieldValue::UInt(u64::from(progress.conflicts)));
    if !progress.steps.is_empty() {
        record.fields.insert(10, FieldValue::List(progress.steps.iter().map(|step| step_record(step).map(FieldValue::Record)).collect::<Result<_, _>>()?));
    }
    Ok(record)
}

fn progress_from_record(record: &RecordValue) -> Result<ToolRunProgress, ToolRunCodecError> {
    let raw_counters = bytes_field(record, 7)?;
    let (counters, rest) = raw_counters.as_chunks::<10>();
    if !rest.is_empty() || counters.len() > TOOL_RUN_COUNTERS_MAX {
        return Err(ToolRunCodecError::Malformed("counters"));
    }
    let counters = counters.iter().map(|[c0, c1, value @ ..]| ToolRunCounter { counter: u16::from_le_bytes([*c0, *c1]), value: u64::from_le_bytes(*value) }).collect();
    let steps = list(record, 10)?.iter().map(|item| step_from_record(item_record(item)?)).collect::<Result<Vec<_>, _>>()?;
    let units_per_second = match record.get(8) {
        Some(FieldValue::Float(value)) => *value as f32,
        _ => return Err(ToolRunCodecError::Malformed("unitsPerSecond")),
    };
    Ok(ToolRunProgress {
        identity: identity_from_field(record, 1)?,
        sequence: uint(record, 2)?,
        state: ToolRunState::from_ordinal(uint(record, 3)?).ok_or(ToolRunCodecError::Malformed("state"))?,
        stage: narrow(uint(record, 4)?, "stage")?,
        completed: uint(record, 5)?,
        total: optional_uint(record, 6)?,
        counters,
        units_per_second,
        conflicts: narrow(uint(record, 9)?, "conflicts")?,
        steps: ToolRunStepRing::from_steps(steps)?,
    })
}

fn page_record(page: &ToolRunTracePage) -> Result<RecordValue, ToolRunCodecError> {
    if page.ops.len() > TOOL_RUN_TRACE_PAGE_OPS_MAX {
        return Err(ToolRunCodecError::Limit("trace page ops"));
    }
    let mut columns: [Vec<u8>; 9] = Default::default();
    let [kinds, keys, verdicts, reasons, subject_kinds, meshes, shapes, entities, floats] = &mut columns;
    for op in &page.ops {
        match *op {
            ToolRunTraceOp::Upsert { key, verdict, reason, subject } => {
                kinds.push(0);
                keys.extend_from_slice(&key.to_le_bytes());
                verdicts.push(verdict.ordinal());
                reasons.extend_from_slice(&reason.to_le_bytes());
                subject_kinds.push(subject.ordinal());
                match subject {
                    ToolRunTraceSubject::Instance3d { mesh, position, rotation, scale } => {
                        meshes.extend_from_slice(&mesh.to_le_bytes());
                        for value in position.into_iter().chain(rotation).chain([scale]) {
                            floats.extend_from_slice(&value.to_le_bytes());
                        }
                    }
                    ToolRunTraceSubject::Placement2d { shape, position, rotation } => {
                        shapes.extend_from_slice(&shape.to_le_bytes());
                        for value in position.into_iter().chain([rotation]) {
                            floats.extend_from_slice(&value.to_le_bytes());
                        }
                    }
                    ToolRunTraceSubject::Entity { entity } => entities.extend_from_slice(&entity.to_le_bytes()),
                }
            }
            ToolRunTraceOp::Retire { key } => {
                kinds.push(1);
                keys.extend_from_slice(&key.to_le_bytes());
            }
            ToolRunTraceOp::Clear => kinds.push(2),
        }
    }
    let mut record = RecordValue::default();
    record.fields.insert(1, FieldValue::Record(identity_record(&page.identity)));
    record.fields.insert(2, FieldValue::UInt(u64::from(page.page)));
    for (id, column) in (3u16..).zip(columns) {
        if !column.is_empty() {
            record.fields.insert(id, FieldValue::Bytes64(column));
        }
    }
    Ok(record)
}

fn page_from_record(record: &RecordValue) -> Result<ToolRunTracePage, ToolRunCodecError> {
    let kinds = bytes_field(record, 3)?;
    if kinds.len() > TOOL_RUN_TRACE_PAGE_OPS_MAX {
        return Err(ToolRunCodecError::Limit("trace page ops"));
    }
    let keys = u64_column(bytes_field(record, 4)?, "keys")?;
    let verdicts = bytes_field(record, 5)?;
    let reasons = bytes_field(record, 6)?;
    let subject_kinds = bytes_field(record, 7)?;
    let meshes = u32_column(bytes_field(record, 8)?, "meshes")?;
    let shapes = u32_column(bytes_field(record, 9)?, "shapes")?;
    let entities = u64_column(bytes_field(record, 10)?, "entities")?;
    let floats = f32_column(bytes_field(record, 11)?)?;
    let upserts = kinds.iter().filter(|kind| **kind == 0).count();
    let keyed = kinds.iter().filter(|kind| **kind < 2).count();
    let instances = subject_kinds.iter().filter(|kind| **kind == 0).count();
    let placements = subject_kinds.iter().filter(|kind| **kind == 1).count();
    let entity_subjects = subject_kinds.iter().filter(|kind| **kind == 2).count();
    if kinds.iter().any(|kind| *kind > 2) || keys.len() != keyed || verdicts.len() != upserts || reasons.len() != 2 * upserts || subject_kinds.len() != upserts || instances + placements + entity_subjects != upserts || meshes.len() != instances || shapes.len() != placements || entities.len() != entity_subjects || floats.len() != 8 * instances + 3 * placements {
        return Err(ToolRunCodecError::Malformed("trace page columns"));
    }
    let (mut key_at, mut upsert_at, mut mesh_at, mut shape_at, mut entity_at, mut float_at) = (0, 0, 0, 0, 0, 0);
    let mut ops = Vec::with_capacity(kinds.len());
    for kind in kinds {
        match kind {
            0 => {
                let subject = match subject_kinds[upsert_at] {
                    0 => {
                        let f = &floats[float_at..float_at + 8];
                        float_at += 8;
                        mesh_at += 1;
                        ToolRunTraceSubject::Instance3d { mesh: meshes[mesh_at - 1], position: [f[0], f[1], f[2]], rotation: [f[3], f[4], f[5], f[6]], scale: f[7] }
                    }
                    1 => {
                        let f = &floats[float_at..float_at + 3];
                        float_at += 3;
                        shape_at += 1;
                        ToolRunTraceSubject::Placement2d { shape: shapes[shape_at - 1], position: [f[0], f[1]], rotation: f[2] }
                    }
                    _ => {
                        entity_at += 1;
                        ToolRunTraceSubject::Entity { entity: entities[entity_at - 1] }
                    }
                };
                let verdict = ToolRunVerdict::from_ordinal(verdicts[upsert_at]).ok_or(ToolRunCodecError::Malformed("verdict"))?;
                let reason = u16::from_le_bytes([reasons[2 * upsert_at], reasons[2 * upsert_at + 1]]);
                ops.push(ToolRunTraceOp::Upsert { key: keys[key_at], verdict, reason, subject });
                upsert_at += 1;
                key_at += 1;
            }
            1 => {
                ops.push(ToolRunTraceOp::Retire { key: keys[key_at] });
                key_at += 1;
            }
            _ => ops.push(ToolRunTraceOp::Clear),
        }
    }
    Ok(ToolRunTracePage { identity: identity_from_field(record, 1)?, page: narrow(uint(record, 2)?, "page")?, ops })
}

fn uint(record: &RecordValue, id: u16) -> Result<u64, ToolRunCodecError> {
    optional_uint(record, id)?.ok_or(ToolRunCodecError::Malformed("required unsigned field"))
}

fn optional_uint(record: &RecordValue, id: u16) -> Result<Option<u64>, ToolRunCodecError> {
    match record.get(id) {
        None | Some(FieldValue::Absent) => Ok(None),
        Some(FieldValue::UInt(value)) => Ok(Some(*value)),
        Some(_) => Err(ToolRunCodecError::Malformed("unsigned field")),
    }
}

fn boolean(record: &RecordValue, id: u16) -> Result<bool, ToolRunCodecError> {
    match record.get(id) {
        Some(FieldValue::Bool(value)) => Ok(*value),
        _ => Err(ToolRunCodecError::Malformed("boolean field")),
    }
}

fn bytes_field(record: &RecordValue, id: u16) -> Result<&[u8], ToolRunCodecError> {
    match record.get(id) {
        None | Some(FieldValue::Absent) => Ok(&[]),
        Some(FieldValue::Bytes64(bytes)) => Ok(bytes),
        Some(_) => Err(ToolRunCodecError::Malformed("bytes field")),
    }
}

fn list(record: &RecordValue, id: u16) -> Result<&[FieldValue], ToolRunCodecError> {
    match record.get(id) {
        None | Some(FieldValue::Absent) => Ok(&[]),
        Some(FieldValue::List(items)) => Ok(items),
        Some(_) => Err(ToolRunCodecError::Malformed("list field")),
    }
}

fn item_bytes(item: &FieldValue) -> Result<&[u8], ToolRunCodecError> {
    match item {
        FieldValue::Bytes64(bytes) => Ok(bytes),
        _ => Err(ToolRunCodecError::Malformed("bytes item")),
    }
}

fn item_record(item: &FieldValue) -> Result<&RecordValue, ToolRunCodecError> {
    match item {
        FieldValue::Record(record) => Ok(record),
        _ => Err(ToolRunCodecError::Malformed("record item")),
    }
}

fn narrow<T: TryFrom<u64>>(value: u64, what: &'static str) -> Result<T, ToolRunCodecError> {
    T::try_from(value).map_err(|_| ToolRunCodecError::Malformed(what))
}

fn u64_column(bytes: &[u8], what: &'static str) -> Result<Vec<u64>, ToolRunCodecError> {
    let (chunks, rest) = bytes.as_chunks::<8>();
    if !rest.is_empty() {
        return Err(ToolRunCodecError::Malformed(what));
    }
    Ok(chunks.iter().map(|chunk| u64::from_le_bytes(*chunk)).collect())
}

fn u32_column(bytes: &[u8], what: &'static str) -> Result<Vec<u32>, ToolRunCodecError> {
    let (chunks, rest) = bytes.as_chunks::<4>();
    if !rest.is_empty() {
        return Err(ToolRunCodecError::Malformed(what));
    }
    Ok(chunks.iter().map(|chunk| u32::from_le_bytes(*chunk)).collect())
}

fn f32_column(bytes: &[u8]) -> Result<Vec<f32>, ToolRunCodecError> {
    let (chunks, rest) = bytes.as_chunks::<4>();
    if !rest.is_empty() {
        return Err(ToolRunCodecError::Malformed("floats"));
    }
    Ok(chunks.iter().map(|chunk| f32::from_le_bytes(*chunk)).collect())
}
//#endregion 🔖️Codec

//#region 🔖️Definition
/// 🧵️ Id of a job kind registered in the artifact tool factory registry.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct JobKindId(pub String);

impl JobKindId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// 🔄️ How a mutating run reacts to head changes (§2.7.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ToolRunRebasePolicy {
    Revalidate,
    Restart,
    Freeze,
}

/// 🎚️ How a run reacts to settings changes (§3.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ToolRunReconfigurePolicy {
    Resume,
    Restart,
}

/// 🫥️ Trace subject kind a run emits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ToolRunTraceKind {
    Instance3d,
    Placement2d,
    Entity,
    None,
}

/// 🪜️ One algorithm stage.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunStageDefinition {
    pub id: String,
    pub label: LocalizedLabel,
}

/// 🔟️ One progress counter.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunCounterDefinition {
    pub id: String,
    pub label: LocalizedLabel,
}

/// 🗯️ One reason code with its verdict and EN/DE template (`{0}`..`{3}` take step arguments).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunReasonDefinition {
    pub code: u16,
    pub id: String,
    pub verdict: ToolRunVerdict,
    pub template: LocalizedLabel,
}

/// 🎚️ The settings a run's jobs read (§3.3): RFC 6901 JSON Pointers into the app config document and, per
/// window kind id, into that kind's window config documents. `settingsChanged` fires only when a value
/// behind one of them changes; an empty declaration reads no settings, so no settings publication ever
/// reconfigures the run.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunSettingsReads {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub config: Vec<String>,
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    #[value(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub window_config: std::collections::BTreeMap<String, Vec<String>>,
}

impl ToolRunSettingsReads {
    pub fn is_empty(&self) -> bool {
        self.config.is_empty() && self.window_config.values().all(Vec::is_empty)
    }

    /// 🔎️ Every declared pointer, config first, then window config in window kind order.
    pub fn pointers(&self) -> impl Iterator<Item = &str> {
        self.config.iter().chain(self.window_config.values().flatten()).map(String::as_str)
    }
}

/// 🧭️ The reference tokens of an RFC 6901 JSON Pointer (`~1` → `/`, `~0` → `~`); `None` for a malformed one.
/// The empty pointer names the whole document.
pub fn tool_run_pointer_tokens(pointer: &str) -> Option<Vec<String>> {
    if pointer.is_empty() {
        return Some(Vec::new());
    }
    pointer.strip_prefix('/')?.split('/').map(|token| (!token.replace("~0", "").replace("~1", "").contains('~')).then(|| token.replace("~1", "/").replace("~0", "~"))).collect()
}

/// 📍️ The value an RFC 6901 JSON Pointer names inside `document`; `None` when it is malformed or names nothing.
pub fn tool_run_pointer_value<'a>(document: &'a dsl::DslValue, pointer: &str) -> Option<&'a dsl::DslValue> {
    tool_run_pointer_tokens(pointer)?.iter().try_fold(document, |value, token| match value {
        dsl::DslValue::Object(fields) => fields.iter().find(|(key, _)| key == token).map(|(_, value)| value),
        dsl::DslValue::Array(items) => (token.bytes().all(|byte| byte.is_ascii_digit()) && (token == "0" || !token.starts_with('0'))).then(|| token.parse::<usize>().ok()).flatten().and_then(|index| items.get(index)),
        _ => None,
    })
}

/// 📜️ Static run declaration attached to tool and utility definitions (§2.4).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ToolRunDefinition {
    pub mutating: bool,
    pub rebase: ToolRunRebasePolicy,
    pub reconfigure: ToolRunReconfigurePolicy,
    pub unit: LocalizedLabel,
    pub stages: Vec<ToolRunStageDefinition>,
    pub counters: Vec<ToolRunCounterDefinition>,
    pub reasons: Vec<ToolRunReasonDefinition>,
    pub trace: ToolRunTraceKind,
    pub run_job: JobKindId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub revalidate_job: Option<JobKindId>,
    #[serde(default, skip_serializing_if = "ToolRunSettingsReads::is_empty")]
    #[value(default, skip_serializing_if = "ToolRunSettingsReads::is_empty")]
    pub settings: ToolRunSettingsReads,    /// 🪟️ Window kind ids whose bodies render this run's state (`ArtifactView::tool_run()`): every tick refreshes them
    /// next to the ToolRun panel, and no other window.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub windows: Vec<String>,
}

/// 🚫️ Why a `ToolRunDefinition` violates the contract.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ToolRunDefinitionError {
    NoStages,
    TooManyStages,
    TooManyCounters,
    DuplicateStageId(String),
    DuplicateCounterId(String),
    DuplicateReasonId(String),
    DuplicateReasonCode(u16),
    ReservedReasonCode(u16),
    InvalidSettingsPointer(String),
}

impl std::fmt::Display for ToolRunDefinitionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoStages => formatter.write_str("toolRun.definition.noStages"),
            Self::TooManyStages => formatter.write_str("toolRun.definition.tooManyStages"),
            Self::TooManyCounters => formatter.write_str("toolRun.definition.tooManyCounters"),
            Self::DuplicateStageId(id) => write!(formatter, "toolRun.definition.duplicateStageId: {id}"),
            Self::DuplicateCounterId(id) => write!(formatter, "toolRun.definition.duplicateCounterId: {id}"),
            Self::DuplicateReasonId(id) => write!(formatter, "toolRun.definition.duplicateReasonId: {id}"),
            Self::DuplicateReasonCode(code) => write!(formatter, "toolRun.definition.duplicateReasonCode: {code}"),
            Self::ReservedReasonCode(code) => write!(formatter, "toolRun.definition.reservedReasonCode: {code}"),
            Self::InvalidSettingsPointer(pointer) => write!(formatter, "toolRun.definition.invalidSettingsPointer: {pointer}"),
        }
    }
}

impl std::error::Error for ToolRunDefinitionError {}

impl ToolRunDefinition {
    /// ✅️ Checks stage, counter and reason tables against the contract limits.
    pub fn validate(&self) -> Result<(), ToolRunDefinitionError> {
        if self.stages.is_empty() {
            return Err(ToolRunDefinitionError::NoStages);
        }
        if self.stages.len() > usize::from(u16::MAX) + 1 {
            return Err(ToolRunDefinitionError::TooManyStages);
        }
        if self.counters.len() > TOOL_RUN_COUNTERS_MAX {
            return Err(ToolRunDefinitionError::TooManyCounters);
        }
        let mut stage_ids = std::collections::HashSet::new();
        if let Some(stage) = self.stages.iter().find(|stage| !stage_ids.insert(stage.id.as_str())) {
            return Err(ToolRunDefinitionError::DuplicateStageId(stage.id.clone()));
        }
        let mut counter_ids = std::collections::HashSet::new();
        if let Some(counter) = self.counters.iter().find(|counter| !counter_ids.insert(counter.id.as_str())) {
            return Err(ToolRunDefinitionError::DuplicateCounterId(counter.id.clone()));
        }
        let mut reason_ids = std::collections::HashSet::new();
        let mut reason_codes = std::collections::HashSet::new();
        for reason in &self.reasons {
            if reason.code >= TOOL_RUN_RESERVED_REASON_FLOOR {
                return Err(ToolRunDefinitionError::ReservedReasonCode(reason.code));
            }
            if !reason_codes.insert(reason.code) {
                return Err(ToolRunDefinitionError::DuplicateReasonCode(reason.code));
            }
            if !reason_ids.insert(reason.id.as_str()) {
                return Err(ToolRunDefinitionError::DuplicateReasonId(reason.id.clone()));
            }
        }
        if let Some(pointer) = self.settings.pointers().find(|pointer| tool_run_pointer_tokens(pointer).is_none()) {
            return Err(ToolRunDefinitionError::InvalidSettingsPointer(pointer.to_string()));
        }
        Ok(())
    }

    pub fn stage(&self, index: u16) -> Option<&ToolRunStageDefinition> {
        self.stages.get(usize::from(index))
    }

    pub fn counter(&self, index: u16) -> Option<&ToolRunCounterDefinition> {
        self.counters.get(usize::from(index))
    }

    pub fn reason(&self, code: u16) -> Option<&ToolRunReasonDefinition> {
        self.reasons.iter().find(|reason| reason.code == code)
    }
}
//#endregion 🔖️Definition

//#region 🔖️Panel
/// 🪧️ Key of the framework ToolRun panel root; each run is one child group [`tool_run_panel_group_id`].
pub const TOOL_RUN_PANEL_ID: &str = "framework.toolRun";

/// 🪧️ The key of run `run`'s group in the ToolRun panel; every id inside the group is scoped under it.
pub fn tool_run_panel_group_id(run: u64) -> String {
    format!("{TOOL_RUN_PANEL_ID}.{run}")
}

/// 🔎️ The run a ToolRun panel group key names; `None` for any other key (a key inside a group included).
pub fn tool_run_panel_group_run(key: &str) -> Option<u64> {
    let digits = key.strip_prefix(TOOL_RUN_PANEL_ID)?.strip_prefix('.')?;
    (!digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit()) && (digits == "0" || !digits.starts_with('0'))).then(|| digits.parse().ok()).flatten()
}

/// 📣️ The runs a rendered ToolRun panel holds and those of them not in `previous` — the runs whose start makes a
/// host reveal the panel.
pub fn tool_run_panel_new_runs<'a>(previous: &std::collections::BTreeSet<u64>, keys: impl IntoIterator<Item = &'a str>) -> (std::collections::BTreeSet<u64>, Vec<u64>) {
    let current: std::collections::BTreeSet<u64> = keys.into_iter().filter_map(tool_run_panel_group_run).collect();
    let new = current.difference(previous).copied().collect();
    (current, new)
}
//#endregion 🔖️Panel

//#region 🔖️Actions
/// ▶️ Framework-reserved action id: start a run.
pub const TOOL_RUN_START_ACTION_ID: &str = "toolRunStart";
/// ⏸️ Framework-reserved action id: pause a running run.
pub const TOOL_RUN_PAUSE_ACTION_ID: &str = "toolRunPause";
/// 🔂️ Framework-reserved action id: resume a paused run.
pub const TOOL_RUN_RESUME_ACTION_ID: &str = "toolRunResume";
/// ⏭️ Framework-reserved action id: drive exactly one algorithm unit.
pub const TOOL_RUN_STEP_ACTION_ID: &str = "toolRunStep";
/// ⏹️ Framework-reserved action id: abort without touching the store.
pub const TOOL_RUN_ABORT_ACTION_ID: &str = "toolRunAbort";
/// 🎉️ Framework-reserved action id: publish the complete run as one edit.
pub const TOOL_RUN_FINALIZE_ACTION_ID: &str = "toolRunFinalize";
/// ✖️ Framework-reserved action id: clear a terminal run's trace and panel.
pub const TOOL_RUN_DISMISS_ACTION_ID: &str = "toolRunDismiss";
/// 📚️ Every framework-reserved tool run action id.
pub const TOOL_RUN_ACTION_IDS: [&str; 7] = [TOOL_RUN_START_ACTION_ID, TOOL_RUN_PAUSE_ACTION_ID, TOOL_RUN_RESUME_ACTION_ID, TOOL_RUN_STEP_ACTION_ID, TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_FINALIZE_ACTION_ID, TOOL_RUN_DISMISS_ACTION_ID];

/// 🎹️ Chord of `toolRunStart`.
pub const TOOL_RUN_START_CHORD: &str = "mod+enter";
/// 🔀️ Toggle chord shared by `toolRunPause` and `toolRunResume`.
pub const TOOL_RUN_PAUSE_RESUME_CHORD: &str = "mod+alt+enter";
/// 👣️ Chord of `toolRunStep`.
pub const TOOL_RUN_STEP_CHORD: &str = "mod+alt+arrowright";
/// ⛔️ Chord of `toolRunAbort`.
pub const TOOL_RUN_ABORT_CHORD: &str = "mod+.";
/// 🥅️ Chord of `toolRunFinalize`.
pub const TOOL_RUN_FINALIZE_CHORD: &str = "mod+shift+enter";
/// 🚪️ Chord of `toolRunDismiss` (panel focus only).
pub const TOOL_RUN_DISMISS_CHORD: &str = "escape";

/// 🆔️ Action argument names.
pub const TOOL_RUN_ARG_TOOL_ID: &str = "toolId";
pub const TOOL_RUN_ARG_WINDOW_ID: &str = "windowId";
pub const TOOL_RUN_ARG_RUN_ID: &str = "runId";
pub const TOOL_RUN_ARG_GENERATION: &str = "generation";

/// 📌️ One declared action argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ToolRunActionArg {
    pub name: &'static str,
    pub required: bool,
}

/// 🕹️ The seven generic tool run actions (§2.5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRunAction {
    Start,
    Pause,
    Resume,
    Step,
    Abort,
    Finalize,
    Dismiss,
}

impl ToolRunAction {
    pub const ALL: [Self; 7] = [Self::Start, Self::Pause, Self::Resume, Self::Step, Self::Abort, Self::Finalize, Self::Dismiss];

    pub fn id(self) -> &'static str {
        TOOL_RUN_ACTION_IDS[self as usize]
    }

    pub fn from_id(id: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|action| action.id() == id)
    }

    pub fn chord(self) -> &'static str {
        match self {
            Self::Start => TOOL_RUN_START_CHORD,
            Self::Pause | Self::Resume => TOOL_RUN_PAUSE_RESUME_CHORD,
            Self::Step => TOOL_RUN_STEP_CHORD,
            Self::Abort => TOOL_RUN_ABORT_CHORD,
            Self::Finalize => TOOL_RUN_FINALIZE_CHORD,
            Self::Dismiss => TOOL_RUN_DISMISS_CHORD,
        }
    }

    pub fn label(self) -> ToolRunLabel {
        match self {
            Self::Start => ToolRunLabel::ActionStart,
            Self::Pause => ToolRunLabel::ActionPause,
            Self::Resume => ToolRunLabel::ActionResume,
            Self::Step => ToolRunLabel::ActionStep,
            Self::Abort => ToolRunLabel::ActionAbort,
            Self::Finalize => ToolRunLabel::ActionFinalize,
            Self::Dismiss => ToolRunLabel::ActionDismiss,
        }
    }

    pub fn args(self) -> &'static [ToolRunActionArg] {
        const START: [ToolRunActionArg; 2] = [ToolRunActionArg { name: TOOL_RUN_ARG_TOOL_ID, required: true }, ToolRunActionArg { name: TOOL_RUN_ARG_WINDOW_ID, required: false }];
        const TARGETED: [ToolRunActionArg; 2] = [ToolRunActionArg { name: TOOL_RUN_ARG_RUN_ID, required: true }, ToolRunActionArg { name: TOOL_RUN_ARG_GENERATION, required: true }];
        const DISMISS: [ToolRunActionArg; 1] = [ToolRunActionArg { name: TOOL_RUN_ARG_RUN_ID, required: true }];
        match self {
            Self::Start => &START,
            Self::Dismiss => &DISMISS,
            _ => &TARGETED,
        }
    }

    /// 🟢️ Whether the action is enabled for a slot in `state` (`None` = no run).
    pub fn is_legal_in(self, state: Option<ToolRunState>) -> bool {
        use ToolRunState as S;
        match (self, state) {
            (Self::Start, None) => true,
            (Self::Start | Self::Dismiss, Some(state)) => state.is_terminal(),
            (Self::Pause, Some(S::Running)) | (Self::Resume | Self::Step, Some(S::Paused)) | (Self::Finalize, Some(S::Running | S::Paused | S::Complete)) => true,
            (Self::Abort, Some(S::Starting | S::Running | S::Paused | S::Complete | S::Finalizing)) => true,
            _ => false,
        }
    }
}
//#endregion 🔖️Actions

//#region 🔖️Labels
/// 💬️ Framework-owned EN/DE text of the tool run panel (§2.5), no default locale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ToolRunLabel {
    StateStarting,
    StateRunning,
    StatePaused,
    StateComplete,
    StateFinalizing,
    StateFinalized,
    StateAborting,
    StateAborted,
    StateFaulted,
    ActionStart,
    ActionPause,
    ActionResume,
    ActionStep,
    ActionAbort,
    ActionFinalize,
    ActionDismiss,
    FinalizeDisabled,
    ReadyToStart,
    RebasingStep,
    ConflictStep,
    TraceTruncatedStep,
    ProvisionalCapStep,
    ProgressValueText,
}

impl ToolRunLabel {
    pub const ALL: [Self; 23] = [
        Self::StateStarting,
        Self::StateRunning,
        Self::StatePaused,
        Self::StateComplete,
        Self::StateFinalizing,
        Self::StateFinalized,
        Self::StateAborting,
        Self::StateAborted,
        Self::StateFaulted,
        Self::ActionStart,
        Self::ActionPause,
        Self::ActionResume,
        Self::ActionStep,
        Self::ActionAbort,
        Self::ActionFinalize,
        Self::ActionDismiss,
        Self::FinalizeDisabled,
        Self::ReadyToStart,
        Self::RebasingStep,
        Self::ConflictStep,
        Self::TraceTruncatedStep,
        Self::ProvisionalCapStep,
        Self::ProgressValueText,
    ];

    /// 🔑️ `(key, en, de)` row of this label.
    fn row(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::StateStarting => ("stateStarting", "Starting", "Wird gestartet"),
            Self::StateRunning => ("stateRunning", "Running", "Läuft"),
            Self::StatePaused => ("statePaused", "Paused", "Pausiert"),
            Self::StateComplete => ("stateComplete", "Complete, ready to finalize", "Fertig, bereit zum Abschließen"),
            Self::StateFinalizing => ("stateFinalizing", "Finalizing", "Wird abgeschlossen"),
            Self::StateFinalized => ("stateFinalized", "Finalized", "Abgeschlossen"),
            Self::StateAborting => ("stateAborting", "Aborting", "Wird abgebrochen"),
            Self::StateAborted => ("stateAborted", "Aborted, nothing was changed", "Abgebrochen, nichts wurde geändert"),
            Self::StateFaulted => ("stateFaulted", "Failed, nothing was changed", "Fehlgeschlagen, nichts wurde geändert"),
            Self::ActionStart => ("actionStart", "Start", "Starten"),
            Self::ActionPause => ("actionPause", "Pause", "Pausieren"),
            Self::ActionResume => ("actionResume", "Resume", "Fortsetzen"),
            Self::ActionStep => ("actionStep", "Step", "Einzelschritt"),
            Self::ActionAbort => ("actionAbort", "Abort", "Abbrechen"),
            Self::ActionFinalize => ("actionFinalize", "Finalize", "Abschließen"),
            Self::ActionDismiss => ("actionDismiss", "Dismiss", "Schließen"),
            Self::FinalizeDisabled => ("finalizeDisabled", "Available once the run has started", "Verfügbar, sobald der Lauf gestartet ist"),
            Self::ReadyToStart => ("readyToStart", "Ready to start", "Bereit zum Starten"),
            Self::RebasingStep => ("rebasingStep", "Artifact changed, re-applying provisional result", "Artefakt geändert, vorläufiges Ergebnis wird neu angewendet"),
            Self::ConflictStep => ("conflictStep", "{0} provisional changes conflict with the current artifact", "{0} vorläufige Änderungen stehen im Konflikt mit dem aktuellen Artefakt"),
            Self::TraceTruncatedStep => ("traceTruncatedStep", "Oldest {0} rejected attempts are no longer shown", "Die ältesten {0} verworfenen Versuche werden nicht mehr angezeigt"),
            Self::ProvisionalCapStep => ("provisionalCapStep", "Provisional change limit of {0} reached, run completed", "Grenze von {0} vorläufigen Änderungen erreicht, Lauf abgeschlossen"),
            Self::ProgressValueText => ("progressValueText", "{stage} ({i}/{n}): {completed} of {total} {unit} ({pct} %)", "{stage} ({i}/{n}): {completed} von {total} {unit} ({pct} %)"),
        }
    }

    pub fn key(self) -> &'static str {
        self.row().0
    }

    pub fn parse(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|label| label.key() == key)
    }

    pub fn text(self, locale: Locale) -> &'static str {
        let (_, en, de) = self.row();
        match locale {
            Locale::En => en,
            Locale::De => de,
        }
    }

    /// 🌐️ Terminology-invariant `LocalizedLabel::native(en, de)`.
    pub fn localized(self) -> LocalizedLabel {
        let (_, en, de) = self.row();
        LocalizedLabel::native(en, de)
    }

    /// 🚨️ Label of a framework-reserved reason code.
    pub fn for_reason(code: u16) -> Option<Self> {
        match code {
            TOOL_RUN_REASON_REBASING => Some(Self::RebasingStep),
            TOOL_RUN_REASON_CONFLICT => Some(Self::ConflictStep),
            TOOL_RUN_REASON_TRACE_TRUNCATED => Some(Self::TraceTruncatedStep),
            TOOL_RUN_REASON_PROVISIONAL_CAP => Some(Self::ProvisionalCapStep),
            _ => None,
        }
    }
}

/// 🔣️ Single-pass `{name}` substitution; unknown or unterminated placeholders stay literal.
pub fn tool_run_format(template: &str, value: impl Fn(&str) -> Option<String>) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        match after.find(['{', '}']).filter(|close| after.as_bytes()[*close] == b'}').and_then(|close| value(&after[..close]).map(|text| (close, text))) {
            Some((close, text)) => {
                out.push_str(&text);
                rest = &after[close + 1..];
            }
            None => {
                out.push('{');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    out
}
//#endregion 🔖️Labels

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
