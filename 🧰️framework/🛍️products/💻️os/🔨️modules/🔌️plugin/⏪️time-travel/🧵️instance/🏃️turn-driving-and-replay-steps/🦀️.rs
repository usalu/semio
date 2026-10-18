//! 🏃️ Turn driving and replay steps of the instance time-travel turns: the per-turn driver, the original review children and discard command, the patch preparation and the replay step. See [`super`].

use super::*;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> TimeTravelCtx<'ctx, A, M, X> {
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
        if self.time_travel_retire_step(grant)?.is_some() {
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
    pub(super) fn drive_original_review_children(&mut self,grant:RetainedCloneGrant)->Result<bool,Fault>{
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

    pub fn drive_original_discard_command(&mut self,grant:RetainedCloneGrant)->Result<bool,Fault>{
        if self.time_travel.command_receipt.is_some(){return Ok(true)}
        if grant.maximum_items==0&&self.time_travel.pending_command.is_some(){self.time_travel.command_receipt=Some((grant,Default::default()));return Ok(true)}
        let TimeTravelCtx{stores,time_travel,child_composition,..}=self;
        let (store,children,child_content_retirements,child_content_generation)=(&mut stores.store,&mut child_composition.children,&mut child_composition.child_content_retirements,&mut child_composition.child_content_generation);
        let receipt=(||->Result<Option<RetainedCloneProgress>,ValueError>{
            if let Some(cursor)=time_travel.discard_cursor.as_mut(){
                if time_travel.closing{cursor.begin_close()}
                if cursor.is_validating(){return cursor.validate(&time_travel.session,grant).map(|step|Some(step.progress()))}
                if cursor.prepared_stage().is_some(){
                    let receipt=match time_travel.member.as_mut(){None=>time_travel.document.publish_discard(store,cursor,&mut time_travel.session,&mut time_travel.editor,grant)?,Some(member)=>{let entry=children.member(member.key.borrowed()).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"Discard original member is absent"))?;let owners=member.owners.as_deref_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"Discard original member owners are absent"))?;entry.member.visit_member(semio_framework_plugin_time_travel::discard_decision::DiscardMemberPublication{owners,cursor,session:&mut time_travel.session,editor:&mut time_travel.editor,review_children:&mut member.review_children,children:&mut member.children,retirements:child_content_retirements,child_generation:child_content_generation,view_copy_bytes:size_of::<TimeTravelReviewChildren>()+size_of::<ChildContentRetirement>()+size_of::<u64>(),grant})?}};
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
    pub(super) async fn prepare_time_travel_patch(&mut self) -> Result<(), Fault> {
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
