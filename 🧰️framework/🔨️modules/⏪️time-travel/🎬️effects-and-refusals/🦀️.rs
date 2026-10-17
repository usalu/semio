//! 🎬️ Effects and refusals of the time-travel session: the effect and its kind, the refusal enum with its code and display, and the error implementation. See [`super`].

use super::*;

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
