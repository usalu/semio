//! 📨️ Events of the time-travel session: the choice, the event and its stable key with their constructors, accessors and wire conversions. See [`super`].

use super::*;

//#region 🔖️Events
/// 🏁️ How a finalize publishes the accepted drafts.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TimeTravelChoice {
    Overwrite,
    Alternative { name: String },
}

/// 🎞️ User actions and driver observations; see the module docs for which carry a generation.
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
