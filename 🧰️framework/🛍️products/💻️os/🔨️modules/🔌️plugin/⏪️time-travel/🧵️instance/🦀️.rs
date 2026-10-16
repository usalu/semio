//! ⏪️ The mounted instance's time-travel turns (history editing sessions, replay and supersession) and the
//! [`TimeTravelExtension`] that registers them with the instance through [`InstanceExtension`].
#[cfg(test)] use semio_framework_os_kernel::HistoryPageStack;
use semio_framework::kernel::{HistoryReprojection, HistoryReprojectionKind};
use semio_framework::{mutation_input_instance, reference_id_text, registered_input_schema_document, ArgSchema, SnapSource, DIALOG_CHOICE_ARG};
use semio_framework::history_edit::{HISTORY_EDIT_ACCEPT_ACTION_ID, HISTORY_EDIT_ARG_EDIT, HISTORY_EDIT_ARG_MUTATION_ID, HISTORY_EDIT_ARG_NAME, HISTORY_EDIT_ARG_PATH, HISTORY_EDIT_ARG_STORE, HISTORY_EDIT_ARG_VALUE, HISTORY_EDIT_BACK_ACTION_ID, HISTORY_EDIT_BEGIN_ACTION_ID, HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID, HISTORY_EDIT_CHOICE_OVERWRITE, HISTORY_EDIT_COMMIT_ACTION_ID, HISTORY_EDIT_DISCARD_ACTION_ID, HISTORY_EDIT_EXIT_ACTION_ID, HISTORY_EDIT_FINALIZE_ACTION_ID, HISTORY_EDIT_FINALIZE_DIALOG_ID, HISTORY_EDIT_INPUT_ACTION_ID, HISTORY_EDIT_INPUT_INSERT, HISTORY_EDIT_INPUT_REMOVE, HISTORY_EDIT_RERUN_ACTION_ID, HISTORY_EDIT_RESTORE_ACTION_ID, HISTORY_EDIT_USE_SELECTION_ACTION_ID, HISTORY_EDIT_WITHDRAW_ACTION_ID};
use semio_framework_time_travel::{
    is_time_travel_alternative_name, TimeTravelBase, TimeTravelChoice, TimeTravelEffect, TimeTravelEvent, TimeTravelLabel, TimeTravelRefusal, TimeTravelStage, TimeTravelTarget, TIME_TRAVEL_BUSY_CODE,
    TIME_TRAVEL_COMMIT_FAILED_CODE, TIME_TRAVEL_MEMBER_GONE_CODE,
    TIME_TRAVEL_REPLAY_FAULTED_CODE,
};
use std::collections::VecDeque;
use std::sync::Arc;
use semio_framework_value::{RetirementDemand,ValueError,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use super::*;
use crate::artifact_instance::InstanceExtension;
use crate::artifact_instance::{history_changed_event, VcsArtifactApp};
use crate::app_emission::ActionMeta;
use crate::command_history::{CommandLogAppend, HISTORY_STEP_BLOCKED_CODE};
use crate::document_ingress::{ActiveDocumentArchiveLoad, ActiveDocumentArchiveLoadState};
use crate::interaction_topology::INTERACTION_ACTION_IDS;
use crate::reserved_jobs::HISTORY_ACTION_IDS;
use crate::tree_kit::TREE_WINDOW_DEFAULT_ROWS;
use semio_framework::{ActionKind, ViewModel, REVERT_TO_COMMAND_ACTION_ID};
use store::{MemberFactory, SpaceMember};
use crate::app_contract::is_time_travel_action_id;
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;

use crate::artifact_instance::item;
use crate::artifact_instance::{ToolOperations};
use crate::artifact_instance::{ExtensionCtx, ExtensionRef, MediaTransfers, TimeTravelCtx, TimeTravelRef};
use crate::tool_jobs::ToolRunLedger;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> TimeTravelCtx<'ctx, A, M, X> {
    /// 📨️ Runs one typed command on the store the session edits: the document's own, or the composed member's through
    /// its typed visitor (design §12).
    async fn time_travel_run(&mut self, command: TimeTravelStoreCommand) -> Result<TimeTravelStoreOutput, Fault> {
        let TimeTravelCtx { stores, time_travel, child_composition, .. } = self;
        let (store, children) = (&mut stores.store, &mut child_composition.children);
        match time_travel.member.as_mut() {
            None => time_travel.document.run(store, command).await,
            Some(member) => {
                let key = member.key.clone();
                let entry = children.member_mut(key.borrowed()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new(TIME_TRAVEL_MEMBER_GONE_CODE), format!("the composed member {} a history edit targets is gone", member.store())))?;
                entry.member.visit_member_mut(TimeTravelMemberRun { owners: &mut member.owners, command }).await
            }
        }
    }

    /// 🪪️ Finalizes the same retained document or composed member replay.
    async fn time_travel_commit(&mut self, drafts: protocol::HistoryInputDrafts, finalization: store::HistoryFinalization, actor: Option<&str>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TimeTravelCommit, Fault> {
        let TimeTravelCtx { stores, time_travel, child_composition, .. } = self;
        let (store, children) = (&mut stores.store, &mut child_composition.children);
        match time_travel.member.as_mut() {
            None => time_travel.document.commit_finished(store, drafts, finalization, actor, identity).await,
            Some(member) => {
                let entry = children.member_mut(member.key.borrowed()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new(TIME_TRAVEL_MEMBER_GONE_CODE), "the composed member a history edit targets is gone"))?;
                entry.member.visit_member_mut(TimeTravelMemberCommit { owners: &mut member.owners, drafts, finalization, actor, identity }).await
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
            let TimeTravelCtx { stores, time_travel, child_composition, .. } = self;
        let (store, children) = (&mut stores.store, &mut child_composition.children);
            time_travel.document.settle(store, stage)?;
            if let Some(member) = time_travel.member.as_mut() {
                let key = member.key.clone();
                if let (Some(owners), Some(entry)) = (member.owners.as_deref_mut(), children.member(key.borrowed())) {
                    entry.member.visit_member(TimeTravelMemberSettle { owners, stage })?;
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
            TimeTravelStoreOutput::Read(Some((read, revision))) => Some(self.child_composition.child_content_root.with_member_read(key.borrowed(), &dialect, revision, read)?),
            _ => None,
        };
        let previous = match self.time_travel.member.as_mut() {
            Some(member) => std::mem::replace(&mut member.children, next),
            None => next,
        };
        if let Some(previous) = previous {
            self.child_composition.retire_time_travel_children(previous)?;
        }
        self.time_travel.document_dirty = true;
        Ok(())
    }

    fn member_subject_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,Fault>{
        let Some(demand)=self.shared().member_subject_retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?else{return Ok(None)};
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
        if self.time_travel.member.as_ref().unwrap().review_children.is_some(){if self.drive_original_review_children(grant)?{let(original,receipt)=self.time_travel.take_command_receipt().expect("original subject close owns its review receipt");assert_eq!(original,grant);return Ok(Some(RetainedCloneStep::Progress(receipt)))}}
        let member=self.time_travel.member.as_mut().unwrap();
        if member.children.is_some(){let Some(generation)=self.child_composition.child_content_generation.checked_add(1).filter(|generation|self.child_composition.child_content_retirements.can_insert(*generation))else{return Ok(Some(RetainedCloneStep::Progress(Default::default())))};let original=member.children.take().unwrap();self.child_composition.child_content_retirements.insert_admitted(generation,ChildContentRetirement::new(original,false));self.child_composition.child_content_generation=generation;return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
        if member.owners.is_some(){drop(member.owners.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})))}
        if let Some(owner)=member.metadata_close.as_mut(){if owner.terminal_is_empty(){drop(member.metadata_close.take());member.metadata_retired=true;return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child).map_err(ValueError::into_fault)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"original composed member metadata").map_err(ValueError::into_fault)?;return Ok(Some(RetainedCloneStep::Progress(step.progress())))}
        if member.metadata_retired{drop(self.time_travel.member.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
        if !<TimeTravelMemberMetadata as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(Fault::from("original member metadata producer is unsupported"))}
        let original=TimeTravelMemberMetadata{key:std::mem::take(&mut member.key),path:member.path.take_original(),dialect:std::mem::replace(&mut member.dialect,ArtifactDialect{artifact_kind:String::new(),standard:String::new(),subset:String::new()})};match ControlledRetirement::new(original){Ok(owner)=>member.metadata_close=Some(owner),Err((error,original))=>{member.key=original.key;member.path=original.path;member.dialect=original.dialect;return Err(error.into_fault())}}
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
    }

    pub(crate) fn time_travel_retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>,Fault> {
        if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if self.time_travel.document.discarded_factory.is_none()&&(!self.time_travel.document.discarded.is_empty()||self.time_travel.document.discarded_pending.is_some()){
            if grant.maximum_depth==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
            let factory=self.stores.store.owned_mutation_retirement_factory().map_err(ValueError::into_fault)?;self.time_travel.document.discarded_factory=Some(Arc::clone(factory));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})));
        }
        if let Some(step)=self.time_travel.retire_step(grant)?{return Ok(Some(step))}self.member_subject_retirement_step(grant)
    }

    pub(crate) fn time_travel_close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep,Fault> {
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.time_travel.has_command_receipt(){return Ok(RetainedCloneStep::Progress(Default::default()))}
        self.time_travel.begin_close();
        self.time_travel.document.cancel_prepared_command();if let Some(owners)=self.time_travel.member.as_mut().and_then(|member|member.owners.as_deref_mut()){owners.cancel_prepared_command()}
        if self.drive_original_discard_command(grant)?{let (original,receipt)=self.time_travel.take_command_receipt().expect("close owns its original command turn receipt");assert_eq!(original,grant);return Ok(RetainedCloneStep::Progress(receipt))}
        if self.drive_original_review_children(grant)?{let (original,receipt)=self.time_travel.take_command_receipt().expect("close owns its original review view turn receipt");assert_eq!(original,grant);return Ok(RetainedCloneStep::Progress(receipt))}
        if let Some(step)=self.time_travel_retire_step(grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
        self.shared().time_travel_retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
        Ok(if self.time_travel.terminal_is_empty(){RetainedCloneStep::Complete(Default::default())}else{RetainedCloneStep::Progress(Default::default())})
    }

    /// 🧾️ Holds back the history row of `action`, a history step whose replay the document store deferred (gap N17): it
    /// is recorded when the store adopts the step and dropped with the step when it is cancelled or refused, so a cancelled
    /// step leaves zero trace in the history too; the body shows the step replaying at once.
    pub(crate) fn defer_history_step_row(&mut self, action: &str) {
        self.time_travel.deferred_history_row = Some(action.to_string());
        self.note_time_travel_changed(false, false);
    }

    /// 🧹️ Dismisses the row of a refused history-change adoption (nothing replays any more) once the user dispatches anything
    /// but a view or interaction verb (audit W2A-8): the refusal was shown, the next change starts clean; the history patch
    /// carries the change.
    pub(crate) fn dismiss_refused_reprojection(&mut self, action: &str) {
        let passive = matches!(self.tools.declared_action_kind(action), Some(ActionKind::View | ActionKind::Interaction)) || INTERACTION_ACTION_IDS.contains(&action);
        if passive || self.time_travel.reprojection_fault.is_none() || self.stores.store.reprojection_progress().is_some() {
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
    async fn step_reprojection_turn(&mut self, deadline_us: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let generation = self.stores.store.generation();
        let local = self.stores.store.local_step_pending();
        let stepped = self.stores.store.step_reprojection(Some(deadline_us)).await;
        if local && !self.stores.store.local_step_pending() {
            if let Some(action) = self.time_travel.deferred_history_row.take().filter(|_| stepped.is_ok()) {
                self.tools.record_command(&mut *self.history, &action, ActionKind::History, None, None, Vec::new(), None);
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
                if self.stores.store.generation() != generation {
                    self.history.cache = None;
                    self.as_dispatch().deliver_base_moved(identity, cx).await?;
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
                self.history.cache = None;
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
        if let Some(active) = self.ingest.live_document_load().and_then(|operation| self.ingest.document_archive_loads.get_mut(operation)) {
            active.request_terminal(ActiveDocumentArchiveLoadState::Cancelled);
            self.note_time_travel_changed(true, true);
            return TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive);
        }
        if self.stores.store.discard_local_step() {
            self.time_travel.reprojection_fault = None;
            self.time_travel.deferred_history_row = None;
            self.note_time_travel_changed(true, true);
            return TimeTravelActionOutcome::Applied(TimeTravelStage::Inactive);
        }
        if self.time_travel.reprojection_paused || self.stores.store.reprojection_progress().is_none() {
            return TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal));
        }
        self.stores.store.cancel_reprojection();
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

    /// 🧾️ A session change: a fresh history cursor, so the next patch is always delivered, carrying every row when
    /// their overlay moved (`rows`); the body refreshes (every window once when the rendered `document` swapped).
    fn note_time_travel_changed(&mut self, document: bool, rows: bool) {
        self.history.log_generation += 1;
        if rows {
            self.history.history_dirty_sequences.extend(self.history.command_log.iter().map(|entry| entry.seq));
        }
        self.time_travel.ui_dirty = true;
        self.time_travel.document_dirty |= document;
        self.time_travel.patch_due = true;
    }

    /// 📮️ Hands the owed scope to the typed UI outbox once it has room; until then the flags stay set.
    fn flush_time_travel_ui_dirty(&mut self) {
        if !self.time_travel.is_ui_dirty() || self.tools.typed_ui_outbox.len() != 0 {
            return;
        }
        if self.tools.typed_ui_outbox.push(self.time_travel.dirty_scope()).is_ok() {
            self.time_travel.ui_dirty = false;
            self.time_travel.document_dirty = false;
        }
    }

    /// 👀️ Store watch (design §4, driver contract): every change of the content revision of the store the session edits
    /// (never of its local generation alone: a document port attaching or detaching is no base move) reaches the
    /// session as `BaseMoved` with the re-resolved target positions — the open editor's too, so the rows downstream of
    /// the edited mutation stay the pending ones when a remote edit sorts before it; a target that vanished (undone
    /// remotely, document replaced, the edited member gone) exits the session, since nothing it drafted is addressable
    /// anymore. Answers whether the base moved.
    async fn watch_time_travel_base(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<bool, Fault> {
        let base = self.shared().time_travel_base();
        if base == Some(self.time_travel.session.base) {
            return Ok(false);
        }
        let mut effects = Vec::new();
        match (base, self.shared().time_travel_positions()) {
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
        self.perform_time_travel_effects(effects, None, &mut dialogs, identity).await?;
        Ok(true)
    }

    /// ⏪️ Host-driven routing of the reserved history-edit verbs (design §7): applied to the session now, never queued
    /// behind guest work. A refusal is a silent `{rejected}` result that still repaints the history body, so a stale
    /// button never keeps its stale identity. The reply carries the owed UI scope; the session status the verb moved to
    /// (its stage and generation) is prepared at once as the history patch of the unsolicited UI progress frame of this
    /// same turn, so every dispatch route — a command's reply, a retained-surface intent's reply that carries no patch —
    /// publishes the new generation with the stage flip and a verb stamped with it is never stale.
    pub(crate) async fn dispatch_time_travel_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let mut effects = Vec::new();
        let outcome = self.apply_time_travel_action(action, args, meta, &mut effects, identity, cx).await?;
        let output = match outcome {
            TimeTravelActionOutcome::Pending => DslValue::Object(vec![("timeTravel".into(),DslValue::String("pending".into()))]),
            TimeTravelActionOutcome::Applied(stage) => DslValue::Object(vec![("timeTravel".into(), DslValue::String(stage.as_str().into()))]),
            TimeTravelActionOutcome::Rejected(refusal) => DslValue::Object(vec![("rejected".into(), DslValue::String(refusal.code().into()))]),
        };
        let ui_scope = self.time_travel.dirty_scope();
        self.time_travel.ui_dirty = false;
        self.time_travel.document_dirty = false;
        self.prepare_time_travel_patch().await?;
        let mut result = ToolOperations::<A>::empty_result(action, meta, effects, Vec::new(), ui_scope).await;
        result.output = output;
        Ok(result)
    }

    /// ⚖️ Applies one history-edit verb: delivers a pending base change first, then the verb's event with its effects.
    pub async fn apply_time_travel_action(&mut self, action: &str, args: Option<&DslValue>, meta: &ActionMeta, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<TimeTravelActionOutcome, Fault> {
        if self.time_travel.owners().any(TimeTravelOwners::command_pending)||self.time_travel.command_receipt.is_some()||self.time_travel.pending_command.is_some()||self.time_travel.discard_cursor.is_some()||!self.time_travel.command_retirements.terminal_is_empty(){return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy))}
        self.watch_time_travel_base(identity).await?;
        if !self.time_travel.is_active() {
            match action {
                HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID => return Ok(self.cancel_reprojection_turns()),
                HISTORY_EDIT_RERUN_ACTION_ID => return Ok(self.resume_reprojection_turns()),
                _ => {}
            }
        }
        let generation = time_travel_arg_generation(args).unwrap_or(self.time_travel.session.generation);
        let event = match action {
            HISTORY_EDIT_BEGIN_ACTION_ID => return self.begin_time_travel(args, false, meta, effects, identity, cx).await,
            HISTORY_EDIT_INPUT_ACTION_ID => {
                let value = args.and_then(|args| args.get(HISTORY_EDIT_ARG_VALUE)).cloned();
                return self.draft_time_travel_input(time_travel_arg_text(args, HISTORY_EDIT_ARG_PATH), value, time_travel_arg_text(args, HISTORY_EDIT_ARG_EDIT), generation, meta, effects, identity).await;
            }
            HISTORY_EDIT_USE_SELECTION_ACTION_ID => return self.draft_time_travel_selection(time_travel_arg_text(args, HISTORY_EDIT_ARG_PATH), generation, meta, effects, identity).await,
            HISTORY_EDIT_WITHDRAW_ACTION_ID if time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID).is_some() => return self.begin_time_travel(args, true, meta, effects, identity, cx).await,
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
        self.apply_time_travel_event(event, Some(meta), effects, identity).await
    }

    /// ⚖️ Applies one event to the session and performs its effects; a refusal changes nothing. Every window body is
    /// re-published only when the event or its effects swapped the document a window shows (the committed document, the
    /// draft preview, the replayed head); any other edge — the finalize prompt opening or closing, an accepted draft
    /// starting its replay, a replay run again — re-publishes the history body and the chips alone.
    async fn reduce_unprepared_time_travel_event(&mut self, event: TimeTravelEvent, meta: Option<&ActionMeta>, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TimeTravelActionOutcome, Fault> {
        let shown = self.shared().time_travel_shown_identity();
        match self.time_travel.session.apply(event) {
            Ok(session_effects) => {
                self.note_time_travel_changed(false, true);
                self.perform_time_travel_effects(session_effects, meta, effects, identity).await?;
                self.time_travel.document_dirty |= shown != self.shared().time_travel_shown_identity();
                Ok(TimeTravelActionOutcome::Applied(self.time_travel.session.stage))
            }
            Err(refusal) => {
                self.time_travel.ui_dirty = true;
                Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(refusal)))
            }
        }
    }

    async fn apply_time_travel_event(&mut self,event:TimeTravelEvent,meta:Option<&ActionMeta>,effects:&mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>)->Result<TimeTravelActionOutcome,Fault>{
        if self.time_travel.owners().any(TimeTravelOwners::command_pending)||self.time_travel.command_receipt.is_some()||self.time_travel.pending_command.is_some()||self.time_travel.discard_cursor.is_some()||!self.time_travel.command_retirements.terminal_is_empty(){return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy))}
        if let TimeTravelEvent::Discard{generation}=&event{
            if *generation!=self.time_travel.session.generation{return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Stale)))}
            if self.time_travel.session.stage!=TimeTravelStage::Editing||self.time_travel.session.pending.is_none(){return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal)))}
        }
        if matches!(event,TimeTravelEvent::Discard{..}){let grant=self.mounted_policy.maintenance;let mut original=Some(event);let receipt=self.time_travel.stage_time_travel_command(&mut original,grant).map_err(ValueError::into_fault)?;if let Some(receipt)=receipt{assert!(receipt.fits(grant));self.time_travel.command_receipt=Some((grant,receipt));self.time_travel.ui_dirty=true;return Ok(TimeTravelActionOutcome::Pending)}return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy))}
        self.reduce_unprepared_time_travel_event(event,meta,effects, identity).await
    }

    /// 🛠️ Performs the session's effects in order; a commit's own `Finalized`/`FinalizeFaulted` effects follow it.
    async fn perform_time_travel_effects(&mut self, effects: Vec<TimeTravelEffect>, meta: Option<&ActionMeta>, kernel: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
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
                TimeTravelEffect::CommitOverwrite { .. } => queue.extend(self.commit_time_travel(store::HistoryFinalization::Overwrite, meta, identity).await?),
                TimeTravelEffect::CommitAlternative { name, .. } => queue.extend(self.commit_time_travel(store::HistoryFinalization::Alternative { name }, meta, identity).await?),
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
        let deadline_us = self.shared().time_travel_turn_deadline();
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
    async fn commit_time_travel(&mut self, finalization: store::HistoryFinalization, meta: Option<&ActionMeta>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Vec<TimeTravelEffect>, Fault> {
        let generation = self.time_travel.session.generation;
        let fault = |code: &str| TimeTravelEvent::FinalizeFaulted { generation, code: code.to_string() };
        let drafts = self.time_travel.accepted_drafts();
        let committed = self.time_travel_commit(drafts, finalization, meta.map(|meta| meta.actor.as_str()), identity).await?;
        let (event, authored) = match committed {
            TimeTravelCommit::Finalized { authored } => {
                self.history.cache = None;
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
            self.as_render().revalidate_interaction_on_document_change(meta, identity).await?;
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
        self.history.push_log_entry(CommandLogAppend { action_id: HISTORY_EDIT_COMMIT_ACTION_ID, label, kind: ActionKind::History, edit_id: None, transition_id: None, timestamp: None, inverse: None });
        let generation = self.child_composition.admit_child_content_publication()?;
        self.child_composition.publish_member_content(generation, key.borrowed()).await?;
        self.child_composition.send_member_lane(&mut *self.stores, &key, authored).await
    }

    /// ✏️ Opens (or retargets) the session on `mutationId` of the store `store` names (`<slot>/<childId>`, a composed
    /// member, design §12; absent: the document's own): refused while a mutating tool run or an agent transaction holds
    /// this instance, for a store the instance does not compose, for a session open on another store, for an unknown
    /// operation, and for one whose inputs cannot be edited. Opening a session delivers [`HostEvent::TimeTravelFrozen`] to
    /// every open window, so an open gesture there ends first. With `withdraw` (a history row's Withdraw, design §22.1)
    /// the session opens on a withdrawn draft and needs no editable inputs — refused only where the store's supersede law
    /// lets no withdrawal through; on the mutation already being edited it is the editor's own Withdraw.
    async fn begin_time_travel(&mut self, args: Option<&DslValue>, withdraw: bool, meta: &ActionMeta, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<TimeTravelActionOutcome, Fault> {
        let Some(mutation) = time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownMutation)) };
        let store = time_travel_arg_text(args, HISTORY_EDIT_ARG_STORE).filter(|store| !store.is_empty());
        let session = &self.time_travel.session;
        if withdraw && session.stage == TimeTravelStage::Editing && self.time_travel.member_store().as_deref() == store && session.pending.as_ref().is_some_and(|pending| pending.target.mutation.0 == mutation) {
            let generation = time_travel_arg_generation(args).unwrap_or(session.generation);
            return self.apply_time_travel_event(TimeTravelEvent::Withdraw { generation }, Some(meta), effects, identity).await;
        }
        if self.shared().time_travel_busy() {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy));
        }
        if self.time_travel.session.stage==TimeTravelStage::Reviewing&&self.time_travel.member.as_ref().is_some_and(|member|member.children.is_some()&&(self.time_travel.review_children_pending()||member.review_children.is_none())){return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy))}
        if self.time_travel.is_active() && self.time_travel.member_store().as_deref() != store {
            return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Busy));
        }
        let opening = !self.time_travel.is_active();
        if opening {
            let member = match store {
                None => None,
                Some(store) => {
                    let Some((path, key, entry)) = MemberPath::parse(store).and_then(|path| self.child_composition.children.resolve(&path).map(|(key, entry)| (path, key, entry))) else {
                        return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownMutation));
                    };
                    Some(TimeTravelMemberSubject { key, path, dialect: entry.reference.dialect.clone(), owners: None, children: None, review_children: None, metadata_close: None, metadata_retired: false })
                }
            };
            self.time_travel.member = member;
            match self.shared().time_travel_base() {
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
                self.perform_time_travel_effects(session_effects, Some(meta), effects, identity).await?;
                if opening {
                    self.as_dispatch().deliver_host_event_to_every_window(|window_id| HostEvent::TimeTravelFrozen { window_id }, meta, identity,cx).await?;
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
    async fn draft_time_travel_input(&mut self, path: Option<&str>, value: Option<DslValue>, edit: Option<&str>, generation: u32, meta: &ActionMeta, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TimeTravelActionOutcome, Fault> {
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
        let outcome = self.apply_time_travel_event(TimeTravelEvent::Draft { generation, replacement }, Some(meta), effects, identity).await?;
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
    async fn draft_time_travel_selection(&mut self, path: Option<&str>, generation: u32, meta: &ActionMeta, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TimeTravelActionOutcome, Fault> {
        let Some(editor) = self.time_travel.editor.as_ref() else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(TimeTravelRefusal::Illegal))) };
        let Some((input, false)) = path.and_then(|path| time_travel_input_at(&editor.inputs, &editor.value, path)) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
        let ArgSchema::Reference { domain: Some(domain), kinds, .. } = &input.schema else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownInput)) };
        let state = self.interaction.interaction_selection_snapshot(&*self.stores);
        let selection = state.selection.get(domain).map(|selection| protocol::DomainSelection { ids: selection.ids.iter().filter_map(|row| A::selection_reference_id(kinds, row)).collect(), ..selection.clone() });
        match time_travel_selection_value(&input, selection.as_ref()) {
            Ok(value) => self.draft_time_travel_input(path, Some(value), None, generation, meta, effects, identity).await,
            Err(refusal) => Ok(TimeTravelActionOutcome::Rejected(refusal)),
        }
    }

    /// ⏯️ One bounded driver turn (≤ [`TIME_TRAVEL_TURN_WALL_US`], design §20.14): owed scope, retirement, base watch, one
    /// replay slice, one authoring slice and one deferred-reprojection slice, all against ONE wall deadline taken when the
    /// turn begins on the ledger's turn clock (audit W2A-3) — a slow operation ends the turn, never an operation count; a
    /// session change prepares the history patch that rides the next unsolicited UI progress frame.
    pub(crate) async fn drive_time_travel_turn(&mut self, grant: RetainedCloneGrant, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        if grant.maximum_items==0{return Ok(());}
        if self.drive_original_discard_command(grant)?{return Ok(())}
        if self.drive_original_review_children(grant)?{return Ok(())}
        let deadline_us = self.shared().time_travel_turn_deadline();
        if self.time_travel.reprojection_paused && self.stores.store.reprojection_progress().is_none() {
            self.time_travel.reprojection_paused = false;
        }
        self.prepare_time_travel_patch().await?;
        self.flush_time_travel_ui_dirty();
        if let Some(step) = self.time_travel_retire_step(grant)? {
            if step.progress()!=RetainedCloneProgress::default() {
                #[cfg(debug_assertions)]
                if self.stores.store.reprojection_progress().is_some() {
                    static TURNS: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
                    let turns = TURNS.fetch_add(1, std::sync::atomic::Ordering::Relaxed).saturating_add(1);
                    if turns >= 1024 && turns.is_power_of_two() {
                        eprintln!("[DEBUG] reprojection driver cleanup turn={turns} step={step:?} progress={:?} documentRetirements={} documentDiscarded={}", self.stores.store.reprojection_progress(), self.time_travel.document.retirements.len(), self.time_travel.document.discarded.len());
                    }
                }
            }
            return Ok(());
        }
        let moved = self.watch_time_travel_base(identity).await?;
        if !moved && self.time_travel.session.stage == TimeTravelStage::Editing && self.time_travel.preview_progress().is_some() {
            if let TimeTravelStoreOutput::Previewed(step) = self.time_travel_run(TimeTravelStoreCommand::StepPreview { deadline_us, clock: self.time_travel.turn_clock }).await? {
                self.adopt_time_travel_preview_step(step).await?;
            }
        }
        if !moved && self.time_travel.session.stage == TimeTravelStage::Replaying && self.time_travel.replaying() {
            self.step_time_travel_replay(deadline_us, identity).await?;
        }
        if self.time_travel.authoring.is_some() && self.step_supersede_authoring(deadline_us, identity).await? != SupersedeAuthored::Pending {
            self.note_time_travel_changed(true, true);
        }
        if self.shared().reprojection_drives() {
            self.step_reprojection_turn(deadline_us, identity, cx).await?;
        }
        self.prepare_time_travel_patch().await?;
        self.flush_time_travel_ui_dirty();
        Ok(())
    }

    /// 🧾️ Prepares the owed history patch (a session change sets `patch_due`) for the next unsolicited UI progress
    fn drive_original_review_children(&mut self,grant:RetainedCloneGrant)->Result<bool,Fault>{
        if self.time_travel.has_command_receipt(){return Ok(true)}
        if !self.time_travel.review_children_pending(){return Ok(false)}
        let member=self.time_travel.member.as_mut().expect("original review member remains present");
        let receipt=if member.review_children.is_some(){
            let copied_bytes=size_of::<TimeTravelReviewChildren>()+size_of::<ChildContentRetirement>()+size_of::<u64>();
            let generation=self.child_composition.child_content_generation.checked_add(1).filter(|generation|self.child_composition.child_content_retirements.can_insert(*generation));
            if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copied_bytes||generation.is_none(){Default::default()}else{let review=member.review_children.take().unwrap();let generation=generation.unwrap();self.child_composition.child_content_retirements.insert_admitted(generation,ChildContentRetirement::new(review.view,false));self.child_composition.child_content_generation=generation;RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}}
        }else{
            let view=member.children.as_ref().expect("original review view remains present");match TimeTravelReviewChildren::capture(view,self.time_travel.session.base,self.time_travel.session.generation,grant){Some((review,receipt))=>{member.review_children=Some(review);receipt},None=>Default::default()}
        };
        self.time_travel.command_receipt=Some((grant,receipt));Ok(true)
    }

    fn drive_original_discard_command(&mut self,grant:RetainedCloneGrant)->Result<bool,Fault>{
        if self.time_travel.command_receipt.is_some(){return Ok(true)}
        if grant.maximum_items==0&&self.time_travel.pending_command.is_some(){self.time_travel.command_receipt=Some((grant,Default::default()));return Ok(true)}
        let TimeTravelCtx{stores,time_travel,child_composition,..}=self;
        let (store,children,child_content_retirements,child_content_generation)=(&mut stores.store,&mut child_composition.children,&mut child_composition.child_content_retirements,&mut child_composition.child_content_generation);
        let receipt=(||->Result<Option<RetainedCloneProgress>,ValueError>{
            if let Some(cursor)=time_travel.discard_cursor.as_mut(){
                if time_travel.closing{cursor.begin_close()}
                if cursor.is_validating(){return cursor.validate(&time_travel.session,grant).map(|step|Some(step.progress()))}
                if cursor.prepared_stage().is_some(){
                    let receipt=match time_travel.member.as_mut(){None=>time_travel.document.publish_discard(store,cursor,&mut time_travel.session,&mut time_travel.editor,grant)?,Some(member)=>{let entry=children.member(member.key.borrowed()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"Discard original member is absent"))?;let owners=member.owners.as_deref_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"Discard original member owners are absent"))?;entry.member.visit_member(discard_decision::DiscardMemberPublication{owners,cursor,session:&mut time_travel.session,editor:&mut time_travel.editor,review_children:&mut member.review_children,children:&mut member.children,retirements:child_content_retirements,child_generation:child_content_generation,view_copy_bytes:size_of::<TimeTravelReviewChildren>()+size_of::<ChildContentRetirement>()+size_of::<u64>(),grant})?}};
                    if time_travel.session.stage!=TimeTravelStage::Editing{time_travel.ui_dirty=true;time_travel.document_dirty=true;time_travel.patch_due=true}return Ok(Some(receipt))
                }
                if grant.maximum_depth<time_travel.command_retirements.len()+1{return Ok(Some(Default::default()))}
                return time_travel.command_retirements.admit_typed_retirement(&mut time_travel.discard_cursor,grant).map(|receipt|Some(receipt.unwrap_or_default()))
            }
            if let Some(custody)=time_travel.pending_command.as_mut(){
                if custody.original().is_some()&&!time_travel.closing{
                    if !time_travel.command_retirements.has_reserved_slot(){if grant.maximum_depth<time_travel.command_retirements.len()+1||grant.maximum_capacity_bytes<time_travel.command_retirements.next_reserve_capacity_byte_demand()?{return Ok(Some(Default::default()))}return time_travel.command_retirements.reserve_step(grant).map(Some)}
                    let parent_copy=0usize;if grant.maximum_copy_bytes<parent_copy||grant.maximum_depth<2{return Ok(Some(Default::default()))}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};
                    if let Some((cursor,mut receipt))=custody.admit_discard(&time_travel.command_retirements,child)?{if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"Discard semantic frame exceeded its original caller"))}receipt.copied_bytes+=parent_copy;time_travel.discard_cursor=Some(cursor);return Ok(Some(receipt))}return Ok(Some(Default::default()))
                }
                if custody.terminal_is_empty(){let copied_bytes=0usize;if grant.maximum_depth==0||grant.maximum_copy_bytes<copied_bytes{return Ok(Some(Default::default()))}drop(time_travel.pending_command.take());return Ok(Some(RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}))}
                return custody.close_step(grant).map(|step|Some(step.progress()))
            }
            if !time_travel.command_retirements.terminal_is_empty(){let owner=&time_travel.command_retirements;if grant.maximum_items==0||grant.maximum_copy_bytes<owner.next_copy_byte_demand()?||grant.maximum_capacity_bytes<owner.next_capacity_byte_demand(grant.maximum_copy_bytes)?||grant.maximum_release_bytes<owner.next_release_byte_demand()?||grant.maximum_depth<owner.next_depth_demand()?{return Ok(Some(Default::default()))}return time_travel.command_retirements.step(grant).map(|step|Some(step.progress()))}
            Ok(None)
        })().map_err(ValueError::into_fault)?;
        if let Some(receipt)=receipt{if !receipt.fits(grant){return Err(Fault::from("original TimeTravel command exceeded its caller grant"))}time_travel.command_receipt=Some((grant,receipt));return Ok(true)}Ok(false)
    }

    /// 🧾️ Prepares the owed history patch (a session change sets `patch_due`) for the next unsolicited UI progress
    /// frame. A patch that has not shipped yet is superseded by one that still carries its rows, so no row upsert is lost
    /// between two changes of one turn.
    async fn prepare_time_travel_patch(&mut self) -> Result<(), Fault> {
        if !std::mem::take(&mut self.time_travel.patch_due) {
            return Ok(());
        }
        if let Some(previous) = self.time_travel.patch.take() {
            self.history.history_dirty_sequences.extend(previous.upserts.iter().map(|entry| entry.seq));
        }
        self.time_travel.patch = Some(self.as_render().history_patch(false).await?);
        Ok(())
    }

    /// ⏭️ Steps the replay of the store the session edits until the turn deadline: progress ticks refresh the history body
    /// at most every [`TIME_TRAVEL_PROGRESS_REFRESH_MS`]; completion hands the report to the session and swaps the
    /// preview to the replayed head; a store refusal faults the replay.
    async fn step_time_travel_replay(&mut self, deadline_us: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
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
        self.apply_time_travel_event(event, None, &mut dialogs, identity).await?;
        Ok(())
    }
}

impl<'v, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> TimeTravelRef<'v, A, M, X> {
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
            None => Some(TimeTravelStoreState::query(Some(&self.time_travel.document), &self.stores.store, query)),
            Some(member) => {
                let entry = self.child_composition.children.member(member.key.borrowed())?;
                Some(entry.member.visit_member(TimeTravelMemberQuery { owners: member.owners.as_deref(), query }))
            }
        }
    }

    /// 🧹️ One bounded close unit of the history-edit ledger: the session vanishes, every typed owner settles against its
    /// store, then one owner retires.
    fn member_subject_retirement_demands(&self,body:usize)->Result<Option<RetirementDemand>,ValueError>{
        if !self.time_travel.closing&&self.time_travel.session.stage!=TimeTravelStage::Inactive{return Ok(None)}
        let Some(member)=self.time_travel.member.as_ref()else{return Ok(None)};
        if member.review_children.is_some()||member.children.is_some(){return Ok(Some(RetirementDemand{copy_bytes:size_of::<TimeTravelReviewChildren>()+size_of::<ChildContentRetirement>()+size_of::<u64>(),depth:1,..Default::default()}))}
        if let Some(owners)=member.owners.as_ref(){if !owners.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"inactive original member retains live typed sources"))}return Ok(Some(RetirementDemand{copy_bytes:size_of_val(&member.owners),release_bytes:size_of_val(owners.as_ref()),depth:1,..Default::default()}))}
        if let Some(owner)=member.metadata_close.as_ref(){return if owner.terminal_is_empty(){Ok(Some(RetirementDemand{copy_bytes:size_of_val(&member.metadata_close),depth:1,..Default::default()}))}else{Ok(Some(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original member metadata depth overflow"))?}))}}
        Ok(Some(RetirementDemand{copy_bytes:if member.metadata_retired{size_of::<TimeTravelMemberSubject>()}else{size_of::<TimeTravelMemberMetadata>()},depth:1,..Default::default()}))
    }

    pub(crate) fn time_travel_retirement_demands(&self, body: usize) -> Result<RetirementDemand,ValueError> {
        if self.time_travel.document.discarded_factory.is_none()&&(!self.time_travel.document.discarded.is_empty()||self.time_travel.document.discarded_pending.is_some()){self.stores.store.owned_mutation_retirement_factory()?;return Ok(RetirementDemand{depth:1,..Default::default()});}
        let demand=self.time_travel.retirement_demands(body)?;if demand!=RetirementDemand::default(){return Ok(demand);}
        if let Some(demand)=self.member_subject_retirement_demands(body)?{return Ok(demand)}
        if self.time_travel.closing&&!self.time_travel.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"time travel close retains an undeclared original session editor preview or authoring frontier"));}
        Ok(Default::default())
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
                    .and_then(|window| self.stores.window_config_store.pointer_values(&window.window_kind_id, std::slice::from_ref(&pointer)).into_iter().find(|(window_id, _)| *window_id == window.id))
                    .and_then(|(_, values)| values.into_iter().next().flatten())
                    .and_then(|value| value.as_f64());
                windowed.or_else(|| time_travel_pointer_get(config.get_or_init(|| semio_framework_value::ToValue::to_value(self.stores.config_store.snapshot_owner().as_ref())), &pointer).and_then(DslValue::as_f64))
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
                let committed = self.stores.store.snapshot_owner();
                semio_framework_value::ToValue::to_value(self.time_travel.render_snapshot_or(&*self.tool_runs, &committed).as_ref())
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
        let committed = self.stores.store.snapshot_owner();
        let shown = self.time_travel.render_snapshot_or(&*self.tool_runs, &committed);
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

    /// 🚜️ Whether driver turns step the document store's waiting reprojection: a local history step always (it cannot be
    /// paused, only dropped), a remote change unless the user paused it.
    fn reprojection_drives(&self) -> bool {
        self.stores.store.reprojection_progress().is_some() && (self.stores.store.local_step_pending() || !self.time_travel.reprojection_paused)
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

    /// 🎞️ The identity of the document snapshot the session shows in its stage on the document's own store (`None`: the
    /// committed document) — what tells whether a session change swapped the document a window renders. A session on a
    /// composed member marks its children view itself ([`Self::refresh_time_travel_children`]).
    fn time_travel_shown_identity(&self) -> Option<usize> {
        self.time_travel.document.shown(self.time_travel.session.stage).map(|shown| Arc::as_ptr(shown.snapshot_owner()) as usize)
    }

    /// ⏳️ Whether this instance refuses to open a history edit now (`timeTravel.busy`): a mutating tool run, an agent
    /// transaction or an open tool transaction holds it, the undo or redo of a finalize is being authored, or its own history
    /// step still replays — what disables every Edit row action with its reason (audit W2A-9).
    pub(crate) fn time_travel_busy(&self) -> bool {
        (!self.time_travel.is_active()&&self.time_travel.member.is_some()) || self.tool_runs.holds_mutating_run() || self.txn.pending_transaction.is_some() || self.stores.store.open_transaction().is_some() || self.time_travel.authoring.is_some() || self.stores.store.local_step_pending()
    }

    /// ⌛️ The wall deadline of a driver turn that begins now: [`TIME_TRAVEL_TURN_WALL_US`] on the ledger's turn clock (from
    /// zero when the clock cannot be read, so the turn ends after its first operation).
    fn time_travel_turn_deadline(&self) -> u64 {
        (self.time_travel.turn_clock)().unwrap_or(0).saturating_add(TIME_TRAVEL_TURN_WALL_US)
    }
}

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> TimeTravelCtx<'ctx, A, M, X> {
    /// ✍️ Authors `inputs` within `scope` (unscoped: every line) resumably (design §16.6) — the restore of an undo or the
    /// re-authoring of a redo: a Report replay of the inputs over the applied history, stepped within this turn's budget and
    /// then per reactor turn, finalized by `commit_finished_replay` without replaying again. Answers whether it landed, is
    /// still replaying, was refused (an input no longer applied, or a report that blocks finalizing; nothing changes) or
    /// waits for an authoring already running.
    async fn author_supersede(&mut self, scope: Option<String>, inputs: Vec<protocol::SupersededInput>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<SupersedeAuthored, Fault> {
        if self.time_travel.authoring.is_some() {
            return Ok(SupersedeAuthored::Busy);
        }
        let drafts: protocol::HistoryInputDrafts = inputs.into_iter().map(|input| (input.target, input.replacement)).collect();
        let finalization = match scope {
            Some(alternative_id) => store::HistoryFinalization::Scope { alternative_id },
            None => store::HistoryFinalization::Overwrite,
        };
        match self.stores.store.begin_derived_report_replay(drafts, None) {
            Ok(replay) => self.time_travel.authoring = Some(SupersedeAuthoring { replay, finalization }),
            Err(vcs::VcsError::ValidationFailed(_) | vcs::VcsError::Rejected { .. }) => return Ok(SupersedeAuthored::Refused),
            Err(error) => return Err(error.into_fault()),
        }
        self.step_supersede_authoring(self.shared().time_travel_turn_deadline(), identity).await
    }

    /// ⏭️ One turn of the undo or redo of a finalize being authored: steps its replay until the turn deadline, then commits
    /// the finished replay in its scope; a store that moved meanwhile replays the same inputs again from the new base.
    async fn step_supersede_authoring(&mut self, deadline_us: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<SupersedeAuthored, Fault> {
        let stepped = {
            let TimeTravelCtx { stores, time_travel, .. } = self;
            let store = &mut stores.store;
            let clock = time_travel.turn_clock;
            let Some(authoring) = time_travel.authoring.as_mut() else { return Ok(SupersedeAuthored::Refused) };
            let mut deadline = || clock().is_none_or(|now| now >= deadline_us);
            let grant=history_planning_retirement_grant(authoring.replay.planning_retirement_demands(semio_framework_job::JOB_PAYLOAD_PAGE_BYTES).map_err(ValueError::into_fault)?);
            store.step_derived_report_replay(&mut authoring.replay, grant, &mut deadline)
        };
        match stepped {
            Ok(store::os_store::replay::ReplayStep::Pending(_)) => return Ok(SupersedeAuthored::Pending),
            Ok(store::os_store::replay::ReplayStep::Finished(_)) => {}
            Err(_) => {
                if let Some(authoring) = self.time_travel.authoring.take() {
                    let SupersedeAuthoring {replay,finalization}=authoring;let mut replay=Some(replay);
                    let grant=history_retirement_frame_grant(ArtifactStore::<A::Snapshot,A::Mutation>::history_read_retirement_birth_bytes());
                    match self.stores.store.retire_derived_report_replay(&mut replay,grant) {
                        Ok(Some((retirement,progress)))=>{assert!(progress.fits(grant));self.time_travel.document.retirements.push_back(retirement);}
                        Ok(None)=>unreachable!("retained supersede authoring is present"),
                        Err(error)=>{self.time_travel.authoring=Some(SupersedeAuthoring {replay:replay.take().unwrap(),finalization});return Err(error.into_fault());}
                    }
                }
                return Ok(SupersedeAuthored::Refused);
            }
        }
        let SupersedeAuthoring { replay, finalization } = self.time_travel.authoring.take().expect("a finished authoring replay was held");
        let Ok((result, head)) = self.stores.store.finish_derived_report_replay(&mut Some(replay)) else { return Ok(SupersedeAuthored::Refused) };
        if let Some(head) = head {
            self.time_travel.document.retire(&self.stores.store, Some(head))?;
        }
        let drafts = result.drafts().clone();
        match self.stores.store.commit_finished_replay(result, finalization.clone(), identity).await {
            Ok(_) => {
                self.history.cache = None;
                Ok(SupersedeAuthored::Landed)
            }
            Err(vcs::VcsError::Stale { .. }) => match self.stores.store.begin_derived_report_replay(drafts, None) {
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
    pub(crate) async fn dispatch_supersede_history_action(&mut self, action: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Option<InvocationResult>, Fault> {
        self.as_render().refresh_cache().await?;
        let (child_applied_tails, child_has_redo_tail) = self.child_composition.child_history_tails().await;
        let authored = match action {
            "undo" => self.history.supersede_undo_target(&*self.stores, &child_applied_tails).map(|entry| self.history.supersedes.restore(entry)),
            "redo" if !child_has_redo_tail => self.history.supersede_redo_target(&*self.stores).map(|(entry, undo)| self.history.supersedes.redo(entry, undo)),
            _ => None,
        };
        let Some((scope, inputs)) = authored.filter(|(_, inputs)| !inputs.is_empty()) else { return Ok(None) };
        let authored = self.author_supersede(scope, inputs, identity).await?;
        Ok(Some(self.supersede_authored_result(action, meta, authored).await))
    }

    /// 🧾️ The result of a verb that authored the undo or redo of a finalize: a landed one changed the history (every body
    /// refreshes), a pending one lands in a later reactor turn, an authoring already running answers `timeTravel.busy`.
    async fn supersede_authored_result(&mut self, action: &str, meta: &ActionMeta, authored: SupersedeAuthored) -> InvocationResult {
        let (events, scope) = match authored {
            SupersedeAuthored::Landed => (vec![history_changed_event().await], UiDirtyScope::Full),
            SupersedeAuthored::Pending | SupersedeAuthored::Refused | SupersedeAuthored::Busy => (Vec::new(), UiDirtyScope::None),
        };
        let mut result = ToolOperations::<A>::empty_result(action, meta, Vec::new(), events, scope).await;
        if authored == SupersedeAuthored::Busy {
            result.output = DslValue::Object(vec![("rejected".into(), DslValue::String(TIME_TRAVEL_BUSY_CODE.into()))]);
        }
        result
    }

    /// ⏪️ `revertToCommand` on a history-edit row: takes back the history edit the row acts for, when it is this replica's
    /// and still in effect.
    pub(crate) async fn revert_history_edit_row(&mut self, transition_id: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<InvocationResult, Fault> {
        let local = self.stores.supersede_author().to_string();
        let applied_entries = self.history.supersedes.applied_entries(self.stores.store.supersessions().iter());
        let target = self.history.supersedes.record(transition_id).filter(|(_, record)| record.role != SupersedeRole::Undo && record.actor.as_str() == local).map(|(_, record)| record.entry).filter(|entry| self.history.supersedes.undoable(&local, *entry, &applied_entries));
        let authored = match target.map(|entry| self.history.supersedes.restore(entry)).filter(|(_, inputs)| !inputs.is_empty()) {
            Some((scope, inputs)) => self.author_supersede(scope, inputs, identity).await?,
            None => SupersedeAuthored::Refused,
        };
        Ok(self.supersede_authored_result(REVERT_TO_COMMAND_ACTION_ID, meta, authored).await)
    }
}

/// 🧊️ The framework-reserved verbs a history-edit session freezes: the history-lane verbs (they move the applied
/// stack or the head and would invalidate the session base silently) and the clipboard verbs that edit the document.
fn time_travel_freezes_reserved_action(action: &str) -> bool {
    HISTORY_ACTION_IDS.contains(&action) || matches!(action, REVERT_TO_COMMAND_ACTION_ID | "cut" | "paste")
}

/// 🥶️ The `timeTravel.frozen` refusal of `verb` while a history edit is open on this instance.
fn time_travel_frozen_fault(verb: &str) -> Fault {
    Fault::new(FaultOrigin::Framework, FaultCode::new(semio_framework_time_travel::TIME_TRAVEL_FROZEN_CODE), format!("verb {verb:?} would change the document while a history edit is open on this instance"))
}

/// 🕰️ Non-destructive history editing as an instance extension: the `historyEdit*` verbs, the document freeze an open
/// session holds, and the session's maintenance turn.
pub(crate) struct TimeTravelExtension;

impl InstanceExtension for TimeTravelExtension {
    fn claims_action(action: &str) -> bool {
        is_time_travel_action_id(action)
    }

    fn reads_history(action: &str) -> bool {
        is_time_travel_action_id(action)
    }

    fn admitted_while_loading(action: &str) -> bool {
        action == HISTORY_EDIT_CANCEL_REPLAY_ACTION_ID
    }

    async fn dispatch_action<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, action: &str, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let mut ctx = TimeTravelCtx::of_extension(ctx);
        ctx.as_render().freeze_tool_machines();
        ctx.dispatch_time_travel_action(action, args, meta, identity, cx).await
    }

    fn reserved_action_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, action: &str) -> Option<Fault> {
        (time_travel.freezes_local_emits() && time_travel_freezes_reserved_action(action)).then(|| time_travel_frozen_fault(action))
    }

    fn freezes_document<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        time_travel.freezes_local_emits()
    }

    fn document_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, verb: &str) -> Option<Fault> {
        time_travel.freezes_local_emits().then(|| time_travel_frozen_fault(verb))
    }

    fn transaction_refusal<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>, txn_id: &str) -> Option<Fault> {
        time_travel.freezes_local_emits().then(|| MediaTransfers::<A>::transaction_fault(FaultOrigin::Framework, semio_framework_time_travel::TIME_TRAVEL_FROZEN_CODE, format!("transaction {txn_id:?} would change the document while a history edit is open on this instance")))
    }

    fn has_pending_work<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &ExtensionRef<'_, A, M, X>) -> bool {
        TimeTravelRef::of_extension_ref(ctx).time_travel_has_pending_work()
    }

    fn holds_receipt<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        time_travel.has_command_receipt()
    }

    fn terminal_is_empty<A: ArtifactApp>(time_travel: &TimeTravelLedger<A>, _tool_runs: &ToolRunLedger<A>) -> bool {
        time_travel.terminal_is_empty()
    }

    async fn drive_turn<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension>(ctx: &mut ExtensionCtx<'_, A, M, X>, retained: &mut RetainedCloneGrant, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, crate::PluginLifecycleFailure> {
        let mut ctx = TimeTravelCtx::of_extension(ctx);
        if !ctx.shared().time_travel_has_pending_work() {
            return Ok(false);
        }
        let grant = crate::plugin_runtime::original_plugin_turn_grant(ctx.mounted_policy.maintenance, *retained);
        ctx.drive_time_travel_turn(grant, identity, cx).await?;
        Ok(ctx.time_travel.has_command_receipt())
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod extension_tests;
