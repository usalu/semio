//! ⏱️ Limits and outcomes of the time-travel session: the replay and refresh budgets, the planning and frame retirement grants, the action refusals and the action outcome. See [`super`].

use super::*;

//#region 🔖️Limits
/// ⏱️ Wall budget of one replay slice per reactor turn.
pub const TIME_TRAVEL_TURN_WALL_US: u64 = 4_000;

/// 📡️ Most operations one reactor turn replays of a history change before the document store adopts it — a remote change
/// or this replica's own interior undo/redo, checkout or alternative switch (design §16.6, gap N17) — the cap behind the
/// turn's wall budget ([`time_travel_replay_turn_budget`]).
pub const TIME_TRAVEL_REPLAY_OPERATIONS: usize = 256;

/// ⏱️ One reactor turn of a deferred reprojection: [`TIME_TRAVEL_TURN_WALL_US`] of the job clock first, at most
/// [`TIME_TRAVEL_REPLAY_OPERATIONS`] operations second (`ArtifactStore::defer_remote_replays`,
/// `ArtifactStore::defer_local_replays`).
pub fn time_travel_replay_turn_budget() -> store::os_store::store_schema::ReplayTurnBudget {
    store::os_store::store_schema::ReplayTurnBudget::wall(TIME_TRAVEL_TURN_WALL_US, TIME_TRAVEL_REPLAY_OPERATIONS)
}

/// 📢️ Least interval between two replay-progress deliveries (history body refresh plus `HistoryPatch.timeTravel`),
/// unless the replay advanced by [`TIME_TRAVEL_PROGRESS_REFRESH_PERMILLE`] of its total since the last one.
pub const TIME_TRAVEL_PROGRESS_REFRESH_MS: u64 = 100;

/// 📢️ Replay advance (per mille of its total) that delivers progress before [`TIME_TRAVEL_PROGRESS_REFRESH_MS`] elapsed.
pub const TIME_TRAVEL_PROGRESS_REFRESH_PERMILLE: u64 = 50;

/// 🧹️ Discarded draft operations cold-retired per driver or close turn.
const TIME_TRAVEL_DISCARD_OPS_PER_TURN: usize = 64;

/// 🗳️ The request id the finalize prompt opens under; nothing awaits a dialog.
#[doc(hidden)]
pub const TIME_TRAVEL_FINALIZE_REQUEST: RequestId = RequestId(0x7417_0001);

#[doc(hidden)]
pub fn history_planning_retirement_grant(demand:RetirementDemand)->RetainedCloneGrant {
    RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:semio_framework_job::JOB_PAYLOAD_PAGE_BYTES.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}

#[doc(hidden)]
pub fn history_retirement_frame_grant(bytes:usize)->RetainedCloneGrant {
    RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1}
}
//#endregion 🔖️Limits

//#region 🔖️Outcome
/// 🙅️ Why a history-edit verb changed nothing: a session refusal, or a plugin-side precondition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeTravelActionRefusal {
    Session(TimeTravelRefusal),
    Busy,
    UnknownMutation,
    NotEditable,
    UnknownInput,
    InvalidInput,
    NoSelection,
    NameRequired,
    NameInvalid,
    SchemaUnavailable,
    NotWithdrawable,
    EditorClosed,
    UnitSpansDocuments,
}

impl TimeTravelActionRefusal {
    /// 🔖️ The `rejected` code of the verb result.
    pub fn code(self) -> &'static str {
        match self {
            Self::Session(refusal) => refusal.code(),
            Self::Busy => TIME_TRAVEL_BUSY_CODE,
            Self::UnknownMutation => TIME_TRAVEL_UNKNOWN_MUTATION_CODE,
            Self::NotEditable => TIME_TRAVEL_NOT_EDITABLE_CODE,
            Self::UnknownInput => TIME_TRAVEL_UNKNOWN_INPUT_CODE,
            Self::InvalidInput => TIME_TRAVEL_INVALID_INPUT_CODE,
            Self::NoSelection => TIME_TRAVEL_NO_SELECTION_CODE,
            Self::NameRequired => TIME_TRAVEL_NAME_REQUIRED_CODE,
            Self::NameInvalid => TIME_TRAVEL_NAME_INVALID_CODE,
            Self::SchemaUnavailable => TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE,
            Self::NotWithdrawable => TIME_TRAVEL_NOT_WITHDRAWABLE_CODE,
            Self::EditorClosed => TIME_TRAVEL_EDITOR_CLOSED_CODE,
            Self::UnitSpansDocuments => HISTORY_UNIT_SPANS_DOCUMENTS_CODE,
        }
    }
}

/// 🎛️ Outcome of one history-edit verb: the stage it left the session in, or why it was refused (a silent no-op).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeTravelActionOutcome {
    Pending,
    Applied(TimeTravelStage),
    Rejected(TimeTravelActionRefusal),
}
//#endregion 🔖️Outcome
