//! 🎟️ Job admission and finalize turn of the instance tool run turns: the job admission, fault, step and completion, the revalidation completion and the finalize turn. See [`super`].

use super::*;
use semio_framework::Fault;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> ToolRunCtx<'ctx, A, M, X> {
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

    /// 🦶️ The mounted caller lends the same original source, decoder and page ledger to native tick receiving.
    pub(super) fn step_tool_run_job(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        if cx.should_yield(){return Ok(())}
        let Some(entry)=selected_entry_mut!(self.tool_runs)else{return Ok(())};
        if !entry.native.initialize(&entry.tool_id,entry.identity,entry.slot.state,cx)?{return Ok(())}
        let(run,generation)=(entry.slot.run,entry.slot.generation);
        let single=entry.slot.state==ToolRunState::Paused;
        let Some(job)=entry.job.as_mut()else{return Ok(())};
        if job.original_failure.is_some(){return Ok(())}
        if let Some(descriptor)=job.outcome.as_ref(){
            if job.outcome_received{
                let d=RetirementDemand{copy_bytes:size_of::<Option<semio_framework_job::JobOutcomeDescriptor>>(),depth:1,..Default::default()};
                semio_framework_job::close_original_retained_step(cx,d,|grant|{
                    let descriptor=job.outcome.as_mut().unwrap();
                    if !descriptor.is_acknowledged(){return Ok(RetainedCloneStep::Progress(descriptor.acknowledge(grant).progress()))}
                    job.outcome=None;job.outcome_received=false;
                    Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))
                })?;return Ok(())
            }
            let kind=descriptor.kind();
            let interactive:&dyn semio_framework_job::InteractiveJob=match &job.job{ToolRunJobHandle::Plain(job)=>job.as_ref(),ToolRunJobHandle::Retargetable(job)=>job.as_ref()};
            match interactive.borrow_outcome(descriptor)?{
                semio_framework_job::JobOutcomeView::PreviewReady{payload,..}=>{
                    let source=semio_framework_plugin_tool_run::original_native_tick::OriginalPreviewPages(payload);
                    if entry.native.receive_with_domain(&source,entry.identity,entry.slot.state,|tick,cx|{if entry.operations.ingest(tick,&self.stores.store,&mut entry.overlay,cx)?{Ok(Some(entry.operations.len()as u32))}else{Ok(None)}},cx)?{
                        let d=RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()};
                        if cx.retained_work_can_enter(d){job.outcome_received=true;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})?;cx.consume_fuel(1);self.mark_tool_run_ui_dirty();}
                    }
                    return Ok(())
                }
                semio_framework_job::JobOutcomeView::CheckpointReady{..}=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original checkpoint requires its retained checkpoint publication owner").into()),
                semio_framework_job::JobOutcomeView::Fault{..}=>return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original fault requires its retained diagnostic receiving owner").into()),
                semio_framework_job::JobOutcomeView::Complete{state,output,..}if state.is_some()||output.is_some()=>{},
                _=>{},
            }
            if !cx.retained_work_can_enter(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}){return Ok(())}
            job.outcome_received=true;let purpose=job.purpose;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()})?;cx.consume_fuel(1);
            match kind{semio_framework_job::JobOutcomeKind::Complete=>{if purpose==ToolRunJobPurpose::Revalidate{self.complete_tool_run_revalidation()}else{self.complete_tool_run_job(run,generation)}},semio_framework_job::JobOutcomeKind::Cancelled|semio_framework_job::JobOutcomeKind::Fault=>self.fault_tool_run(run,generation),_=>{}}
            return Ok(())
        }
        let fuel=if single{1}else{semio_framework_job::INTERACTIVE_LANE_FUEL};let deadline=cx.deadline_us();let mut verdict=None;
        let interactive:&mut dyn semio_framework_job::InteractiveJob=match &mut job.job{ToolRunJobHandle::Plain(job)=>job.as_mut(),ToolRunJobHandle::Retargetable(job)=>job.as_mut()};
        let stepped=cx.with_original_work_loan(fuel,deadline,|cx|{
            let outcome=semio_framework_job::drive_step(interactive,cx,TOOL_RUN_JOB_SITE,semio_framework_job::InteractiveStage::InteractiveStep,&mut verdict)?;
            if let Some(outcome)=outcome{job.outcome=Some(outcome.into_descriptor());}
            Ok(())
        });
        if single&&job.outcome.as_ref().is_some_and(|outcome|!matches!(outcome.kind(),semio_framework_job::JobOutcomeKind::Yield|semio_framework_job::JobOutcomeKind::CheckpointReady{..})){entry.pending_step=false;}
        if let Err(failure)=stepped{*job.original_failure=Some(failure);self.fault_tool_run(run,generation);}
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
    pub(super)  async fn finalize_tool_run_turn(&mut self,cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        let grant=cx.retained_grant();
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
        if self.refold_tool_run(cx)? {
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
                let entry=selected_entry_mut!(self.tool_runs).expect("finalizing original slot");
                if entry.member.is_none()&&!entry.operations.prepare_finalization(&self.stores.store,&mut entry.provisional,cx)?{return Ok(())}
                let next = if entry.definition.revalidate_job.is_some() && !(entry.provisional.is_empty() && entry.member.as_ref().is_none_or(|member| member.ops.is_empty())) {
                    match self.tools.build_tool_run_job_slot(&*self.stores, &*self.child_composition, entry, ToolRunJobPurpose::Revalidate) {
                        Ok(Some(job)) => {
                            selected_entry_mut!(self.tool_runs).expect("finalizing slot").job = Some(job);
                            ToolRunFinalizePhase::Revalidating
                        }
                        Ok(None) | Err(_) => return self.reject_tool_run_publication(run, generation).map_err(Into::into),
                    }
                } else {
                    ToolRunFinalizePhase::Publishing
                };
                if let Some(finalize) = selected_entry_mut!(self.tool_runs).and_then(|entry| entry.finalize.as_mut()) {
                    finalize.phase = next;
                }
                Ok(())
            }
            ToolRunFinalizePhase::Revalidating => self.step_tool_run_job(cx),
            ToolRunFinalizePhase::Publishing => self.publish_tool_run(cx).await,
            ToolRunFinalizePhase::Closing => {let entry=selected_entry_mut!(self.tool_runs).expect("closing original slot");if !entry.operations.terminal_is_empty(){entry.operations.close_step(&self.stores.store,cx)?;return Ok(())}self.close_tool_run_publication(grant).map_err(Into::into)},
        }
    }

}
