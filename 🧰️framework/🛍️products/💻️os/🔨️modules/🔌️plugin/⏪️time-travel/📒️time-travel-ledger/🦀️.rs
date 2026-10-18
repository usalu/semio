//! 📒️ Time-travel ledger: the per-instance ledger hosting the pure session reducer, the supersede authoring records and the ledger operations. See [`super`].

use super::*;

//#region 🔖️Ledger
/// 🕰️ The ephemeral local-only history edit of one document instance (design §4 and §7): the pure session, the draft
/// editor, and the typed owners of the store the session edits — the document's own, or one composed member's (§12):
/// the preview the renderer reads while editing (state before the target with the draft), the running Report replay,
/// its finished result (whose head is the reviewing preview and whose report finalizes without replaying again), and
/// the owners still retiring. Nothing here is persisted or shared; closing the instance discards it.
pub struct TimeTravelLedger<A: ArtifactApp> {
    pub session: TimeTravelSession,
    pub pending_command: Option<semio_framework_time_travel::TimeTravelCommandCustody>,
    pub discard_cursor: Option<Box<semio_framework_time_travel::TimeTravelDiscardCursor>>,
    pub command_retirements: semio_framework_value::retirement_queue::RetirementQueue,
    pub command_receipt: Option<(RetainedCloneGrant,RetainedCloneProgress)>,
    pub(super) session_retirement: Option<ControlledRetirement<TimeTravelSession>>,
    pub(super) session_retired: bool,
    pub editor: Option<TimeTravelEditor>,
    pub document: TimeTravelStoreState<A::Snapshot, A::Mutation>,
    pub member: Option<TimeTravelMemberSubject>,
    pub retiring: Vec<Box<dyn TimeTravelOwners>>,
    pub closing: bool,
    pub ui_dirty: bool,
    pub document_dirty: bool,
    pub refreshed_ms: u64,
    pub refreshed_done: u32,
    pub patch_due: bool,
    pub patch: Option<HistoryPatch>,
    pub reprojection_paused: bool,
    pub reprojection_fault: Option<(String, HistoryReprojectionKind)>,
    pub reprojection_refreshed_ms: u64,
    pub deferred_history_row: Option<String>,
    pub authoring: Option<SupersedeAuthoring<A::Snapshot, A::Mutation>>,
    pub history_unavailable: bool,
    pub turn_clock: fn() -> Option<u64>,
}

/// ✍️ The undo or redo of a finalize being authored resumably: the Report replay of its inputs over the applied history,
/// stepped per reactor turn, and how it finalizes (unscoped, or within the history edit's own line).
#[doc(hidden)]
pub struct SupersedeAuthoring<P, Mu: ::protocol::Mutation<P>> {
    pub replay: store::os_store::snapshot_read::ArtifactDerivedReplay<P, Mu>,
    pub finalization: store::HistoryFinalization,
}

/// 🧮️ Where authoring the undo or redo of a finalize stands.
#[doc(hidden)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SupersedeAuthored {
    Landed,
    Pending,
    Refused,
    Busy,
}

impl<A: ArtifactApp> Default for TimeTravelLedger<A> {
    fn default() -> Self {
        Self {
            session: TimeTravelSession::default(),
            pending_command: None,
            discard_cursor: None,
            command_retirements: Default::default(),
            command_receipt: None,
            session_retirement: None,
            session_retired: false,
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
    pub fn set_turn_clock(&mut self, clock: fn() -> Option<u64>) {
        self.turn_clock = clock;
    }

    /// 🕰️ The pure session state.
    pub fn session(&self) -> &TimeTravelSession {
        &self.session
    }

    /// 🎟️ Transfers the original caller-funded command receipt to its mounted parent.
    pub fn take_command_receipt(&mut self)->Option<(RetainedCloneGrant,RetainedCloneProgress)>{self.command_receipt.take()}

    /// 📨️ Borrows whether the mounted caller still owns an unconsumed original command receipt.
    pub fn has_command_receipt(&self)->bool{self.command_receipt.is_some()}

    /// ✏️ The draft editor while a mutation is being edited.
    pub fn editor(&self) -> Option<&TimeTravelEditor> {
        self.editor.as_ref()
    }

    /// ✏️ The draft editor for a law that exercises a descriptor the fixture app's leaves do not declare.
    #[cfg(test)]
    pub fn editor_mut(&mut self) -> Option<&mut TimeTravelEditor> {
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
            (stage, false) => self.document.shown(stage).map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::snapshot_owner).unwrap_or(committed),
        }
    }

    /// 🪞️ The composed children every render seam reads: while a session on a member shows that member's preview or
    /// replayed head, the live children with that member in its place (design §12); otherwise the member tool run's composed
    /// read or `live` (`ToolRunLedger::children_or`).
    pub fn render_children_or<'a>(&'a self, tool_runs: &'a ToolRunLedger<A>, live: &'a ChildContentView) -> &'a ChildContentView {
        self.member.as_ref().and_then(|member| member.children.as_ref()).filter(|_| self.is_active()).unwrap_or_else(|| tool_runs.children_or(live))
    }

    /// ⏯️ Whether the store the session edits holds a running replay.
    pub fn replaying(&self) -> bool {
        match self.member.as_ref() {
            None => TimeTravelOwners::replaying(&self.document),
            Some(member) => member.owners.as_deref().is_some_and(TimeTravelOwners::replaying),
        }
    }

    pub fn preview_progress(&self) -> Option<store::os_store::replay::ReplayProgress> {
        match self.member.as_ref() {
            None => self.document.preview_progress(),
            Some(member) => member.owners.as_deref().and_then(TimeTravelOwners::preview_progress),
        }
    }

    pub fn processed(&self) -> Option<u32> {
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
    pub fn owners(&self) -> impl Iterator<Item = &dyn TimeTravelOwners> {
        std::iter::once(&self.document as &dyn TimeTravelOwners).chain(self.member.iter().filter_map(|member| member.owners.as_deref())).chain(self.retiring.iter().map(|owners| &**owners))
    }

    /// 🏃️ Whether a driver turn has work: a live replay, an undo or redo of a finalize being authored, retirement, an owed
    /// UI scope or an owed history patch.
    pub fn has_pending_work(&self) -> bool {
        let replaying = self.session.stage == TimeTravelStage::Replaying && self.owners().any(TimeTravelOwners::replaying);
        self.member.as_ref().is_some_and(|_|self.closing||self.session.stage==TimeTravelStage::Inactive) || self.review_children_pending() || self.command_receipt.is_some() || self.pending_command.is_some() || self.discard_cursor.is_some() || !self.command_retirements.terminal_is_empty() || replaying || self.authoring.is_some() || self.owners().any(TimeTravelOwners::has_pending_work) || self.is_ui_dirty() || self.patch_due || self.patch.is_some()
    }

    pub fn review_children_pending(&self)->bool{
        self.member.as_ref().is_some_and(|member|match member.review_children.as_ref(){Some(review)=>self.closing||review.base!=self.session.base||!matches!(self.session.stage,TimeTravelStage::Editing|TimeTravelStage::Reviewing)||self.session.stage==TimeTravelStage::Reviewing&&(review.generation!=self.session.generation||member.children.as_ref().is_none_or(|view|!review.view.same_owner(view))),None=>!self.closing&&self.session.stage==TimeTravelStage::Reviewing&&member.children.is_some()})
    }

    /// 📨️ Pays the original received command slot while retaining unadmitted input in its caller.
    pub fn stage_time_travel_command(&mut self,original:&mut Option<TimeTravelEvent>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError>{
        if self.command_receipt.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"actor original command receipt awaits its caller"))}
        if self.closing||self.pending_command.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"actor already retains an original pending command"))}
        let parent_copy=size_of::<Option<semio_framework_time_travel::TimeTravelCommandCustody>>();
        if grant.maximum_copy_bytes<parent_copy||grant.maximum_depth<2{return Ok(None)}
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};
        let Some((owner,mut receipt))=semio_framework_time_travel::TimeTravelCommandCustody::admit_original(original,child)?else{return Ok(None)};
        if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original command admission exceeded its caller"))}
        receipt.copied_bytes+=parent_copy;self.pending_command=Some(owner);Ok(Some(receipt))
    }

    /// 🫥️ Whether the document holds no history because a pure command hydrated its head alone
    /// (`📓️api-stepped-document-load.md` §4): history verbs and queries answer `pure.history-unavailable`.
    pub fn history_unavailable(&self) -> bool {
        self.history_unavailable
    }

    /// 🫥️ Marks the document as hydrated head-only (`true`) or loaded with its history (`false`).
    pub fn set_history_unavailable(&mut self, unavailable: bool) {
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
    pub fn accepted_drafts(&self) -> protocol::HistoryInputDrafts {
        self.session.accepted.iter().map(|draft| (draft.target.mutation.clone(), draft.replacement.clone())).collect()
    }

    pub fn replace_editor(&mut self, editor: Option<TimeTravelEditor>) {
        self.editor = editor;
    }

    /// 🧹️ One bounded retirement unit of the first typed owner set still retiring; `None` when nothing retires. A member
    /// owner set the session already left is dropped once it is terminal-empty.
    pub fn retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let demand=self.pending_command_retirement_demands(body)?;
        if demand!=RetirementDemand::default(){return Ok(demand);}
        let demand=self.session_retirement_demands(body)?;
        if demand!=RetirementDemand::default(){return Ok(demand);}
        let demand=self.document.retirement_demands(body)?;
        if demand!=RetirementDemand::default(){return Ok(demand);}
        if let Some(owners)=self.member.as_ref().and_then(|member|member.owners.as_ref()){let demand=owners.retirement_demands(body)?;if demand!=RetirementDemand::default(){return Ok(demand);}}
        if let Some(owners)=self.retiring.last(){return if owners.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(owners.as_ref()),depth:1,..Default::default()})}else{owners.retirement_demands(body)};}
        if self.retiring.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.retiring.capacity()*size_of::<Box<dyn TimeTravelOwners>>(),depth:1,..Default::default()});}
        Ok(Default::default())
    }
    pub fn retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>, Fault> {
        if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if let Some(step)=self.pending_command_retirement_step(grant).map_err(ValueError::into_fault)?{return Ok(Some(step));}
        if let Some(step)=self.session_retirement_step(grant).map_err(ValueError::into_fault)?{return Ok(Some(step));}
        if let Some(step)=self.document.retire_step(grant)?{return Ok(Some(RetainedCloneStep::Progress(step.progress())));}
        if let Some(owners)=self.member.as_mut().and_then(|member|member.owners.as_mut()){if let Some(step)=owners.retire_step(grant)?{return Ok(Some(RetainedCloneStep::Progress(step.progress())));}}
        if let Some(owners)=self.retiring.last_mut(){
            if !owners.terminal_is_empty(){return owners.retire_step(grant).map(|step|step.map(|step|RetainedCloneStep::Progress(step.progress())));}
            let demand=self.retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
            if grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
            drop(self.retiring.pop());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})));
        }
        if self.retiring.capacity()!=0{let bytes=self.retiring.capacity()*size_of::<Box<dyn TimeTravelOwners>>();if grant.maximum_release_bytes<bytes||grant.maximum_depth==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}drop(std::mem::take(&mut self.retiring));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})));}
        Ok(None)
    }

    /// 🚪️ Document close, instance retirement or reload: the session vanishes with every draft (ephemeral state). The
    /// typed owners settle at the runtime's close step, which reaches their stores.
    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn terminal_is_empty(&self) -> bool {
        (!self.closing||self.session_retired) && self.command_receipt.is_none() && self.pending_command.is_none() && self.discard_cursor.is_none() && self.command_retirements.terminal_is_empty() && self.session_retirement.is_none() && !self.is_active() && self.session.accepted.capacity()==0 && self.session.pending.is_none() && self.session.report.is_none() && self.session.fault.is_none() && self.editor.is_none() && self.member.is_none() && self.authoring.is_none() && self.patch.is_none() && self.reprojection_fault.is_none() && self.deferred_history_row.is_none() && self.retiring.capacity()==0 && self.owners().all(TimeTravelOwners::terminal_is_empty)
    }

    /// 👉️ The first mutation, in replay order, whose outcome blocks finalizing (`MergePolicy::Normal`, the floor
    /// `ReplayReport::blocks_finalize` reads): what the status names as the next problem (design §22.2); `None` while the
    /// session's report does not block.
    pub fn next_problem(&self) -> Option<String> {
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
