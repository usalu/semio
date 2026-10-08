//! ⏪️ OS runtime of non-destructive history editing: the per-instance [`TimeTravelLedger`] hosting the pure
//! `⏪️time-travel` session reducer, the host-driven `historyEdit*` verbs, the session preview every render seam reads,
//! the Report replay stepped per reactor turn, the finalize commit, the per-mutation history overlay and the history
//! panel's time-travel sections.
//!
//! Contract: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/📋️design.md` §4, §7 and §10.
//! Domain-neutral session: `semio_framework_time_travel`; store read API: `ArtifactStore::{state_before,
//! begin_report_replay, replay_report, commit_finished_replay}`.

use super::*;
use semio_framework_os_kernel::HistoryPageStack;
use semio_framework::kernel::{HistoryMutationEntry, HistoryMutationMessage, HistoryReprojection, HistoryReprojectionKind, HistoryTimeTravel, HistoryTimeTravelProblem, HistoryTimeTravelReview, HistoryTimeTravelStage, RequestId};
use semio_framework::{input_label_glossary, mutation_input_defs, mutation_input_instance, reference_id_text, registered_input_schema_document, ActionArgControl, ActionArgOption, ArgPresentation, ArgSchema, InputSchemaError, OptionSource, SnapSource, DIALOG_CHOICE_ARG, HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_ACTION_IDS, HISTORY_EDIT_ARG_EDIT, HISTORY_EDIT_ARG_GENERATION, HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_NAME, HISTORY_EDIT_ARG_PATH, HISTORY_EDIT_ARG_STORE, HISTORY_EDIT_ARG_VALUE, HISTORY_EDIT_BACK_ACTION_ID, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, HISTORY_EDIT_CHOICE_OVERWRITE, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_DISCARD_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID, HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_FINALIZE_DIALOG_ID, HISTORY_EDIT_INPUT_ACTION_ID, HISTORY_EDIT_INPUT_INSERT, HISTORY_EDIT_INPUT_REMOVE, HISTORY_EDIT_RERUN_ACTION_ID, HISTORY_EDIT_RESTORE_ACTION_ID, HISTORY_EDIT_USE_SELECTION_ACTION_ID, HISTORY_EDIT_WITHDRAW_ACTION_ID};
use semio_framework_time_travel::{
    is_time_travel_alternative_name, TimeTravelBase, TimeTravelChoice, TimeTravelEffect, TimeTravelEvent, TimeTravelLabel, TimeTravelRefusal, TimeTravelReview, TimeTravelSession, TimeTravelStage, TimeTravelTarget, TIME_TRAVEL_BUSY_CODE,
    TIME_TRAVEL_COMMIT_FAILED_CODE, TIME_TRAVEL_EDITOR_CLOSED_CODE, TIME_TRAVEL_INVALID_INPUT_CODE, TIME_TRAVEL_MEMBER_GONE_CODE, TIME_TRAVEL_NAME_INVALID_CODE, TIME_TRAVEL_NAME_REQUIRED_CODE, TIME_TRAVEL_NOT_EDITABLE_CODE, TIME_TRAVEL_NOT_WITHDRAWABLE_CODE, TIME_TRAVEL_NO_SELECTION_CODE,
    TIME_TRAVEL_REPLAY_FAULTED_CODE, TIME_TRAVEL_SCHEMA_UNAVAILABLE_CODE, TIME_TRAVEL_UNKNOWN_INPUT_CODE, TIME_TRAVEL_UNKNOWN_MUTATION_CODE,
};
use std::collections::VecDeque;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
#[path="♻️retirement/🦀️.rs"]
mod original_retirement;

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
pub fn time_travel_replay_turn_budget() -> store::ReplayTurnBudget {
    store::ReplayTurnBudget::wall(TIME_TRAVEL_TURN_WALL_US, TIME_TRAVEL_REPLAY_OPERATIONS)
}
/// 📢️ Least interval between two replay-progress deliveries (history body refresh plus `HistoryPatch.timeTravel`),
/// unless the replay advanced by [`TIME_TRAVEL_PROGRESS_REFRESH_PERMILLE`] of its total since the last one.
pub const TIME_TRAVEL_PROGRESS_REFRESH_MS: u64 = 100;
/// 📢️ Replay advance (per mille of its total) that delivers progress before [`TIME_TRAVEL_PROGRESS_REFRESH_MS`] elapsed.
pub const TIME_TRAVEL_PROGRESS_REFRESH_PERMILLE: u64 = 50;
/// ✏️ Mutation rows one history row carries: the first ones, then every one with messages or a supersession.
pub const HISTORY_ROW_MUTATION_ROWS: usize = 32;
/// 🧹️ Discarded draft operations cold-retired per driver or close turn.
const TIME_TRAVEL_DISCARD_OPS_PER_TURN: usize = 64;
/// 🔎️ Document values one reference-name lookup visits at most ([`time_travel_entity_names`]).
pub const TIME_TRAVEL_ENTITY_NAME_VISITS: usize = 262_144;
/// ✂️ Characters of an entity id a fallback chip shows before an ellipsis ([`time_travel_reference_fallback_label`]).
pub const TIME_TRAVEL_SHORT_ID_CHARS: usize = 12;
/// 🗳️ The request id the finalize prompt opens under; nothing awaits a dialog.
const TIME_TRAVEL_FINALIZE_REQUEST: RequestId = RequestId(0x7417_0001);

fn history_planning_retirement_grant(demand:semio_framework_value::RetirementDemand)->semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:semio_framework_job::JOB_PAYLOAD_PAGE_BYTES.max(demand.copy_bytes),maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}

fn history_retirement_frame_grant(bytes:usize)->semio_framework_value::retained_clone::RetainedCloneGrant {
    semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:0,maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:1}
}
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
    Applied(TimeTravelStage),
    Rejected(TimeTravelActionRefusal),
}
//#endregion 🔖️Outcome

//#region 🔖️Ledger
/// ✏️ The mutation whose inputs are being drafted: its identity, label, input descriptors and the validator of its
/// payload schema, the current draft payload and the draft's own outcome against the state before it. The operation
/// whose kind the draft rebuilds lives with the typed owners of the store it belongs to ([`TimeTravelStoreState`]).
/// A mutation without editable inputs — opened by a history row's Withdraw (design §22.1) — has no `schema` and no
/// inputs: its editor admits no draft but the withdrawal. Whether the draft is withdrawn is the session's pending draft.
pub struct TimeTravelEditor {
    pub target: MutationId,
    pub position: u32,
    pub op_index: u32,
    pub label: LocalizedLabel,
    schema: Option<&'static str>,
    pub inputs: Vec<ActionArgDef>,
    pub inputs_refused: Option<InputSchemaError>,
    validator: Result<semio_framework_schema::OwnedJsonSchemaValidator, String>,
    pub value: DslValue,
    pub outcome: Vec<protocol::MutationMessage>,
    pub refused: Option<(String, String)>,
}

impl TimeTravelEditor {
    /// 🧯️ Stands in for a payload schema that compiles no validator, for the fail-closed law.
    #[cfg(test)]
    pub(crate) fn refuse_validator(&mut self, reason: &str) {
        self.validator = Err(reason.to_string());
    }
}

/// 🧩️ The composed member store a session edits (design §12): its owner edge, owner path and dialect, the typed owners of the
/// edit on that store (created by the first visit, erased here), and the children view every render seam reads while
/// the member's preview or replayed head is shown.
pub(crate) struct TimeTravelMemberSubject {
    pub key: MemberKey,
    pub path: MemberPath,
    pub dialect: ArtifactDialect,
    pub owners: Option<Box<dyn TimeTravelOwners>>,
    pub children: Option<ChildContentView>,
}

impl TimeTravelMemberSubject {
    /// 🧩️ The member store id the history wire names it by: the text of its owner path (`<slot>/<childId>` for a member of the document).
    pub(crate) fn store(&self) -> String {
        self.path.to_string()
    }
}

/// 🕰️ The ephemeral local-only history edit of one document instance (design §4 and §7): the pure session, the draft
/// editor, and the typed owners of the store the session edits — the document's own, or one composed member's (§12):
/// the preview the renderer reads while editing (state before the target with the draft), the running Report replay,
/// its finished result (whose head is the reviewing preview and whose report finalizes without replaying again), and
/// the owners still retiring. Nothing here is persisted or shared; closing the instance discards it.
pub struct TimeTravelLedger<A: ArtifactApp> {
    session: TimeTravelSession,
    editor: Option<TimeTravelEditor>,
    document: TimeTravelStoreState<A::Snapshot, A::Mutation>,
    member: Option<TimeTravelMemberSubject>,
    retiring: Vec<Box<dyn TimeTravelOwners>>,
    closing: bool,
    ui_dirty: bool,
    document_dirty: bool,
    refreshed_ms: u64,
    refreshed_done: u32,
    patch_due: bool,
    patch: Option<HistoryPatch>,
    reprojection_paused: bool,
    reprojection_fault: Option<(String, HistoryReprojectionKind)>,
    reprojection_refreshed_ms: u64,
    deferred_history_row: Option<String>,
    authoring: Option<SupersedeAuthoring<A::Snapshot, A::Mutation>>,
    history_unavailable: bool,
    turn_clock: fn() -> Option<u64>,
}

/// ✍️ The undo or redo of a finalize being authored resumably: the Report replay of its inputs over the applied history,
/// stepped per reactor turn, and how it finalizes (unscoped, or within the history edit's own line).
pub(crate) struct SupersedeAuthoring<P, Mu: ::protocol::Mutation<P>> {
    replay: store::ArtifactDerivedReplay<P, Mu>,
    finalization: store::HistoryFinalization,
}

/// 🧮️ Where authoring the undo or redo of a finalize stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SupersedeAuthored {
    Landed,
    Pending,
    Refused,
    Busy,
}

impl<A: ArtifactApp> Default for TimeTravelLedger<A> {
    fn default() -> Self {
        Self {
            session: TimeTravelSession::default(),
            editor: None,
            document: TimeTravelStoreState::new(time_travel_mutation_label::<A>),
            member: None,
            retiring: Vec::new(),
            closing: false,
            ui_dirty: false,
            document_dirty: false,
            refreshed_ms: 0,
            refreshed_done: 0,
            patch_due: false,
            patch: None,
            reprojection_paused: false,
            reprojection_fault: None,
            reprojection_refreshed_ms: 0,
            deferred_history_row: None,
            authoring: None,
            history_unavailable: false,
            turn_clock: semio_framework_job::default_now_us,
        }
    }
}

impl<A: ArtifactApp> TimeTravelLedger<A> {
    /// ⏱️ Replaces the clock every driver turn reads its wall deadline from (laws drive a counted clock).
    #[cfg(test)]
    pub(crate) fn set_turn_clock(&mut self, clock: fn() -> Option<u64>) {
        self.turn_clock = clock;
    }

    /// 🕰️ The pure session state.
    pub fn session(&self) -> &TimeTravelSession {
        &self.session
    }

    /// ✏️ The draft editor while a mutation is being edited.
    pub fn editor(&self) -> Option<&TimeTravelEditor> {
        self.editor.as_ref()
    }

    /// ✏️ The draft editor for a law that exercises a descriptor the fixture app's leaves do not declare.
    #[cfg(test)]
    pub(crate) fn editor_mut(&mut self) -> Option<&mut TimeTravelEditor> {
        self.editor.as_mut()
    }

    /// 🔗️ The ids every `Reference` input of the open draft holds, per selection domain the input declares (draft order,
    /// each id once) — what a render highlights while the draft targets entities; empty while no draft editor is open.
    pub fn draft_references(&self) -> BTreeMap<String, Vec<String>> {
        let mut references: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let Some(editor) = self.editor.as_ref() else { return references };
        for row in time_travel_input_rows(&editor.inputs, &editor.value) {
            let ArgSchema::Reference { domain: Some(domain), .. } = &row.input.schema else { continue };
            let ids = references.entry(domain.clone()).or_default();
            let held: Vec<String> = match &row.value {
                DslValue::Array(items) => items.iter().filter_map(reference_id_text).collect(),
                other => reference_id_text(other).into_iter().collect(),
            };
            for id in held {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
        references
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
    /// A session on a composed member leaves the parent document as it is: the member's preview reaches the render seams
    /// through [`Self::render_children_or`].
    pub fn render_snapshot_or<'a>(&'a self, tool_runs: &'a ToolRunLedger<A>, committed: &'a Arc<A::Snapshot>) -> &'a Arc<A::Snapshot> {
        match (self.session.stage, self.member.is_some()) {
            (TimeTravelStage::Inactive, _) | (_, true) => tool_runs.overlay_or(committed),
            (stage, false) => self.document.shown(stage).map(store::ArtifactDerivedSnapshot::snapshot_owner).unwrap_or(committed),
        }
    }

    /// 🪞️ The composed children every render seam reads: while a session on a member shows that member's preview or
    /// replayed head, the live children with that member in its place (design §12); otherwise the member tool run's composed
    /// read or `live` (`ToolRunLedger::children_or`).
    pub fn render_children_or<'a>(&'a self, tool_runs: &'a ToolRunLedger<A>, live: &'a ChildContentView) -> &'a ChildContentView {
        self.member.as_ref().and_then(|member| member.children.as_ref()).filter(|_| self.is_active()).unwrap_or_else(|| tool_runs.children_or(live))
    }

    /// ⏯️ Whether the store the session edits holds a running replay.
    pub(crate) fn replaying(&self) -> bool {
        match self.member.as_ref() {
            None => TimeTravelOwners::replaying(&self.document),
            Some(member) => member.owners.as_deref().is_some_and(TimeTravelOwners::replaying),
        }
    }

    fn preview_progress(&self) -> Option<store::ReplayProgress> {
        match self.member.as_ref() {
            None => self.document.preview_progress(),
            Some(member) => member.owners.as_deref().and_then(TimeTravelOwners::preview_progress),
        }
    }

    fn processed(&self) -> Option<u32> {
        match self.member.as_ref() {
            None => self.document.processed(),
            Some(member) => member.owners.as_deref().and_then(TimeTravelOwners::processed),
        }
    }

    /// 🧩️ The composed member store the session edits (`<slot>/<childId>`), `None` for the document's own.
    pub fn member_store(&self) -> Option<String> {
        self.member.as_ref().map(TimeTravelMemberSubject::store)
    }

    /// 🧹️ Every typed owner set the ledger holds: the document's, the member's being edited, and the members' still
    /// retiring after the session left them.
    fn owners(&self) -> impl Iterator<Item = &dyn TimeTravelOwners> {
        std::iter::once(&self.document as &dyn TimeTravelOwners).chain(self.member.iter().filter_map(|member| member.owners.as_deref())).chain(self.retiring.iter().map(|owners| &**owners))
    }

    /// 🏃️ Whether a driver turn has work: a live replay, an undo or redo of a finalize being authored, retirement, an owed
    /// UI scope or an owed history patch.
    pub fn has_pending_work(&self) -> bool {
        let replaying = self.session.stage == TimeTravelStage::Replaying && self.owners().any(TimeTravelOwners::replaying);
        replaying || self.authoring.is_some() || self.owners().any(TimeTravelOwners::has_pending_work) || self.is_ui_dirty() || self.patch_due || self.patch.is_some()
    }

    /// 🫥️ Whether the document holds no history because a pure command hydrated its head alone
    /// (`📓️api-stepped-document-load.md` §4): history verbs and queries answer `pure.history-unavailable`.
    pub fn history_unavailable(&self) -> bool {
        self.history_unavailable
    }

    /// 🫥️ Marks the document as hydrated head-only (`true`) or loaded with its history (`false`).
    pub(crate) fn set_history_unavailable(&mut self, unavailable: bool) {
        self.history_unavailable = unavailable;
    }

    /// ⏸️ Whether the user paused the replay of a waiting remote history change.
    pub fn reprojection_paused(&self) -> bool {
        self.reprojection_paused
    }

    /// ✍️ Whether the undo or redo of a finalize is still replaying before it commits.
    pub fn authoring_pending(&self) -> bool {
        self.authoring.is_some()
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
    fn accepted_drafts(&self) -> protocol::HistoryInputDrafts {
        self.session.accepted.iter().map(|draft| (draft.target.mutation.clone(), draft.replacement.clone())).collect()
    }

    fn replace_editor(&mut self, editor: Option<TimeTravelEditor>) {
        self.editor = editor;
    }

    /// 🧹️ One bounded retirement unit of the first typed owner set still retiring; `None` when nothing retires. A member
    /// owner set the session already left is dropped once it is terminal-empty.
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let demand=self.document.retirement_demands(body)?;
        if demand!=RetirementDemand::default(){return Ok(demand);}
        if let Some(owners)=self.member.as_ref().and_then(|member|member.owners.as_ref()){let demand=owners.retirement_demands(body)?;if demand!=RetirementDemand::default(){return Ok(demand);}}
        if let Some(owners)=self.retiring.last(){return if owners.terminal_is_empty(){Ok(RetirementDemand{release_bytes:std::mem::size_of_val(owners.as_ref()),depth:1,..Default::default()})}else{owners.retirement_demands(body)};}
        if self.retiring.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.retiring.capacity()*std::mem::size_of::<Box<dyn TimeTravelOwners>>(),depth:1,..Default::default()});}
        Ok(Default::default())
    }
    fn retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, Fault> {
        if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if let Some(step)=self.document.retire_step(grant)?{return Ok(Some(RetainedCloneStep::Progress(step.progress())));}
        if let Some(owners)=self.member.as_mut().and_then(|member|member.owners.as_mut()){if let Some(step)=owners.retire_step(grant)?{return Ok(Some(RetainedCloneStep::Progress(step.progress())));}}
        if let Some(owners)=self.retiring.last_mut(){
            if !owners.terminal_is_empty(){return owners.retire_step(grant).map(|step|step.map(|step|RetainedCloneStep::Progress(step.progress())));}
            let demand=self.retirement_demands(grant.maximum_copy_bytes).map_err(|error|Fault::from(error.into_message()))?;
            if grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
            drop(self.retiring.pop());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})));
        }
        if self.retiring.capacity()!=0{let bytes=self.retiring.capacity()*std::mem::size_of::<Box<dyn TimeTravelOwners>>();if grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}drop(std::mem::take(&mut self.retiring));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})));}
        Ok(None)
    }

    /// 🚪️ Document close, instance retirement or reload: the session vanishes with every draft (ephemeral state). The
    /// typed owners settle at the runtime's close step, which reaches their stores.
    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn terminal_is_empty(&self) -> bool {
        !self.is_active() && self.session.accepted.capacity()==0 && self.session.pending.is_none() && self.session.report.is_none() && self.session.fault.is_none() && self.editor.is_none() && self.member.is_none() && self.authoring.is_none() && self.patch.is_none() && self.reprojection_fault.is_none() && self.deferred_history_row.is_none() && self.retiring.capacity()==0 && self.owners().all(TimeTravelOwners::terminal_is_empty)
    }

    /// 👉️ The first mutation, in replay order, whose outcome blocks finalizing (`MergePolicy::Normal`, the floor
    /// `ReplayReport::blocks_finalize` reads): what the status names as the next problem (design §22.2); `None` while the
    /// session's report does not block.
    fn next_problem(&self) -> Option<String> {
        self.session.report.as_ref()?.outcomes.iter().find(|outcome| outcome.worst.is_some_and(|worst| protocol::MergePolicy::Normal.rejects(worst))).map(|outcome| outcome.mutation_id.0.clone())
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
        let preview_progress = (session.stage == TimeTravelStage::Editing).then(|| self.preview_progress()).flatten();
        Some(HistoryTimeTravel {
            session_id: session.id.to_string(),
            generation: session.generation,
            stage,
            target: session.pending.as_ref().map(|pending| pending.target.mutation.0.clone()),
            target_label: self.editor.as_ref().map(|editor| editor.label.clone()),
            done: preview_progress.map(|progress| progress.done).or_else(|| session.progress.map(|progress| progress.done)),
            total: preview_progress.map(|progress| progress.total).or_else(|| session.progress.map(|progress| progress.total)),
            processed: (session.stage == TimeTravelStage::Replaying).then(|| self.processed()).flatten(),
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
            next_problem: self.next_problem().map(|mutation_id| HistoryTimeTravelProblem { mutation_id, store: self.member_store() }),
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
        let accepted: BTreeSet<String> = session.accepted.iter().map(|draft| draft.target.mutation.0.clone()).collect();
        let mut edited = accepted.clone();
        if let Some(pending) = session.pending.as_ref().filter(|pending| !session.unchanged(pending)) {
            edited.insert(pending.target.mutation.0.clone());
        }
        let next_problem = self.next_problem();
        let editor = self.editor.as_ref().map(|editor| TimeTravelEditorPanel {
            target: editor.target.0.clone(),
            label: editor.label.clone(),
            rows: time_travel_input_rows(&editor.inputs, &editor.value),
            editable: editor.schema.is_some(),
            inputs_refused: editor.inputs_refused.as_ref().map(|error| error.detail.clone()),
            withdrawn: session.pending.as_ref().is_some_and(|pending| pending.replacement == protocol::InputReplacement::Withdrawn),
            outcome: editor.outcome.clone(),
            refused: editor.refused.clone(),
            changed: session.pending.as_ref().is_some_and(|pending| !session.unchanged(pending)),
            reference_labels: BTreeMap::new(),
        });
        Some(TimeTravelPanel {
            status,
            store: self.member_store(),
            stage: session.stage,
            pending_after: session.pending.as_ref().and(self.editor.as_ref()).map(|editor| (editor.position, editor.op_index)),
            finalize_refusal: session.finalize_refusal(),
            review: session.review(),
            rerun_refusal: session.rerun_refusal(),
            begin_refusal: session.begin_refusal(),
            next_problem,
            editor,
            outcomes,
            edited,
            accepted,
        })
    }
}
//#endregion 🔖️Ledger

//#region 🔖️StoreState
/// 🧹️ The store-free half of one store's history-edit owners: whether a replay runs, whether owners still retire, the
/// terminal witness and one bounded retirement unit — callable on a composed member's erased owners without its store.
pub(crate) trait TimeTravelOwners: Send {
    fn replaying(&self) -> bool;
    fn preview_progress(&self) -> Option<store::ReplayProgress>;
    fn processed(&self) -> Option<u32>;
    fn has_pending_work(&self) -> bool;
    fn terminal_is_empty(&self) -> bool;
    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError>;
    fn retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, Fault>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// 🗃️ The typed owners of a history edit on ONE store — the document's own or one composed member's (design §12): the
/// edited operation's kind (staged by an open until the session adopts it), the draft preview, the running Report replay,
/// its finished result and replayed head, the retirements of the snapshots they displaced (taken from the store at once,
/// so they retire without it) and the operations a draft discarded.
pub(crate) struct TimeTravelStoreState<P, Mu: ::protocol::Mutation<P>> {
    label_of: fn(&Mu) -> LocalizedLabel,
    staged: Option<Mu>,
    kind: Option<Mu>,
    preview: Option<store::ArtifactDerivedSnapshot<P>>,
    preview_job: Option<store::ArtifactDerivedHistoryPreview<P, Mu>>,
    preview_input: Option<protocol::InputReplacement>,
    replay: Option<store::ArtifactDerivedReplay<P, Mu>>,
    finished: Option<store::EditReplayResult<P, Mu>>,
    head: Option<store::ArtifactDerivedSnapshot<P>>,
    retirements: VecDeque<Box<dyn store::ErasedSnapshotRetirement>>,
    discarded: Vec<Mu>,
    discarded_pending: Option<Mu>,
    discarded_active: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    discarded_factory: Option<Arc<dyn semio_framework_value::ArtifactOwnedValueRetirementFactory<Mu>>>,
    discarded_factory_close: Option<semio_framework_value::FactoryAuthority>,
}

/// ⏭️ What one replay slice did: progress, a completed report (the head is swapped in), or a fault.
pub(crate) enum TimeTravelReplayStep {
    Pending { done: u32, total: u32 },
    Completed(protocol::ReplayReport),
    Faulted,
}

/// 🖼️ One preview slice leaves progress or publishes a complete selected-mutation projection.
pub(crate) enum TimeTravelPreviewStep {
    Pending,
    Completed(Vec<protocol::MutationMessage>),
}

/// 🌿️ How a finalize ended on its store; a finalize carries the encoded envelopes of every transition it authored (an
/// overwrite's `Supersede`; an alternative's pending-edit commit, `Branch` and scoped `Supersede`), in authoring order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum TimeTravelCommit {
    Finalized { authored: Vec<u8> },
    Stale,
    Blocked,
    Failed,
}

/// 📨️ One typed history-edit operation against the store a session edits.
pub(crate) enum TimeTravelStoreCommand {
    Open { target: MutationId, current: Option<protocol::InputReplacement>, withdraw: bool },
    Adopt(bool),
    Rebuild(DslValue),
    Preview { target: MutationId, replacement: protocol::InputReplacement, drafts: protocol::HistoryInputDrafts, deadline_us: u64, clock: fn() -> Option<u64> },
    StepPreview { deadline_us: u64, clock: fn() -> Option<u64> },
    StartReplay { drafts: protocol::HistoryInputDrafts, from: MutationId },
    CancelReplay,
    StepReplay { deadline_us: u64, clock: fn() -> Option<u64> },
    Commit { drafts: protocol::HistoryInputDrafts, finalization: store::HistoryFinalization, actor: Option<String> },
    Read(TimeTravelStage),
}

/// 📬️ What a [`TimeTravelStoreCommand`] answered.
pub(crate) enum TimeTravelStoreOutput {
    Opened(Result<(TimeTravelEditor, protocol::InputReplacement), TimeTravelActionRefusal>),
    Rebuilt(Result<protocol::InputReplacement, String>),
    Previewed(TimeTravelPreviewStep),
    ReplayStarted(bool),
    Stepped(TimeTravelReplayStep),
    Committed(TimeTravelCommit),
    Read(Option<(store::ErasedSnapshotRead, [u8; 32])>),
    Done,
}

/// 🔎️ One store-only read a session takes synchronously.
pub(crate) enum TimeTravelStoreQuery {
    Base,
    Positions(Vec<MutationId>),
    ShownValue(TimeTravelStage),
}

/// 📬️ What a [`TimeTravelStoreQuery`] answered.
pub(crate) enum TimeTravelQueryOutput {
    Base(TimeTravelBase),
    Positions(Option<Vec<TimeTravelTarget>>),
    Value(Option<DslValue>),
}

impl<P, Mu> TimeTravelStoreState<P, Mu>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + Send + Sync + 'static,
    Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
{
    /// 🏗️ Owners at rest, labelling the store's operations with `label_of`.
    pub(crate) fn new(label_of: fn(&Mu) -> LocalizedLabel) -> Self {
        Self { label_of, staged: None, kind: None, preview: None, preview_job: None, preview_input: None, replay: None, finished: None, head: None, retirements: VecDeque::new(), discarded: Vec::new(), discarded_pending: None, discarded_active: None, discarded_factory: None, discarded_factory_close: None }
    }

    /// 🪞️ The snapshot the session shows at `stage`: the draft preview while editing; while replaying that preview — or,
    /// for a replay started from a review (Replay again, a restored draft, a moved base), the head reviewed last, so no
    /// window falls back to the committed document in between; the replayed head while reviewing.
    pub(crate) fn shown(&self, stage: TimeTravelStage) -> Option<&store::ArtifactDerivedSnapshot<P>> {
        match stage {
            TimeTravelStage::Inactive => None,
            TimeTravelStage::Editing => self.preview.as_ref(),
            TimeTravelStage::Replaying => self.preview.as_ref().or(self.head.as_ref()),
            TimeTravelStage::Reviewing | TimeTravelStage::Choosing | TimeTravelStage::Finalizing => self.head.as_ref(),
        }
    }

    fn retire(&mut self, store: &ArtifactStore<P, Mu>, snapshot: Option<store::ArtifactDerivedSnapshot<P>>) -> Result<(), Fault> {
        if let Some(snapshot) = snapshot {
            self.retirements.push_back(store.retire_snapshot_alias(snapshot.into_snapshot_owner()).map_err(|error| error.into_fault())?);
        }
        Ok(())
    }

    fn cancel_preview(&mut self, store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
        let grant=history_retirement_frame_grant(ArtifactStore::<P,Mu>::history_read_retirement_birth_bytes());
        if let Some((retirement,progress)) = store.retire_derived_history_preview(&mut self.preview_job,grant).map_err(|error|Fault::from(error.into_message()))? { assert!(progress.fits(grant));self.retirements.push_back(retirement); }
        self.preview_input = None;
        Ok(())
    }

    fn cancel_replay(&mut self, store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
        let grant=history_retirement_frame_grant(ArtifactStore::<P,Mu>::history_read_retirement_birth_bytes());
        if let Some((retirement,progress)) = store.retire_derived_report_replay(&mut self.replay,grant).map_err(|error|Fault::from(error.into_message()))? { assert!(progress.fits(grant));self.retirements.push_back(retirement); }
        Ok(())
    }

    fn cancel_finished(&mut self, store: &ArtifactStore<P, Mu>) -> Result<(), Fault> {
        let grant=history_retirement_frame_grant(ArtifactStore::<P,Mu>::history_read_retirement_birth_bytes());
        if let Some((retirement,progress)) = store.retire_finished_history_replay(&mut self.finished,grant).map_err(|error|Fault::from(error.into_message()))? { assert!(progress.fits(grant));self.retirements.push_back(retirement); }
        Ok(())
    }

    fn discard(&mut self, op: Option<Mu>) {
        self.discarded.extend(op);
    }

    /// 🧽️ Drops what `stage` no longer shows: the kind outside `Editing`, the draft preview once the replay finished or
    /// the session left, the replay outside `Replaying`, every replay owner once it is inactive.
    pub(crate) fn settle(&mut self, store: &ArtifactStore<P, Mu>, stage: TimeTravelStage) -> Result<(), Fault> {
        if stage != TimeTravelStage::Editing {
            self.cancel_preview(store)?;
            let kind = self.kind.take();
            self.discard(kind);
            let staged = self.staged.take();
            self.discard(staged);
        }
        if !matches!(stage, TimeTravelStage::Editing | TimeTravelStage::Replaying) {
            let preview = self.preview.take();
            self.retire(store, preview)?;
        }
        if stage != TimeTravelStage::Replaying {
            self.cancel_replay(store)?;
        }
        if stage == TimeTravelStage::Inactive {
            self.cancel_finished(store)?;
            let head = self.head.take();
            self.retire(store, head)?;
        }
        Ok(())
    }

    /// 🔎️ A store-only read.
    pub(crate) fn query(owners: Option<&Self>, store: &ArtifactStore<P, Mu>, query: TimeTravelStoreQuery) -> TimeTravelQueryOutput {
        match query {
            TimeTravelStoreQuery::Base => TimeTravelQueryOutput::Base(TimeTravelBase { content_revision: store.content_revision() }),
            TimeTravelStoreQuery::Positions(targets) => TimeTravelQueryOutput::Positions(targets.into_iter().map(|target| store.mutation_position(&target).ok().map(|(position, _)| TimeTravelTarget { position: u32::try_from(position).unwrap_or(u32::MAX), mutation: target })).collect()),
            TimeTravelStoreQuery::ShownValue(stage) => TimeTravelQueryOutput::Value(owners.and_then(|owners| owners.shown(stage)).map(|shown| semio_framework_value::ToValue::to_value(shown.snapshot()))),
        }
    }

    /// 📨️ Runs one typed command against `store`.
    pub(crate) async fn run(&mut self, store: &mut ArtifactStore<P, Mu>, command: TimeTravelStoreCommand) -> Result<TimeTravelStoreOutput, Fault> {
        Ok(match command {
            TimeTravelStoreCommand::Open { target, current, withdraw } => TimeTravelStoreOutput::Opened(self.open(store, &target, current.as_ref(), withdraw)),
            TimeTravelStoreCommand::Adopt(adopt) => {
                let staged = self.staged.take();
                match adopt {
                    true => {
                        let previous = std::mem::replace(&mut self.kind, staged);
                        self.discard(previous);
                    }
                    false => self.discard(staged),
                }
                TimeTravelStoreOutput::Done
            }
            TimeTravelStoreCommand::Rebuild(candidate) => TimeTravelStoreOutput::Rebuilt(self.rebuild(store, candidate)),
            TimeTravelStoreCommand::Preview { target, replacement, drafts, deadline_us, clock } => {
                self.cancel_preview(store)?;
                self.preview_job = Some(store.begin_derived_history_preview(target, drafts).map_err(|error| error.into_fault())?);
                self.preview_input = Some(replacement);
                TimeTravelStoreOutput::Previewed(self.step_preview(store, deadline_us, clock)?)
            }
            TimeTravelStoreCommand::StepPreview { deadline_us, clock } => TimeTravelStoreOutput::Previewed(self.step_preview(store, deadline_us, clock)?),
            TimeTravelStoreCommand::StartReplay { drafts, from } => {
                self.cancel_replay(store)?;
                match store.begin_derived_report_replay(drafts, Some(from)) {
                    Ok(replay) => {
                        self.replay = Some(replay);
                        TimeTravelStoreOutput::ReplayStarted(true)
                    }
                    Err(_) => TimeTravelStoreOutput::ReplayStarted(false),
                }
            }
            TimeTravelStoreCommand::CancelReplay => {
                self.cancel_replay(store)?;
                TimeTravelStoreOutput::Done
            }
            TimeTravelStoreCommand::StepReplay { deadline_us, clock } => TimeTravelStoreOutput::Stepped(self.step_replay(store, deadline_us, clock)?),
            TimeTravelStoreCommand::Commit { drafts, finalization, actor } => {
                if self.finished.as_ref().is_none_or(|finished| *finished.drafts() != drafts) {
                    self.cancel_finished(store)?;
                    return Ok(TimeTravelStoreOutput::Committed(TimeTravelCommit::Stale));
                }
                if actor.as_ref().is_some_and(|actor| *actor != store.local_actor_id().0) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("time-travel.actor"), "history edit actor differs from its opened session"));
                }
                let finished = self.finished.take().expect("admitted finished history replay");
                let known: BTreeSet<MutationId> = store.envelope().transitions.iter().map(|transition| transition.mutation_id.clone()).collect();
                TimeTravelStoreOutput::Committed(match store.commit_finished_replay(finished, finalization).await {
                    Ok(_) => {
                        let authored: Vec<_> = store.envelope().transitions.iter().filter(|transition| !known.contains(&transition.mutation_id)).cloned().collect();
                        TimeTravelCommit::Finalized { authored: store::os_spr::encode_envelopes(&authored) }
                    }
                    Err(vcs::VcsError::Stale { .. }) => TimeTravelCommit::Stale,
                    Err(vcs::VcsError::Rejected { .. }) => TimeTravelCommit::Blocked,
                    Err(_) => TimeTravelCommit::Failed,
                })
            }
            TimeTravelStoreCommand::Read(stage) => TimeTravelStoreOutput::Read(match self.shown(stage) {
                Some(shown) => Some((store.snapshot_read_derived(shown).map_err(|error| error.into_fault())?, store.content_revision())),
                None => None,
            }),
        })
    }

    /// ✏️ The editor of `target` and its effective input before the session (its payload schema, input descriptors,
    /// validator and the value it starts from: `current`, its accepted draft, else its input); its kind is staged until
    /// the session adopts it. Refused for an unknown operation and for one whose inputs cannot be edited: it plans foreign
    /// steps, declares no input schema, or its schema describes no input the editor shows as a row over that value (design
    /// §22.20: an editor with zero rows never opens; a schema that cannot be read still opens, naming why) — unless the
    /// editor opens to `withdraw` it (a history row's Withdraw, design §22.1): then the store's supersede law decides
    /// ([`time_travel_admits_withdrawal`]), and an operation without editable inputs opens on an editor without inputs.
    /// An operation of a cross-document unit takes no supersession in this document alone (`AppliedMutation::unit`, the
    /// store's authoring law): its Withdraw answers the store's own refusal, its inputs are not editable.
    fn open(&mut self, store: &ArtifactStore<P, Mu>, target: &MutationId, current: Option<&protocol::InputReplacement>, withdraw: bool) -> Result<(TimeTravelEditor, protocol::InputReplacement), TimeTravelActionRefusal> {
        let refusal = if withdraw { TimeTravelActionRefusal::NotWithdrawable } else { TimeTravelActionRefusal::NotEditable };
        let row = store.applied_mutation(target).map_err(|_| TimeTravelActionRefusal::UnknownMutation)?;
        if row.unit.is_some() {
            return Err(if withdraw { TimeTravelActionRefusal::UnitSpansDocuments } else { TimeTravelActionRefusal::NotEditable });
        }
        let schema_id = store.envelope().schema.clone();
        let original = match row.supersession {
            Some(supersession) => supersession.replacement.clone(),
            None => protocol::InputReplacement::Input { schema: schema_id, payload: <Mu as ::protocol::OpBinary>::encode_op(row.operation).map_err(|_| refusal)? },
        };
        let decode = |replacement: &protocol::InputReplacement| match replacement {
            protocol::InputReplacement::Input { payload, .. } => <Mu as ::protocol::OpBinary>::decode_op(payload).ok(),
            protocol::InputReplacement::Withdrawn => None,
        };
        let kind = decode(&original).unwrap_or_else(|| row.operation.clone());
        let value = match decode(current.unwrap_or(&original)) {
            Some(op) => {
                let value = op.payload_value();
                op.retire_cold();
                value
            }
            None => kind.payload_value(),
        };
        let read = kind.input_schema().filter(|_| !kind.may_emit_foreign_steps()).map(|schema| (schema, mutation_input_defs(schema, &registered_input_schema_document)));
        let read = read.filter(|(_, inputs)| inputs.as_ref().map_or(true, |inputs| !time_travel_input_rows(inputs, &value).is_empty()));
        let admitted = if withdraw { time_travel_admits_withdrawal::<P, Mu>(row.operation) } else { read.is_some() };
        if !admitted {
            kind.retire_cold();
            return Err(refusal);
        }
        let (schema, inputs, inputs_refused, validator) = match read {
            Some((schema, read)) => {
                let (inputs, inputs_refused) = match read {
                    Ok(inputs) => (inputs, None),
                    Err(error) => (Vec::new(), Some(error)),
                };
                let documents = time_travel_schema_documents(schema);
                (Some(schema), inputs, inputs_refused, semio_framework_schema::OwnedJsonSchemaValidator::compile_with_documents(schema, &documents.iter().map(String::as_str).collect::<Vec<_>>()).map_err(|error| error.to_string()))
            }
            None => (None, Vec::new(), None, Err(TimeTravelLabel::RefusalNotEditable.en().to_string())),
        };
        let editor = TimeTravelEditor {
            target: target.clone(),
            position: u32::try_from(row.position).unwrap_or(u32::MAX),
            op_index: row.op_index,
            label: (self.label_of)(&kind),
            schema,
            inputs,
            inputs_refused,
            validator,
            value,
            outcome: Vec::new(),
            refused: None,
        };
        let previous = self.staged.replace(kind);
        self.discard(previous);
        Ok((editor, original))
    }

    /// 🎚️ The validated draft payload `candidate` rebuilt into the edited kind and canonically encoded.
    fn rebuild(&mut self, store: &ArtifactStore<P, Mu>, candidate: DslValue) -> Result<protocol::InputReplacement, String> {
        let op = self.kind.as_ref().ok_or("the draft editor lost the operation kind it rebuilds")?.with_payload_value(candidate).map_err(|error| error.to_string())?;
        let encoded = <Mu as ::protocol::OpBinary>::encode_op(&op);
        self.discard(Some(op));
        Ok(protocol::InputReplacement::Input { schema: store.envelope().schema.clone(), payload: encoded.map_err(|error| error.to_string())? })
    }

    /// 🖼️ One bounded prefix slice; only a completed preview displaces the last complete projection.
    fn step_preview(&mut self, store: &mut ArtifactStore<P, Mu>, deadline_us: u64, clock: fn() -> Option<u64>) -> Result<TimeTravelPreviewStep, Fault> {
        let Some(job) = self.preview_job.as_mut() else { return Ok(TimeTravelPreviewStep::Pending) };
        let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
        loop {
            let grant=history_planning_retirement_grant(job.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(|error|Fault::from(error.into_message()))?);
            if matches!(store.step_derived_history_preview(job, grant, &mut deadline).map_err(|error| error.into_fault())?, store::ReplayStep::Finished(_)) { break; }
            if deadline() { return Ok(TimeTravelPreviewStep::Pending); }
        }
        let base = store.finish_derived_history_preview(&mut self.preview_job).map_err(|error| error.into_fault())?;
        let replacement = self.preview_input.take().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.preview-input-absent"), "history preview lost its selected input"))?;
        self.cancel_preview(store)?;
        let (preview, outcome) = match replacement {
            protocol::InputReplacement::Withdrawn => (base, Vec::new()),
            protocol::InputReplacement::Input { payload, .. } => {
                let op = <Mu as ::protocol::OpBinary>::decode_op(&payload).map_err(|error| error.into_fault())?;
                let derived = store.derive_history_snapshot(&base, &op);
                self.discard(Some(op));
                let (next, messages) = derived.map_err(|error| error.into_fault())?;
                match next {
                    Some(next) => { self.retire(store, Some(base))?; (next, messages) }
                    None => (base, messages),
                }
            }
        };
        let previous = self.preview.replace(preview);
        self.retire(store, previous)?;
        Ok(TimeTravelPreviewStep::Completed(outcome))
    }

    /// ⏭️ Steps the replay until `deadline_us` on `clock`; completion swaps the replayed head in and keeps the finished result.
    fn step_replay(&mut self, store: &ArtifactStore<P, Mu>, deadline_us: u64, clock: fn() -> Option<u64>) -> Result<TimeTravelReplayStep, Fault> {
        let Some(replay) = self.replay.as_mut() else { return Ok(TimeTravelReplayStep::Faulted) };
        let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
        let grant=history_planning_retirement_grant(replay.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(|error|Fault::from(error.into_message()))?);
        Ok(match store.step_derived_report_replay(replay, grant, &mut deadline) {
            Ok(store::ReplayStep::Pending(progress)) => TimeTravelReplayStep::Pending { done: progress.done, total: progress.total },
            Ok(store::ReplayStep::Finished(_)) => {
                match store.finish_derived_report_replay(&mut self.replay).and_then(|(result, head)| store.replay_report(&result).map(|report| (result, head, report))) {
                    Ok((result, head, report)) => {
                        let previous = std::mem::replace(&mut self.head, head);
                        self.retire(store, previous)?;
                        self.cancel_finished(store)?;
                        self.finished = Some(result);
                        TimeTravelReplayStep::Completed(report)
                    }
                    Err(_) => TimeTravelReplayStep::Faulted,
                }
            }
            Err(_) => {
                self.cancel_replay(store)?;
                TimeTravelReplayStep::Faulted
            }
        })
    }
}

impl<P, Mu> TimeTravelOwners for TimeTravelStoreState<P, Mu>
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + Send + Sync + 'static,
    Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
{
    fn replaying(&self) -> bool {
        self.replay.is_some()
    }

    fn preview_progress(&self) -> Option<store::ReplayProgress> {
        self.preview_job.as_ref().map(store::ArtifactDerivedHistoryPreview::progress)
    }

    fn processed(&self) -> Option<u32> {
        self.replay.as_ref().map(|replay| replay.operation_preparation_progress().items).filter(|items| *items > 0).map(|items| u32::try_from(items).unwrap_or(u32::MAX))
    }

    fn has_pending_work(&self) -> bool {
        self.preview_job.is_some() || self.retirements.capacity()!=0 || self.discarded.capacity()!=0 || self.discarded_pending.is_some() || self.discarded_active.is_some() || self.discarded_factory.is_some() || self.discarded_factory_close.is_some()
    }

    fn terminal_is_empty(&self) -> bool {
        self.staged.is_none() && self.kind.is_none() && self.preview.is_none() && self.preview_job.is_none() && self.preview_input.is_none() && self.replay.is_none() && self.finished.is_none() && self.head.is_none() && self.retirements.capacity()==0 && self.discarded.capacity()==0 && self.discarded_pending.is_none() && self.discarded_active.is_none() && self.discarded_factory.is_none() && self.discarded_factory_close.is_none()
    }

    fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> { self.original_retirement_demands(body) }
    fn retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, Fault> { self.original_retirement_step(grant).map_err(|error|Fault::from(error.into_message())) }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// ✍️ One typed history-edit command against a composed member's store (design §12): creates the member's owners on the
/// first visit, then runs the command on them.
pub(crate) struct TimeTravelMemberRun<'a> {
    pub owners: &'a mut Option<Box<dyn TimeTravelOwners>>,
    pub command: TimeTravelStoreCommand,
}

impl store::MemberStoreVisitorMut for TimeTravelMemberRun<'_> {
    type Output = Result<TimeTravelStoreOutput, Fault>;

    async fn visit_mut<P, Mu>(self, store: &mut ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        for documents in <Mu as ::protocol::Mutation<P>>::INPUT_SCHEMA_DOCUMENTS {
            semio_framework_schema_registry::register_referenced_schema_documents(documents);
        }
        let owners = self.owners.get_or_insert_with(|| Box::new(TimeTravelStoreState::<P, Mu>::new(|op| protocol::SemanticMutation::<P>::label(op))) as Box<dyn TimeTravelOwners>);
        let state = owners.as_any_mut().downcast_mut::<TimeTravelStoreState<P, Mu>>().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.member-store-kind"), "a composed member's history-edit owners belong to another store kind"))?;
        state.run(store, self.command).await
    }
}

/// 🧽️ Settles a composed member's owners for `stage` against its store (design §12).
pub(crate) struct TimeTravelMemberSettle<'a> {
    pub owners: &'a mut dyn TimeTravelOwners,
    pub stage: TimeTravelStage,
}

impl store::MemberStoreVisitor for TimeTravelMemberSettle<'_> {
    type Output = Result<(), Fault>;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        self.owners.as_any_mut().downcast_mut::<TimeTravelStoreState<P, Mu>>().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.member-store-kind"), "a composed member's history-edit owners belong to another store kind"))?.settle(store, self.stage)
    }
}

/// 🔎️ A store-only read of a composed member's store (design §12).
pub(crate) struct TimeTravelMemberQuery<'a> {
    pub owners: Option<&'a dyn TimeTravelOwners>,
    pub query: TimeTravelStoreQuery,
}

impl store::MemberStoreVisitor for TimeTravelMemberQuery<'_> {
    type Output = TimeTravelQueryOutput;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        TimeTravelStoreState::<P, Mu>::query(self.owners.and_then(|owners| owners.as_any().downcast_ref::<TimeTravelStoreState<P, Mu>>()), store, self.query)
    }
}
//#endregion 🔖️StoreState

//#region 🔖️PanelView
/// ✏️ The draft editor as the panel renders it: its rows ([`time_travel_input_rows`]) over the current draft, whether
/// the mutation's inputs can be edited at all (`editable`: an editor a row's Withdraw opened on a mutation without them
/// holds none), and the entity label of every id its reference rows name, as the app reads them in the previewed
/// document (`ArtifactApp::entity_label`; an id without one shows as itself).
#[derive(Clone, Debug, PartialEq)]
pub struct TimeTravelEditorPanel {
    pub target: String,
    pub label: LocalizedLabel,
    pub rows: Vec<TimeTravelInputRow>,
    pub editable: bool,
    pub inputs_refused: Option<String>,
    pub withdrawn: bool,
    pub outcome: Vec<protocol::MutationMessage>,
    pub refused: Option<(String, String)>,
    pub changed: bool,
    pub reference_labels: BTreeMap<String, LocalizedLabel>,
}

/// 🪧️ The live session as the history panel and the history wire read it: the band status, the composed member store it
/// edits (`None` for the document's own), which mutations are pending (downstream of the edited one in that store while
/// editing), edited (a draft, accepted or pending) or `accepted` (an accepted draft a row's Restore takes back), the
/// replay outcomes that override the durable ones, why finalizing is refused, the first blocking mutation, and the
/// draft editor.
#[derive(Clone, Debug, PartialEq)]
pub struct TimeTravelPanel {
    pub status: HistoryTimeTravel,
    pub store: Option<String>,
    pub stage: TimeTravelStage,
    pub pending_after: Option<(u32, u32)>,
    pub finalize_refusal: Option<TimeTravelRefusal>,
    pub review: Option<TimeTravelReview>,
    pub rerun_refusal: Option<TimeTravelRefusal>,
    pub begin_refusal: Option<TimeTravelRefusal>,
    pub next_problem: Option<String>,
    pub editor: Option<TimeTravelEditorPanel>,
    pub outcomes: BTreeMap<String, protocol::MutationReplayOutcome>,
    pub edited: BTreeSet<String>,
    pub accepted: BTreeSet<String>,
}

/// 🚥️ What the action controls of one mutation row answer in the open session and instance: why Edit (`begin`) and
/// Withdraw are refused (`None`: offered), whether the mutation holds an accepted draft — its row then offers Restore
/// in Withdraw's place — and why Restore is refused.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MutationRowRefusals {
    pub begin: Option<TimeTravelLabel>,
    pub withdraw: Option<TimeTravelLabel>,
    pub accepted: bool,
    pub restore: Option<TimeTravelLabel>,
}

/// 🛃️ The [`MutationRowRefusals`] of the mutation `mutation_id` held by `store` (`None`: the document's own): a session
/// on another store, or a `busy` instance, refuses Edit and Withdraw as busy; the session's own `Begin` law refuses both
/// (`TimeTravelSession::begin_refusal`) — except Withdraw on the mutation being edited, which is the editor's own; an
/// accepted draft is restored while reviewing (`TimeTravelSession::restore_refusal`).
pub fn mutation_row_refusals(panel: Option<&TimeTravelPanel>, busy: bool, mutation_id: &str, store: Option<&str>) -> MutationRowRefusals {
    let session = panel.filter(|panel| panel.store.as_deref() == store);
    let begin = match (panel, session) {
        (Some(_), None) => Some(TimeTravelLabel::RefusalBusy),
        (_, Some(session)) => session.begin_refusal.map(TimeTravelRefusal::label),
        (None, None) => None,
    }
    .or_else(|| busy.then_some(TimeTravelLabel::RefusalBusy));
    let edited = session.is_some_and(|session| session.stage == TimeTravelStage::Editing && session.status.target.as_deref() == Some(mutation_id));
    MutationRowRefusals {
        begin,
        withdraw: if edited { None } else { begin },
        accepted: session.is_some_and(|session| session.accepted.contains(mutation_id)),
        restore: session.and_then(|session| (session.stage != TimeTravelStage::Reviewing).then_some(TimeTravelLabel::RefusalIllegal)),
    }
}

/// 🧾️ One mutation row on the history wire: the durable row with the session overlay (replay outcome, pending,
/// edited, introduced) when a history edit is open. `introduced` marks a replay outcome carrying a message (level and
/// code) the durable pre-edit outcome of the same mutation does not ([`history_outcome_introduced`]); a mutation the
/// session's replay withdrew is no longer `withdrawable`.
pub fn history_mutation_entry(view: &MutationView, panel: Option<&TimeTravelPanel>) -> HistoryMutationEntry {
    let mut worst = view.worst;
    let mut messages = &view.messages;
    let (mut superseded, mut withdrawn) = (view.superseded, view.withdrawn);
    let (mut pending, mut edited, mut introduced) = (false, false, false);
    if let Some(panel) = panel {
        if let Some(outcome) = panel.outcomes.get(&view.mutation_id) {
            worst = outcome.worst;
            messages = &outcome.messages;
            superseded = outcome.superseded;
            withdrawn = outcome.withdrawn;
            introduced = history_outcome_introduced(&view.messages, &outcome.messages);
        }
        pending = view.store == panel.store && panel.pending_after.is_some_and(|after| (view.position, view.op_index) > after);
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
        withdrawable: view.withdrawable && !withdrawn,
        pending,
        edited,
        introduced,
        store: view.store.clone(),
    }
}

/// 🆕️ Whether a replayed outcome carries a message whose level and code the durable pre-edit outcome of the same
/// mutation does not — the outcome an edit made new (design §16.5).
pub fn history_outcome_introduced(before: &[protocol::MutationMessage], replayed: &[protocol::MutationMessage]) -> bool {
    replayed.iter().any(|message| !before.iter().any(|earlier| earlier.level == message.level && earlier.code == message.code))
}
//#endregion 🔖️PanelView

//#region 🔖️Pointer
/// 🔢️ The array index an RFC 6901 reference token names: `0` or a digit run without a leading zero (no sign, no padding),
/// so two pointers never address one item (audit W2A-14).
pub(crate) fn time_travel_pointer_index(segment: &str) -> Option<usize> {
    segment.parse::<usize>().ok().filter(|index| index.to_string() == segment)
}

/// 🧭️ The value at RFC 6901 `pointer` inside `value`.
pub(crate) fn time_travel_pointer_get<'a>(value: &'a DslValue, pointer: &str) -> Option<&'a DslValue> {
    let Some(rest) = pointer.strip_prefix('/') else { return (pointer.is_empty()).then_some(value) };
    let mut cursor = value;
    for segment in rest.split('/').map(|segment| segment.replace("~1", "/").replace("~0", "~")) {
        cursor = match cursor {
            DslValue::Object(entries) => &entries.iter().find(|(key, _)| *key == segment)?.1,
            DslValue::Array(items) => items.get(time_travel_pointer_index(&segment)?)?,
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
                let Some(position) = time_travel_pointer_index(segment) else { return false };
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

/// 🪝️ The value at RFC 6901 `pointer` inside `value`, mutably.
fn time_travel_pointer_get_mut<'a>(value: &'a mut DslValue, pointer: &str) -> Option<&'a mut DslValue> {
    let Some(rest) = pointer.strip_prefix('/') else { return pointer.is_empty().then_some(value) };
    let mut cursor = value;
    for segment in rest.split('/').map(|segment| segment.replace("~1", "/").replace("~0", "~")) {
        cursor = match cursor {
            DslValue::Object(entries) => &mut entries.iter_mut().find(|(key, _)| *key == segment)?.1,
            DslValue::Array(items) => items.get_mut(time_travel_pointer_index(&segment)?)?,
            _ => return None,
        };
    }
    Some(cursor)
}

/// ➕️ Inserts `item` into the array RFC 6901 `pointer` ends in: before the item its last segment indexes, or after the last
/// one for `-` (or the array's length); `false` when the pointer names no array slot.
pub(crate) fn time_travel_pointer_insert(value: &mut DslValue, pointer: &str, item: DslValue) -> bool {
    let Some((array, slot)) = pointer.rsplit_once('/') else { return false };
    let Some(DslValue::Array(items)) = time_travel_pointer_get_mut(value, array) else { return false };
    let Some(position) = (if slot == "-" { Some(items.len()) } else { time_travel_pointer_index(slot).filter(|position| *position <= items.len()) }) else { return false };
    items.insert(position, item);
    true
}

/// ➖️ Removes the array item RFC 6901 `pointer` addresses; `false` when it names no item.
pub(crate) fn time_travel_pointer_remove(value: &mut DslValue, pointer: &str) -> bool {
    let Some((array, slot)) = pointer.rsplit_once('/') else { return false };
    let Some(DslValue::Array(items)) = time_travel_pointer_get_mut(value, array) else { return false };
    let Some(position) = time_travel_pointer_index(slot).filter(|position| *position < items.len()) else { return false };
    items.remove(position);
    true
}

/// 🌱️ The value a new item of `schema` starts as: a number's lowest admitted value (a soft or hard minimum, an excluded one
/// stepped over, else 0), a choice's first option, an empty text, `false`, a vector of such numbers, an empty reference
/// list (an empty id for one reference), the fewest items a list admits, and an object of its required fields (their
/// declared defaults first); the payload schema then judges it like any drafted value.
pub(crate) fn time_travel_default_value(schema: &ArgSchema) -> DslValue {
    let number = |min: Option<f64>, max: Option<f64>, exclusive: bool, step: Option<f64>, integer: bool| {
        let low = min.map_or_else(|| max.map_or(0.0, |max| max.min(0.0)), |min| if exclusive { min + step.filter(|step| *step > 0.0).unwrap_or(1.0) } else { min });
        DslValue::json_number(if integer { low.ceil() } else { low })
    };
    match schema {
        ArgSchema::Number { min, min_exclusive, max, step, integer, soft_min, .. } => number(soft_min.or(*min), *max, *min_exclusive && soft_min.is_none(), *step, *integer),
        ArgSchema::String { options, .. } => DslValue::String(options.first().map(|option| option.value.clone()).unwrap_or_default()),
        ArgSchema::Boolean => DslValue::Bool(false),
        ArgSchema::Vector { dims, min, max, .. } => DslValue::Array((0..*dims).map(|_| number(*min, *max, false, None, false)).collect()),
        ArgSchema::Reference { many: true, .. } => DslValue::Array(Vec::new()),
        ArgSchema::Reference { .. } => DslValue::String(String::new()),
        ArgSchema::Array { items, min_items, .. } => DslValue::Array((0..min_items.unwrap_or(0)).map(|_| time_travel_default_value(items)).collect()),
        ArgSchema::Object { fields } => DslValue::Object(fields.iter().filter(|field| field.required).map(|field| (field.key(), field.default.clone().unwrap_or_else(|| time_travel_default_value(&field.schema)))).collect()),
        ArgSchema::Any => DslValue::Null,
    }
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
                time_travel_pointer_index(&segment)?;
                return segments.next().is_none().then_some((input, true));
            }
            ArgSchema::Array { items, .. } => {
                time_travel_pointer_index(&segment)?;
                ActionArgDef { id: format!("/{segment}"), schema: items.as_ref().clone(), required: true, nullable: false, default: None, ..input.clone() }
            }
            _ => return None,
        };
    }
    Some((input, false))
}

/// 🎛️ One row of the draft editor: the RFC 6901 pointer it writes, its label (with the path of labels and item numbers
/// that lead to it), its descriptor and its current value; a list input's own row carries its [`TimeTravelList`], and
/// the first row of a list item its [`TimeTravelListItem`].
#[derive(Clone, Debug, PartialEq)]
pub struct TimeTravelInputRow {
    pub pointer: String,
    pub label: LocalizedLabel,
    pub input: ActionArgDef,
    pub value: DslValue,
    pub list: Option<TimeTravelList>,
    pub item: Option<TimeTravelListItem>,
}

/// 📋️ A list input's row: how many items it holds, whether one more is admitted, and its `maxItems`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeTravelList {
    pub len: usize,
    pub addable: bool,
    pub max: Option<u32>,
}

/// 🔢️ The first row of one list item: its index, whether removing it is admitted, and the list's `minItems`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeTravelListItem {
    pub index: usize,
    pub removable: bool,
    pub min: Option<u32>,
}

/// 🎛️ The draft editor's rows over `value`: the variant selector and every input of the variant `value` is in, objects
/// flattened into their fields, and a list input (an array of any item kind) as its own row ([`TimeTravelList`], which
/// adds an item) followed by every item `value` holds — an object item as one item row then its fields, any other item as
/// its own row ([`TimeTravelListItem`], which removes the item); a hidden input is never a row.
pub(crate) fn time_travel_input_rows(inputs: &[ActionArgDef], value: &DslValue) -> Vec<TimeTravelInputRow> {
    fn nested(parent: &LocalizedLabel, child: &LocalizedLabel) -> LocalizedLabel {
        LocalizedLabel::from_fn(|terminology, locale| format!("{} \u{b7} {}", parent.resolve(terminology, locale), child.resolve(terminology, locale)))
    }
    fn numbered(parent: &LocalizedLabel, index: usize) -> LocalizedLabel {
        LocalizedLabel::from_fn(|terminology, locale| format!("{} {}", parent.resolve(terminology, locale), index + 1))
    }
    fn push(input: &ActionArgDef, pointer: String, label: LocalizedLabel, value: &DslValue, item: Option<TimeTravelListItem>, rows: &mut Vec<TimeTravelInputRow>) {
        if input.presentation == Some(ArgPresentation::Hidden) {
            return;
        }
        let current = time_travel_pointer_get(value, &pointer);
        let row = |list: Option<TimeTravelList>| TimeTravelInputRow { pointer: pointer.clone(), label: label.clone(), input: input.clone(), value: current.cloned().unwrap_or(DslValue::Null), list, item };
        match &input.schema {
            ArgSchema::Object { fields } if !fields.is_empty() => {
                if item.is_some() {
                    rows.push(row(None));
                }
                for field in fields {
                    push(field, format!("{pointer}{}", field.id), nested(&label, &field.label), value, None, rows);
                }
            }
            ArgSchema::Array { items, min_items, max_items } => {
                let len = current.and_then(DslValue::as_array).map_or(0, <[DslValue]>::len);
                let count = u32::try_from(len).unwrap_or(u32::MAX);
                rows.push(row(Some(TimeTravelList { len, addable: max_items.is_none_or(|max| count < max), max: *max_items })));
                for index in 0..len {
                    let element = ActionArgDef { id: format!("/{index}"), schema: items.as_ref().clone(), required: true, nullable: false, default: None, ..input.clone() };
                    push(&element, format!("{pointer}/{index}"), numbered(&label, index), value, Some(TimeTravelListItem { index, removable: min_items.is_none_or(|min| count > min), min: *min_items }), rows);
                }
            }
            _ => rows.push(row(None)),
        }
    }
    let mut rows = Vec::new();
    for input in time_travel_active_inputs(inputs, value) {
        push(input, input.id.clone(), input.label.clone(), value, None, &mut rows);
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

/// 🗝️ The object of `document` an option source names for `payload`: its template walked segment by segment (RFC 6901
/// escapes), each `{field}` segment taking the payload's top-level member `field` (text, or an integer in decimal) — the key
/// of an object, or, where the walk stands on an array, the record whose own `field` member equals it
/// (`/hostSnapshot/widgets/{id}/params`: the `params` of the widget whose `id` is the payload's `id`). `None` while a member
/// is missing or nothing matches.
pub(crate) fn time_travel_option_target<'a>(template: &str, payload: &DslValue, document: &'a DslValue) -> Option<&'a DslValue> {
    let text = |value: &DslValue| match value {
        DslValue::String(text) => Some(text.clone()),
        other => other.as_i64().map(|number| number.to_string()).or_else(|| other.as_u64().map(|number| number.to_string())),
    };
    let mut cursor = document;
    for segment in template.strip_prefix('/')?.split('/') {
        let field = segment.strip_prefix('{').and_then(|rest| rest.strip_suffix('}'));
        let key = match field {
            Some(field) => text(payload.get(field)?)?,
            None => segment.replace("~1", "/").replace("~0", "~"),
        };
        cursor = match (cursor, field) {
            (DslValue::Array(records), Some(field)) => records.iter().find(|record| record.get(field).and_then(text).as_deref() == Some(key.as_str()))?,
            (DslValue::Array(items), None) => items.get(time_travel_pointer_index(&key)?)?,
            (DslValue::Object(entries), _) => &entries.iter().find(|(name, _)| *name == key)?.1,
            _ => return None,
        };
    }
    Some(cursor)
}

/// 🏷️ The options an object of the previewed document offers: one per key, in order, labelled by the record's own `label`
/// or `name` (`{en, de}` or text), else the framework input-label glossary, else the key itself.
pub(crate) fn time_travel_document_options(document: &DslValue) -> Vec<ActionArgOption> {
    let DslValue::Object(entries) = document else { return Vec::new() };
    let named = |record: &DslValue| {
        ["label", "name"].iter().filter_map(|field| record.get(field)).find_map(|label| match label {
            DslValue::String(text) if !text.trim().is_empty() => Some(LocalizedLabel::data(text.clone())),
            DslValue::Object(_) => label.get("en").and_then(DslValue::as_str).zip(label.get("de").and_then(DslValue::as_str)).map(|(en, de)| LocalizedLabel::native(en, de)),
            _ => None,
        })
    };
    entries.iter().map(|(key, record)| ActionArgOption { value: key.clone(), label: named(record).or_else(|| input_label_glossary().get(key).cloned()).unwrap_or_else(|| LocalizedLabel::data(key.clone())) }).collect()
}

/// 🗝️ Makes every `optionSource` of `rows` concrete (design §20.11): the options are the keys of the object `document` — the
/// document the editor previews — holds where the source's template leads for the edited `payload`
/// ([`time_travel_option_target`], [`time_travel_document_options`]). A row's own value stays an option when the document lacks it, and the
/// row keeps its `optionSource`, so its JSON schema lists no enum: the fold, not the editor, refuses an unknown key.
pub(crate) fn time_travel_resolve_options(rows: &mut [TimeTravelInputRow], payload: &DslValue, document: &DslValue) {
    for row in rows {
        let ArgSchema::String { options, option_source: Some(OptionSource::Snapshot { pointer }), .. } = &mut row.input.schema else { continue };
        let pointer = pointer.clone();
        let mut resolved = time_travel_option_target(&pointer, payload, document).map(time_travel_document_options).unwrap_or_default();
        if let Some(current) = row.value.as_str().filter(|current| !current.is_empty() && !resolved.iter().any(|option| option.value == *current)) {
            resolved.push(ActionArgOption { value: current.to_string(), label: LocalizedLabel::data(current) });
        }
        *options = resolved;
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
    let detents = low
        .zip(high)
        .filter(|(low, high)| low <= high && ((high - low) / spacing).floor() < UI_FIXED_LIST_ITEMS as f64)
        .map(|(low, high)| ((low / spacing).ceil() as i64..=(high / spacing).floor() as i64).map(|index| index as f64 * spacing).collect::<Vec<_>>());
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

/// 🎯️ The draft value of reference input `input` read from `selection`, the current selection of its declared domain:
/// the selected ids typed by the input's id type (ids the type cannot spell are skipped), all of them up to `max_items`
/// for a many-reference, else the first. Refused `unknown-input` for an input that is no reference or names no domain,
/// `no-selection` when the domain selects nothing at the input's granularity.
pub(crate) fn time_travel_selection_value(input: &ActionArgDef, selection: Option<&protocol::DomainSelection>) -> Result<DslValue, TimeTravelActionRefusal> {
    let ArgSchema::Reference { domain: Some(_), granularity, many, max_items, id_type, .. } = &input.schema else { return Err(TimeTravelActionRefusal::UnknownInput) };
    let selection = selection.filter(|selection| granularity.as_ref().is_none_or(|granularity| *granularity == selection.granularity)).ok_or(TimeTravelActionRefusal::NoSelection)?;
    let mut ids = selection.ids.iter().filter_map(|id| id_type.id_value(id));
    if !*many {
        return ids.next().ok_or(TimeTravelActionRefusal::NoSelection);
    }
    let ids: Vec<DslValue> = ids.take(max_items.map_or(usize::MAX, |max| max as usize)).collect();
    match ids.is_empty() {
        true => Err(TimeTravelActionRefusal::NoSelection),
        false => Ok(DslValue::Array(ids)),
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
    let Ok(root) = semio_framework_pack_json::parse(schema_json, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return Vec::new() };
    let mut pending = Vec::new();
    references(&semio_framework_pack_json::to_dsl_value(&root), &mut pending);
    let mut seen = BTreeSet::new();
    let mut documents = Vec::new();
    while let Some(id) = pending.pop() {
        if id.is_empty() || !seen.insert(id.clone()) {
            continue;
        }
        let Some(document) = registered_input_schema_document(&id) else { continue };
        references(&document, &mut pending);
        documents.push(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&document)));
    }
    documents
}

/// 🧾️ The JSON text of `value`.
fn time_travel_json(value: &DslValue) -> String {
    semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(value))
}

/// 🔎️ The names the document value gives the entities `ids` — the generic default behind reference chips (gap N3): the
/// first object at any depth whose `id` is one of `ids` is named by its first `label`, `name`, `title` or `text` that is a
/// non-empty string (data in every locale) or an `{en, de}` pair. The walk visits at most
/// [`TIME_TRAVEL_ENTITY_NAME_VISITS`] values and stops once every id is named.
pub fn time_travel_entity_names(document: &DslValue, ids: &BTreeSet<&str>) -> BTreeMap<String, LocalizedLabel> {
    let text = |value: &DslValue| value.as_str().map(str::trim).filter(|text| !text.is_empty()).map(str::to_string);
    let name = |value: &DslValue| match value {
        DslValue::Object(fields) => {
            let locale = |key: &str| fields.iter().find(|(field, _)| field == key).and_then(|(_, value)| text(value));
            Some(LocalizedLabel::native(&locale("en")?, &locale("de")?))
        }
        other => text(other).map(LocalizedLabel::data),
    };
    let mut names = BTreeMap::new();
    let mut stack = vec![document];
    let mut visits = 0;
    while let Some(value) = stack.pop() {
        visits += 1;
        if visits > TIME_TRAVEL_ENTITY_NAME_VISITS || names.len() == ids.len() {
            break;
        }
        match value {
            DslValue::Object(fields) => {
                let id = fields.iter().find(|(key, _)| key == "id").and_then(|(_, id)| id.as_str()).filter(|id| ids.contains(id) && !names.contains_key(*id));
                if let Some((id, label)) = id.and_then(|id| Some((id, ["label", "name", "title", "text"].iter().find_map(|key| fields.iter().find(|(field, _)| field == key).and_then(|(_, value)| name(value)))?))) {
                    names.insert(id.to_string(), label);
                }
                stack.extend(fields.iter().rev().map(|(_, value)| value));
            }
            DslValue::Array(items) => stack.extend(items.iter().rev()),
            _ => {}
        }
    }
    names
}

/// 🏷️ The chip of an entity `id` that neither the app nor the document names (gap N3): the reference's first declared kind
/// as a word — its last segment through the framework input-label glossary in every locale, else that segment spelled out —
/// then a short id (the id before any `!` qualifier, cut to [`TIME_TRAVEL_SHORT_ID_CHARS`] characters). Without a declared
/// kind the short id alone.
pub fn time_travel_reference_fallback_label(kinds: &[String], id: &str) -> LocalizedLabel {
    let bare = id.split('!').next().unwrap_or(id);
    let short = if bare.chars().count() > TIME_TRAVEL_SHORT_ID_CHARS { format!("{}\u{2026}", bare.chars().take(TIME_TRAVEL_SHORT_ID_CHARS).collect::<String>()) } else { bare.to_string() };
    let Some(segment) = kinds.first().and_then(|kind| kind.split('@').next()).and_then(|kind| kind.rsplit(['.', ':', '/', '#']).next()).filter(|segment| !segment.is_empty()) else {
        return LocalizedLabel::data(short);
    };
    let word = input_label_glossary().get(segment).cloned().unwrap_or_else(|| LocalizedLabel::data(time_travel_spelled_out(segment)));
    LocalizedLabel::from_fn(move |terminology, locale| format!("{} {short}", word.resolve(terminology, locale)))
}

/// 🔤️ An identifier segment as words with a leading capital: `targetRegion`, `target-region` and `target_region` read
/// "Target region".
fn time_travel_spelled_out(segment: &str) -> String {
    let mut words = String::new();
    for (index, character) in segment.chars().enumerate() {
        match character {
            '-' | '_' => words.push(' '),
            upper if upper.is_uppercase() && index > 0 => words.extend([' '].into_iter().chain(upper.to_lowercase())),
            first if index == 0 => words.extend(first.to_uppercase()),
            other => words.push(other),
        }
    }
    words
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

/// 🪨️ Whether the store's supersede law admits withdrawing `operation` (`store::admit_replacement`, the one law every
/// authoring, ingest, load and fold site reads): what a mutation row offers its Withdraw by and what a row's Withdraw
/// opens by (design §22.1). A withdrawal names no schema, so the law is asked with none.
pub(crate) fn time_travel_admits_withdrawal<P, Mu>(operation: &Mu) -> bool
where
    Mu: ::protocol::Mutation<P> + ::protocol::OpBinary,
{
    store::admit_replacement::<P, Mu>(operation, &protocol::InputReplacement::Withdrawn, "").is_ok()
}

thread_local! {
    /// 🫙️ Whether a payload schema (by the address and length of its static text) describes an input an editor shows as a
    /// row ([`time_travel_schema_shows_inputs`]): read once per schema, since every mutation row asks.
    static TIME_TRAVEL_SCHEMA_SHOWS_INPUTS: std::cell::RefCell<HashMap<(usize, usize), bool>> = std::cell::RefCell::new(HashMap::new());
}

/// 👁️ Whether the payload schema `schema` describes at least one input a draft editor shows as a row (design §22.20: a
/// mutation whose inputs are all hidden is not editable, it keeps Withdraw): an input that is not hidden — an object one
/// through its fields ([`time_travel_input_rows`]). A schema that cannot be read answers `true` and is not remembered:
/// its editor opens naming why (the referenced documents may register later).
pub(crate) fn time_travel_schema_shows_inputs(schema: &'static str) -> bool {
    fn shown(input: &ActionArgDef) -> bool {
        input.presentation != Some(ArgPresentation::Hidden) && !matches!(&input.schema, ArgSchema::Object { fields } if !fields.is_empty() && !fields.iter().any(shown))
    }
    let key = (schema.as_ptr() as usize, schema.len());
    if let Some(known) = TIME_TRAVEL_SCHEMA_SHOWS_INPUTS.with(|cache| cache.borrow().get(&key).copied()) {
        return known;
    }
    let Ok(inputs) = mutation_input_defs(schema, &registered_input_schema_document) else { return true };
    let shows = inputs.iter().any(shown);
    TIME_TRAVEL_SCHEMA_SHOWS_INPUTS.with(|cache| cache.borrow_mut().insert(key, shows));
    shows
}

/// 🏷️ The history label of one document operation in every shell locale: its leaf's `SemanticMutation::label` — never the
/// operation's text line (design §16.2).
pub(crate) fn time_travel_mutation_label<A: ArtifactApp>(op: &A::Mutation) -> LocalizedLabel {
    protocol::SemanticMutation::<A::Snapshot>::label(op)
}

/// ✏️ The mutation rows of one edit's applied `ops` (in op order): the first [`HISTORY_ROW_MUTATION_ROWS`], then at most as
/// many later ones carrying messages or a supersession, the most severe first — so the row's worst severity is always
/// among them (audit W2A-11) — labelled from their effective input (a label of `labelled`, the
/// previous projection, is reused for an operation that was not superseded) with their durable outcome. `superseded`
/// and `withdrawn` read the store's effective supersession: an input restored to its original (the undo of a history
/// edit) is not superseded.
pub(crate) fn history_mutation_views<A: ArtifactApp>(ops: &[&store::AppliedMutation<'_, A::Mutation>], outcomes: &HashMap<&str, &protocol::MutationReplayOutcome>, labelled: &HashMap<&str, &MutationView>) -> Vec<MutationView> {
    history_mutation_views_of::<A::Snapshot, A::Mutation>(ops, outcomes, labelled, time_travel_mutation_label::<A>, A::ROLE == AppRole::Viewer, None)
}

/// 🧩️ [`history_mutation_views`] over any store's typed operations: the document's own (labelled by the app) or one
/// composed member's (labelled by its leaves, each row naming that member `store`, design §12). A `viewer` edits nothing.
pub(crate) fn history_mutation_views_of<P, Mu>(
    ops: &[&store::AppliedMutation<'_, Mu>],
    outcomes: &HashMap<&str, &protocol::MutationReplayOutcome>,
    labelled: &HashMap<&str, &MutationView>,
    label_of: impl Fn(&Mu) -> LocalizedLabel,
    viewer: bool,
    store: Option<&str>,
) -> Vec<MutationView>
where
    Mu: ::protocol::Mutation<P> + ::protocol::OpBinary,
{
    let outcome_of = |op: &store::AppliedMutation<'_, Mu>| outcomes.get(op.mutation_id.0.as_str()).copied();
    let flagged = ops.iter().enumerate().filter_map(|(index, op)| (op.supersession.is_some() || outcome_of(op).is_some_and(|outcome| outcome.worst.is_some())).then(|| (index, outcome_of(op).and_then(|outcome| outcome.worst))));
    history_projected_operations(ops.len(), flagged).into_iter().map(|index| history_mutation_view_of::<P, Mu>(ops[index], outcome_of(ops[index]), labelled, &label_of, viewer, store)).collect()
}

/// 🔢️ Which operations of an edit of `len` operations one history row projects (audit W2A-11): the first
/// [`HISTORY_ROW_MUTATION_ROWS`], then at most as many later `flagged` ones (index and worst severity — `None` for a
/// supersession without an outcome), the most severe first and the earliest among equals, all back in op order.
pub(crate) fn history_projected_operations(len: usize, flagged: impl IntoIterator<Item = (usize, Option<semio_framework_diagnostic::Severity>)>) -> Vec<usize> {
    let mut later: Vec<(usize, Option<semio_framework_diagnostic::Severity>)> = flagged.into_iter().filter(|(index, _)| *index >= HISTORY_ROW_MUTATION_ROWS && *index < len).collect();
    later.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    later.truncate(HISTORY_ROW_MUTATION_ROWS);
    let mut shown: Vec<usize> = (0..len.min(HISTORY_ROW_MUTATION_ROWS)).chain(later.into_iter().map(|(index, _)| index)).collect();
    shown.sort_unstable();
    shown
}

/// 🧾️ One operation's mutation row — [`history_mutation_views_of`] without the row cap: labelled from its effective input
/// (a label of `labelled`, the previous projection, reused for an operation that was not superseded) with its durable
/// `outcome`; `superseded`/`withdrawn` read the store's effective supersession; `editable` needs a payload schema that
/// shows an input ([`time_travel_schema_shows_inputs`], design §22.20) and no foreign steps; `withdrawable` asks the
/// store's supersede law for an operation that is not withdrawn yet ([`time_travel_admits_withdrawal`]); an operation
/// of a cross-document unit (`AppliedMutation::unit`) is neither — it takes no supersession in one document alone; a
/// `viewer` edits and withdraws nothing.
pub(crate) fn history_mutation_view_of<P, Mu>(
    op: &store::AppliedMutation<'_, Mu>,
    outcome: Option<&protocol::MutationReplayOutcome>,
    labelled: &HashMap<&str, &MutationView>,
    label_of: &impl Fn(&Mu) -> LocalizedLabel,
    viewer: bool,
    store: Option<&str>,
) -> MutationView
where
    Mu: ::protocol::Mutation<P> + ::protocol::OpBinary,
{
    let id = op.mutation_id.0.as_str();
    let effective = match op.supersession.map(|supersession| &supersession.replacement) {
        Some(protocol::InputReplacement::Input { payload, .. }) => <Mu as ::protocol::OpBinary>::decode_op(payload).ok(),
        _ => None,
    };
    let shown = effective.as_ref().unwrap_or(op.operation);
    let label = match labelled.get(id).filter(|_| op.supersession.is_none()) {
        Some(previous) => previous.label.clone(),
        None => label_of(shown),
    };
    let editable = !viewer && op.unit.is_none() && !shown.may_emit_foreign_steps() && shown.input_schema().is_some_and(time_travel_schema_shows_inputs);
    let superseded = match op.supersession.map(|supersession| &supersession.replacement) {
        None => false,
        Some(protocol::InputReplacement::Withdrawn) => true,
        Some(protocol::InputReplacement::Input { payload, .. }) => <Mu as ::protocol::OpBinary>::encode_op(op.operation).map_or(true, |original| original != *payload),
    };
    if let Some(effective) = effective {
        effective.retire_cold();
    }
    let withdrawn = op.supersession.is_some_and(|supersession| supersession.replacement == protocol::InputReplacement::Withdrawn);
    MutationView {
        mutation_id: id.to_string(),
        position: u32::try_from(op.position).unwrap_or(u32::MAX),
        op_index: op.op_index,
        label,
        worst: outcome.and_then(|outcome| outcome.worst),
        messages: outcome.map(|outcome| outcome.messages.clone()).unwrap_or_default(),
        superseded,
        withdrawn,
        editable,
        withdrawable: !viewer && !withdrawn && op.unit.is_none() && time_travel_admits_withdrawal::<P, Mu>(op.operation),
        store: store.map(str::to_string),
    }
}

//#region 🔖️MutationPages
/// 🗂️ The node key of the history body's commands section, the window every history row is a row of.
pub const HISTORY_COMMANDS_SECTION_KEY: &str = "framework.history.commands";
/// 🗝️ The node key prefix of one history row (`framework.history.entry.<seq>`), React's and wgpu's `HISTORY_ROW_KEY_PREFIX`.
pub const HISTORY_ROW_KEY_PREFIX: &str = "framework.history.entry.";

/// 📖️ The mutation rows of one history row past its projected ones, as one host window shows them: `rows` are the edit's
/// remaining operations from the `from`-th on, in op order (gap N1).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct HistoryMutationPage {
    pub from: usize,
    pub rows: Vec<MutationView>,
}

/// 📚️ The [`HistoryMutationPage`] of every history row whose window shows rows past its projection, keyed by the row's `seq`.
pub type HistoryMutationPages = BTreeMap<u64, HistoryMutationPage>;

/// 📏️ How many mutation rows a history row projects and how many it holds in all (gap N1): the projected ones (the first
/// [`HISTORY_ROW_MUTATION_ROWS`] of each edit and its flagged ones, its own and its composed members'), then every other
/// operation of its own edit in op order — or, for a row of composed-member edits alone (design §12, audit W2A-11), every
/// other operation of its member edits in their order. A row without an applied operation of its own (or, member-only,
/// of its members) holds nothing further.
pub fn history_row_mutation_extent(entry: &CommandView) -> (usize, usize) {
    let projected = entry.mutations.len();
    let own = entry.mutations.iter().filter(|mutation| mutation.store.is_none()).count();
    let shown = if entry.edit_id.is_some() { own } else { projected - own };
    let rest = if shown == 0 { 0 } else { entry.op_count.saturating_sub(shown) };
    (projected, projected + rest)
}

/// 🪟️ The window path of history row `seq`'s mutation group inside the commands section — what a host files its
/// `TreeWindowRequest` under.
pub fn history_row_window_path(seq: u64) -> String {
    format!("{HISTORY_COMMANDS_SECTION_KEY}{TREE_WINDOW_PATH_SEPARATOR}{HISTORY_ROW_KEY_PREFIX}{seq}")
}

/// 🔓️ Whether a history row opens by default: while one of its mutation rows (session overlay included) carries an
/// outcome, so a warning or a problem stays visible in history without expanding anything.
pub fn history_row_opens_by_default(mutations: &[HistoryMutationEntry]) -> bool {
    mutations.iter().any(|mutation| mutation.worst.is_some())
}

/// 🔢️ The logical mutation rows `[start, end)` a history row's window materialises at most over `total` rows — the slice
/// `TreeWindows` cuts: a host `request` (its `open`, else `default_open`) at its clamped offset, or, unrequested and open
/// by default, the first rows the shared first-paint budget (`viewport_rows`) can hold. `None` while the row is closed.
pub fn history_row_window_rows(request: Option<&TreeWindowRequest>, default_open: bool, viewport_rows: usize, total: usize) -> Option<(usize, usize)> {
    if total == 0 || !request.and_then(|request| request.open).unwrap_or(default_open) {
        return None;
    }
    let Some(request) = request else { return Some((0, total.min(UI_BUILT_CHILDREN_MAX).min(viewport_rows))) };
    let rows = (request.rows as usize).min(UI_BUILT_CHILDREN_MAX);
    let start = (request.offset as usize).min(total.saturating_sub(rows.max(1)));
    Some((start, start + rows.min(total - start)))
}
//#endregion 🔖️MutationPages

//#region 🔖️MemberHistory
/// 🧩️ One edit of a composed member store as the composing parent's history reads it (design §12): the member that holds
/// it (`<slot>/<childId>`), its authoring transaction and moment, its operations' printed lines and its mutation rows,
/// each labelled from the child's own leaves and naming that member.
#[derive(Clone, Debug, PartialEq)]
pub struct MemberEditHistory {
    pub store: String,
    pub edit_id: String,
    pub started_at: String,
    pub timestamp: Option<HybridLogicalTimestamp>,
    pub transaction: Option<protocol::TransactionRef>,
    pub intent_label: Option<LocalizedLabel>,
    pub op_count: usize,
    pub op_lines: Vec<String>,
    pub mutations: Vec<MutationView>,
}

/// 🔭️ Reads the edits of one member store into [`MemberEditHistory`] rows — every edit, or only the `wanted` ones, each
/// read as one applied-edit slice (O(that edit), design §20.14); a mutation that was not superseded reuses its label from
/// `labelled`, the previous projection.
pub(crate) struct MemberHistoryVisitor<'a, 'b> {
    pub store: String,
    pub labelled: &'a HashMap<&'b str, &'b MutationView>,
    pub viewer: bool,
    pub wanted: Option<&'a HashSet<&'b str>>,
    pub intent_kinds: fn(&str) -> &'static [&'static str],
}

impl store::MemberStoreVisitor for MemberHistoryVisitor<'_, '_> {
    type Output = Vec<MemberEditHistory>;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        let edits = &store.envelope().vcs.edits;
        let history = |edit: &protocol::Edit<Mu>, position: Option<usize>| {
            let ops = position.and_then(|position| store.applied_edit_mutations(position).ok()).unwrap_or_default();
            let durable = position.and_then(|position| store.applied_edit_outcomes(position).ok()).unwrap_or_default();
            let outcomes: HashMap<&str, &protocol::MutationReplayOutcome> = durable.iter().map(|outcome| (outcome.mutation_id.0.as_str(), outcome)).collect();
            let ops: Vec<&store::AppliedMutation<'_, Mu>> = ops.iter().collect();
            let op_count = edit.forwards.len();
            let transaction = edit.mutation_meta.first().and_then(|meta| meta.transaction.clone());
            let mutations = history_mutation_views_of::<P, Mu>(&ops, &outcomes, self.labelled, |op| protocol::SemanticMutation::<P>::label(op), self.viewer, Some(&self.store));
            let intent_label = transaction.as_ref().and_then(|reference| history_intent_label_of::<P, Mu>((self.intent_kinds)(&reference.tool), &ops, &mutations));
            MemberEditHistory {
                store: self.store.clone(),
                edit_id: edit.id.clone(),
                started_at: edit.started_at.clone(),
                timestamp: edit.mutation_meta.first().map(|meta| meta.timestamp),
                transaction,
                intent_label,
                op_count,
                op_lines: edit.forwards[op_count.saturating_sub(HISTORY_ROW_OPERATION_PREVIEW)..].iter().map(|op| op.print_op()).collect(),
                mutations,
            }
        };
        match self.wanted {
            Some(wanted) => {
                let mut found = Vec::new();
                for edit in edits.iter().rev().filter(|edit| wanted.contains(edit.id.as_str())) {
                    found.push(history(edit, store.applied_edit_position(&edit.id)));
                    if found.len() == wanted.len() {
                        break;
                    }
                }
                found
            }
            None => {
                let positions: HashMap<&str, usize> = store.applied_edit_ids().iter().enumerate().map(|(position, edit_id)| (edit_id.as_str(), position)).collect();
                edits.iter().map(|edit| history(edit, positions.get(edit.id.as_str()).copied())).collect()
            }
        }
    }
}
/// 📖️ Reads every mutation row (uncapped, unlabelled by any previous projection) of the `wanted` edits one member store holds,
/// each edit's rows in op order — what a member-only history row's window pages past its projection (audit W2A-11); O(those
/// edits).
pub(crate) struct MemberMutationPageVisitor<'a> {
    pub store: String,
    pub wanted: &'a [String],
    pub viewer: bool,
}

impl store::MemberStoreVisitor for MemberMutationPageVisitor<'_> {
    type Output = Vec<(String, Vec<MutationView>)>;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        let unlabelled = HashMap::new();
        self.wanted
            .iter()
            .filter_map(|edit_id| {
                let position = store.applied_edit_position(edit_id)?;
                let ops = store.applied_edit_mutations(position).ok()?;
                let durable = store.applied_edit_outcomes(position).unwrap_or_default();
                let outcomes: HashMap<&str, &protocol::MutationReplayOutcome> = durable.iter().map(|outcome| (outcome.mutation_id.0.as_str(), outcome)).collect();
                let rows = ops.iter().map(|op| history_mutation_view_of::<P, Mu>(op, outcomes.get(op.mutation_id.0.as_str()).copied(), &unlabelled, &|op: &Mu| protocol::SemanticMutation::<P>::label(op), self.viewer, Some(&self.store))).collect();
                Some((edit_id.clone(), rows))
            })
            .collect()
    }
}

/// 🧾️ One backfilled history row of composed-member edits no row carries yet: every unlogged member edit of one
/// transaction (or one edit without a transaction), ordered by its moment, labelled from its first leaf.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct MemberBackfillGroup {
    pub at: Option<HybridLogicalTimestamp>,
    pub transaction: Option<String>,
    pub edit_ids: Vec<String>,
    pub label: LocalizedLabel,
    pub started_at: String,
}

/// 🧾️ How a reload backfills composed-member edits (design §12): the edits of a transaction that also landed a parent
/// edit join that parent's row (`attached`, keyed by the parent edit id); every other unlogged edit forms a row of its
/// own transaction (`groups`, in moment order).
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct MemberBackfill {
    pub attached: HashMap<String, Vec<String>>,
    pub groups: Vec<MemberBackfillGroup>,
}

/// 🧾️ The [`MemberBackfill`] of `histories` given the edit ids rows already carry (`logged`) and the parent edit each
/// unlogged parent transaction landed (`parent_transactions`, transaction id → parent edit id).
pub(crate) fn member_backfill(histories: impl IntoIterator<Item = MemberEditHistory>, logged: &HashSet<&str>, parent_transactions: &HashMap<String, String>) -> MemberBackfill {
    let mut unlogged: Vec<MemberEditHistory> = histories.into_iter().filter(|history| !logged.contains(history.edit_id.as_str())).collect();
    unlogged.sort_by(|a, b| a.timestamp.partial_cmp(&b.timestamp).unwrap_or(std::cmp::Ordering::Equal).then_with(|| a.edit_id.cmp(&b.edit_id)));
    let mut backfill = MemberBackfill::default();
    for history in unlogged {
        let transaction = history.transaction.as_ref().map(|transaction| transaction.id.clone());
        if let Some(parent) = transaction.as_ref().and_then(|id| parent_transactions.get(id)) {
            backfill.attached.entry(parent.clone()).or_default().push(history.edit_id);
            continue;
        }
        if let Some(group) = backfill.groups.iter_mut().find(|group| transaction.is_some() && group.transaction == transaction) {
            group.edit_ids.push(history.edit_id);
            continue;
        }
        let label = history.intent_label.as_ref().or_else(|| history.mutations.first().map(|first| &first.label)).map_or_else(|| LocalizedLabel::data(history.op_lines.first().cloned().unwrap_or_else(|| history.edit_id.clone())), |leaf| history_leaf_row_label(leaf, history.op_count));
        backfill.groups.push(MemberBackfillGroup { at: history.timestamp, transaction, edit_ids: vec![history.edit_id], label, started_at: history.started_at });
    }
    backfill
}
//#endregion 🔖️MemberHistory

//#region 🔖️HistoryViewStamps
/// 🔢️ How many ids a [`HistoryStoreStamp`] keeps from the top of each of a store's applied and redo stacks: an undo or a
/// redo of up to this many edits between two refreshes still patches the cached history in place (design §20.14).
pub(crate) const HISTORY_STAMP_TAIL: usize = 8;

/// 🧭️ The O(1) facts of one store a cached history view was built against (design §20.14: per-mutation and per-render work
/// is O(change), never O(history)): its generation and content revision (a replaced store may restart at the same
/// generation), the sizes of its edit, checkpoint and alternative ledgers, its line and
/// checkpoint, the tops of its applied and redo stacks and the operation count of its applied top. The transition log is not
/// among them: an undo or redo is a `Revert`/`Reinstate` transition the stacks already show, and a history edit is a supersede
/// record the history view's own stamp counts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct HistoryStoreStamp {
    pub generation: u64,
    pub revision: [u8; 32],
    pub edits: usize,
    pub checkpoints: usize,
    pub alternatives: usize,
    pub line: String,
    pub checkpoint: Option<String>,
    pub applied: usize,
    pub applied_tail: Vec<String>,
    pub redo: usize,
    pub redo_tail: Vec<String>,
    pub tail_ops: usize,
}

/// 🧭️ The [`HistoryStoreStamp`] of `store`: O(1) plus one lookup of the applied top from the ledger's tail.
pub(crate) fn history_store_stamp<P, Mu>(store: &ArtifactStore<P, Mu>) -> HistoryStoreStamp
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + Send + Sync + 'static,
    Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
{
    let envelope = store.envelope();
    let tail = |stack: &crate::dsl::HistoryPageStack<String>| stack.iter_from(stack.len().saturating_sub(HISTORY_STAMP_TAIL)).cloned().collect::<Vec<_>>();
    let (applied, redo) = (store.applied_edit_ids(), store.redo_edit_ids());
    HistoryStoreStamp {
        generation: store.generation(),
        revision: store.content_revision_now(),
        edits: envelope.vcs.edits.len(),
        checkpoints: envelope.vcs.checkpoints.len(),
        alternatives: envelope.vcs.alternatives.len(),
        line: store.active_line_id(),
        checkpoint: store.current_checkpoint_id().map(str::to_string),
        applied: applied.len(),
        applied_tail: tail(applied),
        redo: redo.len(),
        redo_tail: tail(redo),
        tail_ops: applied.last().and_then(|top| envelope.vcs.edits.iter().rev().find(|edit| edit.id == *top)).map_or(0, |edit| edit.forwards.len()),
    }
}

/// 🧩️ Whether `moved` ids left the top of one stack (`from`, its recorded tail before) for the top of the other (`to`, its
/// recorded tail after) — an undo or a redo of `moved` edits: the moved ids are the same set on both sides, and what stayed
/// below them on each stack still agrees with its recorded tail over their overlap (both hold at most [`HISTORY_STAMP_TAIL`]
/// ids). `from_after` and `to_before` are the other two recorded tails.
fn history_stamp_moved(moved: usize, from: &[String], from_after: &[String], to: &[String], to_before: &[String]) -> bool {
    let overlaps = |kept: &[String], before: &[String]| {
        let overlap = kept.len().min(before.len());
        kept[kept.len() - overlap..] == before[before.len() - overlap..]
    };
    if moved > HISTORY_STAMP_TAIL || moved > from.len() || moved > to.len() {
        return false;
    }
    let (mut left, mut arrived): (Vec<&String>, Vec<&String>) = (from[from.len() - moved..].iter().collect(), to[to.len() - moved..].iter().collect());
    left.sort();
    arrived.sort();
    left == arrived && overlaps(from_after, &from[..from.len() - moved]) && overlaps(&to[..to.len() - moved], to_before)
}

/// 🔀️ The edits whose history rows a store change since `previous` touched, each with its applied position (`None` once it
/// is no longer applied): nothing when the store did not move; the appended, amended or redone applied tail with the previous
/// top (its row may have grown, and a composed member row reads its member's top); an undone tail of at most
/// [`HISTORY_STAMP_TAIL`] edits with the top it uncovered. `None` for anything else — a replay, a checkout, a history edit, a
/// checkpoint, a load — and the history view rebuilds (design §20.14).
pub(crate) fn history_store_change(previous: &HistoryStoreStamp, current: &HistoryStoreStamp, applied: &crate::dsl::HistoryPageStack<String>) -> Option<Vec<(String, Option<usize>)>> {
    if (previous.generation, previous.revision) == (current.generation, current.revision) {
        return Some(Vec::new());
    }
    if (previous.checkpoints, previous.alternatives, &previous.line, &previous.checkpoint) != (current.checkpoints, current.alternatives, &current.line, &current.checkpoint) || current.edits < previous.edits {
        return None;
    }
    let position = |at: usize| applied.get(at).map(|edit_id| (edit_id.clone(), Some(at)));
    let raised = || Some((previous.applied.saturating_sub(1)..current.applied).filter_map(position).collect());
    if current.applied < previous.applied {
        let undone = previous.applied - current.applied;
        let exact = current.redo == previous.redo + undone && current.edits == previous.edits && (current.applied == 0 || previous.applied_tail.len() > undone);
        return (exact && history_stamp_moved(undone, &previous.applied_tail, &current.applied_tail, &current.redo_tail, &previous.redo_tail))
            .then(|| previous.applied_tail[previous.applied_tail.len() - undone..].iter().map(|edit_id| (edit_id.clone(), None)).chain(current.applied.checked_sub(1).and_then(position)).collect());
    }
    if current.redo > 0 && current.redo < previous.redo {
        let redone = current.applied - previous.applied;
        let exact = previous.redo - current.redo == redone && current.edits == previous.edits;
        return if exact && history_stamp_moved(redone, &previous.redo_tail, &current.redo_tail, &current.applied_tail, &previous.applied_tail) { raised() } else { None };
    }
    let grown = (current.applied, current.tail_ops, current.edits) != (previous.applied, previous.tail_ops, previous.edits);
    let prefix_kept = previous.applied == 0 || applied.get(previous.applied - 1) == previous.applied_tail.last();
    let redo_kept = current.redo == 0 || (current.redo, &current.redo_tail) == (previous.redo, &previous.redo_tail);
    if grown && prefix_kept && redo_kept { raised() } else { None }
}

/// 🧾️ One member ledger as the command-log backfill reads it: its length and last edit id and, against the length and last
/// id a backfill mark recorded, the ids of every edit after them — `None` when that prefix no longer stands (or none was
/// recorded) and the backfill reads the whole history.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct MemberLedgerTail {
    pub len: usize,
    pub last: Option<String>,
    pub fresh: Option<Vec<String>>,
}

/// 🧾️ Reads one member store's [`MemberLedgerTail`] past `known` (a recorded length and last id): O(new edits).
pub(crate) struct MemberLedgerTailVisitor<'a> {
    pub known: Option<(usize, Option<&'a str>)>,
}

impl store::MemberStoreVisitor for MemberLedgerTailVisitor<'_> {
    type Output = MemberLedgerTail;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        let edits = &store.envelope().vcs.edits;
        let len = edits.len();
        let fresh = self.known.filter(|(known, last)| *known <= len && (*known == 0 || edits.iter().rev().nth(len - known).map(|edit| edit.id.as_str()) == *last)).map(|(known, _)| {
            let mut fresh: Vec<String> = edits.iter().rev().take(len - known).map(|edit| edit.id.clone()).collect();
            fresh.reverse();
            fresh
        });
        MemberLedgerTail { len, last: edits.last().map(|edit| edit.id.clone()), fresh }
    }
}

/// 🧭️ Reads one member store's [`HistoryStoreStamp`] and, against its `previous` stamp, the edits the change touched.
pub(crate) struct MemberHistoryStampVisitor<'a> {
    pub previous: Option<&'a HistoryStoreStamp>,
}

impl store::MemberStoreVisitor for MemberHistoryStampVisitor<'_> {
    type Output = (HistoryStoreStamp, Option<Vec<(String, Option<usize>)>>);

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        let stamp = history_store_stamp(store);
        let change = self.previous.and_then(|previous| history_store_change(previous, &stamp, store.applied_edit_ids()));
        (stamp, change)
    }
}
//#endregion 🔖️HistoryViewStamps

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    /// 🧩️ The composed members' edits as the history reads them, keyed by edit id (design §12): every edit (the rows a full
    /// history build lists for the owned-child edits it carries, and the edits a reload backfills), or only the `wanted` ones
    /// (the rows an incremental refresh rebuilds, O(those edits)).
    pub(crate) fn member_edit_histories(&self, labelled: &HashMap<&str, &MutationView>, wanted: Option<&HashSet<&str>>) -> HashMap<String, MemberEditHistory> {
        let viewer = A::ROLE == AppRole::Viewer;
        let mut histories = HashMap::new();
        if wanted.is_some_and(HashSet::is_empty) {
            return histories;
        }
        for (store, entry) in self.children.addressed_entries() {
            for history in entry.member.visit_member(MemberHistoryVisitor { store, labelled, viewer, wanted, intent_kinds: A::tool_intent_kinds }) {
                histories.insert(history.edit_id.clone(), history);
            }
        }
        histories
    }

    /// 🧾️ Every composed member's [`MemberLedgerTail`] past the lengths and last ids a backfill mark recorded (`known`), keyed
    /// and ordered by member (`<slot>/<childId>`), and whether every member `known` names is still held.
    pub(crate) fn member_ledger_tails(&self, known: Option<&[(String, usize, Option<String>)]>) -> (Vec<(String, MemberLedgerTail)>, bool) {
        let mut tails: Vec<(String, MemberLedgerTail)> = self
            .children
            .addressed_entries()
            .map(|(store, entry)| {
                let recorded = known.and_then(|known| known.iter().find(|(member, _, _)| *member == store)).map(|(_, len, last)| (*len, last.as_deref()));
                let tail = entry.member.visit_member(MemberLedgerTailVisitor { known: recorded });
                (store, tail)
            })
            .collect();
        tails.sort_by(|a, b| a.0.cmp(&b.0));
        let held = known.is_none_or(|known| known.iter().all(|(member, _, _)| tails.iter().any(|(store, _)| store == member)));
        (tails, held)
    }

    /// 🧭️ Every composed member's [`HistoryStoreStamp`], keyed and ordered by member (`<slot>/<childId>`), with the edits a
    /// change since `previous` touched ([`history_store_change`]; `None` for a member that is new or changed otherwise).
    pub(crate) fn member_history_stamps(&self, previous: &[(String, HistoryStoreStamp)]) -> Vec<(String, HistoryStoreStamp, Option<Vec<(String, Option<usize>)>>)> {
        let mut stamps: Vec<(String, HistoryStoreStamp, Option<Vec<(String, Option<usize>)>>)> = self
            .children
            .addressed_entries()
            .map(|(store, entry)| {
                let before = previous.iter().find(|(member, _)| *member == store).map(|(_, stamp)| stamp);
                let (stamp, change) = entry.member.visit_member(MemberHistoryStampVisitor { previous: before });
                (store, stamp, change)
            })
            .collect();
        stamps.sort_by(|a, b| a.0.cmp(&b.0));
        stamps
    }

    /// 🧭️ The store state a session observes — of the store it edits; `None` when that composed member is gone.
    fn time_travel_base(&self) -> Option<TimeTravelBase> {
        match self.time_travel_query(TimeTravelStoreQuery::Base)? {
            TimeTravelQueryOutput::Base(base) => Some(base),
            TimeTravelQueryOutput::Positions(_) | TimeTravelQueryOutput::Value(_) => None,
        }
    }

    /// 🔎️ A store-only read of the store the session edits: the document's own, or the composed member's through its
    /// typed visitor (design §12); `None` when that member is gone.
    fn time_travel_query(&self, query: TimeTravelStoreQuery) -> Option<TimeTravelQueryOutput> {
        match self.time_travel.member.as_ref() {
            None => Some(TimeTravelStoreState::query(Some(&self.time_travel.document), &self.store, query)),
            Some(member) => {
                let entry = self.children.member(member.key.borrowed())?;
                Some(entry.member.visit_member(TimeTravelMemberQuery { owners: member.owners.as_deref(), query }))
            }
        }
    }

    /// 📨️ Runs one typed command on the store the session edits: the document's own, or the composed member's through
    /// its typed visitor (design §12).
    async fn time_travel_run(&mut self, command: TimeTravelStoreCommand) -> Result<TimeTravelStoreOutput, Fault> {
        let VcsArtifactApp { store, time_travel, children, .. } = self;
        match time_travel.member.as_mut() {
            None => time_travel.document.run(store, command).await,
            Some(member) => {
                let key = member.key.clone();
                let entry = children.member_mut(key.borrowed()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new(TIME_TRAVEL_MEMBER_GONE_CODE), format!("the composed member {} a history edit targets is gone", member.store())))?;
                entry.member.visit_member_mut(TimeTravelMemberRun { owners: &mut member.owners, command }).await
            }
        }
    }

    /// 🧽️ Settles the typed owners for the session's stage (design §4): what it no longer shows retires against its own
    /// store, and the draft editor closes outside `Editing`. A member subject the session left hands its owners to the
    /// retiring list and its children view to the child-content retirements.
    fn settle_time_travel_owners(&mut self) -> Result<(), Fault> {
        let stage = self.time_travel.session.stage;
        if stage != TimeTravelStage::Editing {
            self.time_travel.replace_editor(None);
        }
        {
            let VcsArtifactApp { store, time_travel, children, .. } = self;
            time_travel.document.settle(store, stage)?;
            if let Some(member) = time_travel.member.as_mut() {
                let key = member.key.clone();
                if let (Some(owners), Some(entry)) = (member.owners.as_deref_mut(), children.member(key.borrowed())) {
                    entry.member.visit_member(TimeTravelMemberSettle { owners, stage })?;
                }
            }
        }
        if stage == TimeTravelStage::Inactive {
            if let Some(member) = self.time_travel.member.take() {
                self.time_travel.retiring.extend(member.owners);
                if let Some(view) = member.children {
                    self.retire_time_travel_children(view)?;
                }
            }
        }
        Ok(())
    }

    /// 🪞️ Refreshes the children view the render seams read while the session shows a composed member's preview or
    /// replayed head (design §12): the live children with that member's shown snapshot in its place. The view it
    /// replaces retires.
    async fn refresh_time_travel_children(&mut self) -> Result<(), Fault> {
        let Some(member) = self.time_travel.member.as_ref() else { return Ok(()) };
        let (key, dialect) = (member.key.clone(), member.dialect.clone());
        let stage = self.time_travel.session.stage;
        let next = match self.time_travel_run(TimeTravelStoreCommand::Read(stage)).await? {
            TimeTravelStoreOutput::Read(Some((read, revision))) => Some(self.child_content_root.with_member_read(key.borrowed(), &dialect, revision, read)?),
            _ => None,
        };
        let previous = match self.time_travel.member.as_mut() {
            Some(member) => std::mem::replace(&mut member.children, next),
            None => next,
        };
        if let Some(previous) = previous {
            self.retire_time_travel_children(previous)?;
        }
        self.time_travel.document_dirty = true;
        Ok(())
    }

    /// 🧹️ Retires a history edit's children view through the child-content retirements, as one admitted generation.
    fn retire_time_travel_children(&mut self, view: ChildContentView) -> Result<(), Fault> {
        let generation = self.admit_child_content_publication()?;
        self.child_content_retirements.insert_admitted(generation, ChildContentRetirement::new(view, false));
        self.child_content_generation = generation;
        Ok(())
    }

    /// 🧹️ One bounded close unit of the history-edit ledger: the session vanishes, every typed owner settles against its
    /// store, then one owner retires.
    pub(crate) fn time_travel_retirement_demands(&self, body: usize) -> Result<RetirementDemand,ValueError> {
        if self.time_travel.document.discarded_factory.is_none()&&(!self.time_travel.document.discarded.is_empty()||self.time_travel.document.discarded_pending.is_some()){self.store.owned_mutation_retirement_factory()?;return Ok(RetirementDemand{depth:1,..Default::default()});}
        let demand=self.time_travel.retirement_demands(body)?;if demand!=RetirementDemand::default(){return Ok(demand);}
        if self.time_travel.closing&&!self.time_travel.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"time travel close retains an undeclared original session editor preview or authoring frontier"));}
        Ok(Default::default())
    }
    pub(crate) fn time_travel_retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>,Fault> {
        if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if self.time_travel.document.discarded_factory.is_none()&&(!self.time_travel.document.discarded.is_empty()||self.time_travel.document.discarded_pending.is_some()){
            if grant.maximum_depth==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
            let factory=self.store.owned_mutation_retirement_factory().map_err(|error|Fault::from(error.into_message()))?;self.time_travel.document.discarded_factory=Some(Arc::clone(factory));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})));
        }
        self.time_travel.retire_step(grant)
    }
    pub(crate) fn time_travel_close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep,Fault> {
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        self.time_travel.begin_close();
        if let Some(step)=self.time_travel_retire_step(grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
        self.time_travel_retirement_demands(grant.maximum_copy_bytes).map_err(|error|Fault::from(error.into_message()))?;
        Ok(if self.time_travel.terminal_is_empty(){RetainedCloneStep::Complete(Default::default())}else{RetainedCloneStep::Progress(Default::default())})
    }

    /// 📖️ The mutation rows the history body's windows show past each row's projection (gap N1): for every row whose
    /// mutation window reaches past its projected rows ([`history_row_mutation_extent`], [`history_row_window_rows`] — a
    /// host request, or a row open by default under `panel`'s overlay), that slice of its edit's remaining operations in op
    /// order — or of its member edits' for a row of composed-member edits alone — so every mutation of a transaction is
    /// reachable however long it is, while the projection and the wire stay bounded. Each paged row reads only its own edits'
    /// slices (design §20.14), and only when some window reaches past its projection.
    pub(crate) fn history_mutation_pages(&self, history: &HistoryView, panel: Option<&TimeTravelPanel>, view: &ViewModel) -> HistoryMutationPages {
        let viewport_rows = view.tree_viewport_rows.unwrap_or(TREE_WINDOW_DEFAULT_ROWS) as usize;
        let viewer = A::ROLE == AppRole::Viewer;
        let mut pages = HistoryMutationPages::new();
        for entry in &history.commands {
            let (projected, total) = history_row_mutation_extent(entry);
            if total <= projected {
                continue;
            }
            let path = history_row_window_path(entry.seq);
            let request = view.tree_windows.iter().find(|request| request.body_key == FRAMEWORK_HISTORY_BODY_KEY && request.node_key == path);
            let default_open = entry.mutations.iter().any(|mutation| history_mutation_entry(mutation, panel).worst.is_some());
            let Some((start, end)) = history_row_window_rows(request, default_open, viewport_rows, total).filter(|(_, end)| *end > projected) else { continue };
            let (from, to) = (start.max(projected) - projected, end - projected);
            let shown: HashSet<&str> = entry.mutations.iter().map(|mutation| mutation.mutation_id.as_str()).collect();
            let rows: Vec<MutationView> = match entry.edit_id.as_deref() {
                Some(edit_id) => {
                    let hint = entry.mutations.iter().find(|mutation| mutation.store.is_none()).map(|mutation| mutation.position as usize);
                    let position = hint.filter(|position| self.store.applied_edit_ids().get(*position).is_some_and(|held| held == edit_id)).or_else(|| self.store.applied_edit_position(edit_id));
                    let ops = position.and_then(|position| self.store.applied_edit_mutations(position).ok()).unwrap_or_default();
                    let durable = position.and_then(|position| self.store.applied_edit_outcomes(position).ok()).unwrap_or_default();
                    let outcomes: HashMap<&str, &protocol::MutationReplayOutcome> = durable.iter().map(|outcome| (outcome.mutation_id.0.as_str(), outcome)).collect();
                    let unlabelled = HashMap::new();
                    ops.iter()
                        .filter(|op| !shown.contains(op.mutation_id.0.as_str()))
                        .skip(from)
                        .take(to - from)
                        .map(|op| history_mutation_view_of::<A::Snapshot, A::Mutation>(op, outcomes.get(op.mutation_id.0.as_str()).copied(), &unlabelled, &time_travel_mutation_label::<A>, viewer, None))
                        .collect()
                }
                None => {
                    let mut by_edit: HashMap<String, Vec<MutationView>> = HashMap::new();
                    for (store, member) in self.children.addressed_entries() {
                        by_edit.extend(member.member.visit_member(MemberMutationPageVisitor { store, wanted: &entry.child_edit_ids, viewer }));
                    }
                    entry.child_edit_ids.iter().filter_map(|edit_id| by_edit.remove(edit_id)).flatten().filter(|row| !shown.contains(row.mutation_id.as_str())).skip(from).take(to - from).collect()
                }
            };
            pages.insert(entry.seq, HistoryMutationPage { from, rows });
        }
        pages
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
                let windowed = window
                    .and_then(|window| self.window_config_store.pointer_values(&window.window_kind_id, std::slice::from_ref(&pointer)).into_iter().find(|(window_id, _)| *window_id == window.id))
                    .and_then(|(_, values)| values.into_iter().next().flatten())
                    .and_then(|value| value.as_f64());
                windowed.or_else(|| time_travel_pointer_get(config.get_or_init(|| semio_framework_value::ToValue::to_value(self.config_store.snapshot_owner().as_ref())), &pointer).and_then(DslValue::as_f64))
            }
            SnapSource::Snapshot { pointer } => time_travel_pointer_get(preview.get_or_init(|| self.time_travel_previewed_value()), pointer).and_then(DslValue::as_f64),
        });
    }

    /// 🪟️ The document the session previews, as a value: the composed member's shown value, else the app's render snapshot
    /// (the state before the edited mutation with its draft while editing, the replayed head while reviewing).
    fn time_travel_previewed_value(&self) -> DslValue {
        match self.time_travel.member.is_some() {
            true => match self.time_travel_query(TimeTravelStoreQuery::ShownValue(self.time_travel.session.stage)) {
                Some(TimeTravelQueryOutput::Value(Some(value))) => value,
                _ => DslValue::Null,
            },
            false => {
                let committed = self.store.snapshot_owner();
                semio_framework_value::ToValue::to_value(self.time_travel.render_snapshot_or(&self.tool_runs, &committed).as_ref())
            }
        }
    }

    /// 🗝️ Resolves every editor row's `optionSource` to the keys the previewed document holds ([`time_travel_resolve_options`],
    /// design §20.11), its templates filled from the draft payload.
    pub(crate) fn resolve_time_travel_options(&self, panel: &mut TimeTravelPanel) {
        let Some(editor) = panel.editor.as_mut() else { return };
        if !editor.rows.iter().any(|row| matches!(row.input.schema, ArgSchema::String { option_source: Some(_), .. })) {
            return;
        }
        let payload = self.time_travel.editor.as_ref().map_or(DslValue::Null, |draft| draft.value.clone());
        time_travel_resolve_options(&mut editor.rows, &payload, &self.time_travel_previewed_value());
    }

    /// 🏷️ Labels every id the editor's reference rows name, in the document the session previews: the app's entity label
    /// (`ArtifactApp::entity_label` over the reference's kinds — the hook a plugin refines chips by), else the entity's own
    /// name in that document ([`time_travel_entity_names`]), else its kind word and a short id
    /// ([`time_travel_reference_fallback_label`], gap N3). A session on a composed member skips the app's hook, which reads
    /// only its own document, and names the member's entities from the member's preview.
    pub(crate) fn resolve_time_travel_reference_labels(&self, panel: &mut TimeTravelPanel) {
        let Some(editor) = panel.editor.as_mut() else { return };
        let member = self.time_travel.member.is_some();
        let committed = self.store.snapshot_owner();
        let shown = self.time_travel.render_snapshot_or(&self.tool_runs, &committed);
        let mut unnamed: Vec<(String, Vec<String>)> = Vec::new();
        for row in &editor.rows {
            let ArgSchema::Reference { kinds, .. } = &row.input.schema else { continue };
            let ids: Vec<String> = match &row.value {
                DslValue::Array(items) => items.iter().filter_map(reference_id_text).collect(),
                other => reference_id_text(other).into_iter().collect(),
            };
            for id in ids {
                match (!member).then(|| A::entity_label(shown.as_ref(), kinds, &id)).flatten() {
                    Some(label) => {
                        editor.reference_labels.insert(id, label);
                    }
                    None => unnamed.push((id, kinds.clone())),
                }
            }
        }
        if unnamed.is_empty() {
            return;
        }
        let document = match member {
            true => match self.time_travel_query(TimeTravelStoreQuery::ShownValue(self.time_travel.session.stage)) {
                Some(TimeTravelQueryOutput::Value(Some(value))) => value,
                _ => DslValue::Null,
            },
            false => semio_framework_value::ToValue::to_value(shown.as_ref()),
        };
        let names = time_travel_entity_names(&document, &unnamed.iter().map(|(id, _)| id.as_str()).collect());
        for (id, kinds) in unnamed {
            let label = names.get(&id).cloned().unwrap_or_else(|| time_travel_reference_fallback_label(&kinds, &id));
            editor.reference_labels.insert(id, label);
        }
    }

    /// 🏃️ Ledger work, a remote history change the document store still replays (unless paused), or a base change the
    /// watch has not delivered to an open session yet.
    pub(crate) fn time_travel_has_pending_work(&self) -> bool {
        self.time_travel.has_pending_work() || self.reprojection_drives() || (self.time_travel.is_active() && self.time_travel_base() != Some(self.time_travel.session.base))
    }

    /// 🧾️ Holds back the history row of `action`, a history step whose replay the document store deferred (gap N17): it
    /// is recorded when the store adopts the step and dropped with the step when it is cancelled or refused, so a cancelled
    /// step leaves zero trace in the history too; the body shows the step replaying at once.
    pub(crate) fn defer_history_step_row(&mut self, action: &str) {
        self.time_travel.deferred_history_row = Some(action.to_string());
        self.note_time_travel_changed(false, false);
    }

    /// 🚜️ Whether driver turns step the document store's waiting reprojection: a local history step always (it cannot be
    /// paused, only dropped), a remote change unless the user paused it.
    fn reprojection_drives(&self) -> bool {
        self.store.reprojection_progress().is_some() && (self.store.local_step_pending() || !self.time_travel.reprojection_paused)
    }

    /// 📡️ The history change this replica replays before adopting it, as the history wire and body show it: a whole-document
    /// load first (its archive operation's progress), else the document store's waiting reprojection — this replica's own
    /// history step or a remote change — with whether the user paused a remote one and the code of a refused adoption;
    /// `None` while nothing waits.
    pub(crate) fn reprojection_status(&self) -> Option<HistoryReprojection> {
        if let Some(load) = self.live_document_load().and_then(|operation| self.document_archive_loads.get(operation)).map(ActiveDocumentArchiveLoad::status) {
            return Some(HistoryReprojection { done: u32::try_from(load.completed).unwrap_or(u32::MAX), total: u32::try_from(load.total).unwrap_or(u32::MAX), processed: None, kind: HistoryReprojectionKind::Load, paused: false, fault: None });
        }
        let kind = if self.store.local_step_pending() { HistoryReprojectionKind::Step } else { HistoryReprojectionKind::Remote };
        match (self.store.reprojection_progress(), self.time_travel.reprojection_fault.as_ref()) {
            (Some(progress), fault) => {
                let processed = self.store.reprojection_operation_preparation_progress().map(|work| work.items).filter(|items| *items > 0).map(|items| u32::try_from(items).unwrap_or(u32::MAX));
                Some(HistoryReprojection { done: progress.done, total: progress.total, processed, kind, paused: kind == HistoryReprojectionKind::Remote && self.time_travel.reprojection_paused, fault: fault.map(|(code, _)| code.clone()) })
            }
            (None, Some((code, kind))) => Some(HistoryReprojection { done: 0, total: 0, processed: None, kind: *kind, paused: false, fault: Some(code.clone()) }),
            (None, None) => None,
        }
    }

    /// 🛬️ The whole-document load (document archive operation) still running on this instance, if any
    /// (`📓️api-stepped-document-load.md`).
    pub(crate) fn live_document_load(&self) -> Option<u64> {
        let mut live = None;
        self.document_archive_loads.each_id(|operation| {
            if live.is_none() && self.document_archive_loads.get(operation).is_some_and(ActiveDocumentArchiveLoad::loading) {
                live = Some(operation);
            }
        });
        live
    }

    /// 🧹️ Dismisses the row of a refused history-change adoption (nothing replays any more) once the user dispatches anything
    /// but a view or interaction verb (audit W2A-8): the refusal was shown, the next change starts clean; the history patch
    /// carries the change.
    pub(crate) fn dismiss_refused_reprojection(&mut self, action: &str) {
        let passive = matches!(self.declared_action_kind(action), Some(ActionKind::View | ActionKind::Interaction)) || INTERACTION_ACTION_IDS.contains(&action);
        if passive || self.time_travel.reprojection_fault.is_none() || self.store.reprojection_progress().is_some() {
            return;
        }
        self.time_travel.reprojection_fault = None;
        self.note_time_travel_changed(false, false);
    }

    /// ⏭️ One budget of the waiting reprojection (design §16.6, gap N17), a remote change or this replica's own history
    /// step: progress refreshes the history at most every [`TIME_TRAVEL_PROGRESS_REFRESH_MS`]; the adoption swaps the
    /// document in, delivers `BaseMoved` and refreshes every body; a refused adoption (the store drops the change) keeps its
    /// code on the row — a local step whose replay would leave errors reads [`HISTORY_STEP_BLOCKED_CODE`] — until the next
    /// dispatch dismisses it ([`Self::dismiss_refused_reprojection`]). Nothing left waiting lifts a pause (audit W2A-8): the
    /// next remote change is driven again.
    async fn step_reprojection_turn(&mut self, deadline_us: u64) -> Result<(), Fault> {
        let generation = self.store.generation();
        let local = self.store.local_step_pending();
        let stepped = self.store.step_reprojection(Some(deadline_us)).await;
        if local && !self.store.local_step_pending() {
            if let Some(action) = self.time_travel.deferred_history_row.take().filter(|_| stepped.is_ok()) {
                self.record_command(&action, ActionKind::History, None, None, Vec::new(), None);
            }
        }
        match stepped {
            Ok(Some(_)) => {
                self.time_travel.reprojection_fault = None;
                let now_ms = semio_framework_job::default_now_ms().unwrap_or(0);
                if now_ms.saturating_sub(self.time_travel.reprojection_refreshed_ms) >= TIME_TRAVEL_PROGRESS_REFRESH_MS {
                    self.time_travel.reprojection_refreshed_ms = now_ms;
                    self.note_time_travel_changed(false, false);
                }
            }
            Ok(None) => {
                self.time_travel.reprojection_fault = None;
                self.time_travel.reprojection_paused = false;
                if self.store.generation() != generation {
                    self.cache = None;
                    self.deliver_base_moved().await?;
                }
                self.note_time_travel_changed(true, true);
            }
            Err(error) => {
                #[cfg(debug_assertions)]
                eprintln!("[DEBUG] deferred reprojection refused local={local} generation={generation} detail={error:?} message={error}");
                let code = match local && matches!(error, vcs::VcsError::Rejected { .. }) {
                    true => HISTORY_STEP_BLOCKED_CODE.to_string(),
                    false => error.into_fault().code.0,
                };
                self.time_travel.reprojection_fault = Some((code, if local { HistoryReprojectionKind::Step } else { HistoryReprojectionKind::Remote }));
                self.cache = None;
                self.note_time_travel_changed(true, true);
            }
        }
        Ok(())
    }

    /// ⏹️ `historyEditCancelReplay` while no session is open: a whole-document load still running is cancelled (the previous
    /// document stays, zero trace); a local history step that still replays is dropped with zero trace
    /// (`discard_local_step`); a waiting remote change's running replay is dropped (the store keeps the change) and driver
    /// turns leave it until `historyEditRerun`.
    fn cancel_reprojection_turns(&mut self) -> TimeTravelActionOutcome {
        if let Some(active) = self.live_document_load().and_then(|operation| self.document_archive_loads.get_mut(operation)) {
            active.request_terminal(ActiveDocumentArchiveLoadState::Cancelled);
            self.note_time_travel_changed(true, true);
            return TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive);
        }
        if self.store.discard_local_step() {
            self.time_travel.reprojection_fault = None;
            self.time_travel.deferred_history_row = None;
            self.note_time_travel_changed(true, true);
            return TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive);
        }
        if self.time_travel.reprojection_paused || self.store.reprojection_progress().is_none() {
            return TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal));
        }
        self.store.cancel_reprojection();
        self.time_travel.reprojection_paused = true;
        self.note_time_travel_changed(false, false);
        TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive)
    }

    /// ▶️ `historyEditRerun` while no session is open: drives the paused remote replay again from its start.
    fn resume_reprojection_turns(&mut self) -> TimeTravelActionOutcome {
        if !self.time_travel.reprojection_paused {
            return TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal));
        }
        self.time_travel.reprojection_paused = false;
        self.note_time_travel_changed(false, false);
        TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive)
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

    /// 🎯️ The applied position of every session target at the current base of the store it edits; `None` when one of
    /// them vanished or the edited member is gone.
    fn time_travel_positions(&self) -> Option<Vec<TimeTravelTarget>> {
        let session = &self.time_travel.session;
        let targets: Vec<MutationId> = session.accepted.iter().map(|draft| draft.target.mutation.clone()).chain(session.pending.as_ref().map(|pending| pending.target.mutation.clone())).collect();
        if targets.is_empty() {
            return Some(Vec::new());
        }
        match self.time_travel_query(TimeTravelStoreQuery::Positions(targets))? {
            TimeTravelQueryOutput::Positions(positions) => positions,
            TimeTravelQueryOutput::Base(_) | TimeTravelQueryOutput::Value(_) => None,
        }
    }

    /// 👀️ Store watch (design §4, driver contract): every change of the content revision of the store the session edits
    /// (never of its local generation alone: a document port attaching or detaching is no base move) reaches the
    /// session as `BaseMoved` with the re-resolved target positions — the open editor's too, so the rows downstream of
    /// the edited mutation stay the pending ones when a remote edit sorts before it; a target that vanished (undone
    /// remotely, document replaced, the edited member gone) exits the session, since nothing it drafted is addressable
    /// anymore. Answers whether the base moved.
    async fn watch_time_travel_base(&mut self) -> Result<bool, Fault> {
        let base = self.time_travel_base();
        if base == Some(self.time_travel.session.base) {
            return Ok(false);
        }
        let mut effects = Vec::new();
        match (base, self.time_travel_positions()) {
            (Some(base), Some(positions)) => effects.extend(self.time_travel.session.apply(TimeTravelEvent::BaseMoved { base, positions }).unwrap_or_default()),
            (base, _) => {
                let base = base.unwrap_or(self.time_travel.session.base);
                effects.extend(self.time_travel.session.apply(TimeTravelEvent::Exit).unwrap_or_default());
                effects.extend(self.time_travel.session.apply(TimeTravelEvent::BaseMoved { base, positions: Vec::new() }).unwrap_or_default());
            }
        }
        if let (Some(editor), Some(pending)) = (self.time_travel.editor.as_mut(), self.time_travel.session.pending.as_ref()) {
            editor.position = pending.target.position;
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
        if !self.time_travel.is_active() {
            match action {
                HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID => return Ok(self.cancel_reprojection_turns()),
                HISTORY_EDIT_RERUN_ACTION_ID => return Ok(self.resume_reprojection_turns()),
                _ => {}
            }
        }
        let generation = time_travel_arg_generation(args).unwrap_or(self.time_travel.session.generation);
        let event = match action {
            HISTORY_EDIT_BEGIN_ACTION_ID => return self.begin_time_travel(args, false, meta, effects).await,
            HISTORY_EDIT_INPUT_ACTION_ID => {
                let value = args.and_then(|args| args.get(HISTORY_EDIT_ARG_VALUE)).cloned();
                return self.draft_time_travel_input(time_travel_arg_text(args, HISTORY_EDIT_ARG_PATH), value, time_travel_arg_text(args, HISTORY_EDIT_ARG_EDIT), generation, meta, effects).await;
            }
            HISTORY_EDIT_USE_SELECTION_ACTION_ID => return self.draft_time_travel_selection(time_travel_arg_text(args, HISTORY_EDIT_ARG_PATH), generation, meta, effects).await,
            HISTORY_EDIT_WITHDRAW_ACTION_ID if time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID).is_some() => return self.begin_time_travel(args, true, meta, effects).await,
            HISTORY_EDIT_WITHDRAW_ACTION_ID => TimeTravelEvent::Withdraw { generation },
            HISTORY_EDIT_ACCEPT_ACTION_ID => TimeTravelEvent::Accept { generation },
            HISTORY_EDIT_DISCARD_ACTION_ID => TimeTravelEvent::Discard { generation },
            HISTORY_EDIT_RESTORE_ACTION_ID => {
                let Some(mutation) = time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownMutation)) };
                if self.time_travel.member_store().as_deref() != time_travel_arg_text(args, HISTORY_EDIT_ARG_STORE).filter(|store| !store.is_empty()) {
                    return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal)));
                }
                TimeTravelEvent::Restore { generation, target: MutationId(mutation.to_string()) }
            }
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

    /// 🎞️ The identity of the document snapshot the session shows in its stage on the document's own store (`None`: the
    /// committed document) — what tells whether a session change swapped the document a window renders. A session on a
    /// composed member marks its children view itself ([`Self::refresh_time_travel_children`]).
    fn time_travel_shown_identity(&self) -> Option<usize> {
        self.time_travel.document.shown(self.time_travel.session.stage).map(|shown| Arc::as_ptr(shown.snapshot_owner()) as usize)
    }

    /// ⚖️ Applies one event to the session and performs its effects; a refusal changes nothing. Every window body is
    /// re-published only when the event or its effects swapped the document a window shows (the committed document, the
    /// draft preview, the replayed head); any other edge — the finalize prompt opening or closing, an accepted draft
    /// starting its replay, a replay run again — re-publishes the history body and the chips alone.
    async fn apply_time_travel_event(&mut self, event: TimeTravelEvent, meta: Option<&ActionMeta>, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        let shown = self.time_travel_shown_identity();
        match self.time_travel.session.apply(event) {
            Ok(session_effects) => {
                self.note_time_travel_changed(false, true);
                self.perform_time_travel_effects(session_effects, meta, effects).await?;
                self.time_travel.document_dirty |= shown != self.time_travel_shown_identity();
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
                TimeTravelEffect::ShowPreview { target, replacement } => self.show_time_travel_preview(&target, &replacement).await?,
                TimeTravelEffect::StartReplay { drafts, from } => queue.extend(self.start_time_travel_replay(drafts, &from).await?),
                TimeTravelEffect::CancelReplay => {
                    self.time_travel_run(TimeTravelStoreCommand::CancelReplay).await?;
                }
                TimeTravelEffect::OpenFinalizePrompt => {
                    let args = meta
                        .and_then(|meta| meta.view_state.as_ref())
                        .map(|view| DslValue::object([(HISTORY_EDIT_ARG_NAME.to_string(), DslValue::String(TimeTravelLabel::AlternativeNameDefault.localized(LocalizedLabel::native).resolve(Terminology::Native, view.locale).to_string()))]));
                    kernel.push(Effect::OpenDialog { req: TIME_TRAVEL_FINALIZE_REQUEST, dialog_id: HISTORY_EDIT_FINALIZE_DIALOG_ID.to_string(), args });
                }
                TimeTravelEffect::CommitOverwrite { .. } => queue.extend(self.commit_time_travel(store::HistoryFinalization::Overwrite, meta).await?),
                TimeTravelEffect::CommitAlternative { name, .. } => queue.extend(self.commit_time_travel(store::HistoryFinalization::Alternative { name }, meta).await?),
                TimeTravelEffect::Close => self.time_travel.document_dirty = true,
            }
        }
        self.settle_time_travel_owners()?;
        self.refresh_time_travel_children().await
    }

    /// 🖼️ The editing preview on the store the session edits: the state before `target` with the accepted upstream
    /// drafts, then `replacement` folded exactly as the store's replay folds it; the draft's own messages stay on the
    /// editor.
    async fn show_time_travel_preview(&mut self, target: &MutationId, replacement: &protocol::InputReplacement) -> Result<(), Fault> {
        let drafts = self.time_travel.accepted_drafts();
        let deadline_us = self.time_travel_turn_deadline();
        let TimeTravelStoreOutput::Previewed(step) = self.time_travel_run(TimeTravelStoreCommand::Preview { target: target.clone(), replacement: replacement.clone(), drafts, deadline_us, clock: self.time_travel.turn_clock }).await? else {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.preview-mismatch"), "a history-edit preview answered another command"));
        };
        self.adopt_time_travel_preview_step(step).await
    }

    async fn adopt_time_travel_preview_step(&mut self, step: TimeTravelPreviewStep) -> Result<(), Fault> {
        if let TimeTravelPreviewStep::Completed(outcome) = step {
            if let Some(editor) = self.time_travel.editor.as_mut() { editor.outcome = outcome; }
            self.time_travel.document_dirty = true;
            self.refresh_time_travel_children().await?;
        }
        self.note_time_travel_changed(false, false);
        Ok(())
    }

    /// ▶️ Starts (latest-wins) the Report replay of `drafts` from `from` on the store the session edits; a store refusal
    /// faults the replay instead.
    async fn start_time_travel_replay(&mut self, drafts: Vec<protocol::SupersededInput>, from: &MutationId) -> Result<Vec<TimeTravelEffect>, Fault> {
        self.time_travel.refreshed_done = 0;
        let drafts: protocol::HistoryInputDrafts = drafts.into_iter().map(|draft| (draft.target, draft.replacement)).collect();
        let generation = self.time_travel.session.generation;
        Ok(match self.time_travel_run(TimeTravelStoreCommand::StartReplay { drafts, from: from.clone() }).await? {
            TimeTravelStoreOutput::ReplayStarted(true) => Vec::new(),
            _ => self.time_travel.session.apply(TimeTravelEvent::ReplayFaulted { generation, code: TIME_TRAVEL_REPLAY_FAULTED_CODE.to_string() }).unwrap_or_default(),
        })
    }

    /// 🌿️ Commits the finished replay (`commit_finished_replay`: no second replay): the session finalizes, or a stale
    /// base or a blocking report faults the finalize, which replays again on the current base. The history row of the
    /// commit is its `Supersede` transition, backfilled like every remote or reloaded one.
    async fn commit_time_travel(&mut self, finalization: store::HistoryFinalization, meta: Option<&ActionMeta>) -> Result<Vec<TimeTravelEffect>, Fault> {
        let generation = self.time_travel.session.generation;
        let fault = |code: &str| TimeTravelEvent::FinalizeFaulted { generation, code: code.to_string() };
        let drafts = self.time_travel.accepted_drafts();
        let committed = match self.time_travel_run(TimeTravelStoreCommand::Commit { drafts, finalization, actor: meta.map(|meta| meta.actor.clone()) }).await? {
            TimeTravelStoreOutput::Committed(committed) => committed,
            _ => TimeTravelCommit::Failed,
        };
        let (event, authored) = match committed {
            TimeTravelCommit::Finalized { authored } => {
                self.cache = None;
                (TimeTravelEvent::Finalized { generation }, Some(authored))
            }
            TimeTravelCommit::Stale => (fault(TimeTravelRefusal::Stale.code()), None),
            TimeTravelCommit::Blocked => (fault(TimeTravelRefusal::Blocked.code()), None),
            TimeTravelCommit::Failed => (fault(TIME_TRAVEL_COMMIT_FAILED_CODE), None),
        };
        let effects = self.time_travel.session.apply(event).unwrap_or_default();
        self.note_time_travel_changed(true, true);
        if let Some(authored) = authored {
            self.publish_time_travel_member(authored).await?;
        }
        if let Some(meta) = meta.filter(|_| self.time_travel.session.stage == TimeTravelStage::Inactive) {
            self.revalidate_interaction_on_document_change(meta).await?;
        }
        Ok(effects)
    }

    /// 🌿️ After a finalize on a composed member (design §12), once the session already finalized (a failure here never
    /// strands it): the history logs the edit under its glossary label, the parent re-derives from the replayed child — its
    /// immutable children root republishes that member, whose identity stays the live child's — and every transition the
    /// finalize `authored` on the member (an alternative's commit and `Branch` too, never only its last `Supersede`) crosses
    /// the parent's backbone on the member's lane. A finalize on the document's own store needs none of this: its store
    /// announces its own transitions, backfilled like every remote one.
    async fn publish_time_travel_member(&mut self, authored: Vec<u8>) -> Result<(), Fault> {
        let Some(member) = self.time_travel.member.as_ref() else { return Ok(()) };
        let key = member.key.clone();
        let label = TimeTravelLabel::MemberEdited.localized(LocalizedLabel::native);
        self.push_log_entry(CommandLogAppend { action_id: HISTORY_EDIT_COMMIT_ACTION_ID, label, kind: ActionKind::History, edit_id: None, transition_id: None, timestamp: None, inverse: None });
        let generation = self.admit_child_content_publication()?;
        self.publish_member_content(generation, key.borrowed()).await?;
        self.send_member_lane(&key, authored).await
    }

    /// ⏳️ Whether this instance refuses to open a history edit now (`timeTravel.busy`): a mutating tool run, an agent
    /// transaction or an open tool transaction holds it, the undo or redo of a finalize is being authored, or its own history
    /// step still replays — what disables every Edit row action with its reason (audit W2A-9).
    pub(crate) fn time_travel_busy(&self) -> bool {
        self.tool_runs.holds_mutating_run() || self.pending_transaction.is_some() || self.store.open_transaction().is_some() || self.time_travel.authoring.is_some() || self.store.local_step_pending()
    }

    /// ✏️ Opens (or retargets) the session on `mutationId` of the store `store` names (`<slot>/<childId>`, a composed
    /// member, design §12; absent: the document's own): refused while a mutating tool run or an agent transaction holds
    /// this instance, for a store the instance does not compose, for a session open on another store, for an unknown
    /// operation, and for one whose inputs cannot be edited. Opening a session delivers [`HostEvent::TimeTravelFrozen`] to
    /// every open window, so an open gesture there ends first. With `withdraw` (a history row's Withdraw, design §22.1)
    /// the session opens on a withdrawn draft and needs no editable inputs — refused only where the store's supersede law
    /// lets no withdrawal through; on the mutation already being edited it is the editor's own Withdraw.
    async fn begin_time_travel(&mut self, args: Option<&DslValue>, withdraw: bool, meta: &ActionMeta, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        let Some(mutation) = time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownMutation)) };
        let store = time_travel_arg_text(args, HISTORY_EDIT_ARG_STORE).filter(|store| !store.is_empty());
        let session = &self.time_travel.session;
        if withdraw && session.stage == TimeTravelStage::Editing && self.time_travel.member_store().as_deref() == store && session.pending.as_ref().is_some_and(|pending| pending.target.mutation.0 == mutation) {
            let generation = time_travel_arg_generation(args).unwrap_or(session.generation);
            return self.apply_time_travel_event(TimeTravelEvent::Withdraw { generation }, Some(meta), effects).await;
        }
        if self.time_travel_busy() {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy));
        }
        if self.time_travel.is_active() && self.time_travel.member_store().as_deref() != store {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy));
        }
        let opening = !self.time_travel.is_active();
        if opening {
            let member = match store {
                None => None,
                Some(store) => {
                    let Some((path, key, entry)) = MemberPath::parse(store).and_then(|path| self.children.resolve(&path).map(|(key, entry)| (path, key, entry))) else {
                        return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownMutation));
                    };
                    Some(TimeTravelMemberSubject { key, path, dialect: entry.reference.dialect.clone(), owners: None, children: None })
                }
            };
            self.time_travel.member = member;
            match self.time_travel_base() {
                Some(base) if base != self.time_travel.session.base => {
                    let _ = self.time_travel.session.apply(TimeTravelEvent::BaseMoved { base, positions: Vec::new() });
                }
                _ => {}
            }
        }
        let target = MutationId(mutation.to_string());
        let current = if withdraw { Some(protocol::InputReplacement::Withdrawn) } else { self.time_travel.session.accepted_draft(&target).map(|draft| draft.replacement.clone()) };
        let opened = match self.time_travel_run(TimeTravelStoreCommand::Open { target: target.clone(), current, withdraw }).await? {
            TimeTravelStoreOutput::Opened(opened) => opened,
            _ => Err(TimeTravelActionRefusal::UnknownMutation),
        };
        let (editor, original) = match opened {
            Ok(opened) => opened,
            Err(refusal) => {
                if opening {
                    self.settle_time_travel_owners()?;
                }
                return Ok(TimeTravelActionOutcome::Rejected(refusal));
            }
        };
        let target = TimeTravelTarget { mutation: target, position: editor.position };
        let event = if withdraw { TimeTravelEvent::BeginWithdrawn { target, original } } else { TimeTravelEvent::Begin { target, original } };
        match self.time_travel.session.apply(event) {
            Ok(session_effects) => {
                self.time_travel_run(TimeTravelStoreCommand::Adopt(true)).await?;
                self.time_travel.replace_editor(Some(editor));
                self.note_time_travel_changed(true, true);
                self.perform_time_travel_effects(session_effects, Some(meta), effects).await?;
                if opening {
                    self.deliver_host_event_to_every_window(|window_id| HostEvent::TimeTravelFrozen { window_id }, meta).await?;
                }
                Ok(TimeTravelActionOutcome::Applied(self.time_travel.session.stage))
            }
            Err(refusal) => {
                self.time_travel_run(TimeTravelStoreCommand::Adopt(false)).await?;
                if opening {
                    self.settle_time_travel_owners()?;
                }
                self.time_travel.ui_dirty = true;
                Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(refusal)))
            }
        }
    }

    /// 🎚️ Drafts one input: the host value coerced to the shape of the input `path` addresses in the variant the draft is
    /// in, set at `path` in the draft payload (a union's selector switches the variant: [`time_travel_switch_variant`]) —
    /// or, with a list `edit`, one item inserted at `path` (`<list>/<index>` or `<list>/-`; the host value coerced to the
    /// item, else the item's [`time_travel_default_value`]) or removed there — validated against the leaf payload schema
    /// (with its root discriminators spliced back; `minItems`/`maxItems` included), rebuilt into the same kind and
    /// canonically encoded. A refused value keeps the draft and names the reason on the editor (on the list's row for a
    /// list edit). Fails closed: a payload schema that compiles no validator or describes no inputs admits no draft
    /// (`timeTravel.schema-unavailable`), never an unvalidated payload.
    #[allow(clippy::too_many_arguments, reason = "the verb's path, value and list edit beside the session plumbing every draft verb carries")]
    async fn draft_time_travel_input(&mut self, path: Option<&str>, value: Option<DslValue>, edit: Option<&str>, generation: u32, meta: &ActionMeta, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        if generation != self.time_travel.session.generation {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Stale)));
        }
        let Some(editor) = self.time_travel.editor.as_mut().filter(|_| self.time_travel.session.stage == TimeTravelStage::Editing) else {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal)));
        };
        let Some(schema) = editor.schema else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::NotEditable)) };
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
        let Some(path) = path else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
        let refuse = |editor: &mut TimeTravelEditor, at: &str, reason: String| {
            editor.refused = Some((at.to_string(), reason));
            TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::InvalidInput)
        };
        let (at, candidate) = match edit {
            Some(edit) => {
                let Some((list, _)) = path.rsplit_once('/') else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
                let Some((list_input, false)) = time_travel_input_at(&editor.inputs, &editor.value, list) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
                let (item_input, element) = match &list_input.schema {
                    ArgSchema::Array { items, .. } => (ActionArgDef { schema: items.as_ref().clone(), required: true, nullable: false, default: None, ..list_input.clone() }, false),
                    ArgSchema::Reference { many: true, .. } => (list_input.clone(), true),
                    _ => return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)),
                };
                let mut candidate = editor.value.clone();
                let applied = match edit {
                    HISTORY_EDIT_INPUT_INSERT => {
                        let item = match &value {
                            Some(value) => time_travel_coerce(&item_input, element, value),
                            None => (!element).then(|| time_travel_default_value(&item_input.schema)),
                        };
                        let Some(item) = item else {
                            let outcome = refuse(editor, list, format!("{list} does not take this item"));
                            self.time_travel.ui_dirty = true;
                            return Ok(outcome);
                        };
                        time_travel_pointer_insert(&mut candidate, path, item)
                    }
                    HISTORY_EDIT_INPUT_REMOVE => time_travel_pointer_remove(&mut candidate, path),
                    _ => return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::InvalidInput)),
                };
                if !applied {
                    return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput));
                }
                (list, candidate)
            }
            None => {
                let Some((input, element)) = time_travel_input_at(&editor.inputs, &editor.value, path) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
                let Some(coerced) = value.as_ref().and_then(|value| time_travel_coerce(&input, element, value)) else {
                    let outcome = refuse(editor, path, format!("{path} does not take this value"));
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
                (path, candidate)
            }
        };
        let verdict = mutation_input_instance(schema, &registered_input_schema_document, &candidate)
            .map_err(|error| error.to_string())
            .and_then(|instance| editor.validator.as_ref().map_err(String::clone).and_then(|validator| validator.validate_json(&time_travel_json(&instance)).map(|_| ()).map_err(|error| error.to_string())));
        let rebuilt = match verdict {
            Ok(()) => match self.time_travel_run(TimeTravelStoreCommand::Rebuild(candidate.clone())).await? {
                TimeTravelStoreOutput::Rebuilt(rebuilt) => rebuilt,
                _ => Err("a history-edit rebuild answered another command".to_string()),
            },
            Err(reason) => Err(reason),
        };
        let replacement = match rebuilt {
            Ok(replacement) => replacement,
            Err(reason) => {
                let Some(editor) = self.time_travel.editor.as_mut() else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::EditorClosed)) };
                let outcome = refuse(editor, at, reason);
                self.time_travel.ui_dirty = true;
                return Ok(outcome);
            }
        };
        let outcome = self.apply_time_travel_event(TimeTravelEvent::Draft { generation, replacement }, Some(meta), effects).await?;
        if matches!(outcome, TimeTravelActionOutcome::Applied(_)) {
            if let Some(editor) = self.time_travel.editor.as_mut() {
                editor.value = candidate;
                editor.refused = None;
            }
        }
        Ok(outcome)
    }

    /// 🎯️ Drafts the reference input at `path` from the current selection of its declared domain, every row mapped to the
    /// entity id it names (`ArtifactApp::selection_reference_id`) ([`time_travel_selection_value`]), then validates and
    /// rebuilds it like any other input.
    async fn draft_time_travel_selection(&mut self, path: Option<&str>, generation: u32, meta: &ActionMeta, effects: &mut Vec<Effect>) -> Result<TimeTravelActionOutcome, Fault> {
        let Some(editor) = self.time_travel.editor.as_ref() else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal))) };
        let Some((input, false)) = path.and_then(|path| time_travel_input_at(&editor.inputs, &editor.value, path)) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
        let ArgSchema::Reference { domain: Some(domain), kinds, .. } = &input.schema else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
        let state = self.interaction_selection_snapshot();
        let selection = state.selection.get(domain).map(|selection| protocol::DomainSelection { ids: selection.ids.iter().filter_map(|row| A::selection_reference_id(kinds, row)).collect(), ..selection.clone() });
        match time_travel_selection_value(&input, selection.as_ref()) {
            Ok(value) => self.draft_time_travel_input(path, Some(value), None, generation, meta, effects).await,
            Err(refusal) => Ok(TimeTravelActionOutcome::Rejected(refusal)),
        }
    }

    /// ⌛️ The wall deadline of a driver turn that begins now: [`TIME_TRAVEL_TURN_WALL_US`] on the ledger's turn clock (from
    /// zero when the clock cannot be read, so the turn ends after its first operation).
    fn time_travel_turn_deadline(&self) -> u64 {
        (self.time_travel.turn_clock)().unwrap_or(0).saturating_add(TIME_TRAVEL_TURN_WALL_US)
    }

    /// ⏯️ One bounded driver turn (≤ [`TIME_TRAVEL_TURN_WALL_US`], design §20.14): owed scope, retirement, base watch, one
    /// replay slice, one authoring slice and one deferred-reprojection slice, all against ONE wall deadline taken when the
    /// turn begins on the ledger's turn clock (audit W2A-3) — a slow operation ends the turn, never an operation count; a
    /// session change prepares the history patch that rides the next unsolicited UI progress frame.
    pub(crate) async fn drive_time_travel_turn(&mut self) -> Result<(), Fault> {
        let deadline_us = self.time_travel_turn_deadline();
        if self.time_travel.reprojection_paused && self.store.reprojection_progress().is_none() {
            self.time_travel.reprojection_paused = false;
        }
        self.prepare_time_travel_patch().await?;
        self.flush_time_travel_ui_dirty();
        let retirement_grant=history_planning_retirement_grant(self.time_travel_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(|error|Fault::from(error.into_message()))?);
        if let Some(step) = self.time_travel_retire_step(retirement_grant)? {
            if step.progress()!=RetainedCloneProgress::default() {
                #[cfg(debug_assertions)]
                if self.store.reprojection_progress().is_some() {
                    static TURNS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                    let turns = TURNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed).saturating_add(1);
                    if turns >= 1024 && turns.is_power_of_two() {
                        eprintln!("[DEBUG] reprojection driver cleanup turn={turns} step={step:?} progress={:?} documentRetirements={} documentDiscarded={}", self.store.reprojection_progress(), self.time_travel.document.retirements.len(), self.time_travel.document.discarded.len());
                    }
                }
                return Ok(());
            }
        }
        let moved = self.watch_time_travel_base().await?;
        if !moved && self.time_travel.session.stage == TimeTravelStage::Editing && self.time_travel.preview_progress().is_some() {
            if let TimeTravelStoreOutput::Previewed(step) = self.time_travel_run(TimeTravelStoreCommand::StepPreview { deadline_us, clock: self.time_travel.turn_clock }).await? {
                self.adopt_time_travel_preview_step(step).await?;
            }
        }
        if !moved && self.time_travel.session.stage == TimeTravelStage::Replaying && self.time_travel.replaying() {
            self.step_time_travel_replay(deadline_us).await?;
        }
        if self.time_travel.authoring.is_some() && self.step_supersede_authoring(deadline_us).await? != SupersedeAuthored::Pending {
            self.note_time_travel_changed(true, true);
        }
        if self.reprojection_drives() {
            self.step_reprojection_turn(deadline_us).await?;
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

    /// ⏭️ Steps the replay of the store the session edits until the turn deadline: progress ticks refresh the history body
    /// at most every [`TIME_TRAVEL_PROGRESS_REFRESH_MS`]; completion hands the report to the session and swaps the
    /// preview to the replayed head; a store refusal faults the replay.
    async fn step_time_travel_replay(&mut self, deadline_us: u64) -> Result<(), Fault> {
        let generation = self.time_travel.session.generation;
        let stepped = match self.time_travel_run(TimeTravelStoreCommand::StepReplay { deadline_us, clock: self.time_travel.turn_clock }).await? {
            TimeTravelStoreOutput::Stepped(stepped) => stepped,
            _ => TimeTravelReplayStep::Faulted,
        };
        let event = match stepped {
            TimeTravelReplayStep::Pending { done, total } => {
                let _ = self.time_travel.session.apply(TimeTravelEvent::ReplayProgressed { generation, done, total });
                let now_ms = semio_framework_job::default_now_ms().unwrap_or(0);
                let advanced = u64::from(done.saturating_sub(self.time_travel.refreshed_done)) * 1_000 >= u64::from(total) * TIME_TRAVEL_PROGRESS_REFRESH_PERMILLE;
                if advanced || now_ms.saturating_sub(self.time_travel.refreshed_ms) >= TIME_TRAVEL_PROGRESS_REFRESH_MS {
                    self.time_travel.refreshed_ms = now_ms;
                    self.time_travel.refreshed_done = done;
                    self.note_time_travel_changed(false, false);
                }
                return Ok(());
            }
            TimeTravelReplayStep::Completed(report) => TimeTravelEvent::ReplayCompleted { generation, report },
            TimeTravelReplayStep::Faulted => TimeTravelEvent::ReplayFaulted { generation, code: TIME_TRAVEL_REPLAY_FAULTED_CODE.to_string() },
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
        self.records[..before]
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, record)| record.scope.is_none() || record.scope.as_deref() == scope)
            .find_map(|(index, record)| record.inputs.iter().find(|input| input.target == *target).map(|input| (index, &input.replacement)))
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
        &self.store.local_actor_id().0
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
        self.command_prune_generation = self.command_prune_generation.checked_add(1).expect("mounted command visibility generation remains representable");
        self.history_backfill = None;
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
            let shell = entry.inverse.is_some() && entry.edit_id.is_none() && !self.shell_undone.contains(&entry.seq);
            let document = entry.edit_id.as_deref().is_some_and(|id| applied.contains(id) && own_edit(id));
            let child = entry.child_edit_ids.iter().any(|id| child_applied_tails.contains(id));
            if shell || document || child {
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

    /// ✍️ Authors `inputs` within `scope` (unscoped: every line) resumably (design §16.6) — the restore of an undo or the
    /// re-authoring of a redo: a Report replay of the inputs over the applied history, stepped within this turn's budget and
    /// then per reactor turn, finalized by `commit_finished_replay` without replaying again. Answers whether it landed, is
    /// still replaying, was refused (an input no longer applied, or a report that blocks finalizing; nothing changes) or
    /// waits for an authoring already running.
    async fn author_supersede(&mut self, scope: Option<String>, inputs: Vec<protocol::SupersededInput>) -> Result<SupersedeAuthored, Fault> {
        if self.time_travel.authoring.is_some() {
            return Ok(SupersedeAuthored::Busy);
        }
        let drafts: protocol::HistoryInputDrafts = inputs.into_iter().map(|input| (input.target, input.replacement)).collect();
        let finalization = match scope {
            Some(alternative_id) => store::HistoryFinalization::Scope { alternative_id },
            None => store::HistoryFinalization::Overwrite,
        };
        match self.store.begin_derived_report_replay(drafts, None) {
            Ok(replay) => self.time_travel.authoring = Some(SupersedeAuthoring { replay, finalization }),
            Err(vcs::VcsError::ValidationFailed(_) | vcs::VcsError::Rejected { .. }) => return Ok(SupersedeAuthored::Refused),
            Err(error) => return Err(error.into_fault()),
        }
        self.step_supersede_authoring(self.time_travel_turn_deadline()).await
    }

    /// ⏭️ One turn of the undo or redo of a finalize being authored: steps its replay until the turn deadline, then commits
    /// the finished replay in its scope; a store that moved meanwhile replays the same inputs again from the new base.
    async fn step_supersede_authoring(&mut self, deadline_us: u64) -> Result<SupersedeAuthored, Fault> {
        let stepped = {
            let VcsArtifactApp { store, time_travel, .. } = self;
            let clock = time_travel.turn_clock;
            let Some(authoring) = time_travel.authoring.as_mut() else { return Ok(SupersedeAuthored::Refused) };
            let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
            let grant=history_planning_retirement_grant(authoring.replay.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(|error|Fault::from(error.into_message()))?);
            store.step_derived_report_replay(&mut authoring.replay, grant, &mut deadline)
        };
        match stepped {
            Ok(store::ReplayStep::Pending(_)) => return Ok(SupersedeAuthored::Pending),
            Ok(store::ReplayStep::Finished(_)) => {}
            Err(_) => {
                if let Some(authoring) = self.time_travel.authoring.take() {
                    let SupersedeAuthoring {replay,finalization}=authoring;let mut replay=Some(replay);
                    let grant=history_retirement_frame_grant(ArtifactStore::<A::Snapshot,A::Mutation>::history_read_retirement_birth_bytes());
                    match self.store.retire_derived_report_replay(&mut replay,grant) {
                        Ok(Some((retirement,progress)))=>{assert!(progress.fits(grant));self.time_travel.document.retirements.push_back(retirement);}
                        Ok(None)=>unreachable!("retained supersede authoring is present"),
                        Err(error)=>{self.time_travel.authoring=Some(SupersedeAuthoring {replay:replay.take().unwrap(),finalization});return Err(Fault::from(error.into_message()));}
                    }
                }
                return Ok(SupersedeAuthored::Refused);
            }
        }
        let SupersedeAuthoring { replay, finalization } = self.time_travel.authoring.take().expect("a finished authoring replay was held");
        let Ok((result, head)) = self.store.finish_derived_report_replay(&mut Some(replay)) else { return Ok(SupersedeAuthored::Refused) };
        if let Some(head) = head {
            self.time_travel.document.retire(&self.store, Some(head))?;
        }
        let drafts = result.drafts().clone();
        match self.store.commit_finished_replay(result, finalization.clone()).await {
            Ok(_) => {
                self.cache = None;
                Ok(SupersedeAuthored::Landed)
            }
            Err(vcs::VcsError::Stale { .. }) => match self.store.begin_derived_report_replay(drafts, None) {
                Ok(replay) => {
                    self.time_travel.authoring = Some(SupersedeAuthoring { replay, finalization });
                    Ok(SupersedeAuthored::Pending)
                }
                Err(_) => Ok(SupersedeAuthored::Refused),
            },
            Err(vcs::VcsError::ValidationFailed(_) | vcs::VcsError::Rejected { .. } | vcs::VcsError::UnknownAlternative(_)) => Ok(SupersedeAuthored::Refused),
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
        let authored = self.author_supersede(scope, inputs).await?;
        Ok(Some(self.supersede_authored_result(action, meta, authored).await))
    }

    /// 🧾️ The result of a verb that authored the undo or redo of a finalize: a landed one changed the history (every body
    /// refreshes), a pending one lands in a later reactor turn, an authoring already running answers `timeTravel.busy`.
    async fn supersede_authored_result(&mut self, action: &str, meta: &ActionMeta, authored: SupersedeAuthored) -> InvocationResult {
        let (events, scope) = match authored {
            SupersedeAuthored::Landed => (vec![history_changed_event().await], UiDirtyScope::Full),
            SupersedeAuthored::Pending | SupersedeAuthored::Refused | SupersedeAuthored::Busy => (Vec::new(), UiDirtyScope::None),
        };
        let mut result = Self::empty_result(action, meta, Vec::new(), events, scope).await;
        if authored == SupersedeAuthored::Busy {
            result.output = DslValue::Object(vec![("rejected".into(), DslValue::String(TIME_TRAVEL_BUSY_CODE.into()))]);
        }
        result
    }

    /// ⏪️ `revertToCommand` on a history-edit row: takes back the history edit the row acts for, when it is this replica's
    /// and still in effect.
    pub(crate) async fn revert_history_edit_row(&mut self, transition_id: &str, meta: &ActionMeta) -> Result<InvocationResult, Fault> {
        let local = self.supersede_author().to_string();
        let applied_entries = self.supersedes.applied_entries(self.store.supersessions().iter());
        let target = self.supersedes.record(transition_id).filter(|(_, record)| record.role != SupersedeRole::Undo && record.actor == local).map(|(_, record)| record.entry).filter(|entry| self.supersedes.undoable(&local, *entry, &applied_entries));
        let authored = match target.map(|entry| self.supersedes.restore(entry)).filter(|(_, inputs)| !inputs.is_empty()) {
            Some((scope, inputs)) => self.author_supersede(scope, inputs).await?,
            None => SupersedeAuthored::Refused,
        };
        Ok(self.supersede_authored_result(REVERT_TO_COMMAND_ACTION_ID, meta, authored).await)
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
    Restore,
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
    AddItem,
    RemoveItem,
    ItemCount,
    MaxItems,
    MaxItem,
    MinItems,
    MinItem,
    Cleared,
    TooLong,
    Alternatives,
    Current,
    BranchedBy,
    EditedHistory,
    Switch,
    Trunk,
    MutationUnavailable,
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
                Self::FilterWithoutMutations => "Without Mutations",
                Self::FilterOnlyMutations => "Only Mutations",
                Self::Backwards => "Backwards",
                Self::TimeTravel => "History editing",
                Self::Edit => "Edit",
                Self::Accept => "Accept",
                Self::Discard => "Discard",
                Self::Withdraw => "Withdraw",
                Self::Restore => "Restore",
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
                Self::AddItem => "Add item",
                Self::RemoveItem => "Remove item",
                Self::ItemCount => "Items: {count}",
                Self::MaxItems => "Maximum {count} items",
                Self::MaxItem => "Maximum 1 item",
                Self::MinItems => "Minimum {count} items",
                Self::MinItem => "Minimum 1 item",
                Self::Cleared => "Cleared (no value)",
                Self::TooLong => "Too long to edit here: shown shortened",
                Self::Alternatives => "Alternatives",
                Self::Current => "Current",
                Self::BranchedBy => "Branched by {author}, {time}",
                Self::EditedHistory => "Edited history",
                Self::Switch => "Switch",
                Self::Trunk => "Main line",
                Self::MutationUnavailable => "Mutation not available",
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
                Self::FilterWithoutMutations => "Ohne Mutationen",
                Self::FilterOnlyMutations => "Nur Mutationen",
                Self::Backwards => "Zurück bis hier",
                Self::TimeTravel => "Verlaufsbearbeitung",
                Self::Edit => "Bearbeiten",
                Self::Accept => "Übernehmen",
                Self::Discard => "Verwerfen",
                Self::Withdraw => "Zurückziehen",
                Self::Restore => "Wiederherstellen",
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
                Self::AddItem => "Element hinzufügen",
                Self::RemoveItem => "Element entfernen",
                Self::ItemCount => "Elemente: {count}",
                Self::MaxItems => "Höchstens {count} Einträge",
                Self::MaxItem => "Höchstens 1 Eintrag",
                Self::MinItems => "Mindestens {count} Einträge",
                Self::MinItem => "Mindestens 1 Eintrag",
                Self::Cleared => "Geleert (kein Wert)",
                Self::TooLong => "Zu lang, um hier bearbeitet zu werden: gekürzt angezeigt",
                Self::Alternatives => "Alternativen",
                Self::Current => "Aktuell",
                Self::BranchedBy => "Abgezweigt von {author}, {time}",
                Self::EditedHistory => "Bearbeiteter Verlauf",
                Self::Switch => "Wechseln",
                Self::Trunk => "Hauptlinie",
                Self::MutationUnavailable => "Mutation nicht verfügbar",
            },
        }
    }
}

/// 🚦️ A severity's word, EN and DE; a reader hears it, colour is never its only carrier.
pub(crate) fn history_severity_text(level: semio_framework_diagnostic::Severity, locale: Locale) -> &'static str {
    match (locale, level) {
        (Locale::En, semio_framework_diagnostic::Severity::Info) => "Info",
        (Locale::En, semio_framework_diagnostic::Severity::Warning) => "Warning",
        (Locale::En, semio_framework_diagnostic::Severity::Error) => "Error",
        (Locale::En, semio_framework_diagnostic::Severity::Fatal) => "Fatal",
        (Locale::De, semio_framework_diagnostic::Severity::Info) => "Info",
        (Locale::De, semio_framework_diagnostic::Severity::Warning) => "Warnung",
        (Locale::De, semio_framework_diagnostic::Severity::Error) => "Fehler",
        (Locale::De, semio_framework_diagnostic::Severity::Fatal) => "Kritisch",
    }
}

/// 🏷️ An outcome code's words, EN and DE, never the code itself: a frozen `mutation.*` outcome code's short words, else the
/// framework's placeholder-free notice of the code (`kernel::history_notice`, `kernel::framework_fault_notice`), else the
/// generic "could not apply" words.
pub(crate) fn history_code_text(code: &str, locale: Locale) -> &'static str {
    let known = match code {
        "mutation.target-missing" => Some(("Target missing", "Ziel fehlt")),
        "mutation.target-referenced" => Some(("Target still referenced", "Ziel wird noch referenziert")),
        "mutation.target-mismatch" => Some(("Inconsistent with the target", "Widerspricht dem Ziel")),
        "mutation.no-op" => Some(("No change", "Keine Änderung")),
        "mutation.partial" => Some(("Partially applied", "Teilweise angewendet")),
        "mutation.clamped" => Some(("Clamped", "Begrenzt")),
        "mutation.precondition-drifted" => Some(("Precondition drifted", "Vorbedingung nicht mehr erfüllt")),
        "mutation.duplicate-id" => Some(("Duplicate id", "ID bereits vergeben")),
        "mutation.invariant" => Some(("Invalid state", "Ungültiger Zustand")),
        "mutation.inverse-refused" => Some(("Cannot be reversed", "Nicht umkehrbar")),
        "mutation.cascade" => Some(("Cascaded", "Folgeänderung")),
        code if code.starts_with(protocol::APPLY_OUTCOME_CODE_PREFIX) => Some(("Could not apply", "Nicht anwendbar")),
        _ => None,
    };
    let framework = || semio_framework::kernel::history_notice(code).or_else(|| semio_framework::kernel::framework_fault_notice(code)).filter(|(en, de)| !en.contains('{') && !de.contains('{'));
    let (en, de) = known.or_else(framework).unwrap_or(("Could not apply", "Nicht anwendbar"));
    match locale {
        Locale::En => en,
        Locale::De => de,
    }
}

/// 🎨️ The tone and icon of a severity (none: applied cleanly).
pub(crate) fn history_severity_tone(level: Option<semio_framework_diagnostic::Severity>) -> (Tone, &'static str) {
    match level {
        None => (Tone::Success, "check"),
        Some(semio_framework_diagnostic::Severity::Info) => (Tone::Info, "info"),
        Some(semio_framework_diagnostic::Severity::Warning) => (Tone::Warning, "triangle-alert"),
        Some(semio_framework_diagnostic::Severity::Error) => (Tone::Danger, "alert-circle"),
        Some(semio_framework_diagnostic::Severity::Fatal) => (Tone::Danger, "x"),
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
    let introduced = TimeTravelLabel::OutcomeIntroduced.localized(|en, de| match locale {
        Locale::En => en,
        Locale::De => de,
    });
    if entry.introduced {
        parts.push(introduced);
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

/// 🧽️ The `{generation, path, value}` argument map of a button that drafts one fixed value: a nullable input's Clear
/// (`null`), one option of a long choice.
fn time_travel_value_args(generation: u32, path: &str, value: UiValue) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("time-travel-panel.value-args"))?;
    args.push(HISTORY_EDIT_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).map_err(|_| ui_assembly_error("time-travel-panel.value-args"))?;
    args.push(HISTORY_EDIT_ARG_PATH.to_string(), UiValue::Text(UiText::clipped(path))).map_err(|_| ui_assembly_error("time-travel-panel.value-args"))?;
    args.push(HISTORY_EDIT_ARG_VALUE.to_string(), value).map_err(|_| ui_assembly_error("time-travel-panel.value-args"))?;
    Ok(UiValue::Map(args.finish()))
}

/// 🧮️ Why a list edit is disabled, localized: "Maximum 8 items" (`maxItems` reached), "Minimum 1 item" (`minItems` left).
pub(crate) fn time_travel_item_limit_text(limit: u32, max: bool, locale: Locale) -> String {
    let text = match (max, limit == 1) {
        (true, true) => HistoryPanelText::MaxItem,
        (true, false) => HistoryPanelText::MaxItems,
        (false, true) => HistoryPanelText::MinItem,
        (false, false) => HistoryPanelText::MinItems,
    };
    text.text(locale).replace("{count}", &limit.to_string())
}

/// ✂️ The `{generation, path, edit}` argument map of a list edit button: `insert` (the item's default at `path`, `…/-`
/// appends) or `remove` (the item at `path`).
fn time_travel_list_edit_args(generation: u32, path: &str, edit: &str) -> UiAssemblyResult<UiValue> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("time-travel-panel.list-args"))?;
    args.push(HISTORY_EDIT_ARG_GENERATION.to_string(), UiValue::Number(f64::from(generation))).map_err(|_| ui_assembly_error("time-travel-panel.list-args"))?;
    args.push(HISTORY_EDIT_ARG_PATH.to_string(), UiValue::Text(UiText::clipped(path))).map_err(|_| ui_assembly_error("time-travel-panel.list-args"))?;
    args.push(HISTORY_EDIT_ARG_EDIT.to_string(), UiValue::Text(UiText::clipped(edit))).map_err(|_| ui_assembly_error("time-travel-panel.list-args"))?;
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
    let stage_row = ui::tree_item(Label(UiText::clipped(&stage_line)))
        .icon(ui_text("clock", "time-travel-panel.status-icon")?)
        .tone(tone)
        .try_id(format!("{scope}.status"))
        .map_err(|_| error("time-travel-panel.status-id"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.status"))?;
    rows.try_push(stage_row).map_err(|_| error("time-travel-panel.rows"))?;
    if let (Some(done), Some(total)) = (status.done, status.total) {
        let label = if status.stage == HistoryTimeTravelStage::Editing { TimeTravelLabel::PreparationProgressValueText } else { TimeTravelLabel::ReplayProgressValueText };
        let mut value_text = label.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).replace("{done}", &done.to_string()).replace("{total}", &total.to_string());
        if let Some(processed) = status.processed {
            value_text.push_str(" · ");
            value_text.push_str(&TimeTravelLabel::Processed.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).replace("{processed}", &processed.to_string()));
        }
        let progress_row = ui::tree_item(Label(UiText::clipped(&value_text)))
            .icon(ui_text("loader-2", "time-travel-panel.progress-icon")?)
            .try_id(format!("{scope}.progress"))
            .map_err(|_| error("time-travel-panel.progress-id"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.progress"))?;
        rows.try_push(progress_row).map_err(|_| error("time-travel-panel.rows"))?;
    }
    if let Some(fault) = status.fault.as_deref() {
        let label = TimeTravelLabel::for_code(fault).unwrap_or(TimeTravelLabel::ReplayFaulted).localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
        let node = ui::tree_item(Label(UiText::clipped(&label)))
            .icon(ui_text("triangle-alert", "time-travel-panel.fault-icon")?)
            .tone(Tone::Danger)
            .try_id(format!("{scope}.fault"))
            .map_err(|_| error("time-travel-panel.fault-id"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.fault"))?;
        rows.try_push(node).map_err(|_| error("time-travel-panel.rows"))?;
    }
    let generation = status.generation;
    if panel.stage == TimeTravelStage::Replaying {
        rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.cancelReplay"), HistoryPanelText::CancelReplay.text(locale), "square", HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, Some(time_travel_generation_args(generation)?), true)?)
            .map_err(|_| error("time-travel-panel.rows"))?;
    }
    if let Some(problem) = panel.next_problem.as_deref() {
        let mut args = UiMapBuilder::try_new().ok_or_else(|| error("time-travel-panel.problem-args"))?;
        args.push(HISTORY_EDIT_ARG_MUTATION_ID.to_string(), UiValue::Text(UiText::clipped(problem))).map_err(|_| error("time-travel-panel.problem-args"))?;
        if let Some(store) = panel.store.as_deref() {
            args.push(HISTORY_EDIT_ARG_STORE.to_string(), UiValue::Text(UiText::clipped(store))).map_err(|_| error("time-travel-panel.problem-args"))?;
        }
        rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.nextProblem"), HistoryPanelText::NextProblem.text(locale), "arrow-right", HISTORY_EDIT_BEGIN_ACTION_ID, Some(UiValue::Map(args.finish())), true)?)
            .map_err(|_| error("time-travel-panel.rows"))?;
    }
    if matches!(panel.stage, TimeTravelStage::Reviewing | TimeTravelStage::Choosing) {
        let refused = panel.finalize_refusal.map(|refusal| refusal.label().localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string());
        let label = HistoryPanelText::Finalize.text(locale);
        let mut finalize = button(ui_label(label, "time-travel-panel.finalize-label")?)
            .icon(ui_text("list-checks", "time-travel-panel.finalize-icon")?)
            .disabled(refused.is_some())
            .try_id(format!("{scope}.finalize"))
            .map_err(|_| error("time-travel-panel.finalize-id"))?;
        if let Some(reason) = refused.as_deref() {
            finalize = finalize.try_describe(reason).map_err(|_| error("time-travel-panel.finalize-description"))?;
        }
        let action = ActionId::try_v1(controller_id, HISTORY_EDIT_FINALIZE_ACTION_ID).ok_or_else(|| error("time-travel-panel.action-id"))?;
        let finalize = finalize.try_on_with(Trigger::Activate, action, time_travel_generation_args(generation)?).map_err(|_| error("time-travel-panel.finalize-binding"))?.try_build().map_err(|_| error("time-travel-panel.finalize"))?;
        rows.try_push(time_travel_control_row(&format!("{scope}.finalize.row"), label, "list-checks", finalize, refused.is_none(), None, refused.as_deref())?).map_err(|_| error("time-travel-panel.rows"))?;
    }
    if panel.stage == TimeTravelStage::Reviewing {
        let rerun = TimeTravelLabel::ActionRerun.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string();
        rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.rerun"), &rerun, "skip-forward", HISTORY_EDIT_RERUN_ACTION_ID, Some(time_travel_generation_args(generation)?), panel.rerun_refusal.is_none())?)
            .map_err(|_| error("time-travel-panel.rows"))?;
    }
    rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.exit"), HistoryPanelText::Exit.text(locale), "rotate-ccw", HISTORY_EDIT_EXIT_ACTION_ID, None, panel.stage != TimeTravelStage::Finalizing)?)
        .map_err(|_| error("time-travel-panel.rows"))?;
    tree_section(ui_label(HistoryPanelText::TimeTravel.text(locale), "time-travel-panel.label")?)
        .default_open(true)
        .try_id(scope)
        .map_err(|_| error("time-travel-panel.id"))?
        .try_children(rows)
        .map_err(|_| error("time-travel-panel.rows"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.build"))
}

/// 📡️ The history change waiting for its replay as the history body's tree section `framework.history.reprojection`
/// (design §16.6, gap N17) — a remote change, this replica's own history step or a whole-document load: one status row whose
/// title and line are the kernel's one copy ([`semio_framework::kernel::history_reprojection_status`], the line both shells
/// announce; a refused adoption reads its named notice, never a raw code — audit W2A-8), its tone and icon by state and kind,
/// then, when `controls`, Replay again while a remote change is paused (its replay was dropped, so nothing counts yet) or
/// Cancel replay while one runs (a local step is dropped with zero trace) — no controls while a session is open, whose own
/// Cancel replay and Replay again address the session, nor for a viewer.
pub(crate) fn time_travel_reprojection_section(remote: &HistoryReprojection, controls: bool, controller_id: &str, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let scope = "framework.history.reprojection";
    let status = semio_framework::kernel::history_reprojection_status(remote, Terminology::Native, locale);
    let tone = match (&status.fault, status.paused) {
        (Some(_), _) if status.total == 0 => Tone::Danger,
        (_, true) => Tone::Warning,
        _ => Tone::Info,
    };
    let icon = match remote.kind {
        HistoryReprojectionKind::Remote => "cloud-download",
        HistoryReprojectionKind::Step => IconName::Undo.as_str(),
        HistoryReprojectionKind::Load => IconName::Download.as_str(),
    };
    let mut rows = BuiltChildren::default();
    let line = ui::tree_item(Label(UiText::clipped(&status.text)))
        .icon(ui_text(icon, "time-travel-panel.remote-icon")?)
        .tone(tone)
        .try_id(format!("{scope}.status"))
        .map_err(|_| error("time-travel-panel.remote-status-id"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.remote-status"))?;
    rows.try_push(line).map_err(|_| error("time-travel-panel.remote-rows"))?;
    let control = match (controls, status.paused, status.total > 0) {
        (true, true, _) => Some(time_travel_button_row(controller_id, &format!("{scope}.rerun"), TimeTravelLabel::ActionRerun.localized(LocalizedLabel::native).resolve(Terminology::Native, locale), "skip-forward", HISTORY_EDIT_RERUN_ACTION_ID, None, true)?),
        (true, false, true) => Some(time_travel_button_row(controller_id, &format!("{scope}.cancelReplay"), HistoryPanelText::CancelReplay.text(locale), "square", HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, None, true)?),
        _ => None,
    };
    if let Some(control) = control {
        rows.try_push(control).map_err(|_| error("time-travel-panel.remote-rows"))?;
    }
    tree_section(ui_label(status.title.as_str(), "time-travel-panel.remote-label")?)
        .default_open(true)
        .try_id(scope)
        .map_err(|_| error("time-travel-panel.remote-id"))?
        .try_children(rows)
        .map_err(|_| error("time-travel-panel.remote-rows"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.remote"))
}

/// ✏️ The draft editor as two tree sections: `framework.history.editor` — the edited mutation with its draft's own
/// outcome, why no input is editable (a mutation without editable inputs reads the framework's own words, a payload
/// schema that describes none its reason), then Accept, Discard and Withdraw — and `framework.history.editor.inputs`,
/// one row per input row of [`time_travel_input_rows`] ([`time_travel_input_row`]). The inputs section is a tree window
/// over every row, so a payload of any size stays reachable: the host streams the slice it scrolls to.
pub(crate) fn time_travel_editor_sections(panel: &TimeTravelPanel, editor: &TimeTravelEditorPanel, windows: &TreeWindows<'_>, controller_id: &str, locale: Locale) -> UiAssemblyResult<[BuiltNode; 2]> {
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
    let head = ui::tree_item(Label(UiText::clipped(&heading)))
        .icon(ui_text(icon, "time-travel-panel.editor-icon")?)
        .tone(tone)
        .try_id(format!("{scope}.target"))
        .map_err(|_| error("time-travel-panel.editor-target-id"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.editor-target"))?;
    rows.try_push(head).map_err(|_| error("time-travel-panel.editor-rows"))?;
    let uneditable = (!editor.editable).then(|| TimeTravelLabel::RefusalNotEditable.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string());
    if let Some(reason) = uneditable.as_deref().or(editor.inputs_refused.as_deref()) {
        let node = ui::tree_item(Label(UiText::clipped(HistoryPanelText::NoInputs.text(locale))))
            .description(UiText::clipped(reason))
            .tone(Tone::Warning)
            .try_id(format!("{scope}.refused"))
            .map_err(|_| error("time-travel-panel.editor-refused-id"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.editor-refused"))?;
        rows.try_push(node).map_err(|_| error("time-travel-panel.editor-rows"))?;
    }
    let accept_reason = (!editor.changed).then(|| TimeTravelLabel::RefusalUnchanged.localized(LocalizedLabel::native).resolve(Terminology::Native, locale).to_string());
    let accept = time_travel_button(controller_id, &format!("{scope}.accept"), HistoryPanelText::Accept.text(locale), "check", HISTORY_EDIT_ACCEPT_ACTION_ID, Some(time_travel_generation_args(generation)?), editor.changed)?;
    rows.try_push(time_travel_control_row(&format!("{scope}.accept.row"), HistoryPanelText::Accept.text(locale), "check", accept, editor.changed, None, accept_reason.as_deref())?)
        .map_err(|_| error("time-travel-panel.editor-rows"))?;
    rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.discard"), HistoryPanelText::Discard.text(locale), "x", HISTORY_EDIT_DISCARD_ACTION_ID, Some(time_travel_generation_args(generation)?), true)?)
        .map_err(|_| error("time-travel-panel.editor-rows"))?;
    rows.try_push(time_travel_button_row(controller_id, &format!("{scope}.withdraw"), HistoryPanelText::Withdraw.text(locale), "eye-off", HISTORY_EDIT_WITHDRAW_ACTION_ID, Some(time_travel_generation_args(generation)?), !editor.withdrawn)?)
        .map_err(|_| error("time-travel-panel.editor-rows"))?;
    let section = tree_section(ui_label(editor.label.resolve(Terminology::Native, locale), "time-travel-panel.editor-label")?)
        .default_open(true)
        .try_id(scope)
        .map_err(|_| error("time-travel-panel.editor-id"))?
        .try_children(rows)
        .map_err(|_| error("time-travel-panel.editor-rows"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.editor-build"))?;
    let inputs = tree_window_indexed_section(windows, "framework.history.editor.inputs", ui_label(HistoryPanelText::Inputs.text(locale), "time-travel-panel.inputs-label")?, true, editor.rows.len(), |index| {
        time_travel_input_row(&editor.rows[index], editor, windows, controller_id, generation, locale)
    })?;
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

/// 🛝️ The slider (or dial) of a number input — its [`semio_framework::ActionArgNumberFacets`]: travel, step, look, axis,
/// stored and shown unit, display factor, precision, detents, and the hard limits that make the travel soft.
fn time_travel_slider(value: f64, facets: &semio_framework::ActionArgNumberFacets) -> SliderBuilder {
    let mut builder = slider(value).min(facets.min.unwrap_or(0.0)).max(facets.max.unwrap_or(0.0)).appearance(facets.appearance).scale(facets.scale).limits(facets.limits.clone());
    if let Some(step) = facets.step {
        builder = builder.step(step);
    }
    if let Some(unit) = &facets.unit {
        builder = builder.unit(UiText::clipped(unit));
    }
    if let Some(display_unit) = &facets.display_unit {
        builder = builder.display_unit(UiText::clipped(display_unit));
    }
    if let Some(factor) = facets.display_factor {
        builder = builder.display_factor(factor);
    }
    if let Some(precision) = facets.precision {
        builder = builder.precision(precision);
    }
    for &snap in &facets.snaps {
        builder = builder.try_snap(snap).unwrap_or_else(|builder| builder);
    }
    builder
}

/// 📜️ The text field of a text input — one line, or `LongText` for a multi-line one (design §22.7) — committing on blur.
/// A value longer than one UI text holds is shown clipped and read-only, so a commit never writes the clipped text back.
fn time_travel_text_control(kind: InputKind, shown: &str, id: &str, label: &str, action: ActionId, args: UiValue) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    input(kind)
        .value(UiText::clipped(shown))
        .commit(UiText::clipped("blur"))
        .disabled(shown.len() > UI_TEXT_MAX_BYTES)
        .try_id(id)
        .map_err(|_| error("time-travel-panel.input-id"))?
        .try_label(label)
        .map_err(|_| error("time-travel-panel.input-label"))?
        .try_on_with(Trigger::Commit, action, args)
        .map_err(|_| error("time-travel-panel.input-binding"))?
        .try_build()
        .map_err(|_| error("time-travel-panel.input"))
}

/// 🎛️ One input row, its id the pointer with `.` for `/` under `framework.history.editor.input` (`….row`): a tree item
/// reading the row's label (a refused value names its reason as the description) holding the control its descriptor
/// derives (`ActionArgDef::control`) as itself (design §22.7: a segmented choice is a segmented select, an icon choice an
/// icon picker, a multi-line text a multi-line field), bound to the row's pointer. A list's row holds Add item (`historyEditInput{edit:
/// insert}` at `<pointer>/-`, disabled at `maxItems`) and reads its item count; a list item's first row holds — or, under
/// its value control, nests — Remove item (`edit: remove` at its pointer, disabled at `minItems`). A reference row and a
/// choice of more options than one select holds ([`UI_FIXED_LIST_ITEMS`]) are tree windows: "Use selection", then one
/// chip per referenced id named by `editor.reference_labels` (removable by index from a reference list), or one row per
/// option. A `nullable` input nests a Clear row ("Clear <label>") that drafts `null` — disabled, and the row described as
/// cleared, while the input already is.
fn time_travel_input_row(row: &TimeTravelInputRow, editor: &TimeTravelEditorPanel, windows: &TreeWindows<'_>, controller_id: &str, generation: u32, locale: Locale) -> UiAssemblyResult<BuiltNode> {
    let error = ui_assembly_error;
    let (arg, value, pointer) = (&row.input, &row.value, row.pointer.as_str());
    let mut shown_unit: Option<String> = None;
    let label = row.label.resolve(Terminology::Native, locale);
    let id = format!("framework.history.editor.input{}", pointer.replace('/', "."));
    let cleared = arg.nullable && value.is_null();
    let refused = editor.refused.as_ref().filter(|(path, _)| path == pointer).map(|(_, reason)| reason.as_str());
    let described = |item: TreeItemBuilder, shown_unit: Option<String>| {
        if let Some(reason) = refused {
            return item.description(UiText::clipped(reason)).tone(Tone::Danger);
        }
        if cleared {
            return item.description(UiText::clipped(HistoryPanelText::Cleared.text(locale)));
        }
        let description = arg.description.as_ref().map(|description| description.resolve(Terminology::Native, locale).to_string());
        match (description, shown_unit) {
            (Some(description), Some(unit)) => item.description(UiText::clipped(&format!("{description} ({unit})"))),
            (Some(description), None) | (None, Some(description)) => item.description(UiText::clipped(&description)),
            (None, None) => item,
        }
    };
    let floor = |item: TimeTravelListItem| item.min.filter(|_| !item.removable).map(|min| time_travel_item_limit_text(min, false, locale));
    let remove_item = |item: TimeTravelListItem| {
        let remove = time_travel_button(
            controller_id,
            &format!("{id}.remove"),
            HistoryPanelText::RemoveItem.text(locale),
            "trash-2",
            HISTORY_EDIT_INPUT_ACTION_ID,
            Some(time_travel_list_edit_args(generation, pointer, HISTORY_EDIT_INPUT_REMOVE)?),
            item.removable,
        )?;
        time_travel_control_row(&format!("{id}.remove.row"), HistoryPanelText::RemoveItem.text(locale), "trash-2", remove, true, None, floor(item).as_deref())
    };
    let clear_row = || {
        let clear_label = HistoryPanelText::ClearInput.text(locale).replace("{label}", label);
        time_travel_button_row(controller_id, &format!("{id}.clear"), &clear_label, "eraser", HISTORY_EDIT_INPUT_ACTION_ID, Some(time_travel_value_args(generation, pointer, UiValue::Null)?), !cleared)
    };
    let item = ui::tree_item(Label(UiText::clipped(label))).try_id(format!("{id}.row")).map_err(|_| error("time-travel-panel.input-row-id"))?;
    if let Some(list) = row.list {
        let add = time_travel_button(
            controller_id,
            &format!("{id}.add"),
            HistoryPanelText::AddItem.text(locale),
            "plus",
            HISTORY_EDIT_INPUT_ACTION_ID,
            Some(time_travel_list_edit_args(generation, &format!("{pointer}/-"), HISTORY_EDIT_INPUT_INSERT)?),
            list.addable,
        )?;
        let mut count = HistoryPanelText::ItemCount.text(locale).replace("{count}", &list.len.to_string());
        if let Some(max) = list.max.filter(|_| !list.addable) {
            count = format!("{count} \u{b7} {}", time_travel_item_limit_text(max, true, locale));
        }
        let mut item = described(item, Some(count)).try_child(add).map_err(|_| error("time-travel-panel.list-row-child"))?;
        if let Some(entry) = row.item {
            item = item.default_open(true).try_child(remove_item(entry)?).map_err(|_| error("time-travel-panel.list-row-remove"))?;
        }
        return item.try_build().map_err(|_| error("time-travel-panel.list-row"));
    }
    if let (Some(entry), ArgSchema::Object { .. }) = (row.item, &arg.schema) {
        let remove = time_travel_button(
            controller_id,
            &format!("{id}.remove"),
            HistoryPanelText::RemoveItem.text(locale),
            "trash-2",
            HISTORY_EDIT_INPUT_ACTION_ID,
            Some(time_travel_list_edit_args(generation, pointer, HISTORY_EDIT_INPUT_REMOVE)?),
            entry.removable,
        )?;
        return described(item, floor(entry)).try_child(remove).map_err(|_| error("time-travel-panel.item-row-child"))?.try_build().map_err(|_| error("time-travel-panel.item-row"));
    }
    match arg.control() {
        ActionArgControl::Reference { domain, many, min_items, .. } => {
            let ids: Vec<String> = match value {
                DslValue::Array(items) => items.iter().filter_map(reference_id_text).collect(),
                other => reference_id_text(other).into_iter().collect(),
            };
            let removable = many && min_items.is_none_or(|min| ids.len() > min as usize);
            let kept = min_items.filter(|_| many && !removable).map(|min| time_travel_item_limit_text(min, false, locale));
            let chips = ids.len().max(1);
            let total = 1 + chips + usize::from(arg.nullable);
            return tree_window_indexed_item(windows, described(item, None), &format!("{id}.row"), true, total, |index| match index {
                0 => time_travel_button_row(
                    controller_id,
                    &format!("{id}.useSelection"),
                    HistoryPanelText::UseSelection.text(locale),
                    "crosshair",
                    HISTORY_EDIT_USE_SELECTION_ACTION_ID,
                    Some(time_travel_input_args(generation, pointer)?),
                    domain.is_some(),
                ),
                index if index > chips => clear_row(),
                _ if ids.is_empty() => ui::tree_item(Label(UiText::clipped(HistoryPanelText::NothingSelected.text(locale))))
                    .dimmed(true)
                    .try_id(format!("{id}.empty"))
                    .map_err(|_| error("time-travel-panel.reference-empty-id"))?
                    .try_build()
                    .map_err(|_| error("time-travel-panel.reference-empty")),
                index => {
                    let reference = &ids[index - 1];
                    let name = editor.reference_labels.get(reference).map_or_else(|| reference.clone(), |label| label.resolve(Terminology::Native, locale).to_string());
                    let chip = format!("{id}.chip.{}", index - 1);
                    if !many {
                        return ui::tree_item(Label(UiText::clipped(&name)))
                            .icon(ui_text("link", "time-travel-panel.chip-icon")?)
                            .try_id(format!("{chip}.row"))
                            .map_err(|_| error("time-travel-panel.chip-id"))?
                            .try_build()
                            .map_err(|_| error("time-travel-panel.chip"));
                    }
                    let remove = time_travel_button(
                        controller_id,
                        &chip,
                        &format!("{} {name}", HistoryPanelText::Remove.text(locale)),
                        "x",
                        HISTORY_EDIT_INPUT_ACTION_ID,
                        Some(time_travel_list_edit_args(generation, &format!("{pointer}/{}", index - 1), HISTORY_EDIT_INPUT_REMOVE)?),
                        removable,
                    )?;
                    time_travel_control_row(&format!("{chip}.row"), &name, "link", remove, true, None, kept.as_deref())
                }
            });
        }
        ActionArgControl::Select { options } | ActionArgControl::Segmented { options } if options.len() > UI_FIXED_LIST_ITEMS => {
            let selected = value.as_str().unwrap_or_default();
            let current = options.iter().find(|option| option.value == selected).map(|option| option.label.resolve(Terminology::Native, locale).to_string());
            let total = options.len() + usize::from(arg.nullable);
            return tree_window_indexed_item(windows, described(item, current), &format!("{id}.row"), true, total, |index| match options.get(index) {
                Some(option) => {
                    let chosen = option.value == selected;
                    let label = option.label.resolve(Terminology::Native, locale);
                    let icon = if chosen { "check" } else { "circle" };
                    let choose = time_travel_button(controller_id, &format!("{id}.option.{index}"), label, icon, HISTORY_EDIT_INPUT_ACTION_ID, Some(time_travel_value_args(generation, pointer, UiValue::Text(UiText::clipped(&option.value)))?), true)?;
                    ui::tree_item(Label(UiText::clipped(label)))
                        .icon(ui_text(icon, "time-travel-panel.option-icon")?)
                        .selected(chosen)
                        .try_id(format!("{id}.option.{index}.row"))
                        .map_err(|_| error("time-travel-panel.option-id"))?
                        .try_child(choose)
                        .map_err(|_| error("time-travel-panel.option-child"))?
                        .try_build()
                        .map_err(|_| error("time-travel-panel.option"))
                }
                None => clear_row(),
            });
        }
        _ => {}
    }
    let action = ActionId::try_v1(controller_id, HISTORY_EDIT_INPUT_ACTION_ID).ok_or_else(|| error("time-travel-panel.input-action"))?;
    let args = time_travel_input_args(generation, pointer)?;
    let number = value.as_f64().unwrap_or(0.0);
    let facets = arg.number_facets(locale).unwrap_or_default();
    let control: BuiltNode = match arg.control() {
        ActionArgControl::Slider { .. } | ActionArgControl::Dial { .. } => time_travel_slider(number, &facets)
            .try_id(&id)
            .map_err(|_| error("time-travel-panel.input-id"))?
            .try_label(label)
            .map_err(|_| error("time-travel-panel.input-label"))?
            .try_on_with(Trigger::Change, action, args)
            .map_err(|_| error("time-travel-panel.input-binding"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.input"))?,
        ActionArgControl::Stepper { .. } => {
            let mut builder = number_stepper(number).step(facets.step.unwrap_or(1.0)).limits(facets.limits.clone());
            if let Some(min) = facets.min {
                builder = builder.min(min);
            }
            if let Some(max) = facets.max {
                builder = builder.max(max);
            }
            if let Some(precision) = facets.precision {
                builder = builder.precision(precision);
            }
            if let Some(unit) = &facets.unit {
                builder = builder.unit(UiText::clipped(unit));
            }
            if let Some(display_unit) = &facets.display_unit {
                builder = builder.display_unit(UiText::clipped(display_unit));
            }
            if let Some(factor) = facets.display_factor {
                builder = builder.display_factor(factor);
            }
            for &snap in &facets.snaps {
                builder = builder.try_snap(snap).unwrap_or_else(|builder| builder);
            }
            builder
                .try_id(&id)
                .map_err(|_| error("time-travel-panel.input-id"))?
                .try_label(label)
                .map_err(|_| error("time-travel-panel.input-label"))?
                .try_on_with(Trigger::Change, action, args)
                .map_err(|_| error("time-travel-panel.input-binding"))?
                .try_build()
                .map_err(|_| error("time-travel-panel.input"))?
        }
        ActionArgControl::Number { .. } => {
            let mut builder = input(InputKind::Number).commit(UiText::clipped("blur")).limits(facets.limits.clone());
            if let Some(min) = facets.min {
                builder = builder.min(min);
            }
            if let Some(max) = facets.max {
                builder = builder.max(max);
            }
            if let Some(step) = facets.step {
                builder = builder.step(step);
            }
            if let Some(precision) = facets.precision {
                builder = builder.precision(precision);
            }
            if let Some(factor) = facets.display_factor {
                builder = builder.display_factor(factor);
            }
            shown_unit = facets.shown_unit().map(ToOwned::to_owned);
            builder
                .number(number)
                .try_id(&id)
                .map_err(|_| error("time-travel-panel.input-id"))?
                .try_label(label)
                .map_err(|_| error("time-travel-panel.input-label"))?
                .try_on_with(Trigger::Commit, action, args)
                .map_err(|_| error("time-travel-panel.input-binding"))?
                .try_build()
                .map_err(|_| error("time-travel-panel.input"))?
        }
        ActionArgControl::Toggle => toggle(value.as_bool().unwrap_or(false))
            .try_id(&id)
            .map_err(|_| error("time-travel-panel.input-id"))?
            .try_label(label)
            .map_err(|_| error("time-travel-panel.input-label"))?
            .try_on_with(Trigger::Change, action, args)
            .map_err(|_| error("time-travel-panel.input-binding"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.input"))?,
        ActionArgControl::Select { options } | ActionArgControl::Segmented { options } => {
            let appearance = if arg.presentation == Some(ArgPresentation::Segmented) { SelectAppearance::Segmented } else { SelectAppearance::Menu };
            let mut builder = select(UiText::clipped(value.as_str().unwrap_or_default())).appearance(appearance);
            for option in options.iter().take(UI_FIXED_LIST_ITEMS) {
                builder = builder.try_item(UiText::clipped(&option.value), Label(UiText::clipped(option.label.resolve(Terminology::Native, locale)))).map_err(|_| error("time-travel-panel.input-options"))?;
            }
            builder
                .try_id(&id)
                .map_err(|_| error("time-travel-panel.input-id"))?
                .try_label(label)
                .map_err(|_| error("time-travel-panel.input-label"))?
                .try_on_with(Trigger::Change, action, args)
                .map_err(|_| error("time-travel-panel.input-binding"))?
                .try_build()
                .map_err(|_| error("time-travel-panel.input"))?
        }
        ActionArgControl::Vector { dims, .. } => {
            let mut builder = vector_input(Label(UiText::clipped(label))).limits(facets.limits.clone());
            if let Some(factor) = facets.display_factor {
                builder = builder.display_factor(factor);
            }
            if let Some(step) = facets.step {
                builder = builder.step(step);
            }
            if let Some(min) = facets.min {
                builder = builder.min(min);
            }
            if let Some(max) = facets.max {
                builder = builder.max(max);
            }
            for &snap in &facets.snaps {
                builder = builder.try_snap(snap).unwrap_or_else(|builder| builder);
            }
            if let Some(precision) = facets.precision {
                builder = builder.precision(precision);
            }
            if let Some(unit) = facets.shown_unit() {
                builder = builder.unit(UiText::clipped(unit));
            }
            for axis in 0..dims {
                let axis_value = value.as_array().and_then(|items| items.get(axis as usize)).and_then(DslValue::as_f64).unwrap_or(0.0);
                let axis_args = time_travel_input_args(generation, &format!("{pointer}/{axis}"))?;
                let axis_action = action.clone();
                builder = builder
                    .try_axis(&format!("{id}.{axis}"), Label(UiText::clipped(&["x", "y", "z", "w"].get(axis as usize).map_or_else(|| axis.to_string(), |name| (*name).to_string()))), axis_value, |field| {
                        field.commit(UiText::clipped("blur")).try_on_with(Trigger::Commit, axis_action, axis_args).map_err(|(field, _)| field)
                    })
                    .map_err(|_| error("time-travel-panel.vector-axis"))?;
            }
            BuiltNode::from(builder.try_id(&id).map_err(|_| error("time-travel-panel.input-id"))?)
        }
        ActionArgControl::Color { alpha } => time_travel_color_control(value, alpha, &id, label, locale, &action, generation, pointer)?,
        ActionArgControl::IconSelect { classifier_kind } => icon_select(UiText::clipped(value.as_str().unwrap_or_default()), UiText::clipped(&classifier_kind))
            .try_id(&id)
            .map_err(|_| error("time-travel-panel.input-id"))?
            .try_label(label)
            .map_err(|_| error("time-travel-panel.input-label"))?
            .try_on_with(Trigger::Change, action, args)
            .map_err(|_| error("time-travel-panel.input-binding"))?
            .try_build()
            .map_err(|_| error("time-travel-panel.input"))?,
        control => {
            let shown = match value {
                DslValue::String(text) => text.clone(),
                other => time_travel_json(other),
            };
            if shown.len() > UI_TEXT_MAX_BYTES {
                shown_unit = Some(HistoryPanelText::TooLong.text(locale).to_string());
            }
            let kind = if matches!(control, ActionArgControl::Multiline) { InputKind::LongText } else { InputKind::Text };
            time_travel_text_control(kind, &shown, &id, label, action, args)?
        }
    };
    let mut item = described(item, shown_unit).try_child(control).map_err(|_| error("time-travel-panel.input-row-child"))?;
    if let Some(entry) = row.item {
        item = item.default_open(true).try_child(remove_item(entry)?).map_err(|_| error("time-travel-panel.input-row-remove"))?;
    }
    if arg.nullable {
        item = item.default_open(true).try_child(clear_row()?).map_err(|_| error("time-travel-panel.input-row-clear"))?;
    }
    item.try_build().map_err(|_| error("time-travel-panel.input-row"))
}
//#endregion 🔖️Panel
