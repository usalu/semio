//! ⏪️ OS runtime of non-destructive history editing: the per-instance [`TimeTravelLedger`] hosting the pure
//! `⏪️time-travel` session reducer, the host-driven `historyEdit*` verbs, the session preview every render seam reads,
//! the Report replay stepped per reactor turn, the finalize commit, the per-mutation history overlay and the history
//! panel's time-travel sections.
//!
//! Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §4, §7 and §10.
//! Domain-neutral session: `semio_framework_time_travel`; store read API: `ArtifactStore::{state_before,
//! begin_report_replay, replay_report, commit_finished_replay}`.

use super::*;
use semio_framework::kernel::{HistoryMutationEntry, HistoryMutationMessage, HistoryTimeTravel, HistoryTimeTravelReview, HistoryTimeTravelStage, RequestId};
use semio_framework::{
    mutation_input_defs, mutation_input_instance, reference_id_text, registered_input_schema_document, ActionArgControl, ArgPresentation, ArgSchema, SnapSource, InputSchemaError, DIALOG_CHOICE_ARG, HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_ACTION_IDS, HISTORY_EDIT_ARG_GENERATION,
    HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_NAME, HISTORY_EDIT_ARG_PATH, HISTORY_EDIT_ARG_VALUE, HISTORY_EDIT_BACK_ACTION_ID, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, HISTORY_EDIT_CHOICE_OVERWRITE,
    HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_DISCARD_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID, HISTORY_EDIT_RERUN_ACTION_ID, HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_FINALIZE_DIALOG_ID, HISTORY_EDIT_INPUT_ACTION_ID, HISTORY_EDIT_USE_SELECTION_ACTION_ID,
    HISTORY_EDIT_WITHDRAW_ACTION_ID,
};
use semio_framework_time_travel::{is_time_travel_alternative_name, TimeTravelBase, TimeTravelChoice, TimeTravelEffect, TimeTravelEvent, TimeTravelLabel, TimeTravelRefusal, TimeTravelReview, TimeTravelSession, TimeTravelStage, TimeTravelTarget};
use std::collections::VecDeque;
use std::sync::Arc;

//#region 🔖️Limits
/// ⏱️ Wall budget of one replay slice per reactor turn.
pub const TIME_TRAVEL_TURN_WALL_US: u64 = 4_000;
/// 📢️ Least interval between two replay-progress deliveries (history body refresh plus `HistoryPatch.timeTravel`),
/// unless the replay advanced by [`TIME_TRAVEL_PROGRESS_REFRESH_PERMILLE`] of its total since the last one.
pub const TIME_TRAVEL_PROGRESS_REFRESH_MS: u64 = 100;
/// 📢️ Replay advance (per mille of its total) that delivers progress before [`TIME_TRAVEL_PROGRESS_REFRESH_MS`] elapsed.
pub const TIME_TRAVEL_PROGRESS_REFRESH_PERMILLE: u64 = 50;
/// ✏️ Mutation rows one history row carries: the first ones, then every one with messages or a supersession.
pub const HISTORY_ROW_MUTATION_ROWS: usize = 32;
/// 🧷️ Reference chips one input row shows; each costs a list and a map of the one `UiValue` arena page.
const TIME_TRAVEL_PANEL_CHIPS: usize = 8;
/// 🎛️ Input rows the draft editor materialises at most; a payload with more names the rest in one closing row.
pub(crate) const TIME_TRAVEL_EDITOR_INPUT_ROWS: usize = 64;
/// 🧹️ Discarded draft operations cold-retired per driver or close turn.
const TIME_TRAVEL_DISCARD_OPS_PER_TURN: usize = 64;
/// 🗳️ The request id the finalize prompt opens under; nothing awaits a dialog.
const TIME_TRAVEL_FINALIZE_REQUEST: RequestId = RequestId(0x7417_0001);
/// 🔒️ Plugin-owned refusal codes beside the session's own four.
const TIME_TRAVEL_BUSY_CODE: &str = "timeTravel.busy";
const TIME_TRAVEL_UNKNOWN_MUTATION_CODE: &str = "timeTravel.unknown-mutation";
const TIME_TRAVEL_NOT_EDITABLE_CODE: &str = "timeTravel.not-editable";
const TIME_TRAVEL_UNKNOWN_INPUT_CODE: &str = "timeTravel.unknown-input";
const TIME_TRAVEL_INVALID_INPUT_CODE: &str = "timeTravel.invalid-input";
const TIME_TRAVEL_NO_SELECTION_CODE: &str = "timeTravel.no-selection";
const TIME_TRAVEL_NAME_REQUIRED_CODE: &str = "timeTravel.name-required";
const TIME_TRAVEL_NAME_INVALID_CODE: &str = "timeTravel.name-invalid";
const TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE: &str = "timeTravel.schema-unavailable";
/// 🧯️ Fault codes the driver records on the session.
const TIME_TRAVEL_REPLAY_FAULTED_CODE: &str = "timeTravel.replay-faulted";
const TIME_TRAVEL_COMMIT_FAILED_CODE: &str = "timeTravel.commit-failed";
//#endregion 🔖️Limits

/// ⏪️ Whether `action` is one of the reserved, host-driven history-edit verbs.
pub fn is_time_travel_action_id(action: &str) -> bool {
    HISTORY_EDIT_ACTION_IDS.contains(&action)
}

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
        }
    }
}

/// 🎛️ Outcome of one history-edit verb: the stage it left the session in, or why it was refused (a silent no-op).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeTravelActionOutcome {
    Applied(TimeTravelStage),
    Rejected(TimeTravelActionRefusal),
}
//#endregion 🔖️Outcome

//#region 🔖️Ledger
/// ✏️ The mutation whose inputs are being drafted: its identity, label, input descriptors and the validator of its
/// payload schema, the operation whose kind the draft rebuilds, the current draft payload and the draft's own outcome
/// against the state before it.
pub struct TimeTravelEditor<A: ArtifactApp> {
    pub target: MutationId,
    pub position: u32,
    pub op_index: u32,
    pub label: LocalizedLabel,
    schema: &'static str,
    pub inputs: Vec<ActionArgDef>,
    pub inputs_refused: Option<InputSchemaError>,
    validator: Result<semio_framework_schema::OwnedJsonSchemaValidator, String>,
    kind: A::Mutation,
    pub value: DslValue,
    pub withdrawn: bool,
    pub outcome: Vec<protocol::MutationMessage>,
    pub refused: Option<(String, String)>,
}

impl<A: ArtifactApp> TimeTravelEditor<A> {
    /// 🧯️ Stands in for a payload schema that compiles no validator, for the fail-closed law.
    #[cfg(test)]
    pub(crate) fn refuse_validator(&mut self, reason: &str) {
        self.validator = Err(reason.to_string());
    }
}

/// 🕰️ The ephemeral local-only history edit of one document instance (design §4 and §7): the pure session, the draft
/// editor, the preview the renderer reads while editing (state before the target with the draft), the running Report
/// replay, its finished result (whose head is the reviewing preview and whose report finalizes without replaying
/// again), and the owners still retiring. Nothing here is persisted or shared; closing the instance discards it.
pub struct TimeTravelLedger<A: ArtifactApp> {
    session: TimeTravelSession,
    editor: Option<TimeTravelEditor<A>>,
    preview: Option<Arc<A::Snapshot>>,
    replay: Option<store::EditReplay<A::Snapshot, A::Mutation>>,
    finished: Option<store::EditReplayResult<A::Snapshot, A::Mutation>>,
    head: Option<Arc<A::Snapshot>>,
    retired_snapshots: Vec<Arc<A::Snapshot>>,
    snapshot_retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    discarded: Vec<A::Mutation>,
    closing: bool,
    ui_dirty: bool,
    document_dirty: bool,
    refreshed_ms: u64,
    refreshed_done: u32,
    patch_due: bool,
    patch: Option<HistoryPatch>,
}

impl<A: ArtifactApp> Default for TimeTravelLedger<A> {
    fn default() -> Self {
        Self {
            session: TimeTravelSession::default(),
            editor: None,
            preview: None,
            replay: None,
            finished: None,
            head: None,
            retired_snapshots: Vec::new(),
            snapshot_retirement: None,
            discarded: Vec::new(),
            closing: false,
            ui_dirty: false,
            document_dirty: false,
            refreshed_ms: 0,
            refreshed_done: 0,
            patch_due: false,
            patch: None,
        }
    }
}

impl<A: ArtifactApp> TimeTravelLedger<A> {
    /// 🕰️ The pure session state.
    pub fn session(&self) -> &TimeTravelSession {
        &self.session
    }

    /// ✏️ The draft editor while a mutation is being edited.
    pub fn editor(&self) -> Option<&TimeTravelEditor<A>> {
        self.editor.as_ref()
    }

    /// ✏️ The draft editor for a law that exercises a descriptor the fixture app's leaves do not declare.
    #[cfg(test)]
    pub(crate) fn editor_mut(&mut self) -> Option<&mut TimeTravelEditor<A>> {
        self.editor.as_mut()
    }

    /// 🚦️ Whether a history edit is open on this instance.
    pub fn is_active(&self) -> bool {
        self.session.stage != TimeTravelStage::Inactive
    }

    /// 🧊️ Artifact-lane app commands, owned-child edits, agent transactions and history-lane verbs are refused with
    /// `timeTravel.frozen` while a history edit is open; selection, view and remote ingest keep working.
    pub fn freezes_local_emits(&self) -> bool {
        self.is_active()
    }

    /// 🪞️ The document every render seam reads: while editing, the state before the edited mutation with its draft
    /// (downstream not applied); while reviewing, the replayed head; otherwise the tool run overlay or `committed`.
    /// A poll, a command and a context menu keep deciding over what landed, never over this preview.
    pub fn render_snapshot_or<'a>(&'a self, tool_runs: &'a ToolRunLedger<A>, committed: &'a Arc<A::Snapshot>) -> &'a Arc<A::Snapshot> {
        match self.session.stage {
            TimeTravelStage::Inactive => tool_runs.overlay_or(committed),
            TimeTravelStage::Editing | TimeTravelStage::Replaying => self.preview.as_ref().unwrap_or(committed),
            TimeTravelStage::Reviewing | TimeTravelStage::Choosing | TimeTravelStage::Finalizing => self.head.as_ref().unwrap_or(committed),
        }
    }

    /// 🏃️ Whether a driver turn has work: a live replay, retirement, an owed UI scope or an owed history patch.
    pub fn has_pending_work(&self) -> bool {
        (self.replay.is_some() && self.session.stage == TimeTravelStage::Replaying) || !self.retired_snapshots.is_empty() || self.snapshot_retirement.is_some() || !self.discarded.is_empty() || self.is_ui_dirty() || self.patch_due || self.patch.is_some()
    }

    /// 🚩️ Whether a session change still owes the host a UI scope.
    pub fn is_ui_dirty(&self) -> bool {
        self.ui_dirty || self.document_dirty
    }

    /// 🧾️ The history patch a driver turn prepared for the next unsolicited UI progress frame, so replay progress
    /// and stage changes reach every host's band without a dispatch.
    pub fn take_patch(&mut self) -> Option<HistoryPatch> {
        self.patch.take()
    }

    /// 🧾️ Whether a prepared history patch still waits for its unsolicited UI progress frame.
    pub fn has_patch(&self) -> bool {
        self.patch.is_some()
    }

    /// 🎯️ The scope a session change dirties: every body once when the rendered document was swapped, else the
    /// history body and the per-window engagement chips.
    pub fn dirty_scope(&self) -> UiDirtyScope {
        if self.document_dirty {
            return UiDirtyScope::Full;
        }
        UiDirtyScope::Partial { window_bodies: Vec::new(), panel_bodies: vec![FRAMEWORK_HISTORY_BODY_KEY.to_string()], utilities: false, tools: false, engagements: true, measures: false, labels: false }
    }

    /// ✍️ The accepted drafts keyed by target, the shape every store read takes.
    fn accepted_drafts(&self) -> BTreeMap<MutationId, protocol::InputReplacement> {
        self.session.accepted.iter().map(|draft| (draft.target.mutation.clone(), draft.replacement.clone())).collect()
    }

    fn retire_snapshot(&mut self, snapshot: Option<Arc<A::Snapshot>>) {
        self.retired_snapshots.extend(snapshot);
    }

    fn replace_editor(&mut self, editor: Option<TimeTravelEditor<A>>) {
        if let Some(previous) = std::mem::replace(&mut self.editor, editor) {
            self.discarded.push(previous.kind);
        }
    }

    /// 🧽️ Drops what the stage no longer shows: the editor outside `Editing`, the draft preview once the replay
    /// finished or the session left, every replay owner once it is inactive.
    fn settle_owners(&mut self) {
        let stage = self.session.stage;
        if stage != TimeTravelStage::Editing {
            self.replace_editor(None);
        }
        if !matches!(stage, TimeTravelStage::Editing | TimeTravelStage::Replaying) {
            let preview = self.preview.take();
            self.retire_snapshot(preview);
        }
        if stage != TimeTravelStage::Replaying {
            self.replay = None;
        }
        if stage == TimeTravelStage::Inactive {
            self.finished = None;
            let head = self.head.take();
            self.retire_snapshot(head);
        }
    }

    /// 🧹️ Advances the owners that outlived their slot by one bounded unit; `None` when nothing is retiring.
    fn retire_step(&mut self, store: &mut ArtifactStore<A::Snapshot, A::Mutation>, maximum_items: usize, maximum_bytes: usize) -> Result<Option<PluginCloseStep>, Fault> {
        if let Some(retirement) = self.snapshot_retirement.as_mut() {
            return match retirement.close_step(maximum_items.max(1), maximum_bytes).map_err(plugin_sdk_fault)? {
                store::SnapshotRetirementStep::Complete if retirement.terminal_is_empty() => {
                    self.snapshot_retirement = None;
                    Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }))
                }
                store::SnapshotRetirementStep::Complete => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("timeTravel.snapshot-close"), "time travel snapshot retirement closed without its terminal-empty witness")),
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(Some(PluginCloseStep::Pending { released_items, released_bytes })),
                store::SnapshotRetirementStep::Blocked => Ok(Some(PluginCloseStep::Blocked { reason: "time travel snapshot retirement is blocked" })),
            };
        }
        if let Some(alias) = self.retired_snapshots.pop() {
            self.snapshot_retirement = Some(store.retire_snapshot_alias(alias).map_err(|error| error.into_fault())?);
            return Ok(Some(PluginCloseStep::Pending { released_items: 1, released_bytes: 0 }));
        }
        if !self.discarded.is_empty() {
            let count = self.discarded.len().min(maximum_items.max(1)).min(TIME_TRAVEL_DISCARD_OPS_PER_TURN);
            for op in self.discarded.drain(self.discarded.len() - count..) {
                op.retire_cold();
            }
            return Ok(Some(PluginCloseStep::Pending { released_items: count, released_bytes: 0 }));
        }
        Ok(None)
    }

    /// 🚪️ Document close, instance retirement or reload: the session vanishes with every draft (ephemeral state).
    pub fn begin_close(&mut self) {
        if self.closing {
            return;
        }
        self.closing = true;
        self.session = TimeTravelSession::new(self.session.base);
        self.settle_owners();
        self.ui_dirty = false;
        self.document_dirty = false;
        self.patch_due = false;
        self.patch = None;
    }

    /// 🧹️ One bounded close unit of every retiring owner.
    pub fn close_step(&mut self, store: &mut ArtifactStore<A::Snapshot, A::Mutation>, maximum_items: usize, maximum_bytes: usize) -> Result<PluginCloseStep, Fault> {
        self.begin_close();
        Ok(self.retire_step(store, maximum_items, maximum_bytes)?.unwrap_or(PluginCloseStep::Complete))
    }

    pub fn terminal_is_empty(&self) -> bool {
        !self.is_active()
            && self.editor.is_none()
            && self.preview.is_none()
            && self.replay.is_none()
            && self.finished.is_none()
            && self.head.is_none()
            && self.retired_snapshots.is_empty()
            && self.snapshot_retirement.is_none()
            && self.discarded.is_empty()
    }

    /// ⏪️ The session status every host renders its band from (design §10); `None` while inactive.
    pub fn status(&self) -> Option<HistoryTimeTravel> {
        let session = &self.session;
        let stage = match session.stage {
            TimeTravelStage::Inactive => return None,
            TimeTravelStage::Editing => HistoryTimeTravelStage::Editing,
            TimeTravelStage::Replaying => HistoryTimeTravelStage::Replaying,
            TimeTravelStage::Reviewing => HistoryTimeTravelStage::Reviewing,
            TimeTravelStage::Choosing => HistoryTimeTravelStage::Choosing,
            TimeTravelStage::Finalizing => HistoryTimeTravelStage::Finalizing,
        };
        Some(HistoryTimeTravel {
            session_id: session.id.to_string(),
            generation: session.generation,
            stage,
            target: session.pending.as_ref().map(|pending| pending.target.mutation.0.clone()),
            target_label: self.editor.as_ref().map(|editor| editor.label.clone()),
            done: session.progress.map(|progress| progress.done),
            total: session.progress.map(|progress| progress.total),
            worst: session.report.as_ref().and_then(|report| report.worst),
            blocking: session.report.as_ref().is_some_and(protocol::ReplayReport::blocks_finalize),
            fault: session.fault.clone(),
            accepted_count: u32::try_from(session.accepted.len()).unwrap_or(u32::MAX),
            review: session.review().map(|review| match review {
                TimeTravelReview::NoChanges => HistoryTimeTravelReview::NoChanges,
                TimeTravelReview::NeedsReplay => HistoryTimeTravelReview::NeedsReplay,
                TimeTravelReview::Blocked => HistoryTimeTravelReview::Blocked,
                TimeTravelReview::Ready => HistoryTimeTravelReview::Ready,
            }),
            rerunnable: session.rerun_refusal().is_none(),
        })
    }

    /// 👥️ The ephemeral shared summary of the open session a host stamps on `PresencePeer.history_edit`: the mutation
    /// being edited (else the newest accepted draft's), the stage and the accepted draft count; `None` while inactive.
    pub fn presence(&self) -> Option<protocol::PresenceHistoryEdit> {
        let session = &self.session;
        let stage = match session.stage {
            TimeTravelStage::Inactive => return None,
            TimeTravelStage::Editing => protocol::PresenceHistoryEditStage::Editing,
            TimeTravelStage::Replaying => protocol::PresenceHistoryEditStage::Replaying,
            TimeTravelStage::Reviewing => protocol::PresenceHistoryEditStage::Reviewing,
            TimeTravelStage::Choosing => protocol::PresenceHistoryEditStage::Choosing,
            TimeTravelStage::Finalizing => protocol::PresenceHistoryEditStage::Finalizing,
        };
        let mutation_id = self.editor.as_ref().map(|editor| editor.target.0.clone()).or_else(|| session.accepted.last().map(|draft| draft.target.mutation.0.clone()))?;
        Some(protocol::PresenceHistoryEdit { mutation_id, stage, drafts: u32::try_from(session.accepted.len()).unwrap_or(u32::MAX) })
    }

    /// 🪧️ The plain-data view the history panel and the history wire read; `None` while inactive.
    pub fn panel(&self) -> Option<TimeTravelPanel> {
        let status = self.status()?;
        let session = &self.session;
        let outcomes = session.report.as_ref().map(|report| report.outcomes.iter().map(|outcome| (outcome.mutation_id.0.clone(), outcome.clone())).collect()).unwrap_or_default();
        let mut edited: BTreeSet<String> = session.accepted.iter().map(|draft| draft.target.mutation.0.clone()).collect();
        if let Some(pending) = session.pending.as_ref().filter(|pending| !session.unchanged(pending)) {
            edited.insert(pending.target.mutation.0.clone());
        }
        let next_problem = session.report.as_ref().and_then(|report| report.outcomes.iter().find(|outcome| outcome.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst)))).map(|outcome| outcome.mutation_id.0.clone());
        let editor = self.editor.as_ref().map(|editor| TimeTravelEditorPanel {
            target: editor.target.0.clone(),
            label: editor.label.clone(),
            rows: time_travel_input_rows(&editor.inputs, &editor.value),
            inputs_refused: editor.inputs_refused.as_ref().map(|error| error.detail.clone()),
            withdrawn: editor.withdrawn,
            outcome: editor.outcome.clone(),
            refused: editor.refused.clone(),
            changed: session.pending.as_ref().is_some_and(|pending| !session.unchanged(pending)),
        });
        Some(TimeTravelPanel {
            status,
            stage: session.stage,
            pending_after: session.pending.as_ref().and(self.editor.as_ref()).map(|editor| (editor.position, editor.op_index)),
            finalize_refusal: session.finalize_refusal(),
            review: session.review(),
            rerun_refusal: session.rerun_refusal(),
            next_problem,
            editor,
            outcomes,
            edited,
        })
    }
}
//#endregion 🔖️Ledger

//#region 🔖️PanelView
/// ✏️ The draft editor as the panel renders it: its rows ([`time_travel_input_rows`]) over the current draft.
#[derive(Clone, Debug, PartialEq)]
pub struct TimeTravelEditorPanel {
    pub target: String,
    pub label: LocalizedLabel,
    pub rows: Vec<TimeTravelInputRow>,
    pub inputs_refused: Option<String>,
    pub withdrawn: bool,
    pub outcome: Vec<protocol::MutationMessage>,
    pub refused: Option<(String, String)>,
    pub changed: bool,
}

/// 🪧️ The live session as the history panel and the history wire read it: the band status, which mutations are
/// pending (downstream of the edited one while editing) or edited, the replay outcomes that override the durable
/// ones, why finalizing is refused, the first blocking mutation, and the draft editor.
#[derive(Clone, Debug, PartialEq)]
pub struct TimeTravelPanel {
    pub status: HistoryTimeTravel,
    pub stage: TimeTravelStage,
    pub pending_after: Option<(u32, u32)>,
    pub finalize_refusal: Option<TimeTravelRefusal>,
    pub review: Option<TimeTravelReview>,
    pub rerun_refusal: Option<TimeTravelRefusal>,
    pub next_problem: Option<String>,
    pub editor: Option<TimeTravelEditorPanel>,
    pub outcomes: BTreeMap<String, protocol::MutationReplayOutcome>,
    pub edited: BTreeSet<String>,
}

/// 🧾️ One mutation row on the history wire: the durable row with the session overlay (replay outcome, pending,
/// edited) when a history edit is open.
pub fn history_mutation_entry(view: &MutationView, panel: Option<&TimeTravelPanel>) -> HistoryMutationEntry {
    let mut worst = view.worst;
    let mut messages = &view.messages;
    let (mut superseded, mut withdrawn) = (view.superseded, view.withdrawn);
    let (mut pending, mut edited) = (false, false);
    if let Some(panel) = panel {
        if let Some(outcome) = panel.outcomes.get(&view.mutation_id) {
            worst = outcome.worst;
            messages = &outcome.messages;
            superseded = outcome.superseded;
            withdrawn = outcome.withdrawn;
        }
        pending = panel.pending_after.is_some_and(|after| (view.position, view.op_index) > after);
        edited = panel.edited.contains(&view.mutation_id);
    }
    HistoryMutationEntry {
        mutation_id: view.mutation_id.clone(),
        position: view.position,
        op_index: view.op_index,
        label: view.label.clone(),
        worst,
        messages: messages.iter().map(|message| HistoryMutationMessage { level: message.level, code: message.code.0.clone(), message: message.message.clone(), target: message.target.clone(), op_index: message.op_index }).collect(),
        superseded,
        withdrawn,
        editable: view.editable,
        pending,
        edited,
    }
}
//#endregion 🔖️PanelView

//#region 🔖️Pointer
/// 🧭️ The value at RFC 6901 `pointer` inside `value`.
pub(crate) fn time_travel_pointer_get<'a>(value: &'a DslValue, pointer: &str) -> Option<&'a DslValue> {
    let Some(rest) = pointer.strip_prefix('/') else { return (pointer.is_empty()).then_some(value) };
    let mut cursor = value;
    for segment in rest.split('/').map(|segment| segment.replace("~1", "/").replace("~0", "~")) {
        cursor = match cursor {
            DslValue::Object(entries) => &entries.iter().find(|(key, _)| *key == segment)?.1,
            DslValue::Array(items) => items.get(segment.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(cursor)
}

/// 🧭️ Replaces the value at RFC 6901 `pointer` inside `value` (an absent last object member or the next array slot is
/// created); `false` when the pointer does not address a slot.
pub(crate) fn time_travel_pointer_set(value: &mut DslValue, pointer: &str, replacement: DslValue) -> bool {
    let Some(rest) = pointer.strip_prefix('/') else { return false };
    let segments: Vec<String> = rest.split('/').map(|segment| segment.replace("~1", "/").replace("~0", "~")).collect();
    let mut cursor = value;
    for (index, segment) in segments.iter().enumerate() {
        let last = index + 1 == segments.len();
        cursor = match cursor {
            DslValue::Object(entries) => match entries.iter().position(|(key, _)| key == segment) {
                Some(position) => &mut entries[position].1,
                None if last => {
                    entries.push((segment.clone(), DslValue::Null));
                    &mut entries.last_mut().expect("just pushed").1
                }
                None => return false,
            },
            DslValue::Array(items) => {
                let Ok(position) = segment.parse::<usize>() else { return false };
                if position == items.len() && last {
                    items.push(DslValue::Null);
                }
                let Some(item) = items.get_mut(position) else { return false };
                item
            }
            _ => return false,
        };
    }
    *cursor = replacement;
    true
}

/// 🔀️ The variant selector of a union payload's inputs: the ungrouped string input whose options name the group of
/// every grouped input (the reader groups each variant's fields by the variant's value); `None` for a payload that is
/// no union, whose `group`s only lay its inputs out.
pub(crate) fn time_travel_variant_selector(inputs: &[ActionArgDef]) -> Option<&ActionArgDef> {
    let groups: BTreeSet<&str> = inputs.iter().filter_map(|input| input.group.as_deref()).collect();
    if groups.is_empty() {
        return None;
    }
    inputs.iter().filter(|input| input.group.is_none()).find(|input| matches!(&input.schema, ArgSchema::String { options, .. } if groups.iter().all(|group| options.iter().any(|option| option.value == *group))))
}

/// 🔀️ The inputs of the variant `value` is in: the selector, the ungrouped inputs and the active variant's fields; every
/// input of a payload that is no union.
pub(crate) fn time_travel_active_inputs<'a>(inputs: &'a [ActionArgDef], value: &DslValue) -> Vec<&'a ActionArgDef> {
    let Some(selector) = time_travel_variant_selector(inputs) else { return inputs.iter().collect() };
    let active = time_travel_pointer_get(value, &selector.id).and_then(DslValue::as_str);
    inputs.iter().filter(|input| input.group.is_none() || input.group.as_deref() == active).collect()
}

/// 🎛️ The input descriptor RFC 6901 `pointer` addresses in `value`, resolved against the variant `value` is in: object
/// members and array items recurse to any depth (an array item is described by the array's item schema), and a vector
/// or reference list component is one element of its input (`true`).
pub(crate) fn time_travel_input_at(inputs: &[ActionArgDef], value: &DslValue, pointer: &str) -> Option<(ActionArgDef, bool)> {
    let rest = pointer.strip_prefix('/')?;
    let mut segments = rest.split('/').map(|segment| segment.replace("~1", "/").replace("~0", "~"));
    let first = segments.next()?;
    let mut input = time_travel_active_inputs(inputs, value).into_iter().find(|input| input.key() == first)?.clone();
    while let Some(segment) = segments.next() {
        input = match &input.schema {
            ArgSchema::Object { fields } => fields.iter().find(|field| field.key() == segment)?.clone(),
            ArgSchema::Vector { .. } | ArgSchema::Reference { many: true, .. } => {
                segment.parse::<usize>().ok()?;
                return segments.next().is_none().then_some((input, true));
            }
            ArgSchema::Array { items, .. } => {
                segment.parse::<usize>().ok()?;
                ActionArgDef { id: format!("/{segment}"), schema: items.as_ref().clone(), required: true, nullable: false, default: None, ..input.clone() }
            }
            _ => return None,
        };
    }
    Some((input, false))
}

/// 🎛️ One row of the draft editor: the RFC 6901 pointer it writes, its label (with the path of labels and item numbers
/// that lead to it), its descriptor and its current value.
#[derive(Clone, Debug, PartialEq)]
pub struct TimeTravelInputRow {
    pub pointer: String,
    pub label: LocalizedLabel,
    pub input: ActionArgDef,
    pub value: DslValue,
}

/// 🎛️ The draft editor's rows over `value`: the variant selector and every input of the variant `value` is in, objects
/// flattened into their fields and arrays of objects into one row per field of every item `value` holds; a hidden input
/// is never a row.
pub(crate) fn time_travel_input_rows(inputs: &[ActionArgDef], value: &DslValue) -> Vec<TimeTravelInputRow> {
    fn nested(parent: &LocalizedLabel, child: &LocalizedLabel, item: Option<usize>) -> LocalizedLabel {
        LocalizedLabel::from_fn(|terminology, locale| {
            let parent = parent.resolve(terminology, locale);
            let child = child.resolve(terminology, locale);
            match item {
                Some(index) => format!("{parent} {} \u{b7} {child}", index + 1),
                None => format!("{parent} \u{b7} {child}"),
            }
        })
    }
    fn push(input: &ActionArgDef, pointer: String, label: LocalizedLabel, value: &DslValue, rows: &mut Vec<TimeTravelInputRow>) {
        if input.presentation == Some(ArgPresentation::Hidden) {
            return;
        }
        let current = time_travel_pointer_get(value, &pointer);
        match &input.schema {
            ArgSchema::Object { fields } if !fields.is_empty() => {
                for field in fields {
                    push(field, format!("{pointer}{}", field.id), nested(&label, &field.label, None), value, rows);
                }
            }
            ArgSchema::Array { items, .. } if matches!(items.as_ref(), ArgSchema::Object { fields } if !fields.is_empty()) => {
                let ArgSchema::Object { fields } = items.as_ref() else { return };
                for index in 0..current.and_then(DslValue::as_array).map_or(0, <[DslValue]>::len) {
                    for field in fields {
                        push(field, format!("{pointer}/{index}{}", field.id), nested(&label, &field.label, Some(index)), value, rows);
                    }
                }
            }
            _ => rows.push(TimeTravelInputRow { pointer, label, input: input.clone(), value: current.cloned().unwrap_or(DslValue::Null) }),
        }
    }
    let mut rows = Vec::new();
    for input in time_travel_active_inputs(inputs, value) {
        push(input, input.id.clone(), input.label.clone(), value, &mut rows);
    }
    rows
}

/// 🔀️ `value` switched to union variant `variant` (its selector `selector`): the selector set, every member only other
/// variants declare removed, and every absent field of `variant` filled from its declared default.
pub(crate) fn time_travel_switch_variant(inputs: &[ActionArgDef], selector: &ActionArgDef, value: &DslValue, variant: &str) -> DslValue {
    let DslValue::Object(entries) = value else { return value.clone() };
    let declared = |key: &str| inputs.iter().any(|input| input.key() == key);
    let kept = |key: &str| inputs.iter().any(|input| input.key() == key && (input.group.is_none() || input.group.as_deref() == Some(variant)));
    let mut entries: Vec<(String, DslValue)> = entries.iter().filter(|(key, _)| kept(key) || !declared(key)).cloned().collect();
    for input in inputs.iter().filter(|input| input.group.as_deref() == Some(variant)) {
        if let Some(default) = input.default.as_ref().filter(|_| !entries.iter().any(|(key, _)| *key == input.key())) {
            entries.push((input.key(), default.clone()));
        }
    }
    let mut switched = DslValue::Object(entries);
    time_travel_pointer_set(&mut switched, &selector.id, DslValue::String(variant.to_string()));
    switched
}

/// 🧲️ Makes every `snapSource` of `rows` concrete: `spacing` answers the grid a `Config` or `Snapshot` source names
/// ([`time_travel_apply_snap_spacing`]).
pub(crate) fn time_travel_resolve_snaps(rows: &mut [TimeTravelInputRow], mut spacing: impl FnMut(&SnapSource) -> Option<f64>) {
    for row in rows {
        let source = match &row.input.schema {
            ArgSchema::Number { snap_source: Some(source), .. } | ArgSchema::Vector { snap_source: Some(source), .. } => source.clone(),
            _ => continue,
        };
        let resolved = match &source {
            SnapSource::Step => None,
            other => spacing(other),
        };
        time_travel_apply_snap_spacing(&mut row.input.schema, resolved);
    }
}

/// 🔢️ A host value read as a number: numbers as they are, numeric text parsed; an integer input rounds.
fn time_travel_number(value: &DslValue, integer: bool) -> Option<DslValue> {
    let number = value.as_f64().or_else(|| value.as_str().and_then(|text| text.trim().parse::<f64>().ok())).filter(|number| number.is_finite())?;
    Some(DslValue::json_number(if integer { number.round() } else { number }))
}

/// 🎛️ Coerces one host-dispatched control value into the shape `input` declares (an element of it when `element`); `null`
/// clears a nullable input.
pub(crate) fn time_travel_coerce(input: &ActionArgDef, element: bool, value: &DslValue) -> Option<DslValue> {
    let text = |value: &DslValue| value.as_str().map(|text| DslValue::String(text.to_string()));
    if !element && input.nullable && value.is_null() {
        return Some(DslValue::Null);
    }
    match (&input.schema, element) {
        (ArgSchema::Number { integer, .. }, false) => time_travel_number(value, *integer),
        (ArgSchema::Vector { .. }, true) => time_travel_number(value, false),
        (ArgSchema::Vector { dims, .. }, false) if input.presentation == Some(ArgPresentation::Color) && value.as_str().is_some() => {
            let components = time_travel_color_components(value.as_str()?)?;
            (components.len() == 3 || components.len() == *dims as usize).then(|| DslValue::Array(components.into_iter().map(DslValue::float).collect()))
        }
        (ArgSchema::Vector { dims, .. }, false) => {
            let items = value.as_array()?;
            (items.len() == *dims as usize).then(|| items.iter().map(|item| time_travel_number(item, false)).collect::<Option<Vec<_>>>().map(DslValue::Array)).flatten()
        }
        (ArgSchema::Boolean, false) => value.as_bool().or_else(|| value.as_str().and_then(|text| text.parse().ok())).map(DslValue::Bool),
        (ArgSchema::String { .. }, false) => text(value),
        (ArgSchema::Reference { many: false, id_type, .. }, false) | (ArgSchema::Reference { many: true, id_type, .. }, true) => reference_id_text(value).and_then(|id| id_type.id_value(&id)),
        (ArgSchema::Reference { many: true, id_type, .. }, false) => match value {
            DslValue::Array(items) => items.iter().map(|item| reference_id_text(item).and_then(|id| id_type.id_value(&id))).collect::<Option<Vec<_>>>().map(DslValue::Array),
            other => reference_id_text(other).and_then(|id| id_type.id_value(&id)).map(|id| DslValue::Array(vec![id])),
        },
        _ => Some(value.clone()),
    }
}

/// 🎨️ The sRGB components (`0..=1`) of a colour text by the UI contract's colour law ([`parse_ui_color_hex`]): three
/// for a text without alpha (the draft keeps the stored alpha), four for one with it.
pub(crate) fn time_travel_color_components(text: &str) -> Option<Vec<f64>> {
    let rgba = parse_ui_color_hex(text)?;
    let digits = text.trim().trim_start_matches('#').len();
    Some(if digits == 4 || digits == 8 { rgba.to_vec() } else { rgba[..3].to_vec() })
}

/// 🎨️ The components of a colour value, as many as it holds.
fn time_travel_color_rgba(value: &DslValue) -> Vec<f64> {
    value.as_array().map(|items| items.iter().map(|item| item.as_f64().unwrap_or(0.0)).collect()).unwrap_or_default()
}

/// 🧲️ Makes a resolved snap `spacing` concrete on one number or vector schema: the multiples of it inside the travel
/// range become detents when at most [`UI_FIXED_LIST_ITEMS`] of them fit, else the spacing becomes the step. A `Step`
/// source is left to the control; a resolved or unresolvable `Config`/`Snapshot` source is cleared, so a host only ever
/// sees numbers.
pub(crate) fn time_travel_apply_snap_spacing(schema: &mut ArgSchema, spacing: Option<f64>) {
    let (snaps, snap_source, step, low, high) = match schema {
        ArgSchema::Number { snaps, snap_source, step, min, max, soft_min, soft_max, .. } => (snaps, snap_source, step, soft_min.or(*min), soft_max.or(*max)),
        ArgSchema::Vector { snaps, snap_source, step, min, max, .. } => (snaps, snap_source, step, *min, *max),
        _ => return,
    };
    if matches!(snap_source, None | Some(SnapSource::Step)) {
        return;
    }
    *snap_source = None;
    let Some(spacing) = spacing.filter(|spacing| spacing.is_finite() && *spacing > 0.0) else { return };
    let detents = low.zip(high).filter(|(low, high)| low <= high && ((high - low) / spacing).floor() < UI_FIXED_LIST_ITEMS as f64).map(|(low, high)| ((low / spacing).ceil() as i64..=(high / spacing).floor() as i64).map(|index| index as f64 * spacing).collect::<Vec<_>>());
    match detents {
        Some(detents) => {
            snaps.extend(detents);
            snaps.sort_by(f64::total_cmp);
            snaps.dedup();
            snaps.truncate(UI_FIXED_LIST_ITEMS);
        }
        None => *step = Some(spacing),
    }
}

/// 🔗️ The JSON texts of every document `schema_json` references by `$id` (transitively), for the payload validator.
fn time_travel_schema_documents(schema_json: &str) -> Vec<String> {
    fn references(value: &DslValue, into: &mut Vec<String>) {
        match value {
            DslValue::Object(entries) => {
                for (key, value) in entries {
                    match (key.as_str(), value) {
                        ("$ref", DslValue::String(reference)) if !reference.starts_with('#') => into.push(reference.split('#').next().unwrap_or_default().to_string()),
                        _ => references(value, into),
                    }
                }
            }
            DslValue::Array(items) => items.iter().for_each(|item| references(item, into)),
            _ => {}
        }
    }
    let Ok(root) = dsl::os_pack::json::parse(schema_json) else { return Vec::new() };
    let mut pending = Vec::new();
    references(&dsl::os_pack::json::to_dsl_value(&root), &mut pending);
    let mut seen = BTreeSet::new();
    let mut documents = Vec::new();
    while let Some(id) = pending.pop() {
        if id.is_empty() || !seen.insert(id.clone()) {
            continue;
        }
        let Some(document) = registered_input_schema_document(&id) else { continue };
        references(&document, &mut pending);
        documents.push(dsl::os_pack::json::to_string(&dsl::os_pack::json::from_dsl_value(&document)));
    }
    documents
}

/// 🧾️ The JSON text of `value`.
fn time_travel_json(value: &DslValue) -> String {
    dsl::os_pack::json::to_string(&dsl::os_pack::json::from_dsl_value(value))
}
//#endregion 🔖️Pointer

//#region 🔖️Driver
/// 🧿️ The session generation a verb names, in every shape the wire hands it over; `None` addresses the live session.
fn time_travel_arg_generation(args: Option<&DslValue>) -> Option<u32> {
    tool_run::tool_run_arg_u64(args, HISTORY_EDIT_ARG_GENERATION).and_then(|generation| u32::try_from(generation).ok())
}

fn time_travel_arg_text<'a>(args: Option<&'a DslValue>, key: &str) -> Option<&'a str> {
    args?.get(key)?.as_str()
}

/// 🏷️ The history label of one document operation: the app's localized kind label, else its text line (data).
pub(crate) fn time_travel_mutation_label<A: ArtifactApp>(op: &A::Mutation) -> LocalizedLabel {
    A::mutation_label(op).unwrap_or_else(|| LocalizedLabel::data(UiText::clipped(&op.print_op()).as_str()))
}

/// ✏️ Whether history editing may replace this operation's inputs: it declares an input schema and plans no foreign steps.
pub(crate) fn time_travel_editable<A: ArtifactApp>(op: &A::Mutation) -> bool {
    !op.may_emit_foreign_steps() && op.input_schema().is_some()
}

/// ✏️ The mutation rows of one edit's applied `ops` (in op order): the first [`HISTORY_ROW_MUTATION_ROWS`], then every
/// later one carrying messages or a supersession, labelled from their effective input (a label of `labelled`, the
/// previous projection, is reused for an operation that was not superseded) with their durable outcome. `superseded`
/// and `withdrawn` read the store's effective supersession: an input restored to its original (the undo of a history
/// edit) is not superseded.
pub(crate) fn history_mutation_views<A: ArtifactApp>(ops: &[&store::AppliedMutation<'_, A::Mutation>], outcomes: &HashMap<&str, &protocol::MutationReplayOutcome>, labelled: &HashMap<&str, &MutationView>) -> Vec<MutationView> {
    let mut views = Vec::new();
    for (index, op) in ops.iter().enumerate() {
        let id = op.mutation_id.0.as_str();
        let outcome = outcomes.get(id);
        let flagged = op.supersession.is_some() || outcome.is_some_and(|outcome| outcome.worst.is_some());
        if index >= HISTORY_ROW_MUTATION_ROWS && (!flagged || views.len() >= 2 * HISTORY_ROW_MUTATION_ROWS) {
            continue;
        }
        let effective = match op.supersession.map(|supersession| &supersession.replacement) {
            Some(protocol::InputReplacement::Input { payload, .. }) => <A::Mutation as ::protocol::OpBinary>::decode_op(payload).ok(),
            _ => None,
        };
        let shown = effective.as_ref().unwrap_or(op.operation);
        let label = match labelled.get(id).filter(|_| op.supersession.is_none()) {
            Some(previous) => previous.label.clone(),
            None => time_travel_mutation_label::<A>(shown),
        };
        let editable = A::ROLE != AppRole::Viewer && time_travel_editable::<A>(shown);
        let superseded = match op.supersession.map(|supersession| &supersession.replacement) {
            None => false,
            Some(protocol::InputReplacement::Withdrawn) => true,
            Some(protocol::InputReplacement::Input { payload, .. }) => <A::Mutation as ::protocol::OpBinary>::encode_op(op.operation).map_or(true, |original| original != *payload),
        };
        if let Some(effective) = effective {
            effective.retire_cold();
        }
        views.push(MutationView {
            mutation_id: id.to_string(),
            position: u32::try_from(op.position).unwrap_or(u32::MAX),
            op_index: op.op_index,
            label,
            worst: outcome.and_then(|outcome| outcome.worst),
            messages: outcome.map(|outcome| outcome.messages.clone()).unwrap_or_default(),
            superseded,
            withdrawn: op.supersession.is_some_and(|supersession| supersession.replacement == protocol::InputReplacement::Withdrawn),
            editable,
        });
    }
    views
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🧭️ The store state a session observes.
    fn time_travel_base(&self) -> TimeTravelBase {
        TimeTravelBase { store_generation: self.store.generation(), content_revision: self.store.content_revision() }
    }

    /// ✏️ The mutation rows of edit `edit_id` against the live store, reusing the labels of `previous` rows.
    pub(crate) fn history_edit_mutation_views(&self, edit_id: &str, previous: &[MutationView]) -> Vec<MutationView> {
        let applied_ops = self.store.mutation_ops().unwrap_or_default();
        let ops: Vec<&store::AppliedMutation<'_, A::Mutation>> = applied_ops.iter().filter(|op| op.edit_id == edit_id).collect();
        let durable = self.store.mutation_outcomes().unwrap_or_default();
        let outcomes: HashMap<&str, &protocol::MutationReplayOutcome> = durable.iter().filter(|outcome| outcome.edit_id == edit_id).map(|outcome| (outcome.mutation_id.0.as_str(), outcome)).collect();
        let labelled: HashMap<&str, &MutationView> = previous.iter().filter(|mutation| !mutation.superseded).map(|mutation| (mutation.mutation_id.as_str(), mutation)).collect();
        history_mutation_views::<A>(&ops, &outcomes, &labelled)
    }

    /// 🧲️ Resolves every editor row's `snapSource` to concrete numbers before the panel is built
    /// ([`time_travel_resolve_snaps`]): a `Config` key reads the focused window's config, then the app config; a
    /// `Snapshot` pointer reads the document the session previews.
    pub(crate) fn resolve_time_travel_snaps(&self, panel: &mut TimeTravelPanel, view: &ViewModel) {
        let Some(editor) = panel.editor.as_mut() else { return };
        let config = std::cell::OnceCell::new();
        let preview = std::cell::OnceCell::new();
        let window = view.focused_window_id.as_deref().and_then(|id| view.window_instances.iter().find(|window| window.id == id));
        time_travel_resolve_snaps(&mut editor.rows, |source| match source {
            SnapSource::Step => None,
            SnapSource::Config { key } => {
                let pointer = if key.starts_with('/') { key.clone() } else { format!("/{key}") };
                let windowed = window.and_then(|window| self.window_config_store.pointer_values(&window.window_kind_id, std::slice::from_ref(&pointer)).into_iter().find(|(window_id, _)| *window_id == window.id)).and_then(|(_, values)| values.into_iter().next().flatten()).and_then(|value| value.as_f64());
                windowed.or_else(|| time_travel_pointer_get(config.get_or_init(|| protocol::ToValue::to_value(self.config_store.snapshot_owner().as_ref())), &pointer).and_then(DslValue::as_f64))
            }
            SnapSource::Snapshot { pointer } => {
                let document = preview.get_or_init(|| {
                    let committed = self.store.snapshot_owner();
                    protocol::ToValue::to_value(self.time_travel.render_snapshot_or(&self.tool_runs, &committed).as_ref())
                });
                time_travel_pointer_get(document, pointer).and_then(DslValue::as_f64)
            }
        });
    }

    /// 🏃️ Ledger work, or a base change the watch has not delivered to an open session yet.
    pub(crate) fn time_travel_has_pending_work(&self) -> bool {
        self.time_travel.has_pending_work() || (self.time_travel.is_active() && self.time_travel.session.base != self.time_travel_base())
    }

    /// 🔍️ Read-only inspection of this instance's history-edit ledger for app-level tests.
    #[cfg(any(test, feature = "artifact-app-testing"))]
    pub fn time_travel_ledger(&self) -> &TimeTravelLedger<A> {
        &self.time_travel
    }

    /// 🧾️ A session change: a fresh history cursor, so the next patch is always delivered, carrying every row when
    /// their overlay moved (`rows`); the body refreshes (every window once when the rendered `document` swapped).
    fn note_time_travel_changed(&mut self, document: bool, rows: bool) {
        self.log_generation += 1;
        if rows {
            self.history_dirty_sequences.extend(self.command_log.iter().map(|entry| entry.seq));
        }
        self.time_travel.ui_dirty = true;
        self.time_travel.document_dirty |= document;
        self.time_travel.patch_due = true;
    }

    /// 📮️ Hands the owed scope to the typed UI outbox once it has room; until then the flags stay set.
    pub(crate) fn flush_time_travel_ui_dirty(&mut self) {
        if !self.time_travel.is_ui_dirty() || self.typed_ui_outbox.len() != 0 {
            return;
        }
        if self.typed_ui_outbox.push(self.time_travel.dirty_scope()).is_ok() {
            self.time_travel.ui_dirty = false;
            self.time_travel.document_dirty = false;
        }
    }

    /// 🎯️ The applied position of every session target at the current base; `None` when one of them vanished.
    fn time_travel_positions(&self) -> Option<Vec<TimeTravelTarget>> {
        let session = &self.time_travel.session;
        let targets: Vec<&MutationId> = session.accepted.iter().map(|draft| &draft.target.mutation).chain(session.pending.as_ref().map(|pending| &pending.target.mutation)).collect();
        if targets.is_empty() {
            return Some(Vec::new());
        }
        let ops = self.store.mutation_ops().ok()?;
        targets.into_iter().map(|target| ops.iter().find(|op| op.mutation_id == *target).map(|op| TimeTravelTarget { mutation: target.clone(), position: u32::try_from(op.position).unwrap_or(u32::MAX) })).collect()
    }

    /// 👀️ Store watch (design §4, driver contract): every store generation change reaches the session as `BaseMoved`
    /// with the re-resolved target positions; a target that vanished (undone remotely, document replaced) exits the
    /// session, since nothing it drafted is addressable anymore. Answers whether the base moved.
    async fn watch_time_travel_base(&mut self) -> Result<bool, Fault> {
        let base = self.time_travel_base();
        if base == self.time_travel.session.base {
            return Ok(false);
        }
        let mut effects = Vec::new();
        match self.time_travel_positions() {
            Some(positions) => effects.extend(self.time_travel.session.apply(TimeTravelEvent::BaseMoved { base, positions }).unwrap_or_default()),
            None => {
                effects.extend(self.time_travel.session.apply(TimeTravelEvent::Exit).unwrap_or_default());
                effects.extend(self.time_travel.session.apply(TimeTravelEvent::BaseMoved { base, positions: Vec::new() }).unwrap_or_default());
            }
        }
        if self.time_travel.is_active() || !effects.is_empty() {
            self.note_time_travel_changed(true, true);
        }
        let mut dialogs = Vec::new();
        self.perform_time_travel_effects(effects, None, &mut dialogs).await?;
        Ok(true)
    }

    /// ⏪️ Host-driven routing of the reserved history-edit verbs (design §7): applied to the session now, never queued
    /// behind guest work. A refusal is a silent `{rejected}` result that still repaints the history body, so a stale
    /// button never keeps its stale identity. The reply carries the owed UI scope; the session status the verb moved to
    /// (its stage and generation) is prepared at once as the history patch of the unsolicited UI progress frame of this
    /// same turn, so every dispatch route — a command's reply, a retained-surface intent's reply that carries no patch —
    /// publishes the new generation with the stage flip and a verb stamped with it is never stale.
    pub(crate) async fn dispatch_time_travel_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
        let mut effects = Vec::new();
        let outcome = self.apply_time_travel_action(action, args, meta, &mut effects).await?;
        let output = match outcome {
            TimeTravelActionOutcome::Applied(stage) => DslValue::Object(vec![("timeTravel".into(), DslValue::String(stage.as_str().into()))]),
            TimeTravelActionOutcome::Rejected(refusal) => DslValue::Object(vec![("rejected".into(), DslValue::String(refusal.code().into()))]),
        };
        let ui_scope = self.time_travel.dirty_scope();
        self.time_travel.ui_dirty = false;
        self.time_travel.document_dirty = false;
        self.prepare_time_travel_patch().await?;
        let mut result = Self::empty_result(action, meta, effects, Vec::new(), ui_scope).await;
        result.output = output;
        Ok(result)
    }

    /// ⚖️ Applies one history-edit verb: delivers a pending base change first, then the verb's event with its effects.
    pub async fn apply_time_travel_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        self.watch_time_travel_base().await?;
        let generation = time_travel_arg_generation(args).unwrap_or(self.time_travel.session.generation);
        let event = match action {
            HISTORY_EDIT_BEGIN_ACTION_ID => return self.begin_time_travel(args, meta, effects).await,
            HISTORY_EDIT_INPUT_ACTION_ID => {
                let Some(value) = args.and_then(|args| args.get(HISTORY_EDIT_ARG_VALUE)) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::InvalidInput)) };
                return self.draft_time_travel_input(time_travel_arg_text(args, HISTORY_EDIT_ARG_PATH), value.clone(), generation, meta, effects).await;
            }
            HISTORY_EDIT_USE_SELECTION_ACTION_ID => return self.draft_time_travel_selection(time_travel_arg_text(args, HISTORY_EDIT_ARG_PATH), generation, meta, effects).await,
            HISTORY_EDIT_WITHDRAW_ACTION_ID => TimeTravelEvent::Withdraw { generation },
            HISTORY_EDIT_ACCEPT_ACTION_ID => TimeTravelEvent::Accept { generation },
            HISTORY_EDIT_DISCARD_ACTION_ID => TimeTravelEvent::Discard { generation },
            HISTORY_EDIT_FINALIZE_ACTION_ID => TimeTravelEvent::RequestFinalize { generation },
            HISTORY_EDIT_COMMIT_ACTION_ID => {
                let choice = match time_travel_arg_text(args, DIALOG_CHOICE_ARG) {
                    Some(HISTORY_EDIT_CHOICE_OVERWRITE) => TimeTravelChoice::Overwrite,
                    _ => match time_travel_arg_text(args, HISTORY_EDIT_ARG_NAME).filter(|name| !name.trim().is_empty()) {
                        Some(name) if is_time_travel_alternative_name(name) => TimeTravelChoice::Alternative { name: name.to_string() },
                        Some(_) => return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::NameInvalid)),
                        None => match meta.view_state.as_ref() {
                            Some(view) => TimeTravelChoice::Alternative { name: TimeTravelLabel::AlternativeNameDefault.localized(LocalizedLabel::native).resolve(Terminology::Native, view.locale).to_string() },
                            None => return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::NameRequired)),
                        },
                    },
                };
                TimeTravelEvent::Choose { generation, choice }
            }
            HISTORY_EDIT_BACK_ACTION_ID => TimeTravelEvent::Back { generation },
            HISTORY_EDIT_EXIT_ACTION_ID => TimeTravelEvent::Exit,
            HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID => TimeTravelEvent::ReplayCancelled { generation },
            HISTORY_EDIT_RERUN_ACTION_ID => TimeTravelEvent::Rerun { generation },
            _ => return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("timeTravel.unknown-action"), format!("'{action}' is not a history-edit verb"))),
        };
        self.apply_time_travel_event(event, Some(meta), effects).await
    }

    /// ⚖️ Applies one event to the session and performs its effects; a refusal changes nothing.
    async fn apply_time_travel_event(&mut self, event: TimeTravelEvent, meta: Option<&ActionMeta>, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        let preview_changes = matches!(event, TimeTravelEvent::Accept { .. } | TimeTravelEvent::Discard { .. } | TimeTravelEvent::Exit | TimeTravelEvent::Back { .. });
        match self.time_travel.session.apply(event) {
            Ok(session_effects) => {
                self.note_time_travel_changed(preview_changes || !session_effects.is_empty(), true);
                self.perform_time_travel_effects(session_effects, meta, effects).await?;
                Ok(TimeTravelActionOutcome::Applied(self.time_travel.session.stage))
            }
            Err(refusal) => {
                self.time_travel.ui_dirty = true;
                Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(refusal)))
            }
        }
    }

    /// 🛠️ Performs the session's effects in order; a commit's own `Finalized`/`FinalizeFaulted` effects follow it.
    async fn perform_time_travel_effects(&mut self, effects: Vec<TimeTravelEffect>, meta: Option<&ActionMeta>, kernel: &mut Vec<Effect>) -> Result<(), Fault> {
        let mut queue: VecDeque<TimeTravelEffect> = effects.into();
        while let Some(effect) = queue.pop_front() {
            match effect {
                TimeTravelEffect::ShowPreview { target, replacement } => self.show_time_travel_preview(&target, &replacement)?,
                TimeTravelEffect::StartReplay { drafts, from } => queue.extend(self.start_time_travel_replay(drafts, &from)),
                TimeTravelEffect::CancelReplay => self.time_travel.replay = None,
                TimeTravelEffect::OpenFinalizePrompt => {
                    let args = meta.and_then(|meta| meta.view_state.as_ref()).map(|view| DslValue::object([(HISTORY_EDIT_ARG_NAME.to_string(), DslValue::String(TimeTravelLabel::AlternativeNameDefault.localized(LocalizedLabel::native).resolve(Terminology::Native, view.locale).to_string()))]));
                    kernel.push(Effect::OpenDialog { req: TIME_TRAVEL_FINALIZE_REQUEST, dialog_id: HISTORY_EDIT_FINALIZE_DIALOG_ID.to_string(), args });
                }
                TimeTravelEffect::CommitOverwrite { .. } => queue.extend(self.commit_time_travel(store::HistoryFinalization::Overwrite, meta).await?),
                TimeTravelEffect::CommitAlternative { name, .. } => queue.extend(self.commit_time_travel(store::HistoryFinalization::Alternative { name }, meta).await?),
                TimeTravelEffect::Close => self.time_travel.document_dirty = true,
            }
        }
        self.time_travel.settle_owners();
        Ok(())
    }

    /// 🖼️ The editing preview: the state before `target` with the accepted upstream drafts, then `replacement` folded
    /// exactly as the store's replay folds it (keep-and-record: an apply refusal is one `Fatal` message and a no-op);
    /// the draft's own messages stay on the editor.
    fn show_time_travel_preview(&mut self, target: &MutationId, replacement: &protocol::InputReplacement) -> Result<(), Fault> {
        let drafts = self.time_travel.accepted_drafts();
        let base = self.store.state_before(target, &drafts).map_err(|error| error.into_fault())?;
        let (preview, outcome) = match replacement {
            protocol::InputReplacement::Withdrawn => (base, Vec::new()),
            protocol::InputReplacement::Input { payload, .. } => {
                let op = <A::Mutation as ::protocol::OpBinary>::decode_op(payload).map_err(|error| error.into_fault())?;
                let (diff, mut messages) = op.diff(&base).into_parts();
                let applied = diff.apply(&base);
                <<A::Mutation as ::protocol::Mutation<A::Snapshot>>::Diff as MutationDiff<A::Snapshot>>::retire_cold(diff);
                self.time_travel.discarded.push(op);
                match applied {
                    Ok(next) => {
                        self.time_travel.retired_snapshots.push(base);
                        (Arc::new(next), messages)
                    }
                    Err(error) => {
                        messages.push(protocol::MutationMessage::fatal(error.code, error.message).at(error.target));
                        (base, messages)
                    }
                }
            }
        };
        let previous = self.time_travel.preview.replace(preview);
        self.time_travel.retire_snapshot(previous);
        if let Some(editor) = self.time_travel.editor.as_mut() {
            editor.outcome = outcome;
        }
        self.time_travel.document_dirty = true;
        Ok(())
    }

    /// ▶️ Starts (latest-wins) the Report replay of `drafts` from `from`; a store refusal faults the replay instead.
    fn start_time_travel_replay(&mut self, drafts: Vec<protocol::SupersededInput>, from: &MutationId) -> Vec<TimeTravelEffect> {
        self.time_travel.replay = None;
        self.time_travel.refreshed_done = 0;
        let drafts: BTreeMap<MutationId, protocol::InputReplacement> = drafts.into_iter().map(|draft| (draft.target, draft.replacement)).collect();
        let generation = self.time_travel.session.generation;
        match self.store.begin_report_replay(&drafts, Some(from)) {
            Ok(replay) => {
                self.time_travel.replay = Some(replay);
                Vec::new()
            }
            Err(_) => self.time_travel.session.apply(TimeTravelEvent::ReplayFaulted { generation, code: TIME_TRAVEL_REPLAY_FAULTED_CODE.to_string() }).unwrap_or_default(),
        }
    }

    /// 🌿️ Commits the finished replay (`commit_finished_replay`: no second replay): the session finalizes, or a stale
    /// base or a blocking report faults the finalize, which replays again on the current base. The history row of the
    /// commit is its `Supersede` transition, backfilled like every remote or reloaded one.
    async fn commit_time_travel(&mut self, finalization: store::HistoryFinalization, meta: Option<&ActionMeta>) -> Result<Vec<TimeTravelEffect>, Fault> {
        let generation = self.time_travel.session.generation;
        let fault = |code: &str| TimeTravelEvent::FinalizeFaulted { generation, code: code.to_string() };
        let drafts = self.time_travel.accepted_drafts();
        let Some(finished) = self.time_travel.finished.take().filter(|finished| *finished.drafts() == drafts) else {
            return Ok(self.time_travel.session.apply(fault(TimeTravelRefusal::Stale.code())).unwrap_or_default());
        };
        if let Some(meta) = meta {
            self.store.set_local_actor_id(Some(meta.actor.clone())).map_err(|error| error.into_fault())?;
        }
        let event = match self.store.commit_finished_replay(finished, finalization).await {
            Ok(_) => {
                self.cache = None;
                TimeTravelEvent::Finalized { generation }
            }
            Err(vcs::VcsError::Stale { .. }) => fault(TimeTravelRefusal::Stale.code()),
            Err(vcs::VcsError::Rejected { .. }) => fault(TimeTravelRefusal::Blocked.code()),
            Err(_) => fault(TIME_TRAVEL_COMMIT_FAILED_CODE),
        };
        let effects = self.time_travel.session.apply(event).unwrap_or_default();
        self.note_time_travel_changed(true, true);
        if let Some(meta) = meta.filter(|_| self.time_travel.session.stage == TimeTravelStage::Inactive) {
            self.revalidate_interaction_state_after_document_change(meta).await?;
        }
        Ok(effects)
    }

    /// ✏️ Opens (or retargets) the session on `mutationId`: refused while a mutating tool run or an agent transaction
    /// holds this instance, for an unknown operation, and for one whose inputs cannot be edited. Opening a session
    /// delivers [`HostEvent::TimeTravelFrozen`] to every open window, so an open gesture there ends first.
    async fn begin_time_travel(&mut self, args: Option<&DslValue>, meta: &ActionMeta, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        let Some(mutation) = time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownMutation)) };
        if self.tool_runs.holds_mutating_run() || self.pending_transaction.is_some() {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy));
        }
        let target = MutationId(mutation.to_string());
        let (editor, original) = match self.time_travel_editor(&target) {
            Ok(opened) => opened,
            Err(refusal) => return Ok(TimeTravelActionOutcome::Rejected(refusal)),
        };
        let event = TimeTravelEvent::Begin { target: TimeTravelTarget { mutation: target, position: editor.position }, original };
        let opening = !self.time_travel.is_active();
        match self.time_travel.session.apply(event) {
            Ok(session_effects) => {
                self.time_travel.replace_editor(Some(editor));
                self.note_time_travel_changed(true, true);
                self.perform_time_travel_effects(session_effects, Some(meta), effects).await?;
                if opening {
                    self.deliver_host_event_to_every_window(|window_id| HostEvent::TimeTravelFrozen { window_id }, meta).await?;
                }
                Ok(TimeTravelActionOutcome::Applied(self.time_travel.session.stage))
            }
            Err(refusal) => {
                self.time_travel.discarded.push(editor.kind);
                self.time_travel.ui_dirty = true;
                Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(refusal)))
            }
        }
    }

    /// ✏️ The editor of `target` and its effective input before the session: its payload schema, input descriptors,
    /// validator, the kind the draft rebuilds and the value it starts from (its accepted draft, else its input).
    fn time_travel_editor(&self, target: &MutationId) -> Result<(TimeTravelEditor<A>, protocol::InputReplacement), TimeTravelActionRefusal> {
        let ops = self.store.mutation_ops().map_err(|_| TimeTravelActionRefusal::UnknownMutation)?;
        let row = ops.iter().find(|row| row.mutation_id == *target).ok_or(TimeTravelActionRefusal::UnknownMutation)?;
        let schema_id = self.store.envelope().schema.clone();
        let original = match row.supersession {
            Some(supersession) => supersession.replacement.clone(),
            None => protocol::InputReplacement::Input { schema: schema_id, payload: <A::Mutation as ::protocol::OpBinary>::encode_op(row.operation).map_err(|_| TimeTravelActionRefusal::NotEditable)? },
        };
        let decode = |replacement: &protocol::InputReplacement| match replacement {
            protocol::InputReplacement::Input { payload, .. } => <A::Mutation as ::protocol::OpBinary>::decode_op(payload).ok(),
            protocol::InputReplacement::Withdrawn => None,
        };
        let kind = decode(&original).unwrap_or_else(|| row.operation.clone());
        if !time_travel_editable::<A>(&kind) {
            kind.retire_cold();
            return Err(TimeTravelActionRefusal::NotEditable);
        }
        let current = self.time_travel.session.accepted_draft(target).map_or(&original, |draft| &draft.replacement);
        let (value, withdrawn) = match decode(current) {
            Some(op) => {
                let value = op.payload_value();
                op.retire_cold();
                (value, false)
            }
            None => (kind.payload_value(), *current == protocol::InputReplacement::Withdrawn),
        };
        let schema = kind.input_schema().expect("an editable operation declares its input schema");
        let (inputs, inputs_refused) = match mutation_input_defs(schema, &registered_input_schema_document) {
            Ok(inputs) => (inputs, None),
            Err(error) => (Vec::new(), Some(error)),
        };
        let documents = time_travel_schema_documents(schema);
        let validator = semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(schema, &documents.iter().map(String::as_str).collect::<Vec<_>>()).map_err(|error| error.to_string());
        let editor = TimeTravelEditor {
            target: target.clone(),
            position: u32::try_from(row.position).unwrap_or(u32::MAX),
            op_index: row.op_index,
            label: time_travel_mutation_label::<A>(&kind),
            schema,
            inputs,
            inputs_refused,
            validator,
            kind,
            value,
            withdrawn,
            outcome: Vec::new(),
            refused: None,
        };
        Ok((editor, original))
    }

    /// 🎚️ Drafts one input: the host value coerced to the shape of the input `path` addresses in the variant the draft is
    /// in, set at `path` in the draft payload (a union's selector switches the variant: [`time_travel_switch_variant`]),
    /// validated against the leaf payload schema (with its root discriminators spliced back), rebuilt into the same kind
    /// and canonically encoded. A refused value keeps the draft and names the reason on the editor. Fails closed: a
    /// payload schema that compiles no validator or describes no inputs admits no draft
    /// (`timeTravel.schema-unavailable`), never an unvalidated payload.
    async fn draft_time_travel_input(&mut self, path: Option<&str>, value: DslValue, generation: u32, meta: &ActionMeta, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        if generation != self.time_travel.session.generation {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Stale)));
        }
        let schema_id = self.store.envelope().schema.clone();
        let Some(editor) = self.time_travel.editor.as_mut().filter(|_| self.time_travel.session.stage == TimeTravelStage::Editing) else {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal)));
        };
        let unavailable = match (&editor.validator, &editor.inputs_refused) {
            (Err(reason), _) => Some(format!("the payload schema compiles no validator, so no draft is admitted: {reason}")),
            (Ok(_), Some(error)) => Some(format!("the payload schema describes no inputs, so no draft is admitted: {}", error.detail)),
            (Ok(_), None) => None,
        };
        if let Some(reason) = unavailable {
            editor.refused = Some((path.unwrap_or_default().to_string(), reason));
            self.time_travel.ui_dirty = true;
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::SchemaUnavailable));
        }
        let Some((path, (input, element))) = path.and_then(|path| time_travel_input_at(&editor.inputs, &editor.value, path).map(|found| (path, found))) else {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput));
        };
        let refuse = |editor: &mut TimeTravelEditor<A>, reason: String| {
            editor.refused = Some((path.to_string(), reason));
            TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::InvalidInput)
        };
        let Some(coerced) = time_travel_coerce(&input, element, &value) else {
            let outcome = refuse(editor, format!("{path} does not take this value"));
            self.time_travel.ui_dirty = true;
            return Ok(outcome);
        };
        let coerced = match (&input.schema, coerced) {
            (ArgSchema::Vector { dims: 4, .. }, DslValue::Array(mut components)) if components.len() == 3 => {
                components.push(time_travel_pointer_get(&editor.value, &format!("{path}/3")).cloned().unwrap_or_else(|| DslValue::float(1.0)));
                DslValue::Array(components)
            }
            (_, coerced) => coerced,
        };
        let switched = time_travel_variant_selector(&editor.inputs).filter(|selector| selector.id == path).zip(coerced.as_str()).map(|(selector, variant)| time_travel_switch_variant(&editor.inputs, selector, &editor.value, variant));
        let candidate = match switched {
            Some(candidate) => candidate,
            None => {
                let mut candidate = editor.value.clone();
                if !time_travel_pointer_set(&mut candidate, path, coerced) {
                    return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput));
                }
                candidate
            }
        };
        let verdict = mutation_input_instance(editor.schema, &registered_input_schema_document, &candidate)
            .map_err(|error| error.to_string())
            .and_then(|instance| editor.validator.as_ref().map_err(String::clone).and_then(|validator| validator.validate_json(&time_travel_json(&instance)).map(|_| ()).map_err(|error| error.to_string())));
        let rebuilt = verdict.and_then(|()| editor.kind.with_payload_value(candidate.clone()).map_err(|error| error.to_string()));
        let op = match rebuilt {
            Ok(op) => op,
            Err(reason) => {
                let outcome = refuse(editor, reason);
                self.time_travel.ui_dirty = true;
                return Ok(outcome);
            }
        };
        let encoded = <A::Mutation as ::protocol::OpBinary>::encode_op(&op);
        self.time_travel.discarded.push(op);
        let payload = match encoded {
            Ok(payload) => payload,
            Err(error) => {
                let editor = self.time_travel.editor.as_mut().expect("editing keeps its editor");
                let outcome = refuse(editor, error.to_string());
                self.time_travel.ui_dirty = true;
                return Ok(outcome);
            }
        };
        let outcome = self.apply_time_travel_event(TimeTravelEvent::Draft { generation, replacement: protocol::InputReplacement::Input { schema: schema_id, payload } }, Some(meta), effects).await?;
        if matches!(outcome, TimeTravelActionOutcome::Applied(_)) {
            if let Some(editor) = self.time_travel.editor.as_mut() {
                editor.value = candidate;
                editor.withdrawn = false;
                editor.refused = None;
            }
        }
        Ok(outcome)
    }

    /// 🎯️ Drafts the reference input at `path` from the current selection of its declared domain (at its granularity);
    /// refused when the input names no domain, the selection is at another granularity, or nothing is selected.
    async fn draft_time_travel_selection(&mut self, path: Option<&str>, generation: u32, meta: &ActionMeta, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        let Some(editor) = self.time_travel.editor.as_ref() else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal))) };
        let Some((input, false)) = path.and_then(|path| time_travel_input_at(&editor.inputs, &editor.value, path)) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
        let ArgSchema::Reference { domain: Some(domain), granularity, many, max_items, id_type, .. } = &input.schema else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
        let (domain, granularity, many, max_items, id_type) = (domain.clone(), granularity.clone(), *many, *max_items, *id_type);
        let state = self.interaction_selection_snapshot();
        let Some(selection) = state.selection.get(&domain).filter(|selection| !selection.ids.is_empty() && granularity.as_ref().is_none_or(|granularity| *granularity == selection.granularity)) else {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::NoSelection));
        };
        let limit = max_items.map_or(usize::MAX, |max| max as usize);
        let mut ids = selection.ids.iter().filter_map(|id| id_type.id_value(id));
        let value = if many {
            DslValue::Array(ids.take(limit).collect())
        } else {
            let Some(id) = ids.next() else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::NoSelection)) };
            id
        };
        self.draft_time_travel_input(path, value, generation, meta, effects).await
    }

    /// ⏯️ One bounded driver turn (≤ [`TIME_TRAVEL_TURN_WALL_US`]): owed scope, retirement, base watch, one replay slice;
    /// a session change prepares the history patch that rides the next unsolicited UI progress frame.
    pub(crate) async fn drive_time_travel_turn(&mut self) -> Result<(), Fault> {
        self.prepare_time_travel_patch().await?;
        self.flush_time_travel_ui_dirty();
        if let Some(step) = self.time_travel.retire_step(&mut self.store, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES)? {
            if !matches!(step, PluginCloseStep::Blocked { .. }) {
                return Ok(());
            }
        }
        let moved = self.watch_time_travel_base().await?;
        if !moved && self.time_travel.session.stage == TimeTravelStage::Replaying && self.time_travel.replay.is_some() {
            self.step_time_travel_replay().await?;
        }
        self.prepare_time_travel_patch().await?;
        self.flush_time_travel_ui_dirty();
        Ok(())
    }

    /// 🧾️ Prepares the owed history patch (a session change sets `patch_due`) for the next unsolicited UI progress
    /// frame. A patch that has not shipped yet is superseded by one that still carries its rows, so no row upsert is lost
    /// between two changes of one turn.
    async fn prepare_time_travel_patch(&mut self) -> Result<(), Fault> {
        if !std::mem::take(&mut self.time_travel.patch_due) {
            return Ok(());
        }
        if let Some(previous) = self.time_travel.patch.take() {
            self.history_dirty_sequences.extend(previous.upserts.iter().map(|entry| entry.seq));
        }
        self.time_travel.patch = Some(self.history_patch(false).await?);
        Ok(())
    }

    /// ⏭️ Steps the replay until the turn deadline: progress ticks refresh the history body at most every
    /// [`TIME_TRAVEL_PROGRESS_REFRESH_MS`]; completion hands the report to the session and swaps the preview to the
    /// replayed head; a store refusal faults the replay.
    async fn step_time_travel_replay(&mut self) -> Result<(), Fault> {
        let deadline_us = semio_framework_job::default_now_us().unwrap_or(0).saturating_add(TIME_TRAVEL_TURN_WALL_US);
        let generation = self.time_travel.session.generation;
        let stepped = {
            let VcsArtifactApp { store, time_travel, .. } = self;
            let replay = time_travel.replay.as_mut().expect("a replaying session holds its replay");
            let mut deadline = || semio_framework_job::default_now_us().is_none_or(|now| now >= deadline_us);
            replay.step(store.replay_edits(), &mut deadline)
        };
        let event = match stepped {
            Ok(store::ReplayStep::Pending(progress)) => {
                let _ = self.time_travel.session.apply(TimeTravelEvent::ReplayProgressed { generation, done: progress.done, total: progress.total });
                let now_ms = semio_framework_job::default_now_ms().unwrap_or(0);
                let advanced = u64::from(progress.done.saturating_sub(self.time_travel.refreshed_done)) * 1_000 >= u64::from(progress.total) * TIME_TRAVEL_PROGRESS_REFRESH_PERMILLE;
                if advanced || now_ms.saturating_sub(self.time_travel.refreshed_ms) >= TIME_TRAVEL_PROGRESS_REFRESH_MS {
                    self.time_travel.refreshed_ms = now_ms;
                    self.time_travel.refreshed_done = progress.done;
                    self.note_time_travel_changed(false, false);
                }
                return Ok(());
            }
            Ok(store::ReplayStep::Finished(_)) => {
                let replay = self.time_travel.replay.take().expect("a finished replay was held");
                match replay.finish().and_then(|result| self.store.replay_report(&result).map(|report| (result, report))) {
                    Ok((result, report)) => {
                        let head = result.state().cloned();
                        let previous = std::mem::replace(&mut self.time_travel.head, head);
                        self.time_travel.retire_snapshot(previous);
                        self.time_travel.finished = Some(result);
                        TimeTravelEvent::ReplayCompleted { generation, report }
                    }
                    Err(_) => TimeTravelEvent::ReplayFaulted { generation, code: TIME_TRAVEL_REPLAY_FAULTED_CODE.to_string() },
                }
            }
            Err(_) => {
                self.time_travel.replay = None;
                TimeTravelEvent::ReplayFaulted { generation, code: TIME_TRAVEL_REPLAY_FAULTED_CODE.to_string() }
            }
        };
        let mut dialogs = Vec::new();
        self.apply_time_travel_event(event, None, &mut dialogs).await?;
        Ok(())
    }
}
//#endregion 🔖️Driver

//#region 🔖️Supersessions
/// 🔁️ What one `Supersede` transition does to its author's history edits: a new edit, the undo of an earlier one (it
/// restores every input that edit still holds to the input before it, the original input when there was none), or the
/// redo of an undone one (it re-authors that edit's inputs over the state its undo left).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupersedeRole {
    Edit,
    Undo,
    Redo,
}

/// ✏️ One `Supersede` transition of the event log: its id, author, HLC, scope and inputs, its [`SupersedeRole`] and the
/// history edit it acts for (`entry`, an index into [`SupersedeLedger::records`]) — itself for an edit, the undone edit
/// for an undo, the re-authored edit for a redo.
#[derive(Clone, Debug, PartialEq)]
pub struct SupersedeRecord {
    pub transition_id: String,
    pub actor: String,
    pub timestamp: HybridLogicalTimestamp,
    pub scope: Option<String>,
    pub inputs: Vec<protocol::SupersededInput>,
    pub role: SupersedeRole,
    pub entry: usize,
}

/// 📚️ Every `Supersede` transition of a document's event log in fold order `(hlc, id)`, classified per author, with each
/// author's history edits that are not undone, the undone ones a redo re-authors (each with the undo that undid it), each
/// author's newest document-edit undo (`Revert`) and the original input of every superseded operation. A pure function
/// of the transitions and the edits: every replica, and every reload, derives the same rows. Rebuilt only when the
/// transitions changed.
///
/// An input's effective supersession in the view of `scope` is the last one naming it whose scope is `None` or `scope` —
/// the store's fold law — and an undo restores in the view of the undone edit's own scope. Every effective input has an
/// owner: the history edit whose input it is (an edit's own, a redo's re-authored edit's, an undo's the owner of the
/// input it restored), or none for an original input.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SupersedeLedger {
    key: (usize, Option<String>),
    pub records: Vec<SupersedeRecord>,
    index: HashMap<String, usize>,
    originals: HashMap<MutationId, protocol::InputReplacement>,
    undo: HashMap<String, Vec<usize>>,
    redo: HashMap<String, Vec<(usize, usize)>>,
    reverts: HashMap<String, HybridLogicalTimestamp>,
}

impl SupersedeLedger {
    /// 🔄️ Rebuilds from `transitions` when they changed since the last build; `originals` answers the original input,
    /// encoded like a supersede replacement, of every operation a supersession names.
    pub fn refresh(&mut self, transitions: &[store::os_spr::MutationEnvelope], originals: impl FnOnce(&BTreeSet<MutationId>) -> HashMap<MutationId, protocol::InputReplacement>) {
        let key = (transitions.len(), transitions.last().map(|envelope| envelope.mutation_id.0.clone()));
        if key == self.key {
            return;
        }
        let mut records = Vec::new();
        let mut reverts: HashMap<String, HybridLogicalTimestamp> = HashMap::new();
        for envelope in transitions {
            match store::os_spr::history_transition_from_envelope(envelope) {
                Ok(Some(store::os_spr::HistoryTransition::Supersede(supersede))) => {
                    records.push(SupersedeRecord { transition_id: envelope.mutation_id.0.clone(), actor: envelope.actor.0.clone(), timestamp: envelope.timestamp, scope: supersede.scope, inputs: supersede.inputs, role: SupersedeRole::Edit, entry: 0 })
                }
                Ok(Some(store::os_spr::HistoryTransition::Revert { .. })) => {
                    let newest = reverts.entry(envelope.actor.0.clone()).or_insert(envelope.timestamp);
                    *newest = (*newest).max(envelope.timestamp);
                }
                _ => {}
            }
        }
        let originals = originals(&records.iter().flat_map(|record| record.inputs.iter().map(|input| input.target.clone())).collect());
        *self = Self { key, ..Self::from_records(records, reverts, originals) };
    }

    /// 🧮️ The ledger of decoded `Supersede` `records` (any order; sorted into fold order here, `role` and `entry`
    /// recomputed), each author's newest document-edit undo and the original input of every operation they name.
    pub fn from_records(mut records: Vec<SupersedeRecord>, reverts: HashMap<String, HybridLogicalTimestamp>, originals: HashMap<MutationId, protocol::InputReplacement>) -> Self {
        records.sort_by(|a, b| a.timestamp.cmp(&b.timestamp).then_with(|| a.transition_id.cmp(&b.transition_id)));
        let index = records.iter().enumerate().map(|(index, record)| (record.transition_id.clone(), index)).collect();
        let mut ledger = Self { key: (0, None), records, index, originals, undo: HashMap::new(), redo: HashMap::new(), reverts };
        for index in 0..ledger.records.len() {
            ledger.classify(index);
        }
        ledger
    }

    /// 🎯️ The replacement effective for `target` over the first `before` records in the view of `scope`, with the record
    /// that installed it.
    fn effective(&self, before: usize, scope: Option<&str>, target: &MutationId) -> Option<(usize, &protocol::InputReplacement)> {
        self.records[..before].iter().enumerate().rev().filter(|(_, record)| record.scope.is_none() || record.scope.as_deref() == scope).find_map(|(index, record)| record.inputs.iter().find(|input| input.target == *target).map(|input| (index, &input.replacement)))
    }

    /// 👑️ The history edit whose input record `index` installs for `target`: the edit itself, a redo's re-authored edit,
    /// an undo's owner of the input it restored; `None` for a restored original input.
    fn owner(&self, index: usize, target: &MutationId) -> Option<usize> {
        let record = &self.records[index];
        match record.role {
            SupersedeRole::Edit => Some(index),
            SupersedeRole::Redo => Some(record.entry),
            SupersedeRole::Undo => self.effective(record.entry, self.records[record.entry].scope.as_deref(), target).and_then(|(installed, _)| self.owner(installed, target)),
        }
    }

    /// 👑️ The owner of `target`'s input over the first `before` records in the view of `scope`.
    fn owner_at(&self, before: usize, scope: Option<&str>, target: &MutationId) -> Option<usize> {
        self.effective(before, scope, target).and_then(|(installed, _)| self.owner(installed, target))
    }

    /// ✋️ The inputs history edit `entry` still owns over the first `before` records, in the view of its scope.
    fn held(&self, entry: usize, before: usize) -> Vec<&MutationId> {
        let scope = self.records[entry].scope.as_deref();
        self.records[entry].inputs.iter().map(|input| &input.target).filter(|target| self.owner_at(before, scope, target) == Some(entry)).collect()
    }

    /// ⏪️ The input `target` had before history edit `entry`: the effective one in the view of its scope, else the original.
    fn before(&self, entry: usize, target: &MutationId) -> Option<protocol::InputReplacement> {
        match self.effective(entry, self.records[entry].scope.as_deref(), target) {
            Some((_, replacement)) => Some(replacement.clone()),
            None => self.originals.get(target).cloned(),
        }
    }

    /// ⏩️ Whether the inputs undo `undo` restored still have, over the first `before` records, the owners it left them with.
    fn undo_stands(&self, undo: usize, before: usize) -> bool {
        let scope = self.records[undo].scope.as_deref();
        self.records[undo].inputs.iter().all(|input| self.owner_at(before, scope, &input.target) == self.owner(undo, &input.target))
    }

    /// 🔁️ Classifies record `index` against its author's stacks: the undo of a history edit that still owns exactly its
    /// targets and whose inputs before it it restores, else the redo of the newest undone edit whose undo still stands and
    /// whose inputs it re-authors, else a new history edit (clearing the author's redos).
    fn classify(&mut self, index: usize) {
        let record = &self.records[index];
        let targets: BTreeSet<&MutationId> = record.inputs.iter().map(|input| &input.target).collect();
        let undone = self.undo.get(&record.actor).and_then(|stack| {
            stack.iter().rev().copied().find(|&entry| {
                self.records[entry].scope == record.scope && self.held(entry, index).into_iter().collect::<BTreeSet<_>>() == targets && record.inputs.iter().all(|input| self.before(entry, &input.target).as_ref() == Some(&input.replacement))
            })
        });
        let redone = self.redo.get(&record.actor).and_then(|stack| stack.last().copied()).filter(|&(entry, undo)| {
            undone.is_none()
                && self.records[entry].scope == record.scope
                && self.records[undo].inputs.iter().map(|input| &input.target).collect::<BTreeSet<_>>() == targets
                && self.undo_stands(undo, index)
                && record.inputs.iter().all(|input| self.records[entry].inputs.iter().any(|edited| edited.target == input.target && edited.replacement == input.replacement))
        });
        let actor = record.actor.clone();
        let (role, entry) = match (undone, redone) {
            (Some(entry), _) => {
                if let Some(stack) = self.undo.get_mut(&actor) {
                    stack.retain(|held| *held != entry);
                }
                self.redo.entry(actor).or_default().push((entry, index));
                (SupersedeRole::Undo, entry)
            }
            (None, Some((entry, _))) => {
                if let Some(stack) = self.redo.get_mut(&actor) {
                    stack.pop();
                }
                self.undo.entry(actor).or_default().push(entry);
                (SupersedeRole::Redo, entry)
            }
            (None, None) => {
                self.redo.remove(&actor);
                self.undo.entry(actor).or_default().push(index);
                (SupersedeRole::Edit, index)
            }
        };
        self.records[index].role = role;
        self.records[index].entry = entry;
    }

    /// 🔎️ The record of transition `transition_id`, with its index.
    pub fn record(&self, transition_id: &str) -> Option<(usize, &SupersedeRecord)> {
        self.index.get(transition_id).map(|&index| (index, &self.records[index]))
    }

    /// ✅️ The history edits in effect now: the owners of the inputs the store folds with (`supersessions`, the effective
    /// supersessions of the final alternative).
    pub fn applied_entries<'a>(&self, supersessions: impl IntoIterator<Item = (&'a MutationId, &'a protocol::EffectiveSupersession)>) -> HashSet<usize> {
        supersessions.into_iter().filter_map(|(target, supersession)| self.index.get(supersession.transition_id.as_str()).and_then(|&index| self.owner(index, target))).collect()
    }

    /// ✅️ Whether the history row of record `index` reads as applied: an edit or a redo while its edit is in effect, an
    /// undo while the edit it undid is not redone.
    pub fn row_applied(&self, index: usize, applied: &HashSet<usize>) -> bool {
        let record = &self.records[index];
        match record.role {
            SupersedeRole::Undo => !self.undo.get(&record.actor).is_some_and(|stack| stack.contains(&record.entry)),
            SupersedeRole::Edit | SupersedeRole::Redo => applied.contains(&record.entry),
        }
    }

    /// ⏪️ Whether `actor` may take history edit `entry` back now: it is theirs, not undone, and in effect.
    pub fn undoable(&self, actor: &str, entry: usize, applied: &HashSet<usize>) -> bool {
        applied.contains(&entry) && self.undo.get(actor).is_some_and(|stack| stack.contains(&entry))
    }

    /// ⏪️ The undo of history edit `entry`: its scope and each input it still owns, in the view of that scope, back to
    /// the input before it (the original input when there was none).
    pub fn restore(&self, entry: usize) -> (Option<String>, Vec<protocol::SupersededInput>) {
        let inputs = self.held(entry, self.records.len()).into_iter().filter_map(|target| self.before(entry, target).map(|replacement| protocol::SupersededInput { target: target.clone(), replacement })).collect();
        (self.records[entry].scope.clone(), inputs)
    }

    /// ⏩️ `actor`'s next redo: the newest undone history edit and its undo, while that undo still stands.
    pub fn redo_top(&self, actor: &str) -> Option<(usize, usize)> {
        self.redo.get(actor).and_then(|stack| stack.last().copied()).filter(|&(_, undo)| self.undo_stands(undo, self.records.len()))
    }

    /// ⏩️ The redo of history edit `entry` undone by `undo`: its scope and its own input for every input the undo restored.
    pub fn redo(&self, entry: usize, undo: usize) -> (Option<String>, Vec<protocol::SupersededInput>) {
        let inputs = self.records[undo].inputs.iter().filter_map(|restored| self.records[entry].inputs.iter().find(|edited| edited.target == restored.target).cloned()).collect();
        (self.records[entry].scope.clone(), inputs)
    }

    /// ⏱️ `actor`'s newest undo of a document edit.
    pub fn newest_revert(&self, actor: &str) -> Option<HybridLogicalTimestamp> {
        self.reverts.get(actor).copied()
    }

    /// 🏷️ The history row of record `index` in every locale: its role, its scope (an overwrite, or the alternative
    /// `alternative` names) and how many mutations it supersedes.
    pub fn label(&self, index: usize, alternative: Option<&str>) -> LocalizedLabel {
        let record = &self.records[index];
        let count = record.inputs.len();
        let (en_role, de_role) = match record.role {
            SupersedeRole::Edit => ("History edited", "Verlauf bearbeitet"),
            SupersedeRole::Undo => ("History edit undone", "Verlaufsbearbeitung rückgängig"),
            SupersedeRole::Redo => ("History edit redone", "Verlaufsbearbeitung wiederhergestellt"),
        };
        let (en_scope, de_scope) = match (&record.scope, alternative) {
            (None, _) => ("overwrite".to_string(), "überschrieben".to_string()),
            (Some(_), Some(name)) => (format!("alternative {name}"), format!("Alternative {name}")),
            (Some(id), None) => (format!("alternative {id}"), format!("Alternative {id}")),
        };
        let (en_count, de_count) = if count == 1 { ("1 mutation".to_string(), "1 Mutation".to_string()) } else { (format!("{count} mutations"), format!("{count} Mutationen")) };
        LocalizedLabel::native(&format!("{en_role} — {en_scope}: {en_count}"), &format!("{de_role} — {de_scope}: {de_count}"))
    }
}

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🖋️ The actor this replica authors history transitions as — the store's own rule.
    pub(crate) fn supersede_author(&self) -> &str {
        self.store.local_actor_id().unwrap_or("local")
    }

    /// 📚️ Rebuilds the supersede ledger when the store's transitions changed.
    pub(crate) fn refresh_supersede_ledger(&mut self) {
        let envelope = self.store.envelope();
        self.supersedes.refresh(&envelope.transitions, |targets| {
            let mut originals = HashMap::new();
            if targets.is_empty() {
                return originals;
            }
            for edit in envelope.vcs.edits.iter() {
                for (mutation_id, op) in store::os_spr::mutation_ids_for_edit::<A::Snapshot, A::Mutation>(edit).into_iter().zip(edit.forwards.iter()) {
                    if targets.contains(&mutation_id) {
                        if let Ok(payload) = <A::Mutation as ::protocol::OpBinary>::encode_op(op) {
                            originals.insert(mutation_id, protocol::InputReplacement::Input { schema: envelope.schema.clone(), payload });
                        }
                    }
                }
                if originals.len() == targets.len() {
                    break;
                }
            }
            originals
        });
    }

    /// 🗂️ Retires the command-log rows of the document a whole-document replacement displaced (an archive or a pack load): a
    /// row naming an edit or a history transition the published store does not hold belonged to that document, so the history
    /// lists exactly the loaded document's rows, which the next read backfills (ticket 26/09/30 NON-DESTRUCTIVE-HISTORY-EDITING
    /// follow-up 3). Rows naming neither — configuration and shell rows — stay.
    pub(crate) fn retire_displaced_document_rows(&mut self) {
        self.refresh_supersede_ledger();
        let held: HashSet<&str> = self.store.envelope().vcs.edits.iter().map(|edit| edit.id.as_str()).collect();
        let supersedes = &self.supersedes;
        let before = self.command_log.len();
        self.command_log.retain(|entry| entry.edit_id.as_deref().is_none_or(|edit_id| held.contains(edit_id)) && entry.transition_id.as_deref().is_none_or(|transition_id| supersedes.record(transition_id).is_some()));
        if self.command_log.len() == before {
            return;
        }
        let kept: HashSet<u64> = self.command_log.iter().map(|entry| entry.seq).collect();
        self.shell_undone.retain(|seq| kept.contains(seq));
        self.history_dirty_sequences.retain(|seq| kept.contains(seq));
        self.log_generation += 1;
    }

    /// 🏷️ The history label of supersede record `index`, naming its alternative when it is scoped to one.
    pub(crate) fn supersede_row_label(&self, index: usize) -> LocalizedLabel {
        let envelope = self.store.envelope();
        let alternative = self.supersedes.records[index].scope.as_deref().and_then(|scope| envelope.vcs.alternatives.iter().find(|alternative| alternative.id == scope)).map(|alternative| alternative.name.as_str());
        self.supersedes.label(index, alternative)
    }

    /// ⏪️ The history edit a plain `undo` takes back: the newest own undo target of the command log, when it is a history
    /// edit row (the edit or its redo) still in effect. A newer own document, configuration, child or shell target keeps
    /// the plain undo on its own lane; rows of other authors are passed over.
    fn supersede_undo_target(&self, child_applied_tails: &HashSet<String>) -> Option<usize> {
        let local = self.supersede_author();
        let applied_entries = self.supersedes.applied_entries(self.store.supersessions().iter());
        let applied: HashSet<&str> = self.store.applied_edit_ids().iter().map(String::as_str).collect();
        let config_applied: HashSet<&str> = self.config_store.applied_edit_ids().iter().map(String::as_str).collect();
        let envelope = self.store.envelope();
        let own_edit = |edit_id: &str| envelope.vcs.edits.iter().find(|edit| edit.id == edit_id).is_some_and(|edit| edit.actor.as_deref().is_none_or(|actor| actor == local));
        for entry in self.command_log.iter().rev() {
            if let Some(transition_id) = entry.transition_id.as_deref() {
                let Some((_, record)) = self.supersedes.record(transition_id) else { continue };
                if record.actor == local && record.role != SupersedeRole::Undo && self.supersedes.undoable(local, record.entry, &applied_entries) {
                    return Some(record.entry);
                }
                continue;
            }
            let shell = entry.inverse.is_some() && entry.edit_id.is_none() && entry.config_edit_ids.is_empty() && !self.shell_undone.contains(&entry.seq);
            let document = entry.edit_id.as_deref().is_some_and(|id| applied.contains(id) && own_edit(id));
            let config = entry.config_edit_ids.iter().any(|id| config_applied.contains(id.as_str()));
            let child = entry.child_edit_ids.iter().any(|id| child_applied_tails.contains(id));
            if shell || document || config || child {
                return None;
            }
        }
        None
    }

    /// ⏩️ The history edit a plain `redo` re-authors: this replica's newest undone one, while its undo still holds every
    /// input it restored, no own document edit is newer than that undo, and no pending document redo was undone later.
    pub(crate) fn supersede_redo_target(&self) -> Option<(usize, usize)> {
        let local = self.supersede_author();
        let (entry, undo) = self.supersedes.redo_top(local)?;
        let undone_at = self.supersedes.records[undo].timestamp;
        if !self.store.redo_edit_ids().is_empty() && self.supersedes.newest_revert(local).is_some_and(|revert| revert > undone_at) {
            return None;
        }
        let applied: HashSet<&str> = self.store.applied_edit_ids().iter().map(String::as_str).collect();
        let newer_edit = self.store.envelope().vcs.edits.iter().any(|edit| applied.contains(edit.id.as_str()) && edit.actor.as_deref().is_none_or(|actor| actor == local) && edit.mutation_meta.first().is_some_and(|meta| meta.timestamp > undone_at));
        (!newer_edit).then_some((entry, undo))
    }

    /// ↩️ Whether a plain `undo` would take a history edit back now.
    pub(crate) fn supersede_can_undo(&self) -> bool {
        let local = self.supersede_author();
        let applied_entries = self.supersedes.applied_entries(self.store.supersessions().iter());
        self.supersedes.records.iter().enumerate().any(|(index, record)| record.role == SupersedeRole::Edit && record.actor == local && self.supersedes.undoable(local, index, &applied_entries))
    }

    /// ✍️ Authors `inputs` as one `Supersede` in `scope` through the store's one supersede path — the restore of an undo
    /// or the re-authoring of a redo. Answers whether it landed; a refusal (an input no longer applied, or a replay report
    /// that blocks finalizing) changes nothing.
    async fn author_supersede(&mut self, scope: Option<String>, inputs: Vec<protocol::SupersededInput>) -> Result<bool, Fault> {
        let mut typed: Vec<store::SupersedeInput<A::Mutation>> = Vec::with_capacity(inputs.len());
        let mut undecodable = None;
        for input in inputs {
            let replacement = match input.replacement {
                protocol::InputReplacement::Withdrawn => None,
                protocol::InputReplacement::Input { payload, .. } => match <A::Mutation as ::protocol::OpBinary>::decode_op(&payload) {
                    Ok(operation) => Some(operation),
                    Err(error) => {
                        undecodable = Some(error);
                        break;
                    }
                },
            };
            typed.push(store::SupersedeInput { target: input.target, replacement });
        }
        if let Some(error) = undecodable {
            for input in typed {
                if let Some(replacement) = input.replacement {
                    replacement.retire_cold();
                }
            }
            return Err(error.into_fault());
        }
        match self.store.dispatch(ArtifactCommand::Supersede { scope, inputs: typed }).await {
            Ok(_) => {
                self.cache = None;
                Ok(true)
            }
            Err(vcs::VcsError::ValidationFailed(_)) | Err(vcs::VcsError::Rejected { .. }) => Ok(false),
            Err(error) => Err(error.into_fault()),
        }
    }

    /// ⏪️⏩️ A plain `undo` or `redo` whose newest own target is a history edit (design §2: the undo of a finalize is a new
    /// `Supersede` carrying the previous effective input, its redo re-authors the edit's own inputs); `None` leaves the
    /// verb to the document, configuration, child and shell lanes.
    pub(crate) async fn dispatch_supersede_history_action(&mut self, action: &str, meta: &ActionMeta) -> Result<Option<InvocationResult>, Fault> {
        self.refresh_cache().await?;
        let (child_applied_tails, child_has_redo_tail) = self.child_history_tails().await;
        let authored = match action {
            "undo" => self.supersede_undo_target(&child_applied_tails).map(|entry| self.supersedes.restore(entry)),
            "redo" if !child_has_redo_tail => self.supersede_redo_target().map(|(entry, undo)| self.supersedes.redo(entry, undo)),
            _ => None,
        };
        let Some((scope, inputs)) = authored.filter(|(_, inputs)| !inputs.is_empty()) else { return Ok(None) };
        let landed = self.author_supersede(scope, inputs).await?;
        let (events, scope) = if landed { (vec![history_changed_event().await], UiDirtyScope::Full) } else { (Vec::new(), UiDirtyScope::None) };
        Ok(Some(Self::empty_result(action, meta, Vec::new(), events, scope).await))
    }

    /// ⏪️ `revertToCommand` on a history-edit row: takes back the history edit the row acts for, when it is this replica's
    /// and still in effect.
    pub(crate) async fn revert_history_edit_row(&mut self, transition_id: &str, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
        let local = self.supersede_author().to_string();
        let applied_entries = self.supersedes.applied_entries(self.store.supersessions().iter());
        let target = self.supersedes.record(transition_id).filter(|(_, record)| record.role != SupersedeRole::Undo && record.actor == local).map(|(_, record)| record.entry).filter(|entry| self.supersedes.undoable(&local, *entry, &applied_entries));
        let landed = match target.map(|entry| self.supersedes.restore(entry)).filter(|(_, inputs)| !inputs.is_empty()) {
            Some((scope, inputs)) => self.author_supersede(scope, inputs).await?,
            None => false,
        };
        let (events, scope) = if landed { (vec![history_changed_event().await], UiDirtyScope::Full) } else { (Vec::new(), UiDirtyScope::None) };
        Ok(Self::empty_result(REVERT_TO_COMMAND_ACTION_ID, meta, Vec::new(), events, scope).await)
    }
}
//#endregion 🔖️Supersessions

//#region 🔖️Panel
/// 💬️ The framework's history-panel copy, EN and DE, no default locale.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HistoryPanelText {
    Actions,
    Commands,
    Undo,
    Redo,
    CommitCheckpoint,
    CheckIn,
    CreateAlternative,
    Filter,
    FilterAll,
    FilterWithoutMutations,
    FilterOnlyMutations,
    Backwards,
    TimeTravel,
    Edit,
    Accept,
    Discard,
    Withdraw,
    Finalize,
    Exit,
    CancelReplay,
    NextProblem,
    UseSelection,
    NothingSelected,
    Remove,
    Inputs,
    NoInputs,
    Pending,
    Edited,
    Superseded,
    Withdrawn,
    Draft,
    Hex,
    Opacity,
    ClearInput,
    Cleared,
    Alternatives,
    Current,
    BranchedBy,
    EditedHistory,
    Switch,
    Trunk,
    MoreInputs,
}

impl HistoryPanelText {
    pub(crate) fn text(self, locale: Locale) -> &'static str {
        match locale {
            Locale::En => match self {
                Self::Actions => "Actions",
                Self::Commands => "Commands",
                Self::Undo => "Undo",
                Self::Redo => "Redo",
                Self::CommitCheckpoint => "Commit Checkpoint",
                Self::CheckIn => "Check In",
                Self::CreateAlternative => "Create Alternative",
                Self::Filter => "Filter",
                Self::FilterAll => "All",
                Self::FilterWithoutMutations => "Without Operations",
                Self::FilterOnlyMutations => "Only Operations",
                Self::Backwards => "Backwards",
                Self::TimeTravel => "History editing",
                Self::Edit => "Edit",
                Self::Accept => "Accept",
                Self::Discard => "Discard",
                Self::Withdraw => "Withdraw",
                Self::Finalize => "Finalize",
                Self::Exit => "Exit",
                Self::CancelReplay => "Cancel replay",
                Self::NextProblem => "Next problem",
                Self::UseSelection => "Use selection",
                Self::NothingSelected => "Nothing referenced",
                Self::Remove => "Remove",
                Self::Inputs => "Inputs",
                Self::NoInputs => "No editable inputs",
                Self::Pending => "Not applied while editing",
                Self::Edited => "Edited",
                Self::Superseded => "Replaced",
                Self::Withdrawn => "Withdrawn",
                Self::Draft => "Draft",
                Self::Hex => "Hex code",
                Self::Opacity => "Opacity",
                Self::ClearInput => "Clear {label}",
                Self::Cleared => "Cleared (no value)",
                Self::Alternatives => "Alternatives",
                Self::Current => "Current",
                Self::BranchedBy => "Branched by {author}, {time}",
                Self::EditedHistory => "Edited history",
                Self::Switch => "Switch",
                Self::Trunk => "Main line",
                Self::MoreInputs => "{count} more inputs are not shown",
            },
            Locale::De => match self {
                Self::Actions => "Aktionen",
                Self::Commands => "Befehle",
                Self::Undo => "Rückgängig",
                Self::Redo => "Wiederholen",
                Self::CommitCheckpoint => "Checkpoint",
                Self::CheckIn => "Einchecken",
                Self::CreateAlternative => "Alternative erstellen",
                Self::Filter => "Filter",
                Self::FilterAll => "Alle",
                Self::FilterWithoutMutations => "Ohne Operationen",
                Self::FilterOnlyMutations => "Nur Operationen",
                Self::Backwards => "Zurück bis hier",
                Self::TimeTravel => "Verlaufsbearbeitung",
                Self::Edit => "Bearbeiten",
                Self::Accept => "Übernehmen",
                Self::Discard => "Verwerfen",
                Self::Withdraw => "Zurückziehen",
                Self::Finalize => "Abschließen",
                Self::Exit => "Beenden",
                Self::CancelReplay => "Neuanwendung abbrechen",
                Self::NextProblem => "Nächstes Problem",
                Self::UseSelection => "Auswahl verwenden",
                Self::NothingSelected => "Nichts referenziert",
                Self::Remove => "Entfernen",
                Self::Inputs => "Eingaben",
                Self::NoInputs => "Keine bearbeitbaren Eingaben",
                Self::Pending => "Beim Bearbeiten nicht angewendet",
                Self::Edited => "Bearbeitet",
                Self::Superseded => "Ersetzt",
                Self::Withdrawn => "Zurückgezogen",
                Self::Draft => "Entwurf",
                Self::Hex => "Hex-Code",
                Self::Opacity => "Deckkraft",
                Self::ClearInput => "{label} leeren",
                Self::Cleared => "Geleert (kein Wert)",
                Self::Alternatives => "Alternativen",
                Self::Current => "Aktuell",
                Self::BranchedBy => "Abgezweigt von {author}, {time}",
                Self::EditedHistory => "Bearbeiteter Verlauf",
                Self::Switch => "Wechseln",
                Self::Trunk => "Hauptlinie",
                Self::MoreInputs => "{count} weitere Eingaben werden nicht angezeigt",
            },
        }
    }
}

/// 🚦️ A severity's word, EN and DE; a reader hears it, colour is never its only carrier.
pub(crate) fn history_severity_text(level: dsl::Severity, locale: Locale) -> &'static str {
    match (locale, level) {
        (Locale::En, dsl::Severity::Info) => "Info",
        (Locale::En, dsl::Severity::Warning) => "Warning",
        (Locale::En, dsl::Severity::Error) => "Error",
        (Locale::En, dsl::Severity::Fatal) => "Fatal",
        (Locale::De, dsl::Severity::Info) => "Info",
        (Locale::De, dsl::Severity::Warning) => "Warnung",
        (Locale::De, dsl::Severity::Error) => "Fehler",
        (Locale::De, dsl::Severity::Fatal) => "Kritisch",
    }
}

/// 🏷️ A frozen `mutation.*` outcome code's words, EN and DE; any other code is shown as the data it is.
pub(crate) fn history_code_text(code: &str, locale: Locale) -> &str {
    let known = match code {
        "mutation.target-missing" => Some(("Target missing", "Ziel fehlt")),
        "mutation.target-referenced" => Some(("Target still referenced", "Ziel wird noch referenziert")),
        "mutation.target-mismatch" => Some(("Inconsistent with the target", "Widerspricht dem Ziel")),
        "mutation.no-op" => Some(("No change", "Keine Änderung")),
        "mutation.partial" => Some(("Partially applied", "Teilweise angewendet")),
        "mutation.clamped" => Some(("Clamped", "Begrenzt")),
        "mutation.duplicate-id" => Some(("Duplicate id", "ID bereits vergeben")),
        "mutation.invariant" => Some(("Invalid state", "Ungültiger Zustand")),
        "mutation.cascade" => Some(("Cascaded", "Folgeänderung")),
        code if code.starts_with("mutation.apply.") => Some(("Could not apply", "Nicht anwendbar")),
        _ => None,
    };
    match (known, locale) {
        (Some((en, _)), Locale::En) => en,
        (Some((_, de)), Locale::De) => de,
        (None, _) => code,
    }
}

/// 🎨️ The tone and icon of a severity (none: applied cleanly).
pub(crate) fn history_severity_tone(level: Option<dsl::Severity>) -> (Tone, &'static str) {
    match level {
        None => (Tone::Success, "check"),
        Some(dsl::Severity::Info) => (Tone::Info, "info"),
        Some(dsl::Severity::Warning) => (Tone::Warning, "triangle-alert"),
        Some(dsl::Severity::Error) => (Tone::Danger, "alert-circle"),
        Some(dsl::Severity::Fatal) => (Tone::Danger, "x"),
    }
}

/// 📝️ One line naming a mutation's state and its first message: severity word, then the localized code; editing
/// states lead.
pub(crate) fn history_mutation_description(entry: &HistoryMutationEntry, locale: Locale) -> String {
    let mut parts: Vec<&str> = Vec::new();
    if entry.pending {
        parts.push(HistoryPanelText::Pending.text(locale));
    }
    if entry.edited {
        parts.push(HistoryPanelText::Edited.text(locale));
    }
    if entry.withdrawn {
        parts.push(HistoryPanelText::Withdrawn.text(locale));
    } else if entry.superseded {
        parts.push(HistoryPanelText::Superseded.text(locale));
    }
    let mut line = parts.join(" \u{b7} ");
    if let Some(worst) = entry.worst {
        let first = entry.messages.iter().find(|message| message.level == worst);
        let severity = history_severity_text(worst, locale);
        let detail = first.map(|message| history_code_text(&message.code, locale)).unwrap_or_default();
        if !line.is_empty() {
            line.push_str(" \u{b7} ");
        }
        line.push_str(severity);
        if !detail.is_empty() {
            line.push_str(": ");
            line.push_str(detail);
        }
    }
    line
}

/// 🔘️ A button dispatching `verb` on `controller_id` with `args`.
fn time_travel_button(controller_id: &str, id: &str, label: &str, icon: &str, verb: &str, args: Option<UiValue>, enabled: bool) -> UiAssemblyResult<BuiltNode> {
    let action = ActionId::try_v1(controller_id, verb).ok_or_else(|| ui_assembly_error("time-travel-panel.action-id"))?;
    let builder = button(ui_label(label, "time-travel-panel.button-label")?).icon(ui_text(icon, "time-travel-panel.button-icon")?).disabled(!enabled).try_id(id).map_err(|_| ui_assembly_error("time-travel-panel.button-id"))?;
    let builder = match args {
        Some(args) => builder.try_on_with(Trigger::Activate, action, args),
        None => builder.try_on(Trigger::Activate, action),
    };
    builder.map_err(|_| ui_assembly_error("time-travel-panel.button-binding"))?.try_build().map_err(|_| ui_assembly_error("time-travel-panel.button"))
}

/// 🧿️ The `{generation}` argument map every session button carries (a stale press is a silent `timeTravel.stale`).
fn time_travel_generation_args(generation: u32) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("time-travel-panel.args"))?;
    args.push(HISTORY_EDIT_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).map_err(|_| ui_assembly_error("time-travel-panel.args"))?;
    Ok(UiValue::Map(args.finish()))
}

/// 🧭️ The `{generation, path}` argument map of one input control.
fn time_travel_input_args(generation: u32, path: &str) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("time-travel-panel.input-args"))?;
    args.push(HISTORY_EDIT_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).map_err(|_| ui_assembly_error("time-travel-panel.input-args"))?;
    args.push(HISTORY_EDIT_ARG_PATH.to_string(), UiValue::Text(UiText::clipped(path))).map_err(|_| ui_assembly_error("time-travel-panel.input-args"))?;
    Ok(UiValue::Map(args.finish()))
}

/// 🧽️ The `{generation, path, value: null}` argument map a nullable input's Clear button carries.
fn time_travel_clear_args(generation: u32, path: &str) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("time-travel-panel.clear-args"))?;
    args.push(HISTORY_EDIT_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).map_err(|_| ui_assembly_error("time-travel-panel.clear-args"))?;
    args.push(HISTORY_EDIT_ARG_PATH.to_string(), UiValue::Text(UiText::clipped(path))).map_err(|_| ui_assembly_error("time-travel-panel.clear-args"))?;
    args.push(HISTORY_EDIT_ARG_VALUE.to_string(), UiValue::Null).map_err(|_| ui_assembly_error("time-travel-panel.clear-args"))?;
    Ok(UiValue::Map(args.finish()))
}

/// 🌲️ One tree row holding one inline control — the history body's one vocabulary for a control, the shape both hosts
/// mount: a tree section's `tree_item`s, each with at most one single control child (React mounts a row's non-row
/// children as its controls and activates a row's one activatable control; wgpu mounts one input, select, toggle,
/// button, key-value list, slider, stepper, ring or icon select per row and nests every other child as a row).
fn time_travel_control_row(id: &str, label: &str, icon: &str, control: BuiltNode, enabled: bool, tone: Option<Tone>, description: Option<&str>) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let mut row = ui::tree_item(Label(UiText::clipped(label))).icon(ui_text(icon, "time-travel-panel.row-icon")?).disabled(!enabled);
    if let Some(tone) = tone {
        row = row.tone(tone);
    }
    if let Some(description) = description {
        row = row.description(UiText::clipped(description));
    }
    row.try_id(id).map_err(|_| error("time-travel-panel.row-id"))?.try_child(control).map_err(|_| error("time-travel-panel.row-child"))?.try_build().map_err(|_| error("time-travel-panel.row"))
}

/// 🌲️ A button row: the button keeps its own id (`{id}`) and the row wraps it as `{id}.row`.
fn time_travel_button_row(controller_id: &str, id: &str, label: &str, icon: &str, verb: &str, args: Option<UiValue>, enabled: bool) -> UiAssemblyResult<BuiltNode> {
    time_travel_control_row(&format!("{id}.row"), label, icon, time_travel_button(controller_id, id, label, icon, verb, args, enabled)?, enabled, None, None)
}

/// 🪧️ The session band as the history body's first tree section `framework.history.timeTravel`, one row each: the stage
/// status with the edited mutation and the report's worst severity, replay progress in words, the last fault, then Cancel
/// replay, Next problem, Finalize (disabled with its reason while refused), Replay again and Exit as the stage offers
/// them. Rows carry text in their labels, never as a child, so every host renders them; the shell's own band is the one
/// polite live region that announces the session and draws its progress bar.
pub(crate) fn time_travel_band_section(panel: &TimeTravelPanel, controller_id: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let scope = "framework.history.timeTravel";
    let status = &panel.status;
    let mut rows = BuiltChildren::default();
    let mut stage_line = panel.review.map_or_else(|| panel.stage.label(), TimeTravelReview::label).localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
    if let Some(label) = status.target_label.as_ref() {
        stage_line = format!("{stage_line}: {}", label.resolve(Terminology::Native, locale));
    }
    if let Some(worst) = status.worst {
        stage_line = format!("{stage_line} \u{b7} {}", history_severity_text(worst, locale));
    }
    let (tone, _) = history_severity_tone(status.worst);
    let tone = if status.worst.is_some() { tone } else { Tone::Neutral };
    let stage_row = ui::tree_item(Label(UiText::clipped(&stage_line))).icon(ui_text("clock", "time-travel-panel.status-icon")?).tone(tone).try_id(format!("{scope}.status")).map_err(|_| error("time-travel-panel.status-id"))?.try_build().map_err(|_| error("time-travel-panel.status"))?;
    rows.try_push(stage_row).map_err(|_| error("time-travel-panel.rows"))?;
    if let (Some(done), Some(total)) = (status.done, status.total) {
        let value_text = TimeTravelLabel::ReplayProgressValueText.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).replace("{done}", &done.to_string()).replace("{total}", &total.to_string());
        let progress_row = ui::tree_item(Label(UiText::clipped(&value_text))).icon(ui_text("loader-2", "time-travel-panel.progress-icon")?).try_id(format!("{scope}.progress")).map_err(|_| error("time-travel-panel.progress-id"))?.try_build().map_err(|_| error("time-travel-panel.progress"))?;
        rows.try_push(progress_row).map_err(|_| error("time-travel-panel.rows"))?;
    }
    if let Some(fault) = status.fault.as_deref() {
        let label = TimeTravelLabel::for_fault(fault).or_else(|| TimeTravelRefusal::parse(fault).map(TimeTravelRefusal::label)).map_or_else(|| fault.to_string(), |label| label.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string());
        let node = ui::tree_item(Label(UiText::clipped(&label))).icon(ui_text("triangle-alert", "time-travel-panel.fault-icon")?).tone(Tone::Danger).try_id(format!("{scope}.fault")).map_err(|_| error("time-travel-panel.fault-id"))?.try_build().map_err(|_| error("time-travel-panel.fault"))?;
        rows.try_push(node).map_err(|_| error("time-travel-panel.rows"))?;
    }
    let generation = status.generation;
    if panel.stage == TimeTravelStage::Replaying {
        rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.cancelReplay"), HistoryPanelText::CancelReplay.text(locale), "square", HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, Some(time_travel_generation_args(generation)?), true)?).map_err(|_| error("time-travel-panel.rows"))?;
    }
    if let Some(problem) = panel.next_problem.as_deref() {
        let mut args = UiMapBuilder::try_new().ok_or_else(|| error("time-travel-panel.problem-args"))?;
        args.push(HISTORY_EDIT_ARG_MUTATION_ID.to_string(), UiValue::Text(UiText::clipped(problem))).map_err(|_| error("time-travel-panel.problem-args"))?;
        rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.nextProblem"), HistoryPanelText::NextProblem.text(locale), "arrow-right", HISTORY_EDIT_BEGIN_ACTION_ID, Some(UiValue::Map(args.finish())), true)?).map_err(|_| error("time-travel-panel.rows"))?;
    }
    if matches!(panel.stage, TimeTravelStage::Reviewing | TimeTravelStage::Choosing) {
        let refused = panel.finalize_refusal.map(|refusal| refusal.label().localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string());
        let label = HistoryPanelText::Finalize.text(locale);
        let mut finalize = button(ui_label(label, "time-travel-panel.finalize-label")?).icon(ui_text("list-checks", "time-travel-panel.finalize-icon")?).disabled(refused.is_some()).try_id(format!("{scope}.finalize")).map_err(|_| error("time-travel-panel.finalize-id"))?;
        if let Some(reason) = refused.as_deref() {
            finalize = finalize.try_describe(reason).map_err(|_| error("time-travel-panel.finalize-description"))?;
        }
        let action = ActionId::try_v1(controller_id, HISTORY_EDIT_FINALIZE_ACTION_ID).ok_or_else(|| error("time-travel-panel.action-id"))?;
        let finalize = finalize.try_on_with(Trigger::Activate, action, time_travel_generation_args(generation)?).map_err(|_| error("time-travel-panel.finalize-binding"))?.try_build().map_err(|_| error("time-travel-panel.finalize"))?;
        rows.try_push(time_travel_control_row(&format!("{scope}.finalize.row"), label, "list-checks", finalize, refused.is_none(), None, refused.as_deref())?).map_err(|_| error("time-travel-panel.rows"))?;
    }
    if panel.stage == TimeTravelStage::Reviewing {
        let rerun = TimeTravelLabel::ActionRerun.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
        rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.rerun"), &rerun, "skip-forward", HISTORY_EDIT_RERUN_ACTION_ID, Some(time_travel_generation_args(generation)?), panel.rerun_refusal.is_none())?).map_err(|_| error("time-travel-panel.rows"))?;
    }
    rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.exit"), HistoryPanelText::Exit.text(locale), "rotate-ccw", HISTORY_EDIT_EXIT_ACTION_ID, None, panel.stage != TimeTravelStage::Finalizing)?).map_err(|_| error("time-travel-panel.rows"))?;
    tree_section(ui_label(HistoryPanelText::TimeTravel.text(locale), "time-travel-panel.label")?).default_open(true).try_id(scope).map_err(|_| error("time-travel-panel.id"))?.try_children(rows).map_err(|_| error("time-travel-panel.rows"))?.try_build().map_err(|_| error("time-travel-panel.build"))
}

/// ✏️ The draft editor as two tree sections: `framework.history.editor` — the edited mutation with its draft's own
/// outcome, why no input is editable, then Accept, Discard and Withdraw — and `framework.history.editor.inputs`, one row
/// per input (the control its descriptor derives, bound to `historyEditInput{generation, path}` or, for a reference, to
/// "use selection"). The inputs are never windowed: every row up to [`TIME_TRAVEL_EDITOR_INPUT_ROWS`] is materialised the
/// moment the section shows, and a closing row counts the rest.
pub(crate) fn time_travel_editor_sections(panel: &TimeTravelPanel, editor: &TimeTravelEditorPanel, controller_id: &str, locale: Locale) -> UiAssemblyResult<[BuiltNode; 2]> {
    let error = ui_assembly_error;
    let scope = "framework.history.editor";
    let generation = panel.status.generation;
    let mut rows = BuiltChildren::default();
    let mut heading = format!("{}: {}", HistoryPanelText::Draft.text(locale), editor.label.resolve(Terminology::Native, locale));
    if editor.withdrawn {
        heading = format!("{heading} \u{b7} {}", HistoryPanelText::Withdrawn.text(locale));
    }
    let worst = editor.outcome.iter().map(|message| message.level).max();
    if let Some(worst) = worst {
        let detail = editor.outcome.iter().find(|message| message.level == worst).map(|message| history_code_text(&message.code.0, locale)).unwrap_or_default();
        heading = format!("{heading} \u{b7} {}: {detail}", history_severity_text(worst, locale));
    }
    let (tone, icon) = history_severity_tone(worst);
    let head = ui::tree_item(Label(UiText::clipped(&heading))).icon(ui_text(icon, "time-travel-panel.editor-icon")?).tone(tone).try_id(format!("{scope}.target")).map_err(|_| error("time-travel-panel.editor-target-id"))?.try_build().map_err(|_| error("time-travel-panel.editor-target"))?;
    rows.try_push(head).map_err(|_| error("time-travel-panel.editor-rows"))?;
    if let Some(reason) = editor.inputs_refused.as_deref() {
        let node = ui::tree_item(Label(UiText::clipped(HistoryPanelText::NoInputs.text(locale)))).description(UiText::clipped(reason)).tone(Tone::Warning).try_id(format!("{scope}.refused")).map_err(|_| error("time-travel-panel.editor-refused-id"))?.try_build().map_err(|_| error("time-travel-panel.editor-refused"))?;
        rows.try_push(node).map_err(|_| error("time-travel-panel.editor-rows"))?;
    }
    rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.accept"), HistoryPanelText::Accept.text(locale), "check", HISTORY_EDIT_ACCEPT_ACTION_ID, Some(time_travel_generation_args(generation)?), true)?).map_err(|_| error("time-travel-panel.editor-rows"))?;
    rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.discard"), HistoryPanelText::Discard.text(locale), "x", HISTORY_EDIT_DISCARD_ACTION_ID, Some(time_travel_generation_args(generation)?), true)?).map_err(|_| error("time-travel-panel.editor-rows"))?;
    rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.withdraw"), HistoryPanelText::Withdraw.text(locale), "eye-off", HISTORY_EDIT_WITHDRAW_ACTION_ID, Some(time_travel_generation_args(generation)?), !editor.withdrawn)?).map_err(|_| error("time-travel-panel.editor-rows"))?;
    let section = tree_section(ui_label(editor.label.resolve(Terminology::Native, locale), "time-travel-panel.editor-label")?).default_open(true).try_id(scope).map_err(|_| error("time-travel-panel.editor-id"))?.try_children(rows).map_err(|_| error("time-travel-panel.editor-rows"))?.try_build().map_err(|_| error("time-travel-panel.editor-build"))?;
    let mut input_rows = BuiltChildren::default();
    for row in editor.rows.iter().take(TIME_TRAVEL_EDITOR_INPUT_ROWS) {
        input_rows.try_push(time_travel_input_row(row, editor.refused.as_ref(), controller_id, generation, locale)?).map_err(|_| error("time-travel-panel.input-rows"))?;
    }
    if let Some(hidden) = editor.rows.len().checked_sub(TIME_TRAVEL_EDITOR_INPUT_ROWS).filter(|hidden| *hidden > 0) {
        let more = ui::tree_item(Label(UiText::clipped(&HistoryPanelText::MoreInputs.text(locale).replace("{count}", &hidden.to_string())))).try_id(format!("{scope}.inputs.more")).map_err(|_| error("time-travel-panel.inputs-more-id"))?.try_build().map_err(|_| error("time-travel-panel.inputs-more"))?;
        input_rows.try_push(more).map_err(|_| error("time-travel-panel.input-rows"))?;
    }
    let inputs = tree_section(ui_label(HistoryPanelText::Inputs.text(locale), "time-travel-panel.inputs-label")?).default_open(true).try_id("framework.history.editor.inputs").map_err(|_| error("time-travel-panel.inputs-id"))?.try_children(input_rows).map_err(|_| error("time-travel-panel.input-rows"))?.try_build().map_err(|_| error("time-travel-panel.inputs"))?;
    Ok([section, inputs])
}

/// 🎨️ The colour control of an sRGB vector input — the one place that picks its widget: the UI contract's
/// `color_input` recipe. Its swatch and hex field commit the colour text to `pointer` (a text without alpha keeps the
/// stored alpha); with `alpha`, its opacity slider commits the fourth component to `pointer/3`.
fn time_travel_color_control(value: &DslValue, alpha: bool, id: &str, label: &str, locale: Locale, action: &ActionId, generation: u32, pointer: &str) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let labels = ColorInputLabels { hex: Label(UiText::clipped(HistoryPanelText::Hex.text(locale))), alpha: Label(UiText::clipped(HistoryPanelText::Opacity.text(locale))) };
    let alpha_args = time_travel_input_args(generation, &format!("{pointer}/3"))?;
    let bind_hex = |field: InputBuilder| match time_travel_input_args(generation, pointer) {
        Ok(args) => field.try_on_with(Trigger::Commit, action.clone(), args).map_err(|(field, _)| field),
        Err(_) => Err(field),
    };
    let bind_alpha = |slider: SliderBuilder| slider.try_on_with(Trigger::Change, action.clone(), alpha_args).map_err(|(slider, _)| slider);
    let recipe = color_input(Label(UiText::clipped(label)), labels).try_color(&time_travel_color_rgba(value), alpha, bind_hex, bind_alpha).map_err(|_| error("time-travel-panel.color-input"))?;
    Ok(BuiltNode::from(recipe.try_id(id).map_err(|_| error("time-travel-panel.input-id"))?))
}

/// 🎛️ One input row: a tree item reading the row's label (a refused value names its reason as the description) holding
/// the control its descriptor derives (`ActionArgDef::control`), bound to the row's pointer; its id is the pointer with
/// `.` for `/` under `framework.history.editor.input`. A `nullable` input nests a Clear row holding one Clear button
/// ("Clear <label>") that drafts `null` — disabled, and the input row described as cleared, while the input already is.
fn time_travel_input_row(row: &TimeTravelInputRow, refused: Option<&(String, String)>, controller_id: &str, generation: u32, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let (arg, value, pointer) = (&row.input, &row.value, row.pointer.as_str());
    let label = row.label.resolve(Terminology::Native, locale);
    let id = format!("framework.history.editor.input{}", pointer.replace('/', "."));
    let action = ActionId::try_v1(controller_id, HISTORY_EDIT_INPUT_ACTION_ID).ok_or_else(|| error("time-travel-panel.input-action"))?;
    let args = time_travel_input_args(generation, pointer)?;
    let number = value.as_f64().unwrap_or(0.0);
    let control: BuiltNode = match arg.control() {
        ActionArgControl::Slider { min, max, step, unit, snaps, .. } => {
            let mut builder = slider(number).min(min).max(max);
            if let Some(step) = step {
                builder = builder.step(step);
            }
            if let Some(unit) = unit {
                builder = builder.unit(UiText::clipped(&unit));
            }
            for snap in snaps {
                builder = builder.try_snap(snap).unwrap_or_else(|builder| builder);
            }
            builder.try_id(&id).map_err(|_| error("time-travel-panel.input-id"))?.try_label(label).map_err(|_| error("time-travel-panel.input-label"))?.try_on_with(Trigger::Change, action, args).map_err(|_| error("time-travel-panel.input-binding"))?.try_build().map_err(|_| error("time-travel-panel.input"))?
        }
        ActionArgControl::Dial { min, max, step, .. } => {
            let mut builder = slider(number).min(min).max(max);
            if let Some(step) = step {
                builder = builder.step(step);
            }
            builder.try_id(&id).map_err(|_| error("time-travel-panel.input-id"))?.try_label(label).map_err(|_| error("time-travel-panel.input-label"))?.try_on_with(Trigger::Change, action, args).map_err(|_| error("time-travel-panel.input-binding"))?.try_build().map_err(|_| error("time-travel-panel.input"))?
        }
        ActionArgControl::Stepper { min, max, step, precision, .. } => {
            let mut builder = number_stepper(number).step(step.unwrap_or(1.0));
            if let Some(min) = min {
                builder = builder.min(min);
            }
            if let Some(max) = max {
                builder = builder.max(max);
            }
            if let Some(precision) = precision {
                builder = builder.precision(u16::try_from(precision).unwrap_or(u16::MAX));
            }
            builder.try_id(&id).map_err(|_| error("time-travel-panel.input-id"))?.try_label(label).map_err(|_| error("time-travel-panel.input-label"))?.try_on_with(Trigger::Change, action, args).map_err(|_| error("time-travel-panel.input-binding"))?.try_build().map_err(|_| error("time-travel-panel.input"))?
        }
        ActionArgControl::Number { min, max, step, precision, .. } => {
            let mut builder = input(InputKind::Number).commit(UiText::clipped("blur"));
            if let Some(min) = min {
                builder = builder.min(min);
            }
            if let Some(max) = max {
                builder = builder.max(max);
            }
            if let Some(step) = step {
                builder = builder.step(step);
            }
            if let Some(precision) = precision {
                builder = builder.precision(u16::try_from(precision).unwrap_or(u16::MAX));
            }
            builder.number(number).try_id(&id).map_err(|_| error("time-travel-panel.input-id"))?.try_label(label).map_err(|_| error("time-travel-panel.input-label"))?.try_on_with(Trigger::Commit, action, args).map_err(|_| error("time-travel-panel.input-binding"))?.try_build().map_err(|_| error("time-travel-panel.input"))?
        }
        ActionArgControl::Toggle => toggle(value.as_bool().unwrap_or(false)).try_id(&id).map_err(|_| error("time-travel-panel.input-id"))?.try_label(label).map_err(|_| error("time-travel-panel.input-label"))?.try_on_with(Trigger::Change, action, args).map_err(|_| error("time-travel-panel.input-binding"))?.try_build().map_err(|_| error("time-travel-panel.input"))?,
        ActionArgControl::Select { options } | ActionArgControl::Segmented { options } => {
            let mut builder = select(UiText::clipped(value.as_str().unwrap_or_default()));
            for option in options.iter().take(UI_FIXED_LIST_ITEMS) {
                builder = builder.try_item(UiText::clipped(&option.value), Label(UiText::clipped(option.label.resolve(Terminology::Native, locale)))).map_err(|_| error("time-travel-panel.input-options"))?;
            }
            builder.try_id(&id).map_err(|_| error("time-travel-panel.input-id"))?.try_label(label).map_err(|_| error("time-travel-panel.input-label"))?.try_on_with(Trigger::Change, action, args).map_err(|_| error("time-travel-panel.input-binding"))?.try_build().map_err(|_| error("time-travel-panel.input"))?
        }
        ActionArgControl::Vector { dims, min, max, unit, step, precision, snaps, .. } => {
            let mut builder = vector_input(Label(UiText::clipped(label)));
            for snap in snaps {
                builder = builder.try_snap(snap).unwrap_or_else(|builder| builder);
            }
            if let Some(step) = step {
                builder = builder.step(step);
            }
            if let Some(min) = min {
                builder = builder.min(min);
            }
            if let Some(max) = max {
                builder = builder.max(max);
            }
            if let Some(precision) = precision {
                builder = builder.precision(u16::try_from(precision).unwrap_or(u16::MAX));
            }
            if let Some(unit) = unit {
                builder = builder.unit(UiText::clipped(&unit));
            }
            for axis in 0..dims {
                let axis_value = value.as_array().and_then(|items| items.get(axis as usize)).and_then(DslValue::as_f64).unwrap_or(0.0);
                let axis_args = time_travel_input_args(generation, &format!("{pointer}/{axis}"))?;
                let axis_action = action.clone();
                builder = builder
                    .try_axis(&format!("{id}.{axis}"), Label(UiText::clipped(&["x", "y", "z", "w"].get(axis as usize).map_or_else(|| axis.to_string(), |name| (*name).to_string()))), axis_value, |field| field.commit(UiText::clipped("blur")).try_on_with(Trigger::Commit, axis_action, axis_args).map_err(|(field, _)| field))
                    .map_err(|_| error("time-travel-panel.vector-axis"))?;
            }
            BuiltNode::from(builder.try_id(&id).map_err(|_| error("time-travel-panel.input-id"))?)
        }
        ActionArgControl::Color { alpha } => time_travel_color_control(value, alpha, &id, label, locale, &action, generation, pointer)?,
        ActionArgControl::Reference { domain, .. } => {
            let labels = ReferenceListLabels { use_selection: Label(UiText::clipped(HistoryPanelText::UseSelection.text(locale))), empty: Label(UiText::clipped(HistoryPanelText::NothingSelected.text(locale))) };
            let mut builder = reference_list(Label(UiText::clipped(label)), labels);
            let ids: Vec<String> = match value {
                DslValue::Array(items) => items.iter().filter_map(reference_id_text).collect(),
                other => reference_id_text(other).into_iter().collect(),
            };
            for (position, reference) in ids.iter().take(TIME_TRAVEL_PANEL_CHIPS).enumerate() {
                let mut remaining = UiListBuilder::try_new().ok_or_else(|| error("time-travel-panel.reference-args"))?;
                for (_, kept) in ids.iter().enumerate().filter(|(other, _)| *other != position) {
                    remaining.push(UiValue::Text(UiText::clipped(kept))).map_err(|_| error("time-travel-panel.reference-args"))?;
                }
                let mut remove = UiMapBuilder::try_new().ok_or_else(|| error("time-travel-panel.reference-args"))?;
                remove.push(HISTORY_EDIT_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).map_err(|_| error("time-travel-panel.reference-args"))?;
                remove.push(HISTORY_EDIT_ARG_PATH.to_string(), UiValue::Text(UiText::clipped(pointer))).map_err(|_| error("time-travel-panel.reference-args"))?;
                remove.push(HISTORY_EDIT_ARG_VALUE.to_string(), UiValue::List(remaining.finish())).map_err(|_| error("time-travel-panel.reference-args"))?;
                let remove_label = format!("{} {reference}", HistoryPanelText::Remove.text(locale));
                builder = builder.try_chip(&format!("{id}.chip.{position}"), Label(UiText::clipped(reference)), &remove_label, action.clone(), Some(UiValue::Map(remove.finish()))).map_err(|_| error("time-travel-panel.reference-chip"))?;
            }
            let use_selection = ActionId::try_v1(controller_id, HISTORY_EDIT_USE_SELECTION_ACTION_ID).ok_or_else(|| error("time-travel-panel.selection-action"))?;
            let mut selection_args = UiMapBuilder::try_new().ok_or_else(|| error("time-travel-panel.selection-args"))?;
            selection_args.push(HISTORY_EDIT_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).map_err(|_| error("time-travel-panel.selection-args"))?;
            selection_args.push(HISTORY_EDIT_ARG_PATH.to_string(), UiValue::Text(UiText::clipped(pointer))).map_err(|_| error("time-travel-panel.selection-args"))?;
            builder = builder.try_use_selection(use_selection, Some(UiValue::Map(selection_args.finish())), domain.is_some()).map_err(|_| error("time-travel-panel.use-selection"))?;
            BuiltNode::from(builder.try_id(&id).map_err(|_| error("time-travel-panel.input-id"))?)
        }
        _ => {
            let shown = match value {
                DslValue::String(text) => text.clone(),
                other => time_travel_json(other),
            };
            input(InputKind::Text).value(UiText::clipped(&shown)).commit(UiText::clipped("blur")).try_id(&id).map_err(|_| error("time-travel-panel.input-id"))?.try_label(label).map_err(|_| error("time-travel-panel.input-label"))?.try_on_with(Trigger::Commit, action, args).map_err(|_| error("time-travel-panel.input-binding"))?.try_build().map_err(|_| error("time-travel-panel.input"))?
        }
    };
    let cleared = arg.nullable && value.is_null();
    let mut item = ui::tree_item(Label(UiText::clipped(label))).try_id(format!("{id}.row")).map_err(|_| error("time-travel-panel.input-row-id"))?;
    if let Some((_, reason)) = refused.filter(|(path, _)| path == pointer) {
        item = item.description(UiText::clipped(reason)).tone(Tone::Danger);
    } else if cleared {
        item = item.description(UiText::clipped(HistoryPanelText::Cleared.text(locale)));
    } else if let Some(description) = arg.description.as_ref() {
        item = item.description(UiText::clipped(description.resolve(Terminology::Native, locale)));
    }
    item = item.try_child(control).map_err(|_| error("time-travel-panel.input-row-child"))?;
    if arg.nullable {
        let clear_label = HistoryPanelText::ClearInput.text(locale).replace("{label}", label);
        let clear = time_travel_button_row(controller_id, &format!("{id}.clear"), &clear_label, "eraser", HISTORY_EDIT_INPUT_ACTION_ID, Some(time_travel_clear_args(generation, pointer)?), !cleared)?;
        item = item.default_open(true).try_child(clear).map_err(|_| error("time-travel-panel.input-row-clear"))?;
    }
    item.try_build().map_err(|_| error("time-travel-panel.input-row"))
}
//#endregion 🔖️Panel
