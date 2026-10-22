//! 🧹️ Owners and retirement of the instance time-travel turns: the typed store command, the commit, the owners settle, the child refresh, the member subject retirement and the retire, close, defer and dismiss steps. See [`super`].

use super::*;
use semio_framework::Fault;
use semio_framework::FaultOrigin;
use semio_framework::FaultCode;
use semio_framework_plugin_child::ChildContentRetirement;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_value_retirement::retirement::controlled::ControlledRetirement;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TimeTravelCtx<'ctx, A, M, X> {
    /// 📨️ Runs one typed command on the store the session edits: the document's own, or the composed member's through
    /// its typed visitor (design §12).
    pub(super)  async fn time_travel_run(&mut self, command: TimeTravelStoreCommand,receiving:Option<&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>>) -> Result<TimeTravelStoreOutput, Fault> {
        let TimeTravelCtx { stores, time_travel, child_composition, .. } = self;
        let (store, children) = (&mut stores.store, &mut child_composition.children);
        match time_travel.member.as_mut() {
            None => time_travel.document.run(store, command,receiving).await,
            Some(member) => {
                let key = member.key.clone();
                let entry = children.member_mut(key.borrowed()).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new(TIME_TRAVEL_MEMBER_GONE_CODE), format!("the composed member {} a history edit targets is gone", member.store())))?;
                entry.member.visit_member_mut(TimeTravelMemberRun { owners: &mut member.owners, command,receiving }).await
            }
        }
    }

    /// 🪪️ Finalizes the same retained document or composed member replay.
    pub(super) async fn time_travel_commit(&mut self, drafts: protocol::HistoryInputDrafts, finalization: store::HistoryFinalization, actor: Option<&str>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<TimeTravelCommit, Fault> {
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
    pub(super) fn settle_time_travel_owners(&mut self) -> Result<(), Fault> {
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
    pub(super) async fn refresh_time_travel_children(&mut self) -> Result<(), Fault> {
        let Some(member) = self.time_travel.member.as_ref() else { return Ok(()) };
        let (key, dialect) = (member.key.clone(), member.dialect.clone());
        let stage = self.time_travel.session.stage;
        let next = match self.time_travel_run(TimeTravelStoreCommand::Read(stage),None).await? {
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

    pub fn time_travel_retire_step(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneStep>,Fault> {
        if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if self.time_travel.document.discard_issuer_missing(){
            if grant.maximum_depth==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
            let factory=self.stores.store.owned_mutation_retirement_factory().map_err(ValueError::into_fault)?;self.time_travel.document.adopt_discard_issuer(Arc::clone(factory));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})));
        }
        if let Some(step)=self.time_travel.retire_step(grant)?{return Ok(Some(step))}self.member_subject_retirement_step(grant)
    }

    pub fn time_travel_close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep,Fault> {
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.time_travel.has_command_receipt(){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if !self.time_travel.closing{
            let demand=self.shared().time_travel_retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
            if !protocol::retained_mutation_demand_fits(demand,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}
            self.time_travel.begin_close();
            self.time_travel.document.cancel_prepared_command();if let Some(owners)=self.time_travel.member.as_mut().and_then(|member|member.owners.as_deref_mut()){owners.cancel_prepared_command()}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
        }
        if self.drive_original_discard_command(grant)?{let (original,receipt)=self.time_travel.take_command_receipt().expect("close owns its original command turn receipt");assert_eq!(original,grant);return Ok(RetainedCloneStep::Progress(receipt))}
        if self.drive_original_review_children(grant)?{let (original,receipt)=self.time_travel.take_command_receipt().expect("close owns its original review view turn receipt");assert_eq!(original,grant);return Ok(RetainedCloneStep::Progress(receipt))}
        if let Some(step)=self.time_travel_retire_step(grant)?{return Ok(RetainedCloneStep::Progress(step.progress()));}
        self.shared().time_travel_retirement_demands(grant.maximum_copy_bytes).map_err(ValueError::into_fault)?;
        Ok(if self.time_travel.terminal_is_empty(){RetainedCloneStep::Complete(Default::default())}else{RetainedCloneStep::Progress(Default::default())})
    }

    /// 🧾️ Holds back the history row of `action`, a history step whose replay the document store deferred (gap N17): it
    /// is recorded when the store adopts the step and dropped with the step when it is cancelled or refused, so a cancelled
    /// step leaves zero trace in the history too; the body shows the step replaying at once.
    pub fn defer_history_step_row(&mut self, action: &str) {
        self.time_travel.deferred_history_row = Some(action.to_string());
        self.note_time_travel_changed(false, false);
    }

    /// 🧹️ Dismisses the row of a refused history-change adoption (nothing replays any more) once the user dispatches anything
    /// but a view or interaction verb (audit W2A-8): the refusal was shown, the next change starts clean; the history patch
    /// carries the change.
    pub fn dismiss_refused_reprojection(&mut self, action: &str) {
        let passive = matches!(self.tools.declared_action_kind(action), Some(ActionKind::View | ActionKind::Interaction)) || INTERACTION_ACTION_IDS.contains(&action);
        if passive || self.time_travel.reprojection_fault.is_none() || self.stores.store.reprojection_progress().is_some() {
            return;
        }
        self.time_travel.reprojection_fault = None;
        self.note_time_travel_changed(false, false);
    }

}
