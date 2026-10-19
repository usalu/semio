//! 📤️ Publication and member refresh of the instance tool run turns: the publication rejection, the run publication, the finalize owner and publication close and the composed member refresh, settle and publication. See [`super`].

use super::*;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> ToolRunCtx<'ctx, A, M, X> {
    pub(super) fn reject_tool_run_publication(&mut self, run: u64, generation: u32) -> Result<(), Fault> {
        let Some(entry) = selected_entry_mut!(self.tool_runs) else { return Ok(()) };
        entry.framework_step(ToolRunStepKind::Danger, TOOL_RUN_REASON_CONFLICT, &[ToolRunStepArg::Unsigned(entry.provisional.len() as u64)]);
        if let Some(finalize)=entry.finalize.as_mut(){finalize.phase=ToolRunFinalizePhase::Closing;finalize.rejected=true;}
        self.tool_runs.close_current_job();
        let _=(run,generation);
        self.mark_tool_run_ui_dirty();
        Ok(())
    }

    /// ♻️ Each folded op leaves the root it was prepared against returned to the Store; reclaim it before the
    /// next fold, or a large finalize keeps one whole document per op (see `reclaim_document_snapshot_read_returns`).
    pub(super)  async fn publish_tool_run(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(), Fault> {
        if selected_entry!(self.tool_runs).is_some_and(|entry| entry.member.is_some()) {
            return self.publish_tool_run_member(deadline,grant,receiving).await;
        }
        let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        if entry.provisional.is_empty()&&entry.finalize.as_ref().is_none_or(|finalize|finalize.publication.is_none()) {
            entry.finalize.as_mut().expect("finalize owner").phase = ToolRunFinalizePhase::Closing;
            return Ok(());
        }
        if entry.finalize.as_ref().is_some_and(|finalize| finalize.publication.is_none()) {
            if entry.actor.as_str() != self.stores.store.local_actor_id().0.as_str() {
                return self.reject_tool_run_publication(run, generation);
            }
            let finalize=entry.finalize.as_mut().expect("original finalize owner");
            if finalize.ingress.actor.is_none(){
                let progress=finalize.ingress.admit_actor(&entry.actor,grant).map_err(FaultFrom::into_fault)?;
                if let Some(progress)=progress{self.tool_runs.retain_original_receipt(grant,progress)?;}return Ok(());
            }
            if finalize.ingress.transaction.is_none(){
                let clock=semio_framework_tool_machine::authoring_clock(run);
                let progress=finalize.ingress.admit_transaction(&entry.actor,&clock,self.app.instance_id().await,&entry.tool_id,grant).map_err(FaultFrom::into_fault)?;
                if let Some(progress)=progress{self.tool_runs.retain_original_receipt(grant,progress)?;}return Ok(());
            }
            match self.stores.store.admit_apply_batch(semio_framework_job::allocate_operation_id(),entry.base_generation,self.stores.store.content_revision(),&mut finalize.ingress.actor,&mut entry.provisional,HistoryLane::Document,self.stores.artifact_one_item_factory.as_ref(),&mut finalize.ingress.transaction,None,true,grant) {
                Ok(Some((publication,progress)))=>{finalize.publication=Some(publication);self.tool_runs.retain_original_receipt(grant,progress)?;return Ok(());},
                Ok(None)=>return Ok(()),
                Err(store::os_vcs::VcsError::InverseRefused(error))=>{self.tool_runs.retain_original_receipt(grant,error.retained_progress())?;return Err(error.into_fault())},
                Err(_) => return self.reject_tool_run_publication(run, generation),
            }
        }
        {
            if !self.ingest.reclaim_document_snapshot_read_returns(&*self.stores)? {
                if semio_framework_job::default_now_us().is_none_or(|now| now >= deadline) {
                    return Ok(());
                }
                return Ok(());
            }
            let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
            let publication = entry.finalize.as_mut().and_then(|finalize| finalize.publication.as_mut()).expect("admitted publication");
            match self.stores.store.advance_apply_batch(publication, store::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:grant.maximum_items.min(1),maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth},receiving) {
                Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Published(_, _)) => {
                    let group_id = entry.identity.id.group_id();
                    let tool_id = entry.tool_id.clone();
                    let actor = entry.actor.clone();
                    self.stores.store.stamp_tail_group_id(&group_id).await.map_err(|error| error.into_fault())?;
                    let publication = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.finalize.as_mut()).and_then(|finalize| finalize.publication.as_mut()).expect("published publication");
                    self.stores.store.flush_published_apply_batch(publication).await.map_err(|error| error.into_fault())?;
                    publication.acknowledge();
                    let edit_id = self.stores.store.envelope().vcs.edits.last().map(|edit| edit.id.clone());
                    let label = self.tools.registry.tool_run(&tool_id).map(|(label, _)| label.clone());
                    self.tools.record_command(&mut *self.history, &tool_id, ActionKind::Mutation, label, edit_id, Vec::new(), None);
                    ({ let __a0 = &ActionMeta { actor, instance_id: self.tools.live_runtime_instance_id.unwrap_or(1), view_state: None }; self.as_render().revalidate_interaction_on_document_change(__a0, identity).await })?;
                    let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
                    let finalize = entry.finalize.as_mut().expect("finalize owner");
                    finalize.published = true;
                    finalize.phase = ToolRunFinalizePhase::Closing;
                    return Ok(());
                }
                Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::Blocked) => return Ok(()),
                Ok(store::os_store::one_item_publication::ArtifactStoreOneItemAdvance::PreparationProgress(_,progress))=>{self.tool_runs.retain_original_receipt(grant,progress)?;return Ok(());},
                Ok(_) => {}
                Err(store::os_vcs::VcsError::InverseRefused(error))=>{self.tool_runs.retain_original_receipt(grant,error.retained_progress())?;return Err(error.into_fault())},
                Err(_) => return self.reject_tool_run_publication(run, generation),
            }
            return Ok(());
        }
    }

    /// 🎨️ The finalize SUCCEEDED, so the provisional ops are released — but the run's payload is
    /// its result, not provisional work, and its declared windows keep rendering it once the run
    /// is `Finalized`. See `ToolRunLedger::release_provisional`.
    pub(super) fn close_tool_run_finalize_owner(&mut self,grant:RetainedCloneGrant)->Result<bool,Fault>{
        if self.tool_runs.has_original_receipt(){return Ok(false);}
        let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
        let Some(finalize)=entry.finalize.as_mut()else{return Ok(true)};
        if let Some(publication) = finalize.publication.as_mut() {
            if publication.terminal_is_empty(){
                let copy=size_of::<Option<store::os_store::one_item_publication::ArtifactStoreBatchPublication<A::Snapshot,A::Mutation>>>();
                if !tool_grant_funds(grant,RetirementDemand{copy_bytes:copy,depth:1,..Default::default()}){return Ok(false);}
                drop(finalize.publication.take());self.tool_runs.retain_original_receipt(grant,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;return Ok(false);
            }
            let mut demand=publication.retirement_demands(grant.maximum_copy_bytes).map_err(FaultFrom::into_fault)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"original ToolRun final publication depth overflow").into_fault())?;
            if !tool_grant_funds(grant,demand){return Ok(false);}
            publication.begin_close();
            let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=match publication.close_step(child){Ok(step)=>step,Err(error)=>{self.tool_runs.retain_original_receipt(grant,error.retained_progress())?;return Err(error.into_fault());}};
            let step=match admit_retained_clone_close(child,step,publication.terminal_is_empty(),"tool final publication"){Ok(step)=>step,Err(error)=>{self.tool_runs.retain_original_receipt(grant,error.retained_progress())?;return Err(error.into_fault());}};
            self.tool_runs.retain_original_receipt(grant,step.progress())?;return Ok(false);
        }
        if !finalize.ingress.terminal_is_empty(){let step=match finalize.ingress.close_step(grant){Ok(step)=>step,Err(error)=>{self.tool_runs.retain_original_receipt(grant,error.retained_progress())?;return Err(error.into_fault());}};self.tool_runs.retain_original_receipt(grant,step.progress())?;return Ok(false);}
        Ok(true)
    }

    pub(super) fn close_tool_run_publication(&mut self,grant:RetainedCloneGrant) -> Result<(), Fault> {
        if !self.close_tool_run_finalize_owner(grant)?{return Ok(());}
        let copy=size_of::<Option<ToolRunFinalize<A>>>();if !tool_grant_funds(grant,RetirementDemand{copy_bytes:copy,depth:1,..Default::default()}){return Ok(());}
        let entry=selected_entry_mut!(self.tool_runs).expect("finalizing original slot");let(run,generation)=(entry.slot.run,entry.slot.generation);let rejected=entry.finalize.as_ref().is_some_and(|finalize|finalize.rejected);drop(entry.finalize.take());self.tool_runs.retain_original_receipt(grant,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;
        if rejected{self.apply_tool_run_driver_event(ToolRunEvent::StoreRejected{run,generation});self.mark_tool_run_ui_dirty();return Ok(());}
        if self.apply_tool_run_driver_event(ToolRunEvent::PublicationComplete { run, generation }) == Some(ToolRunEffect::ReleaseProvisional) {
            let store_generation = self.stores.store.generation();
            let member_generation = self.shared().tool_run_member_generation();
            let head = self.stores.store.snapshot_owner();
            self.tool_runs.release_provisional(true);
            let entry = selected_entry_mut!(self.tool_runs).expect("finalized slot");
            entry.replace_base(head);
            let overlay = Arc::clone(&entry.base);
            entry.replace_overlay(overlay);
            entry.base_generation = store_generation;
            if let (Some(member), Some(generation)) = (entry.member.as_mut(), member_generation) {
                member.base_generation = generation;
            }
        }
        self.mark_tool_run_document_dirty();
        Ok(())
    }

    /// 🔁️ Brings the selected member run's composed read up to its provisional ops within the turn wall budget: retires the
    /// reads it replaced, folds the ops the overlay does not show yet (all of them after a rebase) and leases the overlay in
    /// the member's place once every op is shown. `true` while the fold still owns this turn — also while the retirement
    /// of the reads it replaced is saturated (retryable), so the job waits instead of leasing more; a fold that cannot
    /// apply its ops (they do not decode as the member's vocabulary) faults the run.
    pub(super)  fn refresh_tool_run_member(&mut self, deadline: u64,grant:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<bool, Fault> {
        let Some(mut member) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.member.take()) else { return Ok(false) };
        let outcome = self.child_composition.refresh_member_overlay(&mut member, deadline,grant,receiving);
        let entry = selected_entry_mut!(self.tool_runs).expect("a member run keeps its slot");
        entry.member = Some(member);
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        match outcome {
            Ok(Some((pending, conflicts, rebased))) => {
                entry.conflicts = entry.conflicts.saturating_add(conflicts);
                if !pending {
                    if rebased {
                        self.mark_tool_run_document_dirty();
                    } else {
                        self.mark_tool_run_ui_dirty();
                    }
                }
                Ok(pending)
            }
            Ok(None) => Ok(false),
            Err(fault) if fault.retryable => Ok(true),
            Err(_) => {
                self.fault_tool_run(run, generation);
                Ok(false)
            }
        }
    }

    /// 🪦️ A terminal member run holds no provisional op and shows none: the composed reads it replaced retire and its
    /// overlay rests on the member head again, so the run owes nothing. A member that is gone has no head to rest on; a
    /// saturated retirement ring (retryable) is retried on the next turn.
    pub(super)  fn settle_tool_run_member(&mut self, deadline: u64,grant:RetainedCloneGrant) -> Result<(), Fault> {
        let Some(mut member) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.member.take()) else { return Ok(()) };
        let outcome = if self.child_composition.children.get(&(member.slot.clone(), member.child_id.clone())).is_some() {
            self.child_composition.refresh_member_overlay(&mut member, deadline,grant,receiving).map(|_| ())
        } else {
            member.rebase = false;
            let stale = std::mem::take(&mut member.stale);
            self.child_composition.retire_tool_run_views(stale, &mut member.stale)
        };
        selected_entry_mut!(self.tool_runs).expect("a member run keeps its slot").member = Some(member);
        match outcome {
            Err(fault) if !fault.retryable => Err(fault),
            _ => Ok(()),
        }
    }

    /// 🏁️ A member run's finalize (§12, §20.14): its provisional ops — at most `TOOL_RUN_MEMBER_OPS_MAX` — decoded through
    /// the member's own vocabulary within the turn wall budget, then published as ONE composite group whose member edit
    /// carries the run's `TransactionRef` and group id: one history row whose mutations are the member's leaves, editable on
    /// that member store. A member that is gone or ops that do not decode reject the publication like a conflicting parent
    /// batch.
    async fn publish_tool_run_member(&mut self, deadline: u64,grant:RetainedCloneGrant,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(), Fault> {
        let app_instance_id = self.app.instance_id().await;
        let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        let member = entry.member.as_mut().expect("a member run");
        if member.ops.is_empty() {
            entry.finalize.as_mut().expect("finalize owner").phase = ToolRunFinalizePhase::Closing;
            return Ok(());
        }
        if member.emit.is_none(){
            let decoded = self.child_composition.children.get(&(member.slot.clone(), member.child_id.clone())).map(|child| child.member.visit_member(ToolRunMemberEmit { slot: &member.slot, child_id: &member.child_id, ops: &member.ops, emit: &mut member.emit, owner:&mut member.emission_owner, grant }));
            match decoded {
                Some(Ok((_,progress)))=>{self.tool_runs.retain_original_receipt(grant,progress)?;return Ok(());},
                _ => return self.reject_tool_run_publication(run, generation),
            }
        }
        let finalize=entry.finalize.as_mut().expect("original member finalizer");
        if finalize.ingress.transaction.is_none(){let clock=semio_framework_tool_machine::authoring_clock(run);let progress=finalize.ingress.admit_transaction(&entry.actor,&clock,app_instance_id,&entry.tool_id,grant).map_err(FaultFrom::into_fault)?;if let Some(progress)=progress{self.tool_runs.retain_original_receipt(grant,progress)?;}return Ok(());}
        let _=deadline;
        Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"ToolRun member retains its original emission and transaction until the mounted group issuer admits their unchanged full grant").into_fault())
    }

}
