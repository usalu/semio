//! 🎮️ The command history an app reads: history, command, mutation and alternative views and their filter.
use semio_framework_value::DslValue;
use semio_framework::ActionKind;
use store::HistoryColumn;
use semio_framework_ui_locale::LocalizedLabel;

/// 🎚️ Tri-state mutations filter of the framework history panel — `All` is the default.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_value::RetireOwned)]
pub enum HistoryCommandFilter {
    #[default]
    All,
    WithoutMutations,
    OnlyMutations,
}

impl HistoryCommandFilter {
    /// 🎚️ Every filter, in the order `setHistoryCommandFilter` declares its options.
    pub const ALL: [Self; 3] = [Self::All, Self::WithoutMutations, Self::OnlyMutations];

    /// 🔖️ The one wire value of this filter: the `setHistoryCommandFilter` option it is declared as and the
    /// `HistoryPatch.commandFilter` it is projected as.
    pub fn value(self) -> &'static str {
        match self {
            Self::All => semio_framework::HISTORY_COMMAND_FILTER_ALL,
            Self::WithoutMutations => semio_framework::HISTORY_COMMAND_FILTER_WITHOUT_MUTATIONS,
            Self::OnlyMutations => semio_framework::HISTORY_COMMAND_FILTER_ONLY_MUTATIONS,
        }
    }

    /// 🔎️ The filter a declared option `value` names; `None` for any other value.
    pub fn from_value(value: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|filter| filter.value() == value)
    }
}

/// ⏪️ A stored, replayable inverse for a `View`/`🐚️Shell`-kind command — the memory-only
/// counterpart to a VCS edit's `Mutation::inverse`. `action_id` is a plugin action id (`View` rows —
/// replayed locally via `dispatch_action`) or a shell command id (`🐚️Shell` rows — bubbled out as
/// `Effect::ReplayShellCommand` since the plugin has no access to shell-owned state). Never
/// persisted: it lives only on the in-memory `CommandLogEntry`/`CommandView`.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct InverseAction {
    pub action_id: String,
    pub args: Option<DslValue>,
}

/// 🧾️ One row of the merged command+operation timeline handed to renderers — `CommandLogEntry`
/// plus everything derived live from the current `envelope.vcs.edits` state.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct CommandView {
    pub seq: u64,
    pub action_id: String,
    pub label: LocalizedLabel,
    pub kind: ActionKind,
    pub timestamp: String,
    pub edit_id: Option<String>,
    /// 🧩️ Verbatim `CommandLogEntry::child_edit_ids` — see that field's own doc comment.
    pub child_edit_ids: Vec<String>,
    /// ✏️ Verbatim `CommandLogEntry::transition_id`; such a row's label, `applied` and `revertible` derive from the
    /// supersede ledger (`SupersedeLedger`).
    pub transition_id: Option<String>,
    /// 🖋️ The actor who authored this row's document edit or history transition.
    pub author: Option<String>,
    /// 📜️ The LAST [`HISTORY_ROW_OPERATION_PREVIEW`] forward operations of this entry's edit, printed via
    /// `OpText::print_op`, newest last — a bounded preview, so reading the history costs the same per key at the end of a
    /// long coalesced typing run as at its start; empty for cursor-motion entries and for a dangling `edit_id` (document
    /// replaced mid-session).
    pub op_lines: Vec<String>,
    /// 🔢️ Forward operations of this entry's edit when `op_lines` was printed — a sealed row's preview is reused only while
    /// its edit still has exactly that many.
    pub op_count: usize,
    /// ✅️ The linked DOCUMENT edit is currently on the applied stack (`false` once undone) —
    /// `false` (not meaningful) for a row with no `edit_id`.
    pub applied: bool,
    /// ⏪️ Either (document edit-linked) `applied` AND authored by the local actor, (config
    /// edit-linked) the config edit is similarly applied+local, (child edit-linked) one of this
    /// row's `child_edit_ids` is a live child's TAIL applied edit, or (memory-only) this row
    /// carries a stored `inverse` — only these entries offer "inverse".
    pub revertible: bool,
    /// 🔢️ See `CommandLogEntry::count`.
    pub count: u32,
    /// ⏪️ See `CommandLogEntry::inverse`.
    pub inverse: Option<InverseAction>,
    /// 🛠️ The committed tool transaction this row's edit carries (`MutationMeta.transaction` of its first op).
    pub transaction: Option<protocol::TransactionRef>,
    /// ✏️ The applied mutations of this row's edit (at most [`HISTORY_ROW_MUTATION_ROWS`]), durable outcomes only;
    /// a history edit overlays its replay on them when projected.
    pub mutations: Vec<MutationView>,
}

/// ✏️ One applied mutation of a history row: its replica-independent id, applied position and index in its edit,
/// localized label (of its effective input), durable outcome and supersession state, and whether history editing
/// may replace its inputs (`editable`) or withdraw it (`withdrawable`: the store's supersede law admits it, it is not
/// withdrawn yet and this is no viewer).
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct MutationView {
    pub mutation_id: String,
    pub position: u32,
    pub op_index: u32,
    pub label: LocalizedLabel,
    pub worst: Option<semio_framework_diagnostic::Severity>,
    pub messages: Vec<protocol::MutationMessage>,
    pub superseded: bool,
    pub withdrawn: bool,
    pub editable: bool,
    pub withdrawable: bool,
    pub store: Option<String>,
}

/// 📜️ Checkpoint/alternative history summary for this viewer — the swimlane columns, the
/// undo/redo availability, this viewer's head, and the merged command+operation timeline.
/// `active_alternative_id` and `current_checkpoint_id` are this replica's head. `alternatives`
/// stays the shared registration. Built once per store generation.
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct HistoryView {
    pub columns: Vec<HistoryColumn>,
    pub can_undo: bool,
    pub can_redo: bool,
    /// 👁 The alternative this viewer is on. `None` is the canonical trunk.
    pub active_alternative_id: Option<String>,
    /// 🌿️ Every alternative of the artifact's history, in declaration order ([`history_alternative_views`]).
    pub alternatives: Vec<AlternativeView>,
    /// 👁 The checkpoint this viewer is looking at: the tip of the active line, or an explicit checkout.
    pub current_checkpoint_id: Option<String>,
    /// 📜️ The session command log merged with live VCS op-text, newest first. Every edit in
    /// `envelope.vcs.edits` is referenced by exactly one entry (see `VcsArtifactApp::backfill_command_log`).
    pub commands: Vec<CommandView>,
    pub command_filter: HistoryCommandFilter,
}

impl HistoryView {
    /// 🕳️ An empty view for hand-built test/fixture `ArtifactView`s that don't exercise history.
    pub fn empty() -> Self {
        Self { columns: Vec::new(), can_undo: false, can_redo: false, active_alternative_id: None, alternatives: Vec::new(), current_checkpoint_id: None, commands: Vec::new(), command_filter: HistoryCommandFilter::default() }
    }
}

/// 🌿️ One alternative of the artifact's history as the history body lists it: its id and name, whether it is the
/// current one, who branched it and when (its `Branch` transition's actor and HLC wall time as RFC 3339 UTC; `None`
/// when the log holds no branch of it) and whether history edits were kept as it (scoped `Supersede` transitions).
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned)]
pub struct AlternativeView {
    pub id: String,
    pub name: String,
    pub current: bool,
    pub author: Option<String>,
    pub branched_at: Option<String>,
    pub edited: bool,
}

/// ⏪️ One undone shell row waiting on the redo stack — chrome-order counterpart to VCS redo.
pub struct ShellHistoryReplay {
    pub seq: u64,
    pub redo: InverseAction,
}

/// 🧾️ One appended session-command record — runtime-only, never persisted (the VCS envelope
/// is the persisted half; see `CommandView::op_lines`, derived live from `envelope.vcs.edits`).
/// Append-only: `VcsArtifactApp` only ever pushes to `command_log`, including for undo/redo.
#[derive(Clone, Debug, PartialEq)]
pub struct CommandLogEntry {
    pub seq: u64,
    pub action_id: String,
    pub label: LocalizedLabel,
    pub kind: ActionKind,
    pub timestamp: String,
    /// 🔗️ Set iff this command created/amended a DOCUMENT VCS edit — `None` for pure cursor motion (undo/redo/revert).
    /// A config-lane edit is never a history row (design §20.13, L4), so no row carries one and a config-only dispatch
    /// records nothing.
    pub edit_id: Option<String>,
    /// 🧩️ UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (C1): every CHILD document edit id this command's composite `dispatch_group`
    /// call produced — one entry per touched CHILD member of a single group dispatch (never folded — a group dispatch
    /// always issues a plain `Apply`, see `Emit::child_emits`'s own doc comment).
    pub child_edit_ids: Vec<String>,
    /// ✏️ Set iff this row is a `Supersede` history transition of the DOCUMENT store's event log (a history edit, or
    /// the undo or redo of one) — backfilled from the transitions, never from a live dispatch, so local, remote and
    /// reloaded history edits are rows alike (`VcsArtifactApp::backfill_command_log`).
    pub transition_id: Option<String>,
    /// 🔢️ How many consecutive identical `View`/`🐚️Shell` dispatches folded into this one row —
    /// see `VcsArtifactApp::record_command`. Always `1` for `Mutation`/`History`/`Clipboard` entries
    /// and for anything carrying an `edit_id`, which never fold.
    pub count: u32,
    /// ⏪️ A real inverse for a `🐚️Shell`-kind row with no `edit_id`
    /// (`noteShellCommand`-authored — shell-owned state this plugin cannot touch itself) — see
    /// `InverseAction`. `None` means this row has no working inverse (not just unauthored/foreign),
    /// which is the normal case for chrome the shell notes without declaring `inverseCommandId`
    /// (window activation/resize/move): such a row is logged but is never an undo target, so `undo`
    /// reaches the user's own document edit instead of replaying chrome into the guest.
    pub inverse: Option<InverseAction>,
}
