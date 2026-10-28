//! 📤️ Publication and member refresh of the instance tool run turns: the publication rejection, the run publication, the finalize owner and publication close and the composed member refresh, settle and publication. See [`super`].

use super::*;
use semio_framework::Fault;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> ToolRunCtx<'ctx, A, M, X> {
    pub(super) fn reject_tool_run_publication(&mut self, run: u64, generation: u32) -> Result<(), Fault> {
        let Some(entry) = selected_entry_mut!(self.tool_runs) else { return Ok(()) };
        entry.framework_step(ToolRunStepKind::Danger, TOOL_RUN_REASON_CONFLICT, &[ToolRunStepArg::Unsigned(entry.provisional.len() as u64)]);
        if let Some(finalize)=entry.finalize.as_mut(){finalize.phase=ToolRunFinalizePhase::Closing;finalize.rejected=true;}
        self.tool_runs.close_current_job();
        let _=(run,generation);
        self.mark_tool_run_ui_dirty();
        Ok(())
    }

    /// 📤️ Publishes the original decoded operations through the installed typed Store factory.
    pub(super) async fn publish_tool_run(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(())}
        let grant=cx.retained_grant();
        let entry=selected_entry_mut!(self.tool_runs).expect("finalizing original slot");
        let(run,generation)=(entry.slot.run,entry.slot.generation);
        let finalize=entry.finalize.as_mut().expect("original finalize owner");
        if entry.member.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"member finalization requires its original composite publication owner").into())}
        if finalize.publication.is_none(){
            if entry.provisional.is_empty(){let d=RetirementDemand{copy_bytes:size_of::<ToolRunFinalizePhase>(),depth:1,..Default::default()};if cx.retained_work_can_enter(d){finalize.phase=ToolRunFinalizePhase::Closing;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);}return Ok(())}
            if !self.stores.store.parts_ref().log.operation_wire.as_ref().is_some_and(|wire|wire.original_message_admission.is_some()){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original ToolRun publication requires its installed native causal encoder").into())}
            if entry.actor.as_str()!=self.stores.store.local_actor_id().0.as_str(){return self.reject_tool_run_publication(run,generation).map_err(Into::into)}
            if finalize.ingress.actor.is_none(){if let Some(p)=finalize.ingress.admit_actor(&entry.actor,grant)?{cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);}return Ok(())}
            if finalize.ingress.transaction.is_none(){let clock=semio_framework_tool_machine::authoring_clock(run);if let Some(p)=finalize.ingress.admit_transaction(&entry.actor,&clock,self.app.instance_id().await,&entry.tool_id,grant)?{cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);}return Ok(())}
            let mut group=[0;32];let group=entry.identity.id.group_id_into(&mut group);
            let admitted=self.stores.store.admit_apply_batch(cx.operation(),entry.base_generation,self.stores.store.content_revision(),&mut finalize.ingress.actor,&mut entry.provisional,HistoryLane::Document,self.stores.artifact_one_item_factory.as_ref(),&mut finalize.ingress.transaction,None,Some(group),true,grant);
            match admitted{Ok(Some((publication,p)))=>{finalize.publication=Some(publication);cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);},Ok(None)=>{},Err(error)=>return Err(error.into())}return Ok(())
        }
        if !self.ingest.reclaim_document_snapshot_read_returns(&*self.stores).map_err(semio_framework_job::OriginalTurnFailure::from)?{return Ok(())}
        let entry=selected_entry_mut!(self.tool_runs).expect("finalizing original slot");
        let publication=entry.finalize.as_mut().unwrap().publication.as_mut().unwrap();
        let result=cx.with_native_receiving(|native|self.stores.store.advance_apply_batch(publication,grant.maximum_copy_bytes,store::os_store::one_item::ArtifactStoreOneItemGrant{maximum_items:grant.maximum_items.min(1),maximum_copy_bytes:grant.maximum_copy_bytes,maximum_capacity_bytes:grant.maximum_capacity_bytes,maximum_release_bytes:grant.maximum_release_bytes,maximum_depth:grant.maximum_depth},native));
        use store::os_store::one_item_publication::ArtifactStoreOneItemAdvance as Advance;
        let p=match &result{Ok(Advance::Published(_,p)|Advance::PreparationProgress(_,p))=>*p,Err(store::os_vcs::VcsError::InverseRefused(error))=>error.retained_progress(),_=>Default::default()};
        if let Some(issued)=cx.receiving_issued_grant(){cx.consume_receiving_retained(issued,p)?;}cx.consume_fuel(p.copied_items as u64);
        match result{
            Ok(Advance::Published(..)|Advance::AwaitingAck(_))=>{
                if publication.outbound{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original ToolRun publication requires its committed native causal intent").into())}
                if cx.should_yield(){return Ok(())}
                let outbound=self.stores.store.advance_original_outbound(cx.retained_grant());
                let p=match &outbound{Ok(Some((_,p)))=>*p,Err(error)=>error.retained_progress(),_=>Default::default()};
                cx.consume_retained(p)?;cx.consume_fuel(p.copied_items as u64);
                if let Err(error)=outbound{return Err(error.into())}
                if p!=Default::default(){return Ok(())}
                let coordination=size_of::<ToolRunFinalizePhase>()+size_of::<bool>();let mut d=publication.acknowledgement_demands();d.copy_bytes=d.copy_bytes.checked_add(coordination).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original ToolRun acknowledgement quote overflow"))?;
                if !cx.retained_work_can_enter(d){return Ok(())}
                let grant=cx.retained_grant();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-coordination,..grant};
                let Some(p)=publication.acknowledge_granted(child)else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original ToolRun acknowledgement lost its prepared publication").into())};
                cx.consume_retained(p.checked_add(RetainedCloneProgress{copied_bytes:coordination,..Default::default()})?)?;cx.consume_fuel(1);
                let finalize=entry.finalize.as_mut().unwrap();finalize.published=true;finalize.phase=ToolRunFinalizePhase::Closing;self.mark_tool_run_document_dirty();
            },
            Err(store::os_vcs::VcsError::InverseRefused(error))=>return Err(error.into()),
            Err(error)=>return Err(error.into_fault().into()),
            _=>{},
        }Ok(())
    }

    /// 🎨️ The finalize SUCCEEDED, so the provisional ops are released — but the run's payload is
    /// its result, not provisional work, and its declared windows keep rendering it once the run
    /// is `Finalized`. See `ToolRunLedger::release_provisional`.
    pub(super) fn close_tool_run_finalize_owner(&mut self,grant:RetainedCloneGrant)->Result<bool,Fault>{
        if self.tool_runs.has_original_receipt(){return Ok(false);}
        let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
        let Some(finalize)=entry.finalize.as_mut()else{return Ok(true)};
        if let Some(publication) = finalize.publication.as_mut() {
            if let Some(p)=self.stores.store.prepare_apply_batch_close(publication,grant).map_err(FaultFrom::into_fault)?{self.tool_runs.retain_original_receipt(grant,p)?;return Ok(false);}
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
    pub(super)  fn refresh_tool_run_member(&mut self, deadline: u64,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<bool, Fault> {
        let Some(mut member) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.member.take()) else { return Ok(false) };
        let mut progress=RetainedCloneProgress::default();
        let outcome = self.child_composition.refresh_member_overlay(&mut member, deadline,grant,receiving,&mut progress);
        let entry = selected_entry_mut!(self.tool_runs).expect("a member run keeps its slot");
        entry.member = Some(member);
        self.tool_runs.retain_original_receipt(grant,progress)?;
        let entry=selected_entry_mut!(self.tool_runs).expect("a member run keeps its slot");
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
    pub(super)  fn settle_tool_run_member(&mut self, deadline: u64,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<(), Fault> {
        let Some(mut member) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.member.take()) else { return Ok(()) };
        let mut progress=RetainedCloneProgress::default();
        let outcome = if self.child_composition.children.get(&(member.slot.clone(), member.child_id.clone())).is_some() {
            self.child_composition.refresh_member_overlay(&mut member, deadline,grant,receiving,&mut progress).map(|_| ())
        } else {
            member.rebase = false;
            let stale = std::mem::take(&mut member.stale);
            self.child_composition.retire_tool_run_views(stale, &mut member.stale)
        };
        selected_entry_mut!(self.tool_runs).expect("a member run keeps its slot").member = Some(member);
        self.tool_runs.retain_original_receipt(grant,progress)?;
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
    async fn publish_tool_run_member(&mut self, deadline: u64,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<(), Fault> {
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
