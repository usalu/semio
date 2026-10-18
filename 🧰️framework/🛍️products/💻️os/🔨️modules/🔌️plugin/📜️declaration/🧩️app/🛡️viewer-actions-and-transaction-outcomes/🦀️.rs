//! 🛡️ Viewer actions and transaction outcomes: the transaction prepare outcome, the ephemeral snapshot and the reserved viewer and time-travel action ids. See [`super`].

use super::*;

/// 🔀️ `PluginApp::transaction_prepare`'s result: `foreign` is this member's own recursively
/// discovered foreign steps (contract §5.4 — empty when its prepared ops touch no other
/// artifact); `rejection` is `Some` iff this instance declines the prepare (one of the frozen
/// rejection codes, contract §5's taxonomy). Both concrete/object-safe (no `A::Mutation`
/// leakage), matching `AppFrame::TransactionPrepared`'s own flat `{foreign, rejection}` shape.
pub struct TransactionPrepareOutcome {
    pub foreign: Vec<protocol::ForeignStep>,
    pub rejection: Option<Fault>,
}

/// 👥️ Object-safe channel snapshot of the two EPHEMERAL lanes plus the zero-app-code
/// declared-broadcast interaction slice (contract-freeze §C7.6) — replaces the earlier flat
/// `(Vec<u8>, u64, u64)` tuple `PluginApp::ephemeral_snapshot` returned, adding `interaction`
/// without further widening that tuple's arity. `interaction` is
/// `encode_presence_interaction(&assemble_presence_interaction(app_id, &interaction_state,
/// &hover_specs, &selection_specs))` over the app's own declared broadcast domains — empty bytes
/// when no domain is declared or broadcasting right now. Frames straight onto
/// `AppFrame::Ephemeral`'s four fields, same order.
#[derive(Clone, Debug, PartialEq)]
pub struct EphemeralSnapshot {
    pub presence: Vec<u8>,
    pub presence_generation: u64,
    pub transient_generation: u64,
    pub interaction: Vec<u8>,
    /// ⏯️ This instance's tool run summary (`PluginApp::tool_run_presence`, contract §3.4) — the local
    /// `PresencePeer.tool_run` the host's heartbeat assembly stamps; `None` without a run.
    pub tool_run: Option<protocol::PresenceToolRun>,
    /// ⏪️ This instance's open history edit (`TimeTravelLedger::presence`) — the local `PresencePeer.history_edit`
    /// a host stamps; `None` while no history edit is open.
    pub history_edit: Option<protocol::PresenceHistoryEdit>,
}

/// 🔒️ Contract §2.3: the string-action verbs a `Viewer`-role instance must reject at
/// `dispatch_action`'s entry — `import` is not a string action (see `PluginApp::import_media`'s
/// own check in `dispatch_import_media`), so it is not in this list. `checkoutCheckpoint` and
/// `switchAlternative` are rejected too: each commits a `Checkout` history transition to the shared event log, which
/// moves the head (and the scoped history edits it projects) for every replica, so it is a write, not a read cursor.
/// `try_build_definition` never declares these on a viewer app, so no host offers or binds them to a
/// Spectator; this guard is the backstop for a dispatch no manifest offered.
pub const VIEWER_REJECTED_ACTION_IDS: [&str; 9] = ["undo", "redo", "commitCheckpoint", "createAlternative", semio_framework::SWITCH_ALTERNATIVE_ACTION_ID, "checkoutCheckpoint", REVERT_TO_COMMAND_ACTION_ID, "cut", "paste"];

/// 🔒️ Whether a `Viewer`-role instance rejects `action`: the [`VIEWER_REJECTED_ACTION_IDS`] and every history-edit
/// verb (editing history supersedes inputs, which a viewer never may).
pub fn viewer_rejects_action(action: &str) -> bool {
    VIEWER_REJECTED_ACTION_IDS.contains(&action) || is_time_travel_action_id(action)
}

/// ⏪️ Whether `action` is one of the reserved, host-driven history-edit verbs.
pub fn is_time_travel_action_id(action: &str) -> bool {
    HISTORY_EDIT_ACTION_IDS.contains(&action)
}
