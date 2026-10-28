//! 🔎️ Preview, replay and session start of the instance time-travel turns: the preview, its adoption, the replay start, the commit, the member publication and the session start. See [`super`].

use super::*;
use semio_framework::Fault;
use semio_framework::FaultOrigin;
use semio_framework::FaultCode;
use semio_framework_plugin_child::MemberPath;
use semio_framework_plugin_decl_emission::HostEvent;
use semio_framework::MutationId;
use semio_framework::Effect;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TimeTravelCtx<'ctx, A, M, X> {
    /// 🖼️ The editing preview on the store the session edits: the state before `target` with the accepted upstream
    /// drafts, then `replacement` folded exactly as the store's replay folds it; the draft's own messages stay on the
    /// editor.
    pub(super) async fn show_time_travel_preview(&mut self, target: &MutationId, replacement: &protocol::InputReplacement) -> Result<(), Fault> {
        let drafts = self.time_travel.accepted_drafts();
        let deadline_us = self.shared().time_travel_turn_deadline();
        let TimeTravelStoreOutput::Previewed(step) = self.time_travel_run(TimeTravelStoreCommand::Preview { target: target.clone(), replacement: replacement.clone(), drafts, deadline_us, clock: self.time_travel.turn_clock },None).await? else {
            return Err(Fault::new(FaultOrigin::Plugin, FaultCode::new("timeTravel.preview-mismatch"), "a history-edit preview answered another command"));
        };
        self.adopt_time_travel_preview_step(step,None).await
    }

    pub(super)  async fn adopt_time_travel_preview_step(&mut self, step: TimeTravelPreviewStep,grant:Option<RetainedCloneGrant>) -> Result<(), Fault> {
        let progress=match &step{TimeTravelPreviewStep::Progress(p)|TimeTravelPreviewStep::Completed(_,p)=>*p,TimeTravelPreviewStep::Pending=>Default::default(),TimeTravelPreviewStep::Refused(error)=>error.retained_progress()};
        if let Some(grant)=grant{if !progress.fits(grant){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preview producer exceeds its original turn grant").into_fault());}if progress!=RetainedCloneProgress::default(){if self.time_travel.command_receipt.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original preview receipt awaits its mounted parent").into_fault());}self.time_travel.command_receipt=Some((grant,progress));}}
        if let TimeTravelPreviewStep::Refused(error)=step{return Err(error.into_fault())}
        if let TimeTravelPreviewStep::Completed(outcome,_) = step {
            if let Some(editor) = self.time_travel.editor.as_mut() { editor.outcome = outcome; }
            self.time_travel.document_dirty = true;
            self.refresh_time_travel_children().await?;
        }
        self.note_time_travel_changed(false, false);
        Ok(())
    }

    /// ▶️ Starts (latest-wins) the Report replay of `drafts` from `from` on the store the session edits; a store refusal
    /// faults the replay instead.
    pub(super) async fn start_time_travel_replay(&mut self, drafts: Vec<protocol::SupersededInput>, from: &MutationId) -> Result<Vec<TimeTravelEffect>, Fault> {
        self.time_travel.refreshed_done = 0;
        let drafts: protocol::HistoryInputDrafts = drafts.into_iter().map(|draft| (draft.target, draft.replacement)).collect();
        let generation = self.time_travel.session.generation;
        Ok(match self.time_travel_run(TimeTravelStoreCommand::StartReplay { drafts, from: from.clone() },None).await? {
            TimeTravelStoreOutput::ReplayStarted(true) => Vec::new(),
            _ => self.time_travel.session.apply(TimeTravelEvent::ReplayFaulted { generation, code: TIME_TRAVEL_REPLAY_FAULTED_CODE.to_string() }).unwrap_or_default(),
        })
    }

    /// 🌿️ The original finalize effect enters a paid typed request slot; maintenance retains and drives its genuine completed result.
    pub(super) async fn commit_time_travel(&mut self,finalization:store::HistoryFinalization,meta:Option<&ActionMeta>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Vec<TimeTravelEffect>,Fault>{let mut request=Some(finalization);self.time_travel_commit(&mut request,meta.map(|meta|meta.actor.as_str()),cx).await?;assert!(request.is_none(),"original finalize effect preflight retains enough request custody");self.note_time_travel_changed(false,true);Ok(Vec::new())}

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
    pub(super) async fn begin_time_travel(&mut self, args: Option<&DslValue>, withdraw: bool, meta: &ActionMeta, effects: &mut Vec<Effect>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<TimeTravelActionOutcome, Fault> {
        let Some(mutation) = time_travel_arg_text(args, HISTORY_EDIT_ARG_MUTATION_ID) else { return Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::UnknownMutation)) };
        let store = time_travel_arg_text(args, HISTORY_EDIT_ARG_STORE).filter(|store| !store.is_empty());
        let session = &self.time_travel.session;
        if withdraw && session.stage == TimeTravelStage::Editing && self.time_travel.member_store().as_deref() == store && session.pending.as_ref().is_some_and(|pending| pending.target.mutation.0 == mutation) {
            let generation = time_travel_arg_generation(args).unwrap_or(session.generation);
            return self.apply_time_travel_event(TimeTravelEvent::Withdraw { generation }, Some(meta), effects, identity,Some(cx)).await;
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
        let opened = match self.time_travel_run(TimeTravelStoreCommand::Open { target: target.clone(), current, withdraw },None).await? {
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
                self.time_travel_run(TimeTravelStoreCommand::Adopt(true),None).await?;
                self.time_travel.replace_editor(Some(editor));
                self.note_time_travel_changed(true, true);
                self.perform_time_travel_effects(session_effects, Some(meta), effects, identity,Some(cx)).await?;
                if opening {
                    self.as_dispatch().deliver_host_event_to_every_window(|window_id| HostEvent::TimeTravelFrozen { window_id }, meta, identity,cx).await?;
                }
                Ok(TimeTravelActionOutcome::Applied(self.time_travel.session.stage))
            }
            Err(refusal) => {
                self.time_travel_run(TimeTravelStoreCommand::Adopt(false),None).await?;
                if opening {
                    self.settle_time_travel_owners()?;
                }
                self.time_travel.ui_dirty = true;
                Ok(TimeTravelActionOutcome::Rejected(TimeTravelActionRefusal::Session(refusal)))
            }
        }
    }

}
