//! ⏪️ Domain-neutral time-travel session: the pure, target-neutral reducer of one ephemeral,
//! local-only history edit (session identity, generation and base stamp, stages, accepted drafts
//! in history order, the pending draft with its return stage, the replay report, progress and
//! fault), its events, effects, refusals and EN/DE labels.
//!
//! Pure: no async, no store, no locale type. Schema of record: `🧬️schema/🔣️.json`; law fixture:
//! `🧫️fixtures/🧫️lifecycle-law/🔣️.json`; contract:
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §4.
//!
//! Driver contract (the host owning a [`TimeTravelSession`]):
//! - [`TimeTravelEvent::Begin`] carries the target's applied `position` resolved at the session base.
//! - Every change of the store's content revision is forwarded as [`TimeTravelEvent::BaseMoved`], in every
//!   stage (also `Inactive`), with the re-resolved positions of every session target; an unchanged base is
//!   `timeTravel.stale`. The store's local generation is no part of the base.
//! - [`TimeTravelEvent::BeginWithdrawn`] is a history row's Withdraw: `Begin` whose pending draft starts as
//!   `Withdrawn`, legal exactly where `Begin` is; the host sends [`TimeTravelEvent::Withdraw`] instead for the
//!   mutation already being edited.
//! - [`TimeTravelEvent::Restore`] is a history row's Restore: it takes the accepted draft of one mutation back while
//!   `Reviewing` and replays the rest; taking the only one back leaves time travel with zero trace.
//! - Every event except `Begin`, `BeginWithdrawn`, `BaseMoved` and `Exit` carries the session `generation` the
//!   host last observed; any other generation is `timeTravel.stale` (a silent no-op).
//! - [`TimeTravelSession::review`] classifies `Reviewing`: nothing accepted is "no changes" (the
//!   preview is the committed head); accepted drafts without a report (a cancelled or faulted replay)
//!   "need a replay", which [`TimeTravelEvent::Rerun`] starts; otherwise the report blocks or allows
//!   finalizing.
//! - Fault codes are non-empty, whitespace-free and at most [`TIME_TRAVEL_TEXT_MAX_BYTES`]; alternative
//!   names are not blank and at most [`TIME_TRAVEL_TEXT_MAX_BYTES`]. A malformed event is `timeTravel.illegal`.

use protocol::{InputReplacement, MutationId, ReplayReport, SupersededInput};
use semio_framework_value_derive::{FromValue, ToValue};
use std::cmp::Ordering;

//#region 🔖️Limits
/// 📏️ UTF-8 byte ceiling of a fault code and of an alternative name.
pub const TIME_TRAVEL_TEXT_MAX_BYTES: usize = 256;

/// 🔒️ Refusal code of an artifact-lane app command while a session is not `Inactive` (issued by the host).
pub const TIME_TRAVEL_FROZEN_CODE: &str = "timeTravel.frozen";

/// 🧯️ Fault a cancelled replay leaves behind, so hosts can label it and offer [`TimeTravelEvent::Rerun`].
pub const TIME_TRAVEL_CANCELLED_CODE: &str = "timeTravel.cancelled";

/// 🛑️ Host refusal: a mutating tool run, an agent transaction or a session on another store holds the instance.
pub const TIME_TRAVEL_BUSY_CODE: &str = "timeTravel.busy";

/// 👻️ Host refusal: the named mutation is not (or no longer) applied in the edited store.
pub const TIME_TRAVEL_UNKNOWN_MUTATION_CODE: &str = "timeTravel.unknown-mutation";

/// 🔐️ Host refusal: the mutation declares no input schema or emits foreign steps.
pub const TIME_TRAVEL_NOT_EDITABLE_CODE: &str = "timeTravel.not-editable";

/// 🔍️ Host refusal: the input pointer addresses no input of the edited mutation.
pub const TIME_TRAVEL_UNKNOWN_INPUT_CODE: &str = "timeTravel.unknown-input";

/// ❎️ Host refusal: the value does not take the input's shape or fails its payload schema; the draft is kept.
pub const TIME_TRAVEL_INVALID_INPUT_CODE: &str = "timeTravel.invalid-input";

/// 🫥️ Host refusal: the input's selection domain holds nothing at its granularity.
pub const TIME_TRAVEL_NO_SELECTION_CODE: &str = "timeTravel.no-selection";

/// 🖊️ Host refusal: a new alternative needs a name and no locale supplied the default.
pub const TIME_TRAVEL_NAME_REQUIRED_CODE: &str = "timeTravel.name-required";

/// 🔤️ Host refusal: the alternative name is blank or longer than [`TIME_TRAVEL_TEXT_MAX_BYTES`].
pub const TIME_TRAVEL_NAME_INVALID_CODE: &str = "timeTravel.name-invalid";

/// 🧬️ Host refusal: the payload schema compiles no validator or describes no inputs, so no draft is admitted.
pub const TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE: &str = "timeTravel.schema-unavailable";

/// 💥️ Driver fault: the store refused or broke the Report replay.
pub const TIME_TRAVEL_REPLAY_FAULTED_CODE: &str = "timeTravel.replay-faulted";

/// 🧨️ Driver fault: the finalize commit failed for a reason other than a stale base or a blocking report.
pub const TIME_TRAVEL_COMMIT_FAILED_CODE: &str = "timeTravel.commit-failed";

/// 🧩️ Driver fault: the composed member store the session edits was closed mid-session.
pub const TIME_TRAVEL_MEMBER_GONE_CODE: &str = "timeTravel.member-gone";

/// 🪨️ Host refusal: the store's supersede law does not admit withdrawing the mutation.
pub const TIME_TRAVEL_NOT_WITHDRAWABLE_CODE: &str = "timeTravel.not-withdrawable";

/// 🥅️ Host refusal: a draft verb arrived while no draft editor is open on the session's mutation.
pub const TIME_TRAVEL_EDITOR_CLOSED_CODE: &str = "timeTravel.editor-closed";

/// 🔣️ A fault code is non-empty, whitespace-free and at most [`TIME_TRAVEL_TEXT_MAX_BYTES`].
pub fn is_time_travel_fault_code(code: &str) -> bool {
    !code.is_empty() && code.len() <= TIME_TRAVEL_TEXT_MAX_BYTES && !code.chars().any(char::is_whitespace)
}

/// 🪪️ An alternative name is not blank (Unicode `White_Space`) and at most [`TIME_TRAVEL_TEXT_MAX_BYTES`].
pub fn is_time_travel_alternative_name(name: &str) -> bool {
    name.len() <= TIME_TRAVEL_TEXT_MAX_BYTES && name.chars().any(|character| !character.is_whitespace())
}
//#endregion 🔖️Limits

//#region 🔖️Identity
/// 🧭️ The store state a session last observed: the 32-byte content revision of its history. The store's local
/// generation is no part of it — a document port attaching or detaching moves that generation without changing an
/// event, and never moves the base.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct TimeTravelBase {
    pub content_revision: [u8; 32],
}

/// 🎯️ One mutation the session addresses: its replica-independent id and its applied position at
/// the session base (the host re-resolves positions on every [`TimeTravelEvent::BaseMoved`]).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TimeTravelTarget {
    pub mutation: MutationId,
    pub position: u32,
}

impl TimeTravelTarget {
    /// 🧮️ History order: applied position, then mutation id (UTF-8 byte order).
    pub fn history_order(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position).then_with(|| self.mutation.cmp(&other.mutation))
    }
}
//#endregion 🔖️Identity

//#region 🔖️Session
/// 🚦️ Stage of the per-instance session; `Inactive` means no history edit is open.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum TimeTravelStage {
    #[default]
    Inactive,
    Editing,
    Replaying,
    Reviewing,
    Choosing,
    Finalizing,
}

impl TimeTravelStage {
    pub const ALL: [Self; 6] = [Self::Inactive, Self::Editing, Self::Replaying, Self::Reviewing, Self::Choosing, Self::Finalizing];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Inactive => "inactive",
            Self::Editing => "editing",
            Self::Replaying => "replaying",
            Self::Reviewing => "reviewing",
            Self::Choosing => "choosing",
            Self::Finalizing => "finalizing",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|stage| stage.as_str() == text)
    }

    /// 🗣️ Framework status label of this stage.
    pub fn label(self) -> TimeTravelLabel {
        match self {
            Self::Inactive => TimeTravelLabel::StageInactive,
            Self::Editing => TimeTravelLabel::StageEditing,
            Self::Replaying => TimeTravelLabel::StageReplaying,
            Self::Reviewing => TimeTravelLabel::StageReviewing,
            Self::Choosing => TimeTravelLabel::StageChoosing,
            Self::Finalizing => TimeTravelLabel::StageFinalizing,
        }
    }
}

/// 📝️ One accepted draft: the target and its replacement input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimeTravelDraft {
    pub target: TimeTravelTarget,
    pub replacement: InputReplacement,
}

/// ✏️ The draft being edited: the target's effective input before the session (`original`), the
/// current draft, and the stage `Discard` returns to (`Inactive` or `Reviewing`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TimeTravelPending {
    pub target: TimeTravelTarget,
    pub original: InputReplacement,
    pub replacement: InputReplacement,
    pub return_stage: TimeTravelStage,
}

/// 📶️ Replay progress in replayed mutations.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TimeTravelProgress {
    pub done: u32,
    pub total: u32,
}

/// 🔭️ What a `Reviewing` session shows: nothing accepted, drafts awaiting a replay, or a report that
/// blocks or allows finalizing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(crate = "::protocol::value", rename_all = "camelCase")]
pub enum TimeTravelReview {
    NoChanges,
    NeedsReplay,
    Blocked,
    Ready,
}

impl TimeTravelReview {
    pub const ALL: [Self; 4] = [Self::NoChanges, Self::NeedsReplay, Self::Blocked, Self::Ready];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::NoChanges => "noChanges",
            Self::NeedsReplay => "needsReplay",
            Self::Blocked => "blocked",
            Self::Ready => "ready",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|review| review.as_str() == text)
    }

    /// 📛️ Status label of this review.
    pub fn label(self) -> TimeTravelLabel {
        match self {
            Self::NoChanges => TimeTravelLabel::NoChanges,
            Self::NeedsReplay => TimeTravelLabel::NeedsReplay,
            Self::Blocked => TimeTravelLabel::ReportBlocking,
            Self::Ready => TimeTravelLabel::ReadyToFinalize,
        }
    }
}

/// 🕰️ Ephemeral local-only state of one app instance's history edit; `id` names the latest
/// session, `generation` only grows and fences every driver result and user action.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TimeTravelSession {
    pub id: u64,
    pub generation: u32,
    pub base: TimeTravelBase,
    pub stage: TimeTravelStage,
    pub accepted: Vec<TimeTravelDraft>,
    pub pending: Option<TimeTravelPending>,
    pub report: Option<ReplayReport>,
    pub progress: Option<TimeTravelProgress>,
    pub fault: Option<String>,
}
//#endregion 🔖️Session

//#region 🔖️Events
/// 🏁️ How a finalize publishes the accepted drafts.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TimeTravelChoice {
    Overwrite,
    Alternative { name: String },
}

/// 📨️ User actions and driver observations; see the module docs for which carry a generation.
#[derive(Clone, Debug, PartialEq)]
pub enum TimeTravelEvent {
    Begin { target: TimeTravelTarget, original: InputReplacement },
    BeginWithdrawn { target: TimeTravelTarget, original: InputReplacement },
    Draft { generation: u32, replacement: InputReplacement },
    Withdraw { generation: u32 },
    Accept { generation: u32 },
    Discard { generation: u32 },
    Restore { generation: u32, target: MutationId },
    ReplayProgressed { generation: u32, done: u32, total: u32 },
    ReplayCompleted { generation: u32, report: ReplayReport },
    ReplayCancelled { generation: u32 },
    ReplayFaulted { generation: u32, code: String },
    Rerun { generation: u32 },
    RequestFinalize { generation: u32 },
    Choose { generation: u32, choice: TimeTravelChoice },
    Back { generation: u32 },
    Finalized { generation: u32 },
    FinalizeFaulted { generation: u32, code: String },
    BaseMoved { base: TimeTravelBase, positions: Vec<TimeTravelTarget> },
    Exit,
}

impl TimeTravelEvent {
    /// 🗝️ Lifecycle-law matrix column of this event.
    pub fn key(&self) -> TimeTravelEventKey {
        use TimeTravelEventKey as K;
        match self {
            Self::Begin { .. } => K::Begin,
            Self::BeginWithdrawn { .. } => K::BeginWithdrawn,
            Self::Draft { .. } => K::Draft,
            Self::Withdraw { .. } => K::Withdraw,
            Self::Accept { .. } => K::Accept,
            Self::Discard { .. } => K::Discard,
            Self::Restore { .. } => K::Restore,
            Self::ReplayProgressed { .. } => K::ReplayProgressed,
            Self::ReplayCompleted { .. } => K::ReplayCompleted,
            Self::ReplayCancelled { .. } => K::ReplayCancelled,
            Self::ReplayFaulted { .. } => K::ReplayFaulted,
            Self::Rerun { .. } => K::Rerun,
            Self::RequestFinalize { .. } => K::RequestFinalize,
            Self::Choose { choice: TimeTravelChoice::Overwrite, .. } => K::ChooseOverwrite,
            Self::Choose { choice: TimeTravelChoice::Alternative { .. }, .. } => K::ChooseAlternative,
            Self::Back { .. } => K::Back,
            Self::Finalized { .. } => K::Finalized,
            Self::FinalizeFaulted { .. } => K::FinalizeFaulted,
            Self::BaseMoved { .. } => K::BaseMoved,
            Self::Exit => K::Exit,
        }
    }

    /// 🧿️ The session generation the event is addressed to; `None` for `Begin`, `BeginWithdrawn`, `BaseMoved`, `Exit`.
    pub fn generation(&self) -> Option<u32> {
        match self {
            Self::Begin { .. } | Self::BeginWithdrawn { .. } | Self::BaseMoved { .. } | Self::Exit => None,
            Self::Draft { generation, .. }
            | Self::Withdraw { generation }
            | Self::Accept { generation }
            | Self::Discard { generation }
            | Self::Restore { generation, .. }
            | Self::ReplayProgressed { generation, .. }
            | Self::ReplayCompleted { generation, .. }
            | Self::ReplayCancelled { generation }
            | Self::ReplayFaulted { generation, .. }
            | Self::Rerun { generation }
            | Self::RequestFinalize { generation }
            | Self::Choose { generation, .. }
            | Self::Back { generation }
            | Self::Finalized { generation }
            | Self::FinalizeFaulted { generation, .. } => Some(*generation),
        }
    }

    /// 🩺️ Whether every fault code and alternative name the event carries is valid.
    pub fn is_well_formed(&self) -> bool {
        match self {
            Self::ReplayFaulted { code, .. } | Self::FinalizeFaulted { code, .. } => is_time_travel_fault_code(code),
            Self::Choose { choice: TimeTravelChoice::Alternative { name }, .. } => is_time_travel_alternative_name(name),
            _ => true,
        }
    }
}

/// 🗺️ Matrix column of an event; `Choose` splits into `ChooseOverwrite` and `ChooseAlternative`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimeTravelEventKey {
    Begin,
    BeginWithdrawn,
    Draft,
    Withdraw,
    Accept,
    Discard,
    Restore,
    ReplayProgressed,
    ReplayCompleted,
    ReplayCancelled,
    ReplayFaulted,
    Rerun,
    RequestFinalize,
    ChooseOverwrite,
    ChooseAlternative,
    Back,
    Finalized,
    FinalizeFaulted,
    BaseMoved,
    Exit,
}

impl TimeTravelEventKey {
    pub const ALL: [Self; 20] = [
        Self::Begin,
        Self::BeginWithdrawn,
        Self::Draft,
        Self::Withdraw,
        Self::Accept,
        Self::Discard,
        Self::Restore,
        Self::ReplayProgressed,
        Self::ReplayCompleted,
        Self::ReplayCancelled,
        Self::ReplayFaulted,
        Self::Rerun,
        Self::RequestFinalize,
        Self::ChooseOverwrite,
        Self::ChooseAlternative,
        Self::Back,
        Self::Finalized,
        Self::FinalizeFaulted,
        Self::BaseMoved,
        Self::Exit,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Begin => "begin",
            Self::BeginWithdrawn => "beginWithdrawn",
            Self::Draft => "draft",
            Self::Withdraw => "withdraw",
            Self::Accept => "accept",
            Self::Discard => "discard",
            Self::Restore => "restore",
            Self::ReplayProgressed => "replayProgressed",
            Self::ReplayCompleted => "replayCompleted",
            Self::ReplayCancelled => "replayCancelled",
            Self::ReplayFaulted => "replayFaulted",
            Self::Rerun => "rerun",
            Self::RequestFinalize => "requestFinalize",
            Self::ChooseOverwrite => "chooseOverwrite",
            Self::ChooseAlternative => "chooseAlternative",
            Self::Back => "back",
            Self::Finalized => "finalized",
            Self::FinalizeFaulted => "finalizeFaulted",
            Self::BaseMoved => "baseMoved",
            Self::Exit => "exit",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|key| key.as_str() == text)
    }
}
//#endregion 🔖️Events

//#region 🔖️Effects
/// 🛠️ What the driver performs for an accepted transition, in order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TimeTravelEffect {
    /// 🖼️ Render the state before `target` with the accepted upstream drafts and `replacement`; downstream is not applied.
    ShowPreview { target: MutationId, replacement: InputReplacement },
    /// ▶️ Start (latest-wins, replacing any running one) a Report-mode replay of `drafts` from `from`.
    StartReplay { drafts: Vec<SupersededInput>, from: MutationId },
    /// ⏹️ Cancel the running replay.
    CancelReplay,
    /// 🗳️ Open the finalize prompt (overwrite or new alternative).
    OpenFinalizePrompt,
    /// ✍️ Commit an unscoped `Supersede` of `inputs`.
    CommitOverwrite { inputs: Vec<SupersededInput> },
    /// 🌿️ Commit a `Branch` named `name` plus a scoped `Supersede` of `inputs`.
    CommitAlternative { name: String, inputs: Vec<SupersededInput> },
    /// 🚪️ Leave time travel and render the committed head.
    Close,
}

impl TimeTravelEffect {
    pub fn kind(&self) -> TimeTravelEffectKind {
        match self {
            Self::ShowPreview { .. } => TimeTravelEffectKind::ShowPreview,
            Self::StartReplay { .. } => TimeTravelEffectKind::StartReplay,
            Self::CancelReplay => TimeTravelEffectKind::CancelReplay,
            Self::OpenFinalizePrompt => TimeTravelEffectKind::OpenFinalizePrompt,
            Self::CommitOverwrite { .. } => TimeTravelEffectKind::CommitOverwrite,
            Self::CommitAlternative { .. } => TimeTravelEffectKind::CommitAlternative,
            Self::Close => TimeTravelEffectKind::Close,
        }
    }
}

/// 🏷️ Payload-free discriminant of a [`TimeTravelEffect`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimeTravelEffectKind {
    ShowPreview,
    StartReplay,
    CancelReplay,
    OpenFinalizePrompt,
    CommitOverwrite,
    CommitAlternative,
    Close,
}

impl TimeTravelEffectKind {
    pub const ALL: [Self; 7] = [Self::ShowPreview, Self::StartReplay, Self::CancelReplay, Self::OpenFinalizePrompt, Self::CommitOverwrite, Self::CommitAlternative, Self::Close];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::ShowPreview => "showPreview",
            Self::StartReplay => "startReplay",
            Self::CancelReplay => "cancelReplay",
            Self::OpenFinalizePrompt => "openFinalizePrompt",
            Self::CommitOverwrite => "commitOverwrite",
            Self::CommitAlternative => "commitAlternative",
            Self::Close => "close",
        }
    }

    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.as_str() == text)
    }
}
//#endregion 🔖️Effects

//#region 🔖️Refusals
/// 🙅️ Why an event was not applied; the session is left untouched.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimeTravelRefusal {
    /// 🚫️ The event is not legal in the current stage, or carries a malformed fault code or name.
    Illegal,
    /// ⌛️ The event addresses another generation, or `BaseMoved` repeats the current base (silent no-op).
    Stale,
    /// 🧱️ Legal in this stage but blocked: a changed pending draft, a missing or blocking report, a commit in flight.
    Blocked,
    /// 🫙️ `RequestFinalize` or `Rerun` with no accepted draft.
    Empty,
    /// 🧾️ `Accept` needs a change against the current draft baseline.
    Unchanged,
}

impl TimeTravelRefusal {
    pub const ALL: [Self; 5] = [Self::Illegal, Self::Stale, Self::Blocked, Self::Empty, Self::Unchanged];

    /// 🔖️ Fault code, e.g. `timeTravel.stale`.
    pub fn code(self) -> &'static str {
        match self {
            Self::Illegal => "timeTravel.illegal",
            Self::Stale => "timeTravel.stale",
            Self::Blocked => "timeTravel.blocked",
            Self::Empty => "timeTravel.empty",
            Self::Unchanged => "timeTravel.unchanged",
        }
    }

    pub fn parse(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|refusal| refusal.code() == code)
    }

    /// 🪧️ Framework label of this refusal.
    pub fn label(self) -> TimeTravelLabel {
        match self {
            Self::Illegal => TimeTravelLabel::RefusalIllegal,
            Self::Stale => TimeTravelLabel::RefusalStale,
            Self::Blocked => TimeTravelLabel::RefusalBlocked,
            Self::Empty => TimeTravelLabel::RefusalEmpty,
            Self::Unchanged => TimeTravelLabel::RefusalUnchanged,
        }
    }
}

impl std::fmt::Display for TimeTravelRefusal {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for TimeTravelRefusal {}
//#endregion 🔖️Refusals

//#region 🔖️Reducer
impl TimeTravelSession {
    /// 🌱️ A fresh, inactive session observing `base`.
    pub fn new(base: TimeTravelBase) -> Self {
        Self { base, ..Self::default() }
    }

    /// ⚖️ Applies `event` in place: generation fence first, then the §4 law; a refusal leaves `self` untouched.
    pub fn apply(&mut self, event: TimeTravelEvent) -> Result<Vec<TimeTravelEffect>, TimeTravelRefusal> {
        use TimeTravelEvent as E;
        use TimeTravelStage as S;
        if event.generation().is_some_and(|generation| generation != self.generation) {
            return Err(TimeTravelRefusal::Stale);
        }
        if !event.is_well_formed() {
            return Err(TimeTravelRefusal::Illegal);
        }
        match (self.stage, event) {
            (_, E::Begin { target, original }) => self.open(target, original, None),
            (_, E::BeginWithdrawn { target, original }) => self.open(target, original, Some(InputReplacement::Withdrawn)),
            (S::Editing, E::Draft { replacement, .. }) => self.redraft(replacement),
            (S::Editing, E::Withdraw { .. }) => self.redraft(InputReplacement::Withdrawn),
            (S::Editing, E::Accept { .. }) => {
                if let Some(refusal) = self.accept_refusal() {
                    return Err(refusal);
                }
                let pending = self.pending.take().ok_or(TimeTravelRefusal::Illegal)?;
                Ok(self.accept(pending))
            }
            (S::Editing, E::Discard { .. }) => {
                let pending = self.pending.take().ok_or(TimeTravelRefusal::Illegal)?;
                Ok(self.resume(pending.return_stage))
            }
            (_, E::Restore { target, .. }) => match self.restore_refusal(&target) {
                Some(refusal) => Err(refusal),
                None => {
                    self.accepted.retain(|draft| draft.target.mutation != target);
                    Ok(if self.accepted.is_empty() { self.close(Vec::new()) } else { self.settle() })
                }
            },
            (S::Replaying, E::ReplayProgressed { done, total, .. }) => {
                self.progress = Some(TimeTravelProgress { done, total });
                Ok(Vec::new())
            }
            (S::Replaying, E::ReplayCompleted { report, .. }) => {
                self.stage = S::Reviewing;
                self.report = Some(report);
                self.progress = None;
                Ok(Vec::new())
            }
            (S::Replaying, E::ReplayCancelled { .. }) => {
                self.stage = S::Reviewing;
                self.progress = None;
                self.fault = Some(TIME_TRAVEL_CANCELLED_CODE.to_string());
                Ok(Vec::new())
            }
            (S::Replaying, E::ReplayFaulted { code, .. }) => {
                self.stage = S::Reviewing;
                self.progress = None;
                self.fault = Some(code);
                Ok(Vec::new())
            }
            (_, E::Rerun { .. }) => match self.rerun_refusal() {
                Some(refusal) => Err(refusal),
                None => Ok(self.settle()),
            },
            (_, E::RequestFinalize { .. }) => match self.finalize_refusal() {
                Some(refusal) => Err(refusal),
                None => {
                    self.stage = S::Choosing;
                    Ok(vec![TimeTravelEffect::OpenFinalizePrompt])
                }
            },
            (S::Choosing, E::Choose { choice, .. }) => {
                let inputs = self.inputs();
                self.stage = S::Finalizing;
                self.generation = self.generation.wrapping_add(1);
                Ok(vec![match choice {
                    TimeTravelChoice::Overwrite => TimeTravelEffect::CommitOverwrite { inputs },
                    TimeTravelChoice::Alternative { name } => TimeTravelEffect::CommitAlternative { name, inputs },
                }])
            }
            (S::Choosing, E::Back { .. }) => {
                self.stage = S::Reviewing;
                Ok(Vec::new())
            }
            (S::Finalizing, E::Finalized { .. }) => Ok(self.close(Vec::new())),
            (S::Finalizing, E::FinalizeFaulted { code, .. }) => {
                let effects = self.settle();
                self.fault = Some(code);
                Ok(effects)
            }
            (stage, E::BaseMoved { base, positions }) => {
                if base == self.base {
                    return Err(TimeTravelRefusal::Stale);
                }
                self.base = base;
                self.reposition(&positions);
                Ok(match stage {
                    S::Inactive | S::Finalizing => Vec::new(),
                    S::Editing => {
                        self.report = None;
                        self.pending.as_ref().map_or_else(Vec::new, |pending| vec![TimeTravelEffect::ShowPreview { target: pending.target.mutation.clone(), replacement: pending.replacement.clone() }])
                    }
                    S::Replaying | S::Reviewing | S::Choosing => self.settle(),
                })
            }
            (S::Inactive, E::Exit) => Err(TimeTravelRefusal::Illegal),
            (S::Finalizing, E::Exit) => Err(TimeTravelRefusal::Blocked),
            (S::Replaying, E::Exit) => Ok(self.close(vec![TimeTravelEffect::CancelReplay])),
            (_, E::Exit) => Ok(self.close(Vec::new())),
            _ => Err(TimeTravelRefusal::Illegal),
        }
    }

    /// 🔎️ The accepted draft of `mutation`, if any.
    pub fn accepted_draft(&self, mutation: &MutationId) -> Option<&TimeTravelDraft> {
        self.accepted.iter().find(|draft| draft.target.mutation == *mutation)
    }

    /// 🧾️ The accepted drafts as `Supersede` inputs, in history order.
    pub fn inputs(&self) -> Vec<SupersededInput> {
        self.accepted.iter().map(|draft| SupersededInput { target: draft.target.mutation.clone(), replacement: draft.replacement.clone() }).collect()
    }

    /// 🧷️ The value a pending draft started from: the target's accepted draft, else its original input.
    pub fn start_of<'session>(&'session self, pending: &'session TimeTravelPending) -> &'session InputReplacement {
        self.accepted_draft(&pending.target.mutation).map_or(&pending.original, |draft| &draft.replacement)
    }

    /// 🟰️ Whether `pending` still equals the value it started from.
    pub fn unchanged(&self, pending: &TimeTravelPending) -> bool {
        pending.replacement == *self.start_of(pending)
    }

    /// 🧾️ Why `Accept` would be refused; an unchanged input keeps the draft open.
    pub fn accept_refusal(&self) -> Option<TimeTravelRefusal> {
        match (self.stage, self.pending.as_ref()) {
            (TimeTravelStage::Editing, Some(pending)) if self.unchanged(pending) => Some(TimeTravelRefusal::Unchanged),
            (TimeTravelStage::Editing, Some(_)) => None,
            _ => Some(TimeTravelRefusal::Illegal),
        }
    }

    /// 🚧️ Why `RequestFinalize` would be refused, `None` when it would open the prompt.
    pub fn finalize_refusal(&self) -> Option<TimeTravelRefusal> {
        if self.stage != TimeTravelStage::Reviewing {
            Some(TimeTravelRefusal::Illegal)
        } else if self.accepted.is_empty() {
            Some(TimeTravelRefusal::Empty)
        } else if self.report.as_ref().is_none_or(ReplayReport::blocks_finalize) {
            Some(TimeTravelRefusal::Blocked)
        } else {
            None
        }
    }

    /// ✏️ Why `Begin` (and `BeginWithdrawn`) would be refused, `None` when it would open (or switch) the draft editor: it is
    /// legal from `Inactive` and `Reviewing`, from `Editing` only while the pending draft still equals its start (`Blocked`
    /// otherwise), and `Illegal` while replaying, choosing or finalizing — what a host disables a row's Edit and Withdraw
    /// controls by.
    pub fn begin_refusal(&self) -> Option<TimeTravelRefusal> {
        match (self.stage, self.pending.as_ref()) {
            (TimeTravelStage::Inactive | TimeTravelStage::Reviewing, _) => None,
            (TimeTravelStage::Editing, Some(pending)) if self.unchanged(pending) => None,
            (TimeTravelStage::Editing, Some(_)) => Some(TimeTravelRefusal::Blocked),
            _ => Some(TimeTravelRefusal::Illegal),
        }
    }

    /// 🔙️ Why `Restore` of `mutation` would be refused, `None` when it would take the mutation's accepted draft back: it
    /// needs `Reviewing` and an accepted draft of that mutation (`Illegal` otherwise) — what a host disables a row's
    /// Restore control by.
    pub fn restore_refusal(&self, mutation: &MutationId) -> Option<TimeTravelRefusal> {
        (self.stage != TimeTravelStage::Reviewing || self.accepted_draft(mutation).is_none()).then_some(TimeTravelRefusal::Illegal)
    }

    /// 🔁️ Why `Rerun` would be refused, `None` when it would start a replay: it needs `Reviewing`,
    /// accepted drafts, and either no report or a fault.
    pub fn rerun_refusal(&self) -> Option<TimeTravelRefusal> {
        if self.stage != TimeTravelStage::Reviewing {
            Some(TimeTravelRefusal::Illegal)
        } else if self.accepted.is_empty() {
            Some(TimeTravelRefusal::Empty)
        } else if self.report.is_some() && self.fault.is_none() {
            Some(TimeTravelRefusal::Illegal)
        } else {
            None
        }
    }

    /// 🧐️ Classification of a `Reviewing` session; `None` in every other stage.
    pub fn review(&self) -> Option<TimeTravelReview> {
        (self.stage == TimeTravelStage::Reviewing).then(|| match &self.report {
            _ if self.accepted.is_empty() => TimeTravelReview::NoChanges,
            None => TimeTravelReview::NeedsReplay,
            Some(report) if report.blocks_finalize() => TimeTravelReview::Blocked,
            Some(_) => TimeTravelReview::Ready,
        })
    }

    /// 🧪️ First violated session invariant (law fixture `invariants`), `None` when coherent.
    pub fn invariant_violation(&self) -> Option<&'static str> {
        use TimeTravelStage as S;
        if (self.stage == S::Editing) != self.pending.is_some() {
            return Some("pending-iff-editing");
        }
        if self.pending.as_ref().is_some_and(|pending| !matches!(pending.return_stage, S::Inactive | S::Reviewing)) {
            return Some("return-stage-inactive-or-reviewing");
        }
        if self.pending.as_ref().is_some_and(|pending| pending.return_stage == S::Inactive && !self.accepted.is_empty()) {
            return Some("return-inactive-means-nothing-accepted");
        }
        if self.stage == S::Inactive && (!self.accepted.is_empty() || self.report.is_some() || self.fault.is_some()) {
            return Some("inactive-holds-nothing");
        }
        if matches!(self.stage, S::Replaying | S::Choosing | S::Finalizing) && self.accepted.is_empty() {
            return Some("work-needs-accepted-drafts");
        }
        if self.progress.is_some() && self.stage != S::Replaying {
            return Some("progress-only-while-replaying");
        }
        if self.stage == S::Replaying && self.report.is_some() {
            return Some("replaying-has-no-report");
        }
        if matches!(self.stage, S::Choosing | S::Finalizing) && self.report.as_ref().is_none_or(ReplayReport::blocks_finalize) {
            return Some("finalize-needs-a-clean-report");
        }
        if self.accepted.windows(2).any(|pair| pair[0].target.history_order(&pair[1].target) != Ordering::Less) {
            return Some("accepted-in-history-order");
        }
        if self.accepted.iter().enumerate().any(|(index, draft)| self.accepted[..index].iter().any(|earlier| earlier.target.mutation == draft.target.mutation)) {
            return Some("accepted-targets-unique");
        }
        if self.fault.is_some() && self.accepted.is_empty() {
            return Some("fault-needs-drafts");
        }
        None
    }

    /// 🔦️ Opens (or retargets) the draft editor on `target` where [`Self::begin_refusal`] admits it: the pending draft starts
    /// as `draft`, else as the target's accepted draft, else as `original`, and returns to the stage the session was opened
    /// from; a session opened from `Inactive` is the next one.
    fn open(&mut self, target: TimeTravelTarget, original: InputReplacement, draft: Option<InputReplacement>) -> Result<Vec<TimeTravelEffect>, TimeTravelRefusal> {
        if let Some(refusal) = self.begin_refusal() {
            return Err(refusal);
        }
        let return_stage = self.pending.as_ref().map_or(self.stage, |pending| pending.return_stage);
        if self.stage == TimeTravelStage::Inactive {
            self.id = self.id.wrapping_add(1);
        }
        let replacement = draft.or_else(|| self.accepted_draft(&target.mutation).map(|accepted| accepted.replacement.clone())).unwrap_or_else(|| original.clone());
        let effect = TimeTravelEffect::ShowPreview { target: target.mutation.clone(), replacement: replacement.clone() };
        self.stage = TimeTravelStage::Editing;
        self.generation = self.generation.wrapping_add(1);
        self.pending = Some(TimeTravelPending { target, original, replacement, return_stage });
        Ok(vec![effect])
    }

    fn redraft(&mut self, replacement: InputReplacement) -> Result<Vec<TimeTravelEffect>, TimeTravelRefusal> {
        let pending = self.pending.as_mut().ok_or(TimeTravelRefusal::Illegal)?;
        pending.replacement = replacement;
        Ok(vec![TimeTravelEffect::ShowPreview { target: pending.target.mutation.clone(), replacement: pending.replacement.clone() }])
    }

    fn accept(&mut self, pending: TimeTravelPending) -> Vec<TimeTravelEffect> {
        self.accepted.retain(|draft| draft.target.mutation != pending.target.mutation);
        if pending.replacement != pending.original {
            let draft = TimeTravelDraft { target: pending.target, replacement: pending.replacement };
            let index = self.accepted.partition_point(|existing| existing.target.history_order(&draft.target) == Ordering::Less);
            self.accepted.insert(index, draft);
        }
        self.settle()
    }

    fn resume(&mut self, return_stage: TimeTravelStage) -> Vec<TimeTravelEffect> {
        if return_stage != TimeTravelStage::Reviewing {
            return self.close(Vec::new());
        }
        if self.report.is_none() && !self.accepted.is_empty() {
            return self.settle();
        }
        self.stage = TimeTravelStage::Reviewing;
        Vec::new()
    }

    fn settle(&mut self) -> Vec<TimeTravelEffect> {
        self.report = None;
        self.progress = None;
        let Some(first) = self.accepted.first() else {
            self.stage = TimeTravelStage::Reviewing;
            self.fault = None;
            return Vec::new();
        };
        let from = first.target.mutation.clone();
        self.stage = TimeTravelStage::Replaying;
        self.fault = None;
        self.generation = self.generation.wrapping_add(1);
        vec![TimeTravelEffect::StartReplay { drafts: self.inputs(), from }]
    }

    fn close(&mut self, mut effects: Vec<TimeTravelEffect>) -> Vec<TimeTravelEffect> {
        self.stage = TimeTravelStage::Inactive;
        self.generation = self.generation.wrapping_add(1);
        self.accepted.clear();
        self.pending = None;
        self.report = None;
        self.progress = None;
        self.fault = None;
        effects.push(TimeTravelEffect::Close);
        effects
    }

    fn reposition(&mut self, positions: &[TimeTravelTarget]) {
        for moved in positions {
            for target in self.accepted.iter_mut().map(|draft| &mut draft.target).chain(self.pending.as_mut().map(|pending| &mut pending.target)) {
                if target.mutation == moved.mutation {
                    target.position = moved.position;
                }
            }
        }
        self.accepted.sort_by(|left, right| left.target.history_order(&right.target));
    }
}
//#endregion 🔖️Reducer

//#region 🔖️Labels
/// 🗂️ Every `timeTravel.*` code a history-edit verb or driver answers, with the label a host shows for it.
pub const TIME_TRAVEL_CODE_LABELS: [(&str, TimeTravelLabel); 21] = [
    (TIME_TRAVEL_FROZEN_CODE, TimeTravelLabel::Frozen),
    ("timeTravel.illegal", TimeTravelLabel::RefusalIllegal),
    ("timeTravel.stale", TimeTravelLabel::RefusalStale),
    ("timeTravel.blocked", TimeTravelLabel::RefusalBlocked),
    ("timeTravel.empty", TimeTravelLabel::RefusalEmpty),
    ("timeTravel.unchanged", TimeTravelLabel::RefusalUnchanged),
    (TIME_TRAVEL_CANCELLED_CODE, TimeTravelLabel::ReplayCancelled),
    (TIME_TRAVEL_BUSY_CODE, TimeTravelLabel::RefusalBusy),
    (TIME_TRAVEL_UNKNOWN_MUTATION_CODE, TimeTravelLabel::RefusalUnknownMutation),
    (TIME_TRAVEL_NOT_EDITABLE_CODE, TimeTravelLabel::RefusalNotEditable),
    (TIME_TRAVEL_UNKNOWN_INPUT_CODE, TimeTravelLabel::RefusalUnknownInput),
    (TIME_TRAVEL_INVALID_INPUT_CODE, TimeTravelLabel::RefusalInvalidInput),
    (TIME_TRAVEL_NO_SELECTION_CODE, TimeTravelLabel::RefusalNoSelection),
    (TIME_TRAVEL_NAME_REQUIRED_CODE, TimeTravelLabel::RefusalNameRequired),
    (TIME_TRAVEL_NAME_INVALID_CODE, TimeTravelLabel::RefusalNameInvalid),
    (TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE, TimeTravelLabel::RefusalSchemaUnavailable),
    (TIME_TRAVEL_REPLAY_FAULTED_CODE, TimeTravelLabel::ReplayFaulted),
    (TIME_TRAVEL_COMMIT_FAILED_CODE, TimeTravelLabel::CommitFailed),
    (TIME_TRAVEL_MEMBER_GONE_CODE, TimeTravelLabel::RefusalMemberGone),
    (TIME_TRAVEL_NOT_WITHDRAWABLE_CODE, TimeTravelLabel::RefusalNotWithdrawable),
    (TIME_TRAVEL_EDITOR_CLOSED_CODE, TimeTravelLabel::RefusalEditorClosed),
];

/// 💬️ Framework-owned EN/DE text of history editing, no default locale.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TimeTravelLabel {
    StageInactive,
    StageEditing,
    StageReplaying,
    StageReviewing,
    StageChoosing,
    StageFinalizing,
    RefusalIllegal,
    RefusalStale,
    RefusalBlocked,
    RefusalEmpty,
    RefusalUnchanged,
    Frozen,
    ChoiceOverwrite,
    ChoiceOverwriteDescription,
    ChoiceAlternative,
    ChoiceAlternativeDescription,
    AlternativeNameDefault,
    NoChanges,
    NeedsReplay,
    ReportBlocking,
    ReadyToFinalize,
    ReplayCancelled,
    ActionRerun,
    PreparationProgress,
    PreparationProgressValueText,
    ReplayProgressValueText,
    Processed,
    RefusalBusy,
    RefusalUnknownMutation,
    RefusalNotEditable,
    RefusalUnknownInput,
    RefusalInvalidInput,
    RefusalNoSelection,
    RefusalNameRequired,
    RefusalNameInvalid,
    RefusalSchemaUnavailable,
    ReplayFaulted,
    CommitFailed,
    OutcomeIntroduced,
    RefusalMemberGone,
    MemberEdited,
    RefusalNotWithdrawable,
    RefusalEditorClosed,
    RefusalReadOnly,
}

impl TimeTravelLabel {
    pub const ALL: [Self; 44] = [
        Self::StageInactive,
        Self::StageEditing,
        Self::StageReplaying,
        Self::StageReviewing,
        Self::StageChoosing,
        Self::StageFinalizing,
        Self::RefusalIllegal,
        Self::RefusalStale,
        Self::RefusalBlocked,
        Self::RefusalEmpty,
        Self::RefusalUnchanged,
        Self::Frozen,
        Self::ChoiceOverwrite,
        Self::ChoiceOverwriteDescription,
        Self::ChoiceAlternative,
        Self::ChoiceAlternativeDescription,
        Self::AlternativeNameDefault,
        Self::NoChanges,
        Self::NeedsReplay,
        Self::ReportBlocking,
        Self::ReadyToFinalize,
        Self::ReplayCancelled,
        Self::ActionRerun,
        Self::PreparationProgress,
        Self::PreparationProgressValueText,
        Self::ReplayProgressValueText,
        Self::Processed,
        Self::RefusalBusy,
        Self::RefusalUnknownMutation,
        Self::RefusalNotEditable,
        Self::RefusalUnknownInput,
        Self::RefusalInvalidInput,
        Self::RefusalNoSelection,
        Self::RefusalNameRequired,
        Self::RefusalNameInvalid,
        Self::RefusalSchemaUnavailable,
        Self::ReplayFaulted,
        Self::CommitFailed,
        Self::OutcomeIntroduced,
        Self::RefusalMemberGone,
        Self::MemberEdited,
        Self::RefusalNotWithdrawable,
        Self::RefusalEditorClosed,
        Self::RefusalReadOnly,
    ];

    /// 🔑️ `(key, en, de)` row of this label.
    fn row(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::StageInactive => ("stageInactive", "Not editing history", "Verlauf wird nicht bearbeitet"),
            Self::StageEditing => ("stageEditing", "Editing a mutation", "Mutation wird bearbeitet"),
            Self::StageReplaying => ("stageReplaying", "Replaying later mutations", "Spätere Mutationen werden neu angewendet"),
            Self::StageReviewing => ("stageReviewing", "Reviewing the edited history", "Bearbeiteter Verlauf wird geprüft"),
            Self::StageChoosing => ("stageChoosing", "Choose how to finalize", "Art des Abschlusses wählen"),
            Self::StageFinalizing => ("stageFinalizing", "Finalizing the history edit", "Verlaufsbearbeitung wird abgeschlossen"),
            Self::RefusalIllegal => ("refusalIllegal", "Not possible right now", "Derzeit nicht möglich"),
            Self::RefusalStale => ("refusalStale", "Outdated request ignored", "Veraltete Anfrage ignoriert"),
            Self::RefusalBlocked => ("refusalBlocked", "Blocked: resolve the pending change or the errors first", "Blockiert: zuerst die offene Änderung oder die Fehler auflösen"),
            Self::RefusalEmpty => ("refusalEmpty", "Nothing to finalize: no accepted changes", "Nichts abzuschließen: keine übernommenen Änderungen"),
            Self::RefusalUnchanged => ("refusalUnchanged", "Change an input before accepting", "Vor dem Übernehmen eine Eingabe ändern"),
            Self::Frozen => ("frozen", "Editing is paused while history is being edited", "Bearbeiten ist pausiert, solange der Verlauf bearbeitet wird"),
            Self::ChoiceOverwrite => ("choiceOverwrite", "Overwrite history", "Verlauf überschreiben"),
            Self::ChoiceOverwriteDescription => ("choiceOverwriteDescription", "Replaces the inputs in every alternative that contains these mutations", "Ersetzt die Eingaben in jeder Alternative, die diese Mutationen enthält"),
            Self::ChoiceAlternative => ("choiceAlternative", "New alternative", "Neue Alternative"),
            Self::ChoiceAlternativeDescription => ("choiceAlternativeDescription", "Keeps the original history and continues in a new alternative", "Behält den ursprünglichen Verlauf und arbeitet in einer neuen Alternative weiter"),
            Self::AlternativeNameDefault => ("alternativeNameDefault", "Edited history", "Bearbeiteter Verlauf"),
            Self::NoChanges => ("noChanges", "No changes: showing the current history", "Keine Änderungen: aktueller Verlauf wird angezeigt"),
            Self::NeedsReplay => ("needsReplay", "Replay needed: later mutations are not checked yet", "Neu anwenden nötig: spätere Mutationen sind noch nicht geprüft"),
            Self::ReportBlocking => ("reportBlocking", "Errors must be fixed or withdrawn before finalizing", "Fehler müssen vor dem Abschließen behoben oder zurückgezogen werden"),
            Self::ReadyToFinalize => ("readyToFinalize", "Ready to finalize", "Bereit zum Abschließen"),
            Self::ReplayCancelled => ("replayCancelled", "Replay cancelled", "Neu anwenden abgebrochen"),
            Self::ActionRerun => ("actionRerun", "Replay again", "Erneut anwenden"),
            Self::PreparationProgress => ("preparationProgress", "Preparing history preview", "Verlaufsvorschau wird vorbereitet"),
            Self::PreparationProgressValueText => ("preparationProgressValueText", "Preparing history preview: {done} of {total} steps", "Verlaufsvorschau wird vorbereitet: {done} von {total} Schritten"),
            Self::ReplayProgressValueText => ("replayProgressValueText", "Replaying history: {done} of {total} steps", "Verlauf wird neu angewendet: {done} von {total} Schritten"),
            Self::Processed => ("processed", "Work completed: {processed}", "Arbeitsfortschritt: {processed}"),
            Self::RefusalBusy => ("refusalBusy", "History editing is busy: finish the running tool or the other history edit first", "Verlaufsbearbeitung beschäftigt: zuerst das laufende Werkzeug oder die andere Verlaufsbearbeitung abschließen"),
            Self::RefusalUnknownMutation => ("refusalUnknownMutation", "This mutation is no longer in the history", "Diese Mutation ist nicht mehr im Verlauf"),
            Self::RefusalNotEditable => ("refusalNotEditable", "The inputs of this mutation cannot be edited", "Die Eingaben dieser Mutation können nicht bearbeitet werden"),
            Self::RefusalUnknownInput => ("refusalUnknownInput", "This input does not exist in the mutation", "Diese Eingabe gibt es in der Mutation nicht"),
            Self::RefusalInvalidInput => ("refusalInvalidInput", "Invalid value: the input keeps its previous value", "Ungültiger Wert: Die Eingabe behält ihren bisherigen Wert"),
            Self::RefusalNoSelection => ("refusalNoSelection", "Nothing suitable is selected for this input", "Für diese Eingabe ist nichts Passendes ausgewählt"),
            Self::RefusalNameRequired => ("refusalNameRequired", "Name the new alternative", "Einen Namen für die neue Alternative eingeben"),
            Self::RefusalNameInvalid => ("refusalNameInvalid", "Invalid alternative name: use 1 to 256 characters", "Ungültiger Name der Alternative: 1 bis 256 Zeichen verwenden"),
            Self::RefusalSchemaUnavailable => ("refusalSchemaUnavailable", "The input schema of this mutation is unavailable", "Das Eingabeschema dieser Mutation ist nicht verfügbar"),
            Self::ReplayFaulted => ("replayFaulted", "Replay failed: later mutations could not be checked", "Erneutes Anwenden fehlgeschlagen: Spätere Mutationen konnten nicht geprüft werden"),
            Self::CommitFailed => ("commitFailed", "Finalizing failed: the history is unchanged", "Abschließen fehlgeschlagen: Der Verlauf ist unverändert"),
            Self::OutcomeIntroduced => ("outcomeIntroduced", "New since this edit", "Neu durch diese Bearbeitung"),
            Self::RefusalMemberGone => ("refusalMemberGone", "The part this history edit targets was closed", "Der Teil, den diese Verlaufsbearbeitung betrifft, wurde geschlossen"),
            Self::MemberEdited => ("memberEdited", "History of a composed part edited", "Verlauf eines eingebetteten Teils bearbeitet"),
            Self::RefusalNotWithdrawable => ("refusalNotWithdrawable", "This mutation cannot be withdrawn here", "Diese Mutation kann hier nicht zurückgezogen werden"),
            Self::RefusalEditorClosed => ("refusalEditorClosed", "The draft editor is closed: open the mutation again", "Der Entwurfseditor ist geschlossen: die Mutation erneut öffnen"),
            Self::RefusalReadOnly => ("refusalReadOnly", "History cannot be edited in a read-only view", "Der Verlauf kann in einer schreibgeschützten Ansicht nicht bearbeitet werden"),
        }
    }

    pub fn key(self) -> &'static str {
        self.row().0
    }

    pub fn parse(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|label| label.key() == key)
    }

    pub fn en(self) -> &'static str {
        self.row().1
    }

    pub fn de(self) -> &'static str {
        self.row().2
    }

    /// 🩹️ Label of every `timeTravel.*` code a host shows — the session's refusals, the frozen and cancelled codes, the
    /// hosting runtime's refusals and driver faults ([`TIME_TRAVEL_CODE_LABELS`]); `None` for any other code.
    pub fn for_code(code: &str) -> Option<Self> {
        TIME_TRAVEL_CODE_LABELS.iter().find(|(known, _)| *known == code).map(|(_, label)| *label)
    }

    /// 🌐️ Hands both locales to a carrier constructor, e.g. `label.localized(LocalizedLabel::native)`.
    pub fn localized<L>(self, native: impl FnOnce(&'static str, &'static str) -> L) -> L {
        let (_, en, de) = self.row();
        native(en, de)
    }
}
//#endregion 🔖️Labels

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
