//! 🎟️ Job admission and finalize turn of the instance tool run turns: the job admission, fault, step and completion, the revalidation completion and the finalize turn. See [`super`].

use super::*;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> ToolRunCtx<'ctx, A, M, X> {
    pub(super) async fn admit_tool_run_job(&mut self) -> Result<(), Fault> {
        let Some(entry) = selected_entry!(self.tool_runs) else { return Ok(()) };
        let (run, generation, state) = (entry.slot.run, entry.slot.generation, entry.slot.state);
        entry.port.wake();
        match self.tools.build_tool_run_job_slot(&*self.stores, &*self.child_composition, entry, ToolRunJobPurpose::Run) {
            Ok(Some(job)) => {
                selected_entry_mut!(self.tool_runs).expect("live slot").job = Some(job);
                if state == ToolRunState::Starting {
                    self.apply_tool_run_driver_event(ToolRunEvent::JobAdmitted { run, generation });
                }
            }
            Ok(None) | Err(_) => self.fault_tool_run(run, generation),
        }
        self.mark_tool_run_ui_dirty();
        Ok(())
    }

    pub(super) fn fault_tool_run(&mut self, run: u64, generation: u32) {
        if self.apply_tool_run_driver_event(ToolRunEvent::JobFault { run, generation }) == Some(ToolRunEffect::DiscardProvisional) {
            self.tool_runs.close_current_job();
            self.tool_runs.discard_provisional();
            if let Some(entry) = selected_entry_mut!(self.tool_runs) {
                entry.framework_step(ToolRunStepKind::Danger, TOOL_RUN_REASON_CONFLICT, &[ToolRunStepArg::Unsigned(0)]);
            }
            self.mark_tool_run_document_dirty();
        }
    }

    /// 🦶️ Drives the current job until the turn deadline, a pause, a single step or a terminal outcome.
    pub(super) fn step_tool_run_job(&mut self,deadline:u64,grant:RetainedCloneGrant)->Result<(),Fault>{
        if semio_framework_job::default_now_us().is_none_or(|now|now>=deadline){return Ok(())}
        if self.tool_runs.has_original_receipt(){return Ok(())}
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(())}
        let Some(entry)=selected_entry_mut!(self.tool_runs)else{return Ok(())};
        let (run,generation)=(entry.slot.run,entry.slot.generation);let single=entry.slot.state==ToolRunState::Paused;
        let Some(job)=entry.job.as_mut()else{return Ok(())};
        if job.cancel.is_none(){if let Some((cancel,progress))=semio_framework_job::CancelToken::admit_root(grant).map_err(FaultFrom::into_fault)?{job.cancel=Some(cancel);self.tool_runs.retain_original_receipt(grant,progress)?}return Ok(())}
        if job.authority.is_none(){if let Some((authority,progress))=semio_framework_job::JobPayloadAuthority::admit(job.operation,job.generation,grant).map_err(FaultFrom::into_fault)?{job.authority=Some(authority);self.tool_runs.retain_original_receipt(grant,progress)?}return Ok(())}
        if job.original_failure.is_some(){return Ok(())}
        if let Some(descriptor)=job.outcome.as_mut(){
            if job.outcome_received{if !descriptor.is_acknowledged(){let step=descriptor.acknowledge(grant);self.tool_runs.retain_original_receipt(grant,step.progress())?;return Ok(())}drop(job.outcome.take());job.outcome_received=false;self.tool_runs.retain_original_receipt(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;return Ok(())}
            let kind=descriptor.kind();
            let view=job.borrow_outcome().map_err(FaultFrom::into_fault)?;
            match view{Some(semio_framework_job::JobOutcomeView::PreviewReady{..}|semio_framework_job::JobOutcomeView::CheckpointReady{..}|semio_framework_job::JobOutcomeView::Fault{..})=>return Ok(()),Some(semio_framework_job::JobOutcomeView::Complete{state,output,..})if state.is_some()||output.is_some()=>return Ok(()),Some(_)=>{},None=>return Ok(())}
            job.outcome_received=true;let purpose=job.purpose;
            self.tool_runs.retain_original_receipt(grant,RetainedCloneProgress{copied_items:1,..Default::default()})?;
            match kind{semio_framework_job::JobOutcomeKind::Complete=>{selected_entry_mut!(self.tool_runs).unwrap().settle_refold();if purpose==ToolRunJobPurpose::Revalidate{self.complete_tool_run_revalidation()}else{self.complete_tool_run_job(run,generation)}},semio_framework_job::JobOutcomeKind::Cancelled|semio_framework_job::JobOutcomeKind::Fault=>self.fault_tool_run(run,generation),_=>{}}
            return Ok(())
        }
        let fuel=if single{1}else{semio_framework_job::INTERACTIVE_LANE_FUEL};let budget=semio_framework_job::StepBudget::new(fuel,deadline,grant);
        let mut ownership=Default::default();let mut verdict=None;
        let interactive:&mut dyn semio_framework_job::InteractiveJob=match &mut job.job{ToolRunJobHandle::Plain(job)=>job.as_mut(),ToolRunJobHandle::Retargetable(job)=>job.as_mut()};
        let mut context=semio_framework_job::StepContext::with_payload_authority(job.operation,job.generation,budget,job.cancel.as_ref().unwrap(),semio_framework_job::default_now_us,&mut job.preview_sequence,&mut ownership,job.authority.as_ref().unwrap()).map_err(FaultFrom::into_fault)?;
        let stepped=semio_framework_job::drive_step(interactive,&mut context,TOOL_RUN_JOB_SITE,semio_framework_job::InteractiveStage::InteractiveStep,&mut verdict).map(|loan|loan.map(semio_framework_job::JobOutcomeBorrow::into_descriptor));
        drop(context);
        let failed=match stepped{Ok(descriptor)=>{if single&&descriptor.as_ref().is_some_and(|descriptor|!matches!(descriptor.kind(),semio_framework_job::JobOutcomeKind::Yield|semio_framework_job::JobOutcomeKind::CheckpointReady{..})){entry.pending_step=false}job.outcome=descriptor;false},Err(original)=>{*job.original_failure=Some(original);true}};
        self.tool_runs.retain_original_receipt(grant,ownership)?;
        if failed{self.fault_tool_run(run,generation)}
        Ok(())
    }

    /// 🏁️ A finished job's completion — and, for a READ-ONLY run, its finalization too.
    ///
    /// 🪪️ `Complete` means "the job is done and the run awaits the finalize that publishes its
    /// provisional edits". A run that declares `mutating: false` authors no provisional edit at all, so
    /// there is nothing for a finalize to publish and nothing for a user to review: leaving it parked in
    /// `Complete` only holds the tool's single run slot open, and a `toolRunStart` against a non-terminal
    /// run is `ToolRunRejection::Busy`. That is how generation3d's read-only `previewEval` ran exactly
    /// ONCE per session — every later start, whether a poll's or a gesture's own, was refused by the
    /// finished run still sitting in the slot, and the 3d preview stopped re-evaluating
    /// (`📓️preview-rearm-after-inspector-edit-2026-09-14.md`, `📋️tool-run-contract.md` §2.2). Asking a
    /// read-only run's owner to dispatch a finalize it has nothing to finalize is a round trip the host
    /// can always lose: it happens on the host's own refresh cadence, and a run action that publishes
    /// nothing moves no UI that would provoke the next pass.
    fn complete_tool_run_job(&mut self, run: u64, generation: u32) {
        let resident = selected_entry_mut!(self.tool_runs).is_some_and(|entry| entry.definition.mutating && entry.job.as_mut().is_some_and(|job| job.retargetable().is_some()));
        if resident {
            selected_entry_mut!(self.tool_runs).expect("a resident job has a slot").settle_refold();
        } else {
            self.tool_runs.close_current_job();
        }
        self.apply_tool_run_driver_event(ToolRunEvent::JobComplete { run, generation });
        if let Some(entry) = selected_entry_mut!(self.tool_runs) {
            entry.pending_step = false;
        }
        if selected_entry!(self.tool_runs).is_some_and(|entry| !entry.definition.mutating && entry.slot.state == ToolRunState::Complete) {
            let (run, generation) = self.tool_runs.slot().map(|slot| (slot.run, slot.generation)).expect("a complete run has a slot");
            if let Some(ToolRunEffect::BeginFinalize) = self.apply_tool_run_driver_event(ToolRunEvent::Finalize { run, generation }) {
                if let Some(entry) = selected_entry_mut!(self.tool_runs) {
                    entry.finalize = Some(ToolRunFinalize { phase: ToolRunFinalizePhase::Pending, publication: None, ingress:Default::default(),rejected:false,retracted: 0, published: false });
                }
            }
        }
        self.mark_tool_run_document_dirty();
    }

    fn complete_tool_run_revalidation(&mut self) {
        self.tool_runs.close_current_job();
        let Some(entry) = selected_entry_mut!(self.tool_runs) else { return };
        let (run, generation) = (entry.slot.run, entry.slot.generation);
        let retracted = entry.finalize.as_ref().map_or(0, |finalize| finalize.retracted);
        if retracted == 0 {
            if let Some(finalize) = entry.finalize.as_mut() {
                finalize.phase = ToolRunFinalizePhase::Publishing;
            }
            return;
        }
        entry.conflicts = entry.conflicts.saturating_add(retracted);
        entry.framework_step(ToolRunStepKind::Danger, TOOL_RUN_REASON_CONFLICT, &[ToolRunStepArg::Unsigned(u64::from(retracted))]);
        let finalize = entry.finalize.take();
        self.tool_runs.retire_owners(None, Vec::new(), finalize);
        self.apply_tool_run_driver_event(ToolRunEvent::RevalidationConflicts { run, generation });
        self.mark_tool_run_document_dirty();
    }

    /// 🏁️ Finalize (§2.7.4): freshness refold on the head, optional revalidate job, one outbound batched `Edit`.
    pub(super)  async fn finalize_tool_run_turn(&mut self, deadline: u64,grant:RetainedCloneGrant,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,receiving:&mut semio_framework_value::NativeDecodeControl<'_>) -> Result<(), Fault> {
        let store_generation = self.stores.store.generation();
        let member_generation = self.shared().tool_run_member_generation();
        let entry = selected_entry_mut!(self.tool_runs).expect("finalizing slot");
        let phase = entry.finalize.as_ref().map_or(ToolRunFinalizePhase::Pending, |finalize| finalize.phase);
        let moved = match (entry.member.as_ref(), member_generation) {
            (Some(member), Some(generation)) => member.base_generation != generation,
            (Some(_), None) => false,
            (None, _) => entry.base_generation != store_generation,
        };
        let republish = matches!(phase, ToolRunFinalizePhase::Pending | ToolRunFinalizePhase::Revalidating) || (phase == ToolRunFinalizePhase::Publishing && entry.member.is_some());
        if moved && republish {
            entry.replace_base(self.stores.store.snapshot_owner());
            entry.base_generation = store_generation;
            entry.begin_refold(true);
            if let (Some(member), Some(generation)) = (entry.member.as_mut(), member_generation) {
                member.base_generation = generation;
                member.rebase = true;
                member.retired_emits.extend(member.emit.take());
                member.retired_emission_owners.extend(member.emission_owner.take());
            }
            entry.framework_step(ToolRunStepKind::Warning, TOOL_RUN_REASON_REBASING, &[]);
            if let Some(finalize) = entry.finalize.as_mut() {
                finalize.phase = ToolRunFinalizePhase::Pending;
                finalize.retracted = 0;
            }
            self.tool_runs.close_current_job();
            return Ok(());
        }
        if self.refold_tool_run(deadline) {
            return Ok(());
        }
        match phase {
            ToolRunFinalizePhase::Pending => {
                if selected_entry!(self.tool_runs).is_some_and(|entry| entry.job.as_ref().is_some_and(|job| job.purpose == ToolRunJobPurpose::Run)) {
                    self.tool_runs.close_current_job();
                    return Ok(());
                }
                let entry = selected_entry!(self.tool_runs).expect("finalizing slot");
                let (run, generation) = (entry.slot.run, entry.slot.generation);
                let next = if entry.definition.revalidate_job.is_some() && !(entry.provisional.is_empty() && entry.member.as_ref().is_none_or(|member| member.ops.is_empty())) {
                    match self.tools.build_tool_run_job_slot(&*self.stores, &*self.child_composition, entry, ToolRunJobPurpose::Revalidate) {
                        Ok(Some(job)) => {
                            selected_entry_mut!(self.tool_runs).expect("finalizing slot").job = Some(job);
                            ToolRunFinalizePhase::Revalidating
                        }
                        Ok(None) | Err(_) => return self.reject_tool_run_publication(run, generation),
                    }
                } else {
                    ToolRunFinalizePhase::Publishing
                };
                if let Some(finalize) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.finalize.as_mut()) {
                    finalize.phase = next;
                }
                Ok(())
            }
            ToolRunFinalizePhase::Revalidating => self.step_tool_run_job(deadline,grant),
            ToolRunFinalizePhase::Publishing => self.publish_tool_run(deadline,grant,identity,receiving).await,
            ToolRunFinalizePhase::Closing => self.close_tool_run_publication(grant),
        }
    }

}
