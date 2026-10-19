//! 🪪️ Identity and session state of the time-travel session: the base stamp and target, the stage machine, the draft, pending and progress records, the review verdicts and the session itself. See [`super`].

use super::*;

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
    pub accepted: semio_framework_value_persistent::list::PagedList<TimeTravelDraft, {usize::MAX}>,
    pub pending: Option<TimeTravelPending>,
    pub report: Option<ReplayReport>,
    pub progress: Option<TimeTravelProgress>,
    pub fault: Option<String>,
}
//#endregion 🔖️Session
