//! 🚪️ Ingress of the mounted artifact instance: framework-reserved job driving, constructors, store-constructor units, envelope ingress and decode, document archive genesis and merge, child members and packs.

use super::*;

impl<A: ArtifactApp, M: SpaceMember + MemberFactory + 'static> VcsArtifactApp<A, M> {
    #[cfg(test)]
    pub(crate) fn test_registered_tool_keys(&self) -> BTreeSet<(String, String)> {
        self.tool_jobs.keys().into_iter().map(|key| (key.controller_id, key.tool_id)).collect()
    }

    #[cfg(test)]
    pub(crate) fn test_framework_tool_registrations(&self) -> BTreeMap<String, (semio_framework::ToolFactoryKey, String, std::any::TypeId, &'static str, semio_framework::ToolExecutionContract)> {
        self.framework_tool_registrations
            .iter()
            .map(|(tool_id, registration)| (tool_id.clone(), (registration.key.clone(), registration.schema_id.clone(), registration.factory_type_id, registration.factory_type_name, registration.contract)))
            .collect()
    }

    pub fn tool_cancellation_handle(&self) -> &ToolCancellationHandle {
        &self.tool_cancellations
    }

    /// 🎟️ Seals one genuine publication slot and keyed lease before a reserved producer receives source owners.
    pub(super) fn admit_reserved_emit_producer(&mut self,verb:&str,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<FrameworkReservedCommitPermit>,semio_framework_job::OriginalTurnFailure>{
        let proof=self.qualified_tool_proof(verb)?;
        if !matches!(proof,QualifiedToolProof::FrameworkOwned(_)|QualifiedToolProof::AppOwned(_)){return Err(plugin_sdk_fault("reserved producer lacks its exact registered publication proof").into())}
        let Some((operation,lease))=self.admit_original_tool_lease(verb,meta,true,cx)?else{return Ok(None)};
        FrameworkReservedCommitPermit::new(operation,lease,proof.contract().max_output_bytes).map(Some).map_err(Into::into)
    }

    /// 🧷️ Clears only the original reserved publication identity without changing a foreign slot.
    pub(super) fn release_reserved_emit_slot(&mut self, operation: u64) {
        let reservation = &mut self.typed_operation_reservations[operation as usize % ARTIFACT_LIVE_OUTPUT_SLOTS];
        if *reservation == Some(operation) { *reservation = None; }
    }

    /// 🏁️ Returns a reserved producer's original slot and keyed cancellation authority after ordinary completion.
    pub(super) fn finish_reserved_emit_producer(&mut self, permit: FrameworkReservedCommitPermit) {
        self.release_reserved_emit_slot(permit.operation.operation.0);
        permit.finish();
    }

    /// 🔬️ Exercises real reserved slot admission without exporting private producer authority.
    #[cfg(test)]
    pub(crate) fn test_reserved_emit_admission_case(&mut self, occupied: usize, meta: &ActionMeta) -> bool {
        let before = self.typed_operation_reservations;
        for slot in 0..occupied { self.typed_operation_reservations[slot] = Some((slot + 128) as u64); }
        let original = self.typed_operation_reservations;
        let admitted = match self.admit_reserved_emit_producer("cut", meta) {
            Ok(permit) => {
                let id = permit.operation.operation.0;
                let slot = id as usize % ARTIFACT_LIVE_OUTPUT_SLOTS;
                assert_eq!(slot, occupied);
                assert_eq!(self.typed_operation_reservations[slot], Some(id));
                assert!(permit.lease.keyed);
                assert_eq!(permit.lease.key.operation_id, permit.operation.operation);
                assert_eq!(&self.typed_operation_reservations[..occupied], &original[..occupied]);
                self.release_reserved_emit_slot(id + ARTIFACT_LIVE_OUTPUT_SLOTS as u64);
                assert_eq!(self.typed_operation_reservations[slot], Some(id));
                self.finish_reserved_emit_producer(permit);
                assert_eq!(self.typed_operation_reservations, original);
                true
            }
            Err(_) => { assert_eq!(self.typed_operation_reservations, original); false }
        };
        self.typed_operation_reservations = before;
        admitted
    }

    /// 🧪️ Keeps cancelled worker completion tied to the actual original permit through handoff.
    #[cfg(test)]
    pub(crate) async fn test_reserved_emit_cancelled_worker_handoff(&mut self, meta: &ActionMeta) -> bool {
        let permit = self.admit_reserved_emit_producer("cut", meta).expect("original cut producer admission");
        let id = permit.operation.operation;
        permit.lease.cancel();
        let job = ArtifactReservedToolJob::new(FrameworkCutJob::new(Vec::new(), 0));
        match self.run_framework_reserved_job("cut", &[], 0, job, meta, None, Some(permit)).await {
            Ok(permit) => { assert_eq!(permit.operation.operation, id); assert_eq!(self.typed_operation_reservations[id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS], Some(id.0)); self.finish_reserved_emit_producer(permit); true }
            Err(_) => false,
        }
    }

    /// 🧫️ Observes busy original completion retention without exposing producer authority.
    #[cfg(test)]
    pub(crate) async fn test_reserved_emit_busy_completion_owner(&mut self, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> bool {
        let permit=self.admit_reserved_emit_producer("cut",meta).expect("fixture cut admission");
        let id=permit.operation.operation.0;
        let completion=ArtifactToolCompletion::<A>::new();
        let mut emit=Emit::default();emit.artifact_mutations=Vec::with_capacity(3);
        completion.complete(Ok(emit),EphemeralEmit::default()).expect("fixture original completion");
        let original=completion.inner.clone();
        let guard=original.lock().expect("fixture original lock");
        let result=self.commit_framework_clipboard_completion("cut",Some(completion),meta,permit, cx).await;
        let retained=result.is_ok()&&self.tool_operations.get(id).is_some_and(|owner|owner.completion.as_ref().is_some_and(|cell|cell.inner.same_cell(&original))&&owner.terminal_fault.is_some()&&owner.cancellation_lease.as_ref().is_some_and(|lease|lease.token.is_cancelled_now()));
        drop(guard);
        let mut original=semio_framework_value::retirement::controlled::ControlledRetirement::new(ArtifactToolCompletion::<A>{inner:original}).unwrap_or_else(|(_,original)|{let _original=std::mem::ManuallyDrop::new(original);panic!("original completion observation alias retirement was refused");});
        for _ in 0..100000 {
            if original.terminal_is_empty(){break;}
            let copy=original.next_copy_byte_demand().expect("original completion alias copy quote").max(4096);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:original.next_capacity_byte_demand(copy).expect("original completion alias capacity quote"),maximum_release_bytes:original.next_release_byte_demand().expect("original completion alias release quote"),maximum_depth:original.next_depth_demand().expect("original completion alias depth quote")};
            assert!(original.step(grant).expect("funded original completion alias closure").progress().fits(grant));
        }
        assert!(original.terminal_is_empty(),"original completion observation alias remains in custody");
        if !retained {self.release_reserved_emit_slot(id);}
        retained
    }

    /// 🛡️ Keeps producer admission failures tied to the original reserved identity.
    pub(super) async fn run_framework_reserved_job<J: semio_framework_job::InteractiveJob + Send + 'static>(
        &mut self, verb: &str, raw: &[u8], decoded_items: usize, job: J, meta: &ActionMeta,
        supplied_admission: Option<semio_framework::ToolWireAdmission>,
        supplied_authority: FrameworkReservedCommitPermit,
     cx:&mut semio_framework_job::StepContext<'_>) -> Result<FrameworkReservedCommitPermit, (semio_framework_job::OriginalTurnFailure,FrameworkReservedCommitPermit)> {
        let mut permit=supplied_authority;
        match self.run_framework_reserved_job_inner(verb, raw, decoded_items, job, meta, supplied_admission, &permit,cx).await {
            Ok(()) => Ok(permit),
            Err(fault) => { permit.lease.cancel(); Err((fault,permit)) },
        }
    }

    async fn run_framework_reserved_job_inner<J: semio_framework_job::InteractiveJob + Send + 'static>(&mut self,verb:&str,raw:&[u8],decoded_items:usize,job:J,meta:&ActionMeta,supplied_admission:Option<semio_framework::ToolWireAdmission>,supplied_authority:&FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        if supplied_authority.lease.keyed && (self.typed_operation_reservations[supplied_authority.operation.operation.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS]!=Some(supplied_authority.operation.operation.0)||supplied_authority.lease.key.operation_id!=supplied_authority.operation.operation){return Err(plugin_sdk_fault("reserved producer lost its original keyed slot admission").into());}
        self.drive_framework_reserved_worker(verb,raw,decoded_items,job,meta,supplied_admission,supplied_authority.operation,supplied_authority.lease.cancel_token(),false,cx).await
    }

    async fn drive_framework_reserved_worker<J: semio_framework_job::InteractiveJob + Send + 'static>(
        &mut self,
        verb: &str,
        raw: &[u8],
        decoded_items: usize,
        job: J,
        _meta: &ActionMeta,
        supplied_admission: Option<semio_framework::ToolWireAdmission>,
        operation: semio_framework_job::Operation,
        cancel: &semio_framework_job::CancelToken,
        mounted_owner: bool,
     cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        let proof = self.qualified_tool_proof(verb)?;
        if !matches!(proof, QualifiedToolProof::FrameworkOwned(_) | QualifiedToolProof::AppOwned(_)) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-proof"), format!("framework route '{verb}' did not resolve its route-specific factory"))).into());
        }
        let key = proof.key();
        let admission = match supplied_admission {
            Some(admission) => admission,
            None => self.tool_jobs.admit_exact_wire(key.controller_id.clone(), key.tool_id.clone(), proof.schema_id(), raw).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?,
        };
        if decoded_items > proof.contract().max_decoded_items || !proof.admits::<A>(&admission) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.reserved-admission"), format!("framework route '{verb}' failed its exact owner/factory/schema/contract admission"))).into());
        }
        let generation = operation.generation;
        if mounted_owner && !self.tool_operations.get(operation.operation.0).is_some_and(|owner| owner.operation.operation == operation.operation && owner.operation.base_revision == operation.base_revision && owner.operation.generation == operation.generation) { return Err(plugin_sdk_fault("reserved producer lost its original mounted operation authority").into()); }
        let operation_id = operation.operation;
        let dispatch = self
            .tool_jobs
            .dispatch(semio_framework::ToolOperationSpec::new(key.controller_id, key.tool_id, proof.schema_id(), job, operation))
            .map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;

        let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
        let pool = semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores));
        let step_grant = self.mounted_policy.preparation;
        let close_grant = self.mounted_policy.close;
        let source=(cx.operation(),cx.generation());
        let mut pending=semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatch.job,source);
        let config=semio_framework_job_worker_authority::BatchDriveConfig{retained:step_grant,site:"plugin_framework_reserved_route",stage:semio_framework_job::InteractiveStage::InteractiveStep,fuel_per_step:proof.contract().max_work_units_per_step,step_budget_us:semio_framework_job::INTERACTIVE_LANE_WALL_US};
        let mut session=loop{plugin_job_yield_once().await;if let Some(session)=pending.admit_pending(config,cx)?{break session}};
        drop(pending);
        let mut checkpoint_progress = 0u64;
        let mut step_overruns = semio_framework_trace::StepOverrunLedger::new();
        loop {
            plugin_job_yield_once().await;
            let poll = match session.pump_one(&pool, semio_framework_async::Lane::Interactive, step_grant) {
                Ok(poll) => poll,
                Err(fault) if reserved_pump_fault_is_transient(&fault) => {
                    #[cfg(target_arch = "wasm32")]
                    if let Some(now_ms) = semio_framework_job::default_now_ms() {
                        pool.pump(now_ms);
                    }
                    continue;
                }
                Err(fault) => {
                    return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.worker-pump"), format!("framework route '{verb}' lost its mounted worker transition: {}", reserved_pump_fault_detail(&fault)))).into());
                }
            };
            if !matches!(poll, semio_framework_job_worker::WorkerJobPoll::Outcome | semio_framework_job_worker::WorkerJobPoll::Terminal) {
                #[cfg(target_arch = "wasm32")]
                if let Some(now_ms) = semio_framework_job::default_now_ms() {
                    pool.pump(now_ms);
                }
                continue;
            }
            let Some((issued, receipt)) = session.take_checked_out_retained_step_receipt() else { return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.outcome-missing"), "framework reserved mounted worker checkout lost its exact outcome")).into()) };
            if !receipt.fits(issued) {
                cancel.cancel_now();
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.step-receipt"), format!("framework route '{verb}' reported a step receipt beyond its issued grant"))).into());
            }
            let mut protocol_fault = None;
            let (terminal, terminal_kind) = {
                let Some(view) = session.checked_out_outcome().map_err(|error| plugin_sdk_fault(error.into_message()))? else {
                    session.resume().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.resume"), "framework reserved paid step lost its exact resume authority"))?;
                    continue;
                };
                let kind = match &view {
                    semio_framework_job::JobOutcomeView::Yield { .. } | semio_framework_job::JobOutcomeView::PreviewReady { .. } => 0,
                    semio_framework_job::JobOutcomeView::CheckpointReady { state, applied_progress, .. } => {
                        if *applied_progress < checkpoint_progress || state.len() > proof.contract().max_output_bytes {
                            protocol_fault = Some("interactive-job.checkpoint");
                        }
                        checkpoint_progress = *applied_progress;
                        0
                    }
                    semio_framework_job::JobOutcomeView::Complete { output, .. } => {
                        if output.map_or(!raw.is_empty(), |output| output.len() > proof.contract().max_output_bytes || !retained_payload_eq_slice(output, raw)) {
                            protocol_fault = Some("interactive-job.output-envelope");
                        }
                        1
                    }
                    semio_framework_job::JobOutcomeView::Cancelled { .. } => 2,
                    semio_framework_job::JobOutcomeView::Fault { .. } => 3,
                };
                (view.is_terminal(), kind)
            };
            let overrun = session.callback_verdict().copied().is_some_and(|verdict| step_overruns.admit(&verdict).is_terminal());
            if !terminal {
                let mut acknowledged = false;
                for _ in 0..RESERVED_OUTCOME_ACKNOWLEDGE_TURNS {
                    plugin_job_yield_once().await;
                    if session.checked_out_outcome_descriptor().is_none() {
                        acknowledged = true;
                        break;
                    }
                    let step = session.acknowledge_checked_out_outcome(close_grant);
                    if step.progress() == RetainedCloneProgress::default() {
                        break;
                    }
                }
                if !acknowledged {
                    cancel.cancel_now();
                    return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.outcome-acknowledge"), format!("framework route '{verb}' could not acknowledge its retained outcome"))).into());
                }
                if let Some(code) = protocol_fault {
                    cancel.cancel_now();
                    return Err((Fault::new(FaultOrigin::Framework, FaultCode::new(code), format!("framework route '{verb}' returned an invalid retained checkpoint"))).into());
                }
                if overrun {
                    cancel.cancel_now();
                }
                session.resume().map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.resume"), "framework reserved outcome lost its exact resume authority"))?;
                continue;
            }
            session.begin_close();
            loop {
                plugin_job_yield_once().await;
                let grant=close_grant;
                if matches!(session.has_original_cancel_alias_witness(&cancel),Ok(true)){
                    match session.return_original_cancel_alias_step(&cancel,grant){
                        Ok(_)|Err(semio_framework_job_worker::WorkerJobDemandError::Contention(_))=>continue,
                        Err(semio_framework_job_worker::WorkerJobDemandError::Refused(error))=>return Err(error.into()),
                    }
                }
                let demand=session.retirement_demands(grant.maximum_copy_bytes).map_err(|error|match error{semio_framework_job_worker::WorkerJobDemandError::Refused(error)=>semio_framework_job::OriginalTurnFailure::from(error),semio_framework_job_worker::WorkerJobDemandError::Contention(_)=>semio_framework_job::OriginalTurnFailure::from(plugin_sdk_fault("reserved original session close remains contended"))})?;
                if !plugin_grant_funds(demand,grant){continue}
                match cx.with_retained_loan(grant,|same|session.close_step(same))?{
                    semio_framework_job_worker_authority::WorkerJobCloseStep::Pending{..}|semio_framework_job_worker_authority::WorkerJobCloseStep::Blocked=>{},
                    semio_framework_job_worker_authority::WorkerJobCloseStep::Complete{..}if session.terminal_is_empty()=>break,
                    semio_framework_job_worker_authority::WorkerJobCloseStep::Complete{..}=>return Err(plugin_sdk_fault("reserved original session closed without its terminal witness").into()),
                }
            }
            if overrun {
                return Err((Fault::new(
                    FaultOrigin::Framework,
                    FaultCode::new("interactive-job.step-overrun"),
                    format!("framework route '{verb}' overran the 8 ms step ceiling for {} consecutive steps, worst {}us", step_overruns.consecutive_overruns(), step_overruns.worst_elapsed_us()),
                )).into());
            }
            if let Some(code) = protocol_fault {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new(code), format!("framework route '{verb}' did not preserve its admitted envelope"))).into());
            }
            match terminal_kind {
                1 => break,
                2 => return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{verb}' was cancelled"))).into()),
                3 => return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.fault"), format!("framework route '{verb}' returned a retained fault"))).into()),
                _ => return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.terminal-kind"), "framework reserved session closed without a terminal outcome")).into()),
            }
        }
        if cancel.is_cancelled().await {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled-or-output"), format!("framework route '{verb}' was cancelled before commit"))).into());
        }
        let live_revision_bytes = self.store.content_revision();
        let live_revision = semio_framework_job::RevisionId(u64::from_be_bytes(live_revision_bytes[..8].try_into().expect("revision lane width")));
        let live_generation = semio_framework_job::Generation(self.store.generation());
        if !matches!(semio_framework_job::validate_commit(&operation, live_revision, live_generation), semio_framework_job::CommitValidation::Accepted) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.stale"), format!("framework route '{verb}' completed against a stale revision or generation"))).into());
        }
        Ok(())
    }

    pub(super) async fn validate_framework_reserved_commit(&self, verb: &str, permit: &FrameworkReservedCommitPermit) -> Result<(), Fault> {
        if self.time_travel.freezes_local_emits() && time_travel_freezes_reserved_action(verb) {
            return Err(time_travel_frozen_fault(verb));
        }
        if permit.lease.is_cancelled().await {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.cancelled"), format!("framework route '{verb}' was cancelled before commit")));
        }
        let live_revision_bytes = self.store.content_revision();
        let live_revision = semio_framework_job::RevisionId(u64::from_be_bytes(live_revision_bytes[..8].try_into().expect("revision lane width")));
        let live_generation = semio_framework_job::Generation(self.store.generation());
        if !matches!(semio_framework_job::validate_commit(&permit.operation, live_revision, live_generation), semio_framework_job::CommitValidation::Accepted) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.stale"), format!("framework route '{verb}' reached commit with a stale revision or generation")));
        }
        Ok(())
    }

    pub(super) fn qualified_tool_proof(&self, verb: &str) -> Result<QualifiedToolProof, Fault> {
        if let Some(lane) = self.unsupported_publication_contracts.get(verb) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-authority-missing"), format!("typed command '{verb}' declares the unsupported {lane} publication lane")));
        }
        if let Some(registration) = self.app_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::AppOwned(registration.clone()));
        }
        if let Some(registration) = self.framework_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::FrameworkOwned(registration.clone()));
        }
        if self.bounded_tool_proofs.contains_key(verb) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-reducer"), format!("typed command '{verb}' has only a generic proof and no exact app-owned retained decoder/reducer factory")));
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-factory"), format!("typed command '{verb}' has no exact controller/owner/factory/tool/schema proof")))
    }

    fn qualified_host_configuration_tool_proof(&self, verb: &str) -> Result<QualifiedToolProof, Fault> {
        if let Some(lane) = self.unsupported_publication_contracts.get(verb) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.publication-authority-missing"), format!("host configuration command '{verb}' declares the unsupported {lane} publication lane")));
        }
        if let Some(registration) = self.app_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::AppOwned(registration.clone()));
        }
        if let Some(registration) = self.framework_tool_registrations.get(verb) {
            return Ok(QualifiedToolProof::FrameworkOwned(registration.clone()));
        }
        if let Some(proof) = self.bounded_tool_proofs.get(verb) {
            return Ok(QualifiedToolProof::Bounded(proof.clone()));
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-factory"), format!("host configuration command '{verb}' has no exact controller/owner/factory/tool/schema proof")))
    }

    /// 🚦️ The UI-safety backstop, read from the app's own manifest declaration: a verb that is not
    /// `Migrated` is refused by NAME here, before any factory, proof or wire admission is resolved.
    /// Resolving the proof first answered `interactive-job.missing-factory` for a verb whose real
    /// refusal is `interactive-job.not-ui-safe` — a non-migrated verb has no factory precisely
    /// BECAUSE it is not migrated, so the proof lookup reported the consequence, not the cause.
    fn require_ui_safe_declaration(&self, verb: &str) -> Result<(), Fault> {
        let declaration = match self.registry.get(verb) {
            Some(definition) => definition.semantics.execution.interactive_job,
            None => self
                .registry
                .get_command(verb)
                .map(|definition| definition.semantics.execution.interactive_job)
                .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.unknown-key"), format!("typed command '{verb}' has no exact manifest declaration")))?,
        };
        validate_ui_dispatch_classification("owner", verb, declaration)
    }

    async fn admit_command_wire_with_proof(&self, proof: QualifiedToolProof, verb: &str, payload: &[u8], decoded_items: usize) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let key = proof.key();
        let (admission, input) =
            self.tool_jobs.begin_exact_wire(key.controller_id, key.tool_id, proof.schema_id(), payload.len()).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        let mut writer = TypedCommandWireWriter { input, page: [0; semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES], len: 0 };
        std::io::Write::write_all(&mut writer, payload).map_err(|error| plugin_sdk_fault(error.to_string()))?;
        let raw_wire = writer.finish()?;
        if decoded_items > proof.contract().max_decoded_items {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.decoded-limit"), format!("decoded command contains {decoded_items} items; maximum is {}", proof.contract().max_decoded_items)));
        }
        if !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.factory-identity"), format!("typed command '{verb}' resolved outside its exact controller/owner/factory/tool/schema proof")));
        }
        Ok(AdmittedToolCommand { verb: verb.to_string(), proof, wire_admission: admission, decoded_items, raw_wire })
    }

    pub(crate) async fn admit_command_wire(&self, verb: &str, payload: &[u8], decoded_items: usize) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let proof = self.qualified_tool_proof(verb)?;
        self.admit_command_wire_with_proof(proof, verb, payload, decoded_items).await
    }

    async fn admit_command_json_with_proof(&self, proof: QualifiedToolProof, verb: &str, args: Option<&DslValue>) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let key = proof.key();
        let (admission, input) = self
            .tool_jobs
            .begin_exact_wire(key.controller_id, key.tool_id, proof.schema_id(), proof.contract().max_raw_wire_bytes)
            .map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        let mut writer = TypedCommandWireWriter { input, page: [0; semio_framework::action_bus::TOOL_WIRE_PAGE_BYTES], len: 0 };
        let wire_args = args.cloned();
        let encoded = semio_framework_pack_json::to_json_string(&(verb, wire_args));
        std::io::Write::write_all(&mut writer, encoded.as_bytes()).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.raw-wire-limit"), "typed command raw JSON exceeds its registered retained-page admission"))?;
        let raw_wire = writer.finish_admitted_prefix()?;
        if !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.factory-identity"), format!("typed command '{verb}' resolved outside its exact controller/owner/factory/tool/schema proof")));
        }
        Ok(AdmittedToolCommand { verb: verb.to_string(), proof, wire_admission: admission, decoded_items: 0, raw_wire })
    }

    pub(super) async fn admit_host_configuration_json(&self, verb: &str, args: Option<&DslValue>) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let proof = self.qualified_host_configuration_tool_proof(verb)?;
        self.admit_command_json_with_proof(proof, verb, args).await
    }

    /// 🧬️ Constructs a store-only wrapper with an empty registry. Every action and typed
    /// command fails closed before decoding until a registry-backed wrapper is constructed.
    pub async fn new(app: A, actor: ActorId, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Self {
        Self::with_registry(app, AppActionRegistry::default(), actor, mounted_policy, identity).await
    }

    /// 🧬️ Constructs a wrapper carrying the app's {@link AppActionRegistry} so `handle_action`
    /// enforces default materialization, required-arg validation, and kind discipline.

    /// 🧱 Runs `initial_snapshot` on its own frame so fixture sync cannot nest inside envelope construction.
    #[inline(never)]
    async fn isolate_artifact_app_initial_snapshot() -> A::Snapshot {
        A::initial_snapshot().await
    }

    pub async fn with_registry(app: A, registry: AppActionRegistry, actor: ActorId, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Self {
        Self::with_registry_on_bus(app, registry, semio_framework::ActionBus::new(), actor, mounted_policy, identity).await
    }

    /// 🫂️ Activates an app controller against Platform's injected shared production registry.
    pub async fn with_registry_on_bus(app: A, registry: AppActionRegistry, tool_jobs: semio_framework::ActionBus, actor: ActorId, mounted_policy: crate::MountedOwnerPolicyV1, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Self {
        let mounted_policy = mounted_policy.validate().expect("explicit app embedding policy");
        let app_id = app.instance_id().await.to_string();
        PluginRuntimeRegistry::publish_framework_schema_documents().expect("the OS's own schema documents never conflict");
        for documents in <A::Mutation as ::protocol::Mutation<A::Snapshot>>::INPUT_SCHEMA_DOCUMENTS {
            semio_framework_schema_registry::register_referenced_schema_documents(documents);
        }
        let snapshot = Self::isolate_artifact_app_initial_snapshot().await;
        let mut envelope = create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, &app_id, snapshot, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        envelope.dialect = Some(A::DIALECT.into());
        let config_id = format!("{}-config", app_id);
        let config_envelope = create_config_envelope::<A::Config, A::ConfigMutation>(A::config_schema().await, &config_id, A::initial_config().await, None);
        let draft_id = format!("{}-draft", app_id);
        let draft_envelope = create_document_envelope::<A::Draft, A::DraftMutation>("draft.empty", &draft_id, A::initial_draft().await, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let interaction_id = format!("{}-interaction", app_id);
        let interaction_envelope = create_document_envelope::<protocol::InteractionState, InteractionConfigMutation>("framework.interaction", &interaction_id, protocol::InteractionState::default(), None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
        let mut store = ArtifactStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, actor.clone()).await.expect("failed to create document store");
        store.install_operation_wire(store::ArtifactOperationWire::semantic::<A::Snapshot>().with_batch_digest());
        store.enable_convergence_early_exit();
        store.defer_remote_replays(Some(time_travel::time_travel_replay_turn_budget()));
        store.defer_local_replays(Some(time_travel::time_travel_replay_turn_budget()));
        let config_store = ConfigStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(config_envelope.await, actor.clone()).await.expect("failed to create config store");
        let draft_store = store::DraftStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(draft_envelope, actor.clone()).await.expect("failed to create draft store");
        let interaction_store = ConfigStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(interaction_envelope, actor.clone()).await.expect("failed to create interaction store");
        let genesis_mutations = A::genesis().await;
        if !genesis_mutations.is_empty() {
            store.dispatch(ArtifactCommand::Apply { mutations: genesis_mutations, transaction: None }, identity).await.expect("ArtifactApp::genesis mutations must apply cleanly onto a freshly constructed store");
        }
        let dispatch_report = protocol::DispatchReport { policy: store.merge_policy(), worst: None, messages: Vec::new() };
        let mut framework_tool_registry = ArtifactToolFactoryRegistry::<A>::new(&tool_jobs, &app_id);
        register_framework_reserved_factories(&mut framework_tool_registry).expect("framework-owned reserved factories must preserve exact owner/controller/schema/tool authority");
        let framework_tool_registrations = framework_tool_registry.finish();
        let mut app_tool_registry = ArtifactToolFactoryRegistry::<A>::new(&tool_jobs, &app_id);
        A::register_tool_job_factories(&mut app_tool_registry).expect("app-owned tool factories must preserve exact owner/controller/schema/tool authority");
        let app_tool_registrations = app_tool_registry.finish();
        let (tool_job_controller_id, tool_job_registrations) = crate::reserved_jobs::tool_job_registration::<A>(&registry, &app_id, A::DOCUMENT_SCHEMA, <A::Command as ::protocol::OpBinary>::TOOL_JOB_IDS, &tool_jobs, &app_tool_registrations)
            .expect("tool proof catalog must exactly join migrated generated declarations to live concrete factories");
        let artifact_one_item_factory = A::build_artifact_store_one_item_preparation_factory();
        let config_one_item_factory = A::build_config_store_one_item_preparation_factory();
        let draft_one_item_factory = A::build_draft_store_one_item_preparation_factory();
        let presence_one_item_factory = A::build_presence_store_one_item_preparation_factory();
        let transient_one_item_factory = A::build_transient_store_one_item_preparation_factory();
        let presence_local_root_retirement_factory = A::build_presence_local_root_retirement_factory();
        let transient_local_root_retirement_factory = A::build_transient_local_root_retirement_factory();
        let mut window_config_store = WindowConfigOwnerRegistry::new(actor);
        A::register_window_config_owners(&mut window_config_store).expect("window config owners must be uniquely registered by exact window kind");
        let mut window_transient_store = WindowTransientOwnerRegistry::default();
        A::register_window_transient_owners(&mut window_transient_store).expect("window transient owners must be uniquely registered by exact window kind");
        let mut unsupported_publication_contracts = BTreeMap::new();
        for registration in framework_tool_registrations.values().chain(app_tool_registrations.values()) {
            let unsupported = registration.publication_lanes.iter().copied().find_map(|lane| match lane {
                ArtifactToolPublicationLane::Artifact if artifact_one_item_factory.is_none() => Some("artifact"),
                ArtifactToolPublicationLane::Config if config_one_item_factory.is_none() => Some("config"),
                ArtifactToolPublicationLane::Draft if draft_one_item_factory.is_none() => Some("draft"),
                ArtifactToolPublicationLane::Presence if presence_one_item_factory.is_none() || presence_local_root_retirement_factory.is_none() => Some("presence"),
                ArtifactToolPublicationLane::Transient if transient_one_item_factory.is_none() || transient_local_root_retirement_factory.is_none() => Some("transient"),
                ArtifactToolPublicationLane::WindowConfig if window_config_store.is_empty() => Some("window-config"),
                ArtifactToolPublicationLane::WindowTransient if window_transient_store.is_empty() => Some("window-transient"),
                ArtifactToolPublicationLane::Child => None,
                ArtifactToolPublicationLane::Interaction => None,
                ArtifactToolPublicationLane::HostOnly
                | ArtifactToolPublicationLane::Artifact
                | ArtifactToolPublicationLane::Config
                | ArtifactToolPublicationLane::Draft
                | ArtifactToolPublicationLane::Presence
                | ArtifactToolPublicationLane::Transient => None,
                ArtifactToolPublicationLane::WindowConfig => None,
                ArtifactToolPublicationLane::WindowTransient => None,
            });
            if let Some(lane) = unsupported {
                unsupported_publication_contracts.insert(registration.key.tool_id.clone(), lane);
            }
        }
        let instance_operation_owner = ArtifactInstanceOperationOwnerHandle::new(A::build_instance_operation_owner());
        let mut bounded_tool_proofs = BTreeMap::new();
        let mut bounded_tool_contracts = Vec::with_capacity(tool_job_registrations.len());
        for proof in tool_job_registrations {
            let tool_id = proof.row.tool_id.to_string();
            if app_tool_registrations.contains_key(&tool_id) || framework_tool_registrations.contains_key(&tool_id) {
                continue;
            }
            bounded_tool_contracts.push(ArtifactToolPublicContract {
                owner: proof.owner,
                controller_id: proof.row.controller_id.to_string(),
                tool_id: tool_id.clone(),
                schema_id: proof.schema_id(),
                max_raw_wire_bytes: proof.contract().max_raw_wire_bytes,
                publication_lanes: &[ArtifactToolPublicationLane::HostOnly],
            });
            let factory = TypedCommandFullOperationJobFactory::<A>::from_proof(proof.clone()).expect("validated bounded reducer proof must retain its concrete owner authority");
            tool_jobs.register_once(factory).expect("validated bounded reducer factory must register exactly once");
            assert!(bounded_tool_proofs.insert(tool_id, proof).is_none(), "validated bounded reducer proofs must remain unique by tool id");
        }
        let mut presence_store = store::PresenceStore::new(A::Presence::default());
        if let Some(factory) = presence_local_root_retirement_factory.as_ref() {
            presence_store.install_local_retirement_factory(factory.clone()).expect("presence local retirement factory installs exactly once during app construction");
        }
        if let Some(factory) = A::build_presence_peer_retirement_factory() {
            presence_store.install_peer_retirement_factory(factory).expect("presence peer retirement factory installs exactly once during app construction");
        }
        let mut this = Self {
            app,
            mounted_policy,
            constructor_receipts: std::mem::MaybeUninit::uninit(),
            constructor_recipient_admitted: false,
            constructor_recipient_retired: false,
            constructor_terminal_receipt: None,
            constructor_document: OriginalCatalogConstructor::new(),
            constructor_config: OriginalCatalogConstructor::new(),
            constructor_draft: OriginalCatalogConstructor::new(),
            constructor_interaction: OriginalCatalogConstructor::new(),
            constructor_lane: 0,
            store,
            config_store,
            draft_store,
            presence_store,
            own_color: None,
            peer_presence: ManuallyDrop::new(PeerPresenceRoot::empty_shared()),
            peer_presence_generation: 0,
            peer_roster_admission_generation: 0,
            peer_roster_processed_generation: 0,
            peer_roster_outcome_generation: 0,
            peer_roster_reservations: std::array::from_fn(|_| None),
            peer_roster_scope: semio_framework_job::CancelToken::root_now(),
            peer_roster_publications: ArtifactFixedRegistry::new(),
            peer_roster_outcomes: ArtifactFixedRegistry::new(),
            peer_presence_retirements: ArtifactFixedRegistry::new(),
            presence_peer_retirements: ArtifactFixedRegistry::new(),
            transient_store: store::TransientStore::new(A::Transient::default()),
            window_config_store,
            window_transient_store,
            retired_window_transient_stores: ArtifactFixedRegistry::new(),
            maintenance_window_retirement_cursor: 0,
            close_window_retirement_cursor: 0,
            artifact_one_item_factory,
            config_one_item_factory,
            draft_one_item_factory,
            presence_one_item_factory,
            transient_one_item_factory,
            presence_local_root_retirement_factory,
            transient_local_root_retirement_factory,
            unsupported_publication_contracts,
            last_emit_wire: None,
            action_ingress_sources: ArtifactFixedRegistry::unallocated(16),
            pending_tool_lease_admission:ManuallyDrop::new(None),
            pending_media_import_permit:ManuallyDrop::new(None),
            pending_action_ingress:ManuallyDrop::new(None),
            pending_action_ingress_caller: std::mem::ManuallyDrop::new(None),
            pending_action_ingress_identity:None,
            pending_action_ingress_retirement:ManuallyDrop::new(None),
            typed_effect_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_event_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_ui_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            operation_progress_retired: false,
            typed_inline_interaction_verbs: Vec::new(),
            typed_inline_interaction_leftover: None,
            typed_completion_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            typed_composed_outbox: ArtifactFixedQueue::new(TYPED_OPERATION_HOST_OUTBOX_SLOTS),
            dispatch_report,
            cache: None,
            registry,
            tool_job_controller_id,
            tool_jobs,
            pending_render_tree:std::mem::ManuallyDrop::new(None),
            pending_render_tree_identity:None,
            pending_render_tree_ready:false,
            pending_render_tree_close:std::mem::ManuallyDrop::new(None),
            pending_configuration_completion:None,
            pending_configuration_command:std::mem::ManuallyDrop::new(None),
            pending_configuration_command_close:None,
            pending_configuration_permit:None,
            pending_configuration_failure:std::mem::ManuallyDrop::new(None),
            pending_configuration_identity:None,
            pending_configuration_retirement:None,
            pending_render_failure:std::mem::ManuallyDrop::new(None),
            pending_render_identity:None,
            pending_render_close_failure:std::mem::ManuallyDrop::new(None),
            pending_render_compound:std::mem::ManuallyDrop::new(None),
            pending_render_retirement:std::mem::ManuallyDrop::new(None),
            agent_lane_preview:None,
            bounded_tool_proofs,
            bounded_tool_contracts,
            framework_tool_registrations,
            app_tool_registrations,
            instance_operation_owner,
            tool_cancellations: crate::tool_cancellation::ToolCancellationIssuer::empty(),
            live_runtime_instance_id: None,
            tool_operations: ArtifactFixedRegistry::new(),
            #[cfg(test)]
            test_tool_clock: None,
            typed_operation_reservations: [None; ARTIFACT_LIVE_OUTPUT_SLOTS],
            typed_publication_cursor: 0,
            typed_result_cursor: 0,
            latest_wins_commands: ArtifactFixedRegistry::new(),
            latest_wins_order: ArtifactFixedQueue::new(ARTIFACT_LIVE_OUTPUT_SLOTS),
            latest_wins_keys: ToolLatestWinsRegistry::new(),
            latest_wins_turn: true,
            media_exports: ArtifactFixedRegistry::new(),
            media_export_preparation:ManuallyDrop::new(None),
            media_download_preparation:ManuallyDrop::new(None),
            pending_reserved: ArtifactFixedRegistry::new(),
            reserved_commits: std::collections::VecDeque::with_capacity(ARTIFACT_LIVE_OUTPUT_SLOTS),
            reserved_commit_outcome: None,
            media_closures: ArtifactFixedRegistry::new(),
            snapshot_retirements: ArtifactFixedRegistry::new(),
            current_media_export: None,
            media_submission_refusal: std::mem::ManuallyDrop::new(None),
            media_submission_refusal_identity: None,
            media_submission_refusal_retirement: std::mem::ManuallyDrop::new(None),
            segmented_downloads: ArtifactFixedRegistry::new(),
            segmented_closures: ArtifactFixedRegistry::new(),
            close_started: false,
            close_cancellation_cursor: 0,
            pending_document_window_admission:std::mem::ManuallyDrop::new(None),
            close_typed_operation_cursor: 0,
            close_media_cursor: 0,
            close_media_cleanup_cursor: 0,
            close_segment_cursor: 0,
            close_segment_cleanup_cursor: 0,
            close_snapshot_cursor: 0,
            close_instance_operation_owner_drained: false,
            close_child_root_cursor: 0,
            close_child_root_detached: false,
            close_child_member_cursor: 0,
            close_child_member: ManuallyDrop::new(None),
            close_peer_presence_cursor: 0,
            close_peer_presence_detached: false,
            close_presence_peer_cursor: 0,
            close_peer_roster_cursor: 0,
            maintenance_stage: 0,
            maintenance_cancellation_cursor: 0,
            maintenance_tool_cursor: 0,
            maintenance_media_cursor: 0,
            maintenance_segment_cursor: 0,
            maintenance_snapshot_cursor: 0,
            maintenance_child_root_cursor: 0,
            maintenance_peer_presence_cursor: 0,
            maintenance_presence_peer_cursor: 0,
            maintenance_peer_roster_cursor: 0,
            envelope_ingress: ArtifactFixedRegistry::new(),
            maintenance_envelope_ingress_cursor: 0,
            close_envelope_ingress_cursor: 0,
            close_envelope_ingress_drained: false,
            envelope_decode_jobs: ArtifactFixedRegistry::new(),
            maintenance_envelope_decode_cursor: 0,
            close_envelope_decode_cursor: 0,
            close_envelope_decode_stage: 0,
            close_envelope_decode_jobs_drained: false,
            store_replacement_jobs: ArtifactFixedRegistry::new(),
            maintenance_store_replacement_cursor: 0,
            close_store_replacement_cursor: 0,
            close_store_replacement_jobs_drained: false,
            import_staging: semio_framework::kernel::ImportStaging::default(),
            document_archive_loads: ArtifactFixedRegistry::with_slots(DOCUMENT_ARCHIVE_LOAD_SLOTS),
            maintenance_document_archive_cursor: 0,
            close_document_archive_cursor: 0,
            close_document_archive_loads_drained: false,
            envelope_field_decoders: store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoderRegistry::new(),
            envelope_field_decoder_retirements: ArtifactFixedRegistry::new(),
            maintenance_envelope_field_decoder_cursor: 0,
            close_envelope_field_decoder_cursor: 0,
            close_envelope_field_decoders_drained: false,
            envelope_completed_records: store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordRegistry::new(),
            envelope_completed_record_retirements: ArtifactFixedRegistry::new(),
            maintenance_envelope_completed_record_cursor: 0,
            close_envelope_completed_record_cursor: 0,
            close_envelope_completed_records_drained: false,
            document_snapshot_read_returns: SnapshotReadReturnPump::new(),
            close_snapshot_reads_drained: false,
            close_owned_stage: 0,
            close_owned_advance_ready: false,
            close_document_disposer: ManuallyDrop::new(A::build_document_store_disposer()),
            close_config_disposer: ManuallyDrop::new(A::build_config_store_disposer()),
            close_draft_disposer: ManuallyDrop::new(A::build_draft_store_disposer()),
            close_presence_disposer: ManuallyDrop::new(A::build_presence_store_disposer()),
            close_transient_disposer: ManuallyDrop::new(A::build_transient_store_disposer()),
            close_interaction_disposer: ManuallyDrop::new(Some(Box::new(ArtifactDocumentStoreDisposer::<protocol::InteractionState, InteractionConfigMutation>::new()))),
            invocation_kind: None,
            host_event_utilities: HashMap::new(),
            host_event_roster: Vec::new(),
            host_event_operations: HashSet::new(),
            delivering_host_event: false,
            command_log: PagedCommandLog::new(),
            command_prune_generation: 0,
            command_prune_processed: 0,
            pending_command_prune: None,
            next_command_seq: 0,
            log_generation: 0,
            history_dirty_sequences: HashSet::new(),
            history_view: None,
            history_backfill: None,
            shell_undone: HashSet::new(),
            shell_redo: Vec::new(),
            history_filter: HistoryCommandFilter::default(),
            children: ChildMemberRegistry::new(),
            private_child_groups: ArtifactFixedRegistry::new(),
            child_content_root: ManuallyDrop::new(ChildContentView::EMPTY),
            child_content_generation: 0,
            child_content_retirements: ArtifactFixedRegistry::new(),
            child_member_retirements: ArtifactFixedRegistry::new(),
            child_member_retirement_generation: 0,
            child_member_retirement_cursor: 0,
            followed_parent_generation: 0,
            child_follow_awaits_retirement: false,
            pending_child_pins: Vec::new(),
            composition: CompositionCoordinator::new().await,
            interaction_store,
            local_interaction_query: None,
            local_interaction_query_receipt: None,
            local_interaction_authority: Default::default(),
            snapshot_read_return_stage: 0,
            local_interaction_query_turn: false,
            local_interaction_emitted_bytes: 0,
            local_interaction_retired_bytes: 0,
            config_snapshot_read_returns: SnapshotReadReturnPump::new(),
            interaction_snapshot_read_returns: SnapshotReadReturnPump::new(),
            interaction_hover: InteractionHoverState::new(),
            interaction_leftover_selection: None,
            interaction_leftover_ids: Vec::new(),
            interaction_ui_topology: HashMap::new(),
            interaction_topology_memo: None,
            pending_transaction: None,
            pending_transaction_proposal: None,
            pending_presence: Vec::new(),
            presence_marked_keys: BTreeMap::new(),
            tool_runs: ToolRunLedger::default(),
            time_travel: TimeTravelLedger::default(),
            tool_machines: ToolMachineRuntime::default(),
            supersedes: SupersedeLedger::default(),
        };
        this.seed_genesis_children(identity).await.expect("ArtifactApp::genesis_child_pack members must open cleanly onto a freshly constructed store");
        this.followed_parent_generation = this.store.generation_now();
        this
    }

    pub(super) fn has_constructor_receipt(&self)->bool { self.constructor_terminal_receipt.is_some() || (self.constructor_recipient_admitted && unsafe { self.constructor_receipts.assume_init_ref() }.iter().any(Option::is_some)) }
    pub(super) fn constructor_receipt(&self)->Option<&(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)> { if let Some(receipt)=self.constructor_terminal_receipt.as_ref(){return Some(receipt)} if !self.constructor_recipient_admitted{return None} unsafe {self.constructor_receipts.assume_init_ref()}.iter().find_map(Option::as_ref) }
    pub(super) fn take_constructor_receipt(&mut self)->Option<(ArtifactStoreConstructorKind,ArtifactStoreConstructorPhase,RetainedCloneGrant,RetainedCloneProgress)> { if self.constructor_terminal_receipt.is_some(){return self.constructor_terminal_receipt.take()} if !self.constructor_recipient_admitted{return None} unsafe {self.constructor_receipts.assume_init_mut()}.iter_mut().find_map(Option::take) }
    pub(super) fn store_constructor_is_pending(&self)->bool { (!self.constructor_recipient_admitted&&!self.constructor_recipient_retired) || self.has_constructor_receipt() || self.constructor_lane<4 }
    pub(crate) fn drive_store_constructor_unit(&mut self,grant:RetainedCloneGrant)->Result<bool,ValueError> {self.drive_store_constructor_unit_in_phase(grant,ArtifactStoreConstructorPhase::Preparation)}
    fn drive_store_constructor_unit_in_phase(&mut self,grant:RetainedCloneGrant,phase:ArtifactStoreConstructorPhase)->Result<bool,ValueError> {
        if self.has_constructor_receipt(){return Ok(true)}
        if !self.constructor_recipient_admitted {
            let copied_bytes=0usize;
            if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(true)}
            let recipient=self.constructor_receipts.write([None;24]);self.constructor_recipient_admitted=true;
            recipient[0]=Some((ArtifactStoreConstructorKind::Recipient,phase,grant,RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}));return Ok(true)
        }
        if self.constructor_lane>=4{return Ok(false)}
        let copied_bytes=0usize;
        if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth<2{return Ok(true)}
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-copied_bytes,maximum_depth:grant.maximum_depth-1,..grant};
        let (kind,mut receipt,done)=match self.constructor_lane {
            0=>{let receipt=self.constructor_document.advance(&mut self.store,child,A::build_document_store_owners)?;(ArtifactStoreConstructorKind::Document,receipt,self.constructor_document.terminal_is_empty())},
            1=>{let receipt=self.constructor_config.advance(&mut self.config_store,child,A::build_config_store_owners)?;(ArtifactStoreConstructorKind::Config,receipt,self.constructor_config.terminal_is_empty())},
            2=>{let receipt=self.constructor_draft.advance(&mut self.draft_store,child,A::build_draft_store_owners)?;(ArtifactStoreConstructorKind::Draft,receipt,self.constructor_draft.terminal_is_empty())},
            _=>{let receipt=self.constructor_interaction.advance(&mut self.interaction_store,child,|grant|Some(crate::local_interaction::retirement::interaction_store_owners(grant)))?;(ArtifactStoreConstructorKind::Interaction,receipt,self.constructor_interaction.terminal_is_empty())},
        };
        if receipt==Default::default(){return Ok(true)}
        if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original catalog parent exceeded its full grant"))}
        if done { self.constructor_lane+=1; }
        receipt.copied_bytes=receipt.copied_bytes.checked_add(copied_bytes).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original catalog outer header extent overflow"))?;
        unsafe{self.constructor_receipts.assume_init_mut()[0]=Some((kind,phase,grant,receipt));}Ok(true)
    }

    pub(crate) fn constructor_terminal_is_empty(&self)->bool{self.constructor_recipient_retired&&!self.has_constructor_receipt()&&self.constructor_document.terminal_is_empty()&&self.constructor_config.terminal_is_empty()&&self.constructor_draft.terminal_is_empty()&&self.constructor_interaction.terminal_is_empty()}
    pub(crate) fn constructor_close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.has_constructor_receipt(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"original constructor receipt awaits its recipient"))}
        if self.constructor_recipient_retired{return Ok(Default::default())}
        if !self.constructor_recipient_admitted{return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}
        if self.constructor_lane>=4{return Ok(RetirementDemand{copy_bytes:0,depth:1,..Default::default()})}
        let parent=0usize;
        let mut demand=match self.constructor_lane{0=>self.constructor_document.demands(body.saturating_sub(parent),self.store.owned_disposer_installed(),A::document_store_owners_source_demands)?,1=>self.constructor_config.demands(body.saturating_sub(parent),self.config_store.owned_disposer_installed(),A::config_store_owners_source_demands)?,2=>self.constructor_draft.demands(body.saturating_sub(parent),self.draft_store.owned_disposer_installed(),A::draft_store_owners_source_demands)?,_=>self.constructor_interaction.demands(body.saturating_sub(parent),self.interaction_store.owned_disposer_installed(),crate::local_interaction::retirement::interaction_store_owners_source_demands)?};
        demand.copy_bytes=demand.copy_bytes.checked_add(parent).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"constructor close parent quote overflow"))?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"constructor close parent depth overflow"))?;Ok(demand)
    }
    pub(crate) fn close_store_constructor_unit(&mut self,grant:RetainedCloneGrant)->Result<bool,ValueError>{
        let phase=ArtifactStoreConstructorPhase::Close;
        if self.has_constructor_receipt(){return Ok(true)}
        if self.constructor_recipient_retired{return Ok(false)}
        let installed=match self.constructor_lane{0=>self.store.owned_disposer_installed(),1=>self.config_store.owned_disposer_installed(),2=>self.draft_store.owned_disposer_installed(),_=>self.interaction_store.owned_disposer_installed()};
        if self.constructor_lane<4&&!installed{return self.drive_store_constructor_unit_in_phase(grant,ArtifactStoreConstructorPhase::Close)}
        if !self.constructor_recipient_admitted{return self.drive_store_constructor_unit_in_phase(grant,ArtifactStoreConstructorPhase::Close)}
        if self.constructor_lane>=4{
            let copied_bytes=0usize;
            if grant.maximum_items==0||grant.maximum_copy_bytes<copied_bytes||grant.maximum_depth==0{return Ok(true)}
            unsafe{self.constructor_receipts.assume_init_drop()}self.constructor_recipient_admitted=false;self.constructor_recipient_retired=true;self.constructor_terminal_receipt=Some((ArtifactStoreConstructorKind::Recipient,phase,grant,RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()}));return Ok(true)
        }
        let parent=0usize;
        if grant.maximum_items==0||grant.maximum_copy_bytes<parent||grant.maximum_depth<2{return Ok(true)}
        let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent,maximum_depth:grant.maximum_depth-1,..grant};
        let (kind,mut receipt,done)=match self.constructor_lane{0=>{let receipt=self.constructor_document.close(child)?;(ArtifactStoreConstructorKind::Document,receipt,self.constructor_document.terminal_is_empty())},1=>{let receipt=self.constructor_config.close(child)?;(ArtifactStoreConstructorKind::Config,receipt,self.constructor_config.terminal_is_empty())},2=>{let receipt=self.constructor_draft.close(child)?;(ArtifactStoreConstructorKind::Draft,receipt,self.constructor_draft.terminal_is_empty())},_=>{let receipt=self.constructor_interaction.close(child)?;(ArtifactStoreConstructorKind::Interaction,receipt,self.constructor_interaction.terminal_is_empty())}};
        if receipt==Default::default(){return Ok(true)}
        if !receipt.fits(child){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original catalog close exceeded its full grant"))}
        if done{self.constructor_lane+=1}receipt.copied_bytes+=parent;unsafe{self.constructor_receipts.assume_init_mut()[0]=Some((kind,phase,grant,receipt));}Ok(true)
    }

    /// 🌱️ Opens every composed child the initial snapshot declares and the app derives
    /// (`ArtifactApp::genesis_child_pack`) through the ordinary restore path, so `children`, the
    /// composition graph and the child content root are complete from the first frame — a saved
    /// archive of a never-edited document then carries its derivable members instead of relying on
    /// the loading side to re-derive them.
    async fn seed_genesis_children(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let mut genesis = Vec::new();
        {
            let snapshot = self.store.snapshot_ref();
            let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("initial snapshot child projection failed: {error}")))?;
            for index in 0..projection.len() {
                let Some((slot, fields)) = projection.get(index) else { break };
                let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? else { continue };
                let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
                genesis.push((slot.to_string(), fields.child_id.to_string(), ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect }, initial_pack));
            }
        }
        let parent = ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: A::DIALECT.into() };
        for (slot, child_id, expected, initial_pack) in genesis {
            let dialect = expected.dialect.clone();
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.clone(), child_id: child_id.clone() };
            let envelope_pack = store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack).await.map_err(|error| plugin_sdk_fault(format!("genesis child {slot}/{child_id} envelope: {error}")))?;
            self.open_child(slot, child_id, dialect, &envelope_pack, identity).await?;
        }
        Ok(())
    }

    /// 🪆️ Derivable composed children follow their coordinate. A content-addressed child is minted from the
    /// parent's own content, so a parent-lane change can re-point a declared coordinate at a child id no store
    /// holds (P8 `composedChildOrphaned`). This opens every declared child no store holds that
    /// `ArtifactApp::genesis_child_pack` derives — the restore path `seed_genesis_children` uses — and retires
    /// every held child whose slot now names another derived child, so exporters, archives, hub members,
    /// agents and the next verb read the child the parent names. Gated by the parent store's generation and
    /// deferred while a child admission is in flight or while the named child is still retiring (an undo back
    /// to it) — the pass then repeats until that retirement drained. It also waits, before it changes anything,
    /// while the retirements its own publications need are not yet admissible ([`Self::reclaim_child_retirements`]):
    /// the parent already holds every edit in order, the child it names follows once maintenance returned them, and
    /// the instance stays runnable meanwhile (`child_follow_awaits_retirement`).
    pub(crate) async fn follow_derivable_children(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let generation = self.store.generation_now();
        if generation == self.followed_parent_generation || self.children.has_admission_in_flight() {
            return Ok(());
        }
        let mut genesis = Vec::new();
        let mut retire = Vec::new();
        let mut deferred = false;
        {
            let snapshot = self.store.snapshot_ref();
            let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("derivable child projection failed: {error}")))?;
            let mut declared = Vec::with_capacity(projection.len());
            for index in 0..projection.len() {
                let Some((slot, fields)) = projection.get(index) else { break };
                let key = (slot.to_string(), fields.child_id.to_string());
                if self.children.get(&key).is_none() {
                    if crate::child_content::member_is_retiring(&self.child_member_retirements, fields.artifact_id) {
                        deferred = true;
                    } else if let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? {
                        let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
                        genesis.push((key.0.clone(), key.1.clone(), ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect }, initial_pack));
                    }
                }
                declared.push(key);
            }
            for (member, _) in self.children.keyed_entries().filter(|(member, _)| member.owner.is_empty()) {
                let key = (member.slot.to_string(), member.child_id.to_string());
                if !declared.contains(&key) && genesis.iter().any(|(slot, _, _, _)| *slot == key.0) {
                    retire.push(key);
                }
            }
        }
        let publications = retire.len() + genesis.len();
        if !self.reclaim_child_retirements(publications, publications)? {
            self.child_follow_awaits_retirement = true;
            return Ok(());
        }
        for (slot, child_id) in retire {
            self.retire_followed_child(&slot, &child_id).await?;
        }
        let parent = ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: A::DIALECT.into() };
        for (slot, child_id, expected, initial_pack) in genesis {
            let dialect = expected.dialect.clone();
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.clone(), child_id: child_id.clone() };
            let envelope_pack = store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack).await.map_err(|error| plugin_sdk_fault(format!("followed child {slot}/{child_id} envelope: {error}")))?;
            self.open_child(slot, child_id, dialect, &envelope_pack, identity).await?;
        }
        if !deferred {
            self.followed_parent_generation = generation;
        }
        self.child_follow_awaits_retirement = deferred;
        Ok(())
    }

    /// 🍂️ Moves one held child its coordinate left out of every live authority — the member map, the ownership
    /// graph and the published child-content root (the previous root goes to bounded content retirement) —
    /// into bounded child-member retirement, where it stays the disposer of the snapshot leases that previous
    /// root still holds (`ChildContentRetirement::close_step` finds it there) and closes only after them.
    async fn retire_followed_child(&mut self, slot: &str, child_id: &str) -> Result<(), Fault> {
        let retirement_generation = self.child_member_retirement_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("child member retirement generation exhausted"))?;
        if !self.reclaim_child_retirements(0, 1)? {
            return Err(plugin_sdk_fault("child member retirement waits on retirements that are all still borrowed").with_retryable(true));
        }
        let publication_generation = self.admit_child_content_publication()?;
        let next = self.child_content_root.without_member(MemberKeyRef::root(slot, child_id))?;
        let entry = self.children.remove(MemberKeyRef::root(slot, child_id))?.ok_or_else(|| plugin_sdk_fault("followed child left the member map before its exact retirement"))?;
        self.composition.graph_mut().await.remove_owns(&entry.reference.artifact_id).await;
        let previous = std::mem::replace(&mut *self.child_content_root, next);
        if !previous.is_empty() {
            self.child_content_retirements.insert_admitted(publication_generation, ChildContentRetirement::new(previous, false));
        }
        self.child_content_generation = publication_generation;
        self.child_member_retirements.insert_admitted(retirement_generation, ChildMemberRetirement::new(entry));
        self.child_member_retirement_generation = retirement_generation;
        Ok(())
    }

    pub fn artifact_generation_now(&self) -> semio_framework_job::Generation {
        semio_framework_job::Generation(self.store.generation_now())
    }

    /// 🫧️ Reads the immutable transient projection of one exact live window.
    pub fn window_transient_snapshot(&mut self, view_state: &ViewModel) -> Result<Option<WindowTransientSnapshot>, Fault> {
        Ok(self.window_transient_store.capture(Some(view_state))?.map(|authority| authority.snapshot))
    }

    pub fn window_transient_generation(&mut self, view_state: &ViewModel) -> Result<Option<u64>, Fault> {
        Ok(self.window_transient_store.capture(Some(view_state))?.map(|authority| authority.generation))
    }

    pub async fn window_config_generation(&mut self, view_state: &ViewModel) -> Result<Option<u64>, Fault> {
        Ok(self.window_config_store.capture(Some(view_state)).await?.map(|authority| authority.generation))
    }

    /// 📃️ One bounded unit of the live local-interaction query. Its own captured roots are the
    /// only ownership it advances: the Store-side reclamation of the leases it hands back is
    /// [`Self::advance_snapshot_read_returns_one`]'s cooperative-maintenance business, because a
    /// registry's one-slot-per-step cleanup cursor is bounded by the registry's fixed capacity
    /// and would otherwise put a whole capacity's worth of probes inside a host continuation
    /// budget before a read could publish its terminal reply.
    pub(super) fn advance_local_interaction_query_one(&mut self, grant: store::os_store::one_item::ArtifactStoreOneItemGrant, output_width: usize) -> Result<(), Fault> {
        if self.local_interaction_query_receipt.is_some(){return Ok(())}
        let Some(query)=self.local_interaction_query.as_mut()else{return Ok(())};
        let result=if query.is_closing(){
            let before=query.retired_bytes();let result=query.close_step(grant.retained_grant());self.local_interaction_retired_bytes+=(query.retired_bytes()-before)as u128;result.map(|step|step.progress())
        }else{
            query.advance(grant,output_width).map(|step|{if let crate::local_interaction::live::LocalInteractionLiveStep::Advanced{emitted_bytes,retired_bytes,..}=step{self.local_interaction_emitted_bytes+=emitted_bytes as u128;self.local_interaction_retired_bytes+=retired_bytes as u128;}step.progress()})
        };
        let progress=match &result{Ok(progress)=>*progress,Err(error)=>error.retained_progress()};
        if progress!=Default::default(){self.local_interaction_query_receipt=Some((grant.retained_grant(),progress));}
        result.map(|_|()).map_err(|error|plugin_sdk_fault(error.to_string()))
    }

    /// ♻️ One bounded unit of the fixed document/config/interaction/transient returned-read rotation: each
    /// pump either drains the disposer it owns or probes its Store for the next returned lease.
    pub(super) fn snapshot_read_returns_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let stage=self.next_snapshot_read_return_stage().unwrap_or(self.snapshot_read_return_stage);
        let parent_copy=size_of::<usize>();let child=match stage{0=>self.document_snapshot_read_returns.retirement_demands(body.saturating_sub(parent_copy),||self.store.returned_snapshot_read_retirement_demand())?,1=>self.config_snapshot_read_returns.retirement_demands(body.saturating_sub(parent_copy),||self.config_store.returned_snapshot_read_retirement_demand())?,2=>self.interaction_snapshot_read_returns.retirement_demands(body.saturating_sub(parent_copy),||self.interaction_store.returned_snapshot_read_retirement_demand())?,_=>match self.transient_local_root_retirement_factory.as_deref(){Some(factory)=>self.transient_store.maintenance_returned_reads_demands_root(factory,body.saturating_sub(parent_copy))?,None=>Default::default()}};ChildContentRetirement::parent_snapshot_demand(child,parent_copy)
    }

    pub(super) fn advance_snapshot_read_returns_one(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        let empty=RetainedCloneProgress::default();let demand=self.snapshot_read_returns_demands(grant.maximum_copy_bytes).map_err(|error|plugin_sdk_fault(error.into_message()))?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}
        let stage = self.next_snapshot_read_return_stage().unwrap_or(self.snapshot_read_return_stage);
        let next_demand=match stage{0=>self.store.returned_snapshot_read_retirement_demand(),1=>self.config_store.returned_snapshot_read_retirement_demand(),2=>self.interaction_store.returned_snapshot_read_retirement_demand(),_=>Ok(Default::default())}.map_err(|error|plugin_sdk_fault(error.into_message()))?;
        let parent_copy=size_of::<usize>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let step=match stage{0=>self.document_snapshot_read_returns.drive(next_demand,|original|self.store.take_returned_snapshot_read_retirement(original).map_err(|error|plugin_sdk_fault(error.into_message())),child),1=>self.config_snapshot_read_returns.drive(next_demand,|original|self.config_store.take_returned_snapshot_read_retirement(original).map_err(|error|plugin_sdk_fault(error.into_message())),child),2=>self.interaction_snapshot_read_returns.drive(next_demand,|original|self.interaction_store.take_returned_snapshot_read_retirement(original).map_err(|error|plugin_sdk_fault(error.into_message())),child),_=>match self.transient_local_root_retirement_factory.clone(){Some(factory)=>self.transient_store.maintenance_returned_reads_step_root(factory.as_ref(),child).map_err(|error|plugin_sdk_fault(error.into_message())),None=>Ok(RetainedCloneStep::Progress(empty))}}?;let mut receipt=step.progress();self.snapshot_read_return_stage=(stage+1)%4;receipt.copied_items=receipt.copied_items.max(1);receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original snapshot rotation preflight");let receipt=semio_framework_value::retained_clone::admit_retained_clone_progress(grant,receipt,"original snapshot return rotation").map_err(|error|plugin_sdk_fault(error.into_message()))?;Ok(RetainedCloneStep::Progress(receipt))
    }

    /// 👁️ Borrows whether the original returned-read frontier allows another publication item.
    /// Paid retirement remains on the mounted maintenance turn before another semantic callback.
    ///
    /// 🐛️ Every batch item is prepared against a registry lease on the previous item's post root, and the registry
    /// keeps that root alive after the lease returns until a pump takes it. The maintenance rotation reclaims one
    /// returned read per rotation at one item per step, while a publication folds an item every turn — so a
    /// 408-placement fill finalize (816 ops over a 409-object puzzle 3d document) retained one ~5 MB root per
    /// folded op and trapped the guest at its 512 MiB ceiling (`memory allocation of 1232 bytes failed`, ticket
    /// 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS). Draining here bounds a publication to the staged root plus the
    /// base its current item reads, whatever the batch size.
    ///
    /// A disposer that waits on outside ownership cannot be hurried here; the publication must not stall on it.
    pub(super) fn reclaim_document_snapshot_read_returns(&self)->Result<bool,Fault>{
        Ok(self.document_snapshot_read_returns.terminal_is_empty() && self.store.returned_snapshot_read_count() == 0)
    }

    /// 🪹️ No capture-store disposer or queued document read still awaits acknowledgement.
    pub(super) fn snapshot_read_returns_terminal_is_empty(&self) -> bool {
        self.next_snapshot_read_return_stage().is_none()
    }

    /// 🧭️ The first rotation stage at or after the cursor whose pump or store still holds a returned read.
    pub(super) fn next_snapshot_read_return_stage(&self) -> Option<u8> {
        (0..4u8).map(|offset| (self.snapshot_read_return_stage + offset) % 4).find(|stage| match stage {
            0 => !self.document_snapshot_read_returns.terminal_is_empty() || self.store.returned_snapshot_read_count() != 0,
            1 => !self.config_snapshot_read_returns.terminal_is_empty() || self.config_store.returned_snapshot_read_count() != 0,
            2 => !self.interaction_snapshot_read_returns.terminal_is_empty() || self.interaction_store.returned_snapshot_read_count() != 0,
            _ => self.transient_local_root_retirement_factory.is_some() && self.transient_store.returned_read_pending(),
        })
    }

    /// 🕰️ The fixed maintenance stage the NEXT [`PluginApp::maintenance_step`] call will run.
    /// A step-budget law reads this before each unit so an over-budget unit names its own stage
    /// instead of the stale round-robin cursor an idle early return leaves behind.
    pub fn next_maintenance_stage(&self) -> u8 {
        self.maintenance_stage
    }

    /// 🎟️ Pre-admits the exact page and byte extent before a producer constructs any
    /// envelope input owner. Occupied fixed slots and allocation failure leave no input owner.
    pub fn begin_artifact_envelope_ingress(&mut self, maximum_pages: usize, maximum_bytes: usize) -> Result<ArtifactEnvelopeDecodeOperationHandle, Fault> {
        let generation = self.artifact_generation_now();
        let Some(slot) = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot| self.envelope_ingress.slot_is_vacant(*slot) && self.envelope_decode_jobs.slot_is_vacant(*slot) && self.store_replacement_jobs.slot_is_vacant(*slot)) else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-saturated"), "every fixed artifact envelope ingress slot already owns a live operation"));
        };
        let operation = semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, slot as u64);
        let pages = store::os_store::owned_schema_decode::OwnedSchemaDecodePages::try_with_credits(store::os_store::owned_schema_decode::OwnedSchemaDecodeCredits { maximum_pages, maximum_bytes })
            .map_err(|fault| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-credits"), format!("artifact envelope ingress credits were rejected: {fault:?}")))?;
        self.envelope_ingress.insert_admitted(operation.0, ActiveArtifactEnvelopeIngress::new(pages));
        Ok(ArtifactEnvelopeDecodeOperationHandle { operation, generation })
    }

    /// 📦️ Admits one already-decoded authoritative `.pack` + `.spr` envelope directly
    /// into the app-owned retained initializer. This path preserves the binary document format;
    /// the fresh JSON envelope decoder remains a separate ingress contract.
    #[expect(clippy::result_large_err, reason = "Refusal returns the exact decoded envelope so its domain retirement catalog can close every nested owner.")]
    pub(super) fn begin_persisted_document_store_replacement(&mut self, envelope: ArtifactEnvelope<A::Snapshot, A::Mutation>,cx:&mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeDecodeOperationHandle, (Fault, ArtifactEnvelope<A::Snapshot, A::Mutation>)> {
        let generation = self.artifact_generation_now();
        let expected_dialect: ArtifactDialect = A::DIALECT.into();
        if envelope.schema != A::DOCUMENT_SCHEMA || envelope.dialect.as_ref() != Some(&expected_dialect) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.persisted-envelope-identity"), "persisted document envelope does not match the app's exact schema and dialect"), envelope));
        }
        let Some(slot) = (0..ARTIFACT_LIVE_OUTPUT_SLOTS).find(|slot| self.envelope_ingress.slot_is_vacant(*slot) && self.envelope_decode_jobs.slot_is_vacant(*slot) && self.store_replacement_jobs.slot_is_vacant(*slot)) else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.persisted-ingress-saturated"), "every fixed persisted document replacement slot already owns a live operation"), envelope));
        };
        let operation = semio_framework_job::allocate_operation_id_in_slot(ARTIFACT_LIVE_OUTPUT_SLOTS as u64, slot as u64);
        let job = match A::build_document_store_initialization_job(envelope, operation, generation, self.store.local_actor_id().clone()) {
            Ok(job) => job,
            Err(envelope) => {
                return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.persisted-initializer-refused"), "app refused the persisted document's retained initialization authority"), envelope));
            }
        };
        self.store_replacement_jobs.insert_admitted(operation.0, ActiveArtifactStoreReplacement::new(operation,generation,self.child_content_generation,job,(cx.operation(),cx.generation())));
        Ok(ArtifactEnvelopeDecodeOperationHandle { operation, generation })
    }

    /// 🔎️ Rejects stale, closed, full, or over-byte page ingress before the producer copies its raw owner.
    pub fn preflight_artifact_envelope_ingress_page(&self, handle: ArtifactEnvelopeDecodeOperationHandle, page_bytes: usize) -> Result<(), Fault> {
        if self.artifact_generation_now() != handle.generation {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-stale"), "artifact envelope generation changed before page preflight"));
        }
        let ingress = self.envelope_ingress.get(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle"))?;
        if ingress.closing {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-closing"), "artifact envelope ingress is closing"));
        }
        ingress
            .pages
            .as_ref()
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority"))?
            .preflight_page_bytes(page_bytes)
            .map_err(|fault| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-page"), format!("artifact envelope page preflight was rejected before owner construction: {fault:?}")))
    }

    /// 📨️ Runs the bounded page producer only after the same mutable ingress authority accepts its extent.
    pub fn construct_and_admit_artifact_envelope_ingress_page<Build>(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, page_bytes: usize, build: Build) -> Result<(), Fault>
    where
        Build: FnOnce() -> store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage,
    {
        self.preflight_artifact_envelope_ingress_page(handle, page_bytes)?;
        let ingress = self.envelope_ingress.get_mut(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle after preflight"))?;
        let pages = ingress.pages.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority after preflight"))?;
        build().admit_preflighted_into(pages);
        Ok(())
    }

    /// 📥️ Moves one exact fixed page into a pre-admitted operation or returns that page
    /// untouched with the fault. No page clone or aggregate command buffer is constructed.
    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    pub fn admit_artifact_envelope_ingress_page(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, page: store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage) -> Result<(), (Fault, store::os_store::owned_schema_decode::ArtifactEnvelopeDecodePage)> {
        if self.artifact_generation_now() != handle.generation {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-stale"), "artifact envelope generation changed before page admission"), page));
        }
        let Some(ingress) = self.envelope_ingress.get_mut(handle.operation.0) else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle"), page));
        };
        if ingress.closing {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-closing"), "artifact envelope ingress is closing"), page));
        }
        let Some(pages) = ingress.pages.as_mut() else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority"), page));
        };
        page.admit_into(pages).map_err(|(fault, page)| (Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-page"), format!("artifact envelope page was rejected untouched: {fault:?}")), page))
    }

    /// 🔒️ Seals and transfers the exact page authority into the retained decoder. A
    /// saturated decoder keeps the sealed ingress owner in its original fixed slot for retry.
    pub fn seal_artifact_envelope_ingress(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, Fault> {
        if self.artifact_generation_now() != handle.generation {
            self.cancel_artifact_envelope_load(handle)?;
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-stale"), "artifact envelope generation changed before sealing"));
        }
        if !self.envelope_decode_jobs.can_insert(handle.operation.0) {
            return Ok(false);
        }
        {
            let ingress = self.envelope_ingress.get_mut(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-handle"), "unknown artifact envelope ingress handle"))?;
            if ingress.closing {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-closing"), "artifact envelope ingress is closing"));
            }
            let pages = ingress.pages.as_mut().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact page authority"))?;
            if !pages.is_sealed() {
                pages.seal().map_err(|fault| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-seal"), format!("artifact envelope ingress could not seal: {fault:?}")))?;
            }
        }
        let mut ingress = self.envelope_ingress.remove(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress changed before exact transfer"))?;
        let pages = ingress.take_pages().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.ingress-owner"), "artifact envelope ingress lost its exact transfer owner"))?;
        match self.submit_artifact_envelope_decode(handle.operation, handle.generation, pages,cx) {
            Ok(submitted) if submitted == handle => {
                drop(ingress);
                Ok(true)
            }
            Ok(_) => unreachable!("exact envelope transfer preserves its operation and generation"),
            Err(pages) => {
                *ingress.pages = Some(pages);
                self.envelope_ingress.insert_admitted(handle.operation.0, ingress);
                Ok(false)
            }
        }
    }

    /// 🔄️ Advances decode-to-initializer handoff without running either job inline. A decode that
    /// has not reached [`ArtifactEnvelopeDecodeOperationPoll::Ready`] reports its OWN poll — the
    /// store replacement it will hand to does not exist yet, and reading that absent job here
    /// turned every in-progress decode into a terminal `Fault` for all eight plugin loops
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn advance_artifact_envelope_load(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<ArtifactEnvelopeDecodeOperationPoll, crate::PluginLifecycleFailure> {
        if let Some(active) = self.store_replacement_jobs.get(handle.operation.0) {
            if active.operation != handle.operation || active.generation != handle.generation {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-stale-handle"), "store replacement handle is stale").into());
            }
            return Ok(self.poll_artifact_store_replacement(handle));
        }
        if let Some(active) = self.envelope_decode_jobs.get(handle.operation.0) {
            if active.operation != handle.operation || active.generation != handle.generation {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-handle"), "envelope decode handle is stale").into());
            }
            let decode = active.poll();
            if decode != ArtifactEnvelopeDecodeOperationPoll::Ready {
                return Ok(decode);
            }
            let _=self.advance_ready_envelope_decode_worker_owners(handle,cx.retained_grant(),cx)?;
            if !self.artifact_envelope_decode_worker_owners_are_terminal(handle) {
                return Ok(ArtifactEnvelopeDecodeOperationPoll::Progress);
            }
            if !self.try_begin_artifact_store_replacement(handle,cx)? {
                return Ok(ArtifactEnvelopeDecodeOperationPoll::Progress);
            }
            return Ok(self.poll_artifact_store_replacement(handle));
        }
        if self.envelope_ingress.get(handle.operation.0).is_some() {
            return Ok(ArtifactEnvelopeDecodeOperationPoll::Pending);
        }
        Ok(ArtifactEnvelopeDecodeOperationPoll::Fault)
    }

    pub fn cancel_artifact_envelope_load(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        if let Some(ingress) = self.envelope_ingress.get_mut(handle.operation.0) {
            ingress.closing = true;
            return Ok(());
        }
        if self.envelope_decode_jobs.get(handle.operation.0).is_some() {
            return self.cancel_artifact_envelope_decode(handle);
        }
        if self.store_replacement_jobs.get(handle.operation.0).is_some() {
            return self.cancel_artifact_store_replacement(handle);
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.load-stale-handle"), "unknown artifact envelope load handle"))
    }

    /// 📥️ Admits one exact fixed-page envelope owner into app maintenance. A missing
    /// domain catalog or occupied operation slot returns every page untouched.
    pub fn submit_artifact_envelope_decode(
        &mut self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        pages: store::os_store::owned_schema_decode::OwnedSchemaDecodePages,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeDecodeOperationHandle, store::os_store::owned_schema_decode::OwnedSchemaDecodePages> {
        if !self.envelope_decode_jobs.can_insert(operation.0) {
            return Err(pages);
        }
        let Some(bundle) = A::build_envelope_decode_owner_bundle() else { return Err(pages) };
        let completion = store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion::new();
        let fields = bundle.begin_fresh_decoder(operation, generation, std::sync::Arc::clone(&self.envelope_completed_records), std::sync::Arc::clone(&completion));
        self.admit_artifact_envelope_decode_owner(operation, generation, pages, fields, completion,cx)
    }

    /// 📥️ Admits one exact envelope decode against a caller-supplied field owner. The domain
    /// route builds that owner from its declared catalog; the framework's own decode-ladder laws
    /// supply a bare one so the ladder is provable without a domain field catalog.
    pub(crate) fn admit_artifact_envelope_decode_owner(
        &mut self,
        operation: semio_framework_job::OperationId,
        generation: semio_framework_job::Generation,
        pages: store::os_store::owned_schema_decode::OwnedSchemaDecodePages,
        fields: Box<dyn store::os_store::owned_schema_decode::ArtifactEnvelopeFieldDecoder<A::Snapshot, A::Mutation>>,
        completion: std::sync::Arc<store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeCompletion>,
        cx:&mut semio_framework_job::StepContext<'_>,
    ) -> Result<ArtifactEnvelopeDecodeOperationHandle, store::os_store::owned_schema_decode::OwnedSchemaDecodePages> {
        if !self.envelope_decode_jobs.can_insert(operation.0) {
            return Err(pages);
        }
        let record = store::os_store::owned_schema_decode::artifact_envelope_decode_record(operation, generation, pages)?;
        let active = match store::os_store::owned_schema_decode::ArtifactEnvelopeDecodeAuthority::try_new(record, &self.envelope_field_decoders, fields) {
            Ok(decode) => ActiveArtifactEnvelopeDecode::admitted(operation,generation,decode,completion,(cx.operation(),cx.generation())),
            Err((record, _fault, fields)) => ActiveArtifactEnvelopeDecode::rejected(operation, generation, Box::new(store::os_store::owned_schema_decode::ArtifactEnvelopeUnadmittedDecodeRejected::new(record, fields)), completion),
        };
        self.envelope_decode_jobs.insert_admitted(operation.0, active);
        Ok(ArtifactEnvelopeDecodeOperationHandle { operation, generation })
    }

    pub fn poll_artifact_envelope_decode(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> ArtifactEnvelopeDecodeOperationPoll {
        self.envelope_decode_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).map_or(ArtifactEnvelopeDecodeOperationPoll::Fault, ActiveArtifactEnvelopeDecode::poll)
    }

    /// 🧹️ Whether the decode worker session/outcome ladder is fully retired for one live handle.
    pub fn artifact_envelope_decode_worker_owners_are_terminal(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> bool {
        self.envelope_decode_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).is_none_or(ActiveArtifactEnvelopeDecode::decode_worker_owners_are_terminal)
    }

    /// 🎟️ Advances the original ready worker ladder once under the caller's unchanged grant.
    pub(crate) fn advance_ready_envelope_decode_worker_owners(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, grant: RetainedCloneGrant,cx:&mut semio_framework_job::StepContext<'_>) -> Result<PluginLifecycleStep, crate::PluginLifecycleFailure> {
        let Some(active)=self.envelope_decode_jobs.get_mut(handle.operation.0)else{return Ok(PluginLifecycleStep::Complete(Default::default()));};
        if active.operation!=handle.operation||active.generation!=handle.generation{return Err(plugin_sdk_fault("ready envelope worker advancement lost its original handle").into());}
        if active.decode_worker_owners_are_terminal(){return Ok(PluginLifecycleStep::Complete(Default::default()));}
        active.drive(cx,active.generation,&self.envelope_completed_records,grant)
    }

    /// 📤️ Applies the sole non-suspending completed-record publication after exact live
    /// generation validation. Backpressure leaves both the record and active operation retained.
    pub fn try_publish_artifact_envelope_decode(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, target: &mut dyn store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordTarget<A::Snapshot, A::Mutation>) -> Result<bool, Fault> {
        if semio_framework_job::Generation(self.store.generation_now()) != handle.generation {
            self.cancel_artifact_envelope_decode(handle)?;
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-publication"), "decoded envelope generation changed before exact publication"));
        }
        let active = self
            .envelope_decode_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-handle"), "unknown or stale envelope decode handle"))?;
        if active.state != ActiveArtifactEnvelopeDecodeState::Ready || active.cancel.is_cancelled_now() {
            return Ok(false);
        }
        let ticket = active.completion.ticket().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.output-missing"), "ready envelope decode has no exact completed-record ticket"))?;
        match self.envelope_completed_records.try_publish_to(ticket, target) {
            Ok(false) | Err(store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordFault::Contended) => Ok(false),
            Ok(true) => {
                active.state = ActiveArtifactEnvelopeDecodeState::Complete;
                if !active.terminal_is_empty(&self.envelope_completed_records) {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.publication-terminal"), "published envelope operation retained a nonterminal owner"));
                }
                let active =
                    self.envelope_decode_jobs.remove(handle.operation.0).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.publication-owner"), "published envelope operation changed before exact removal"))?;
                drop(active);
                Ok(true)
            }
            Err(_) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.publication-ticket"), "completed envelope ticket was stale or invalid")),
        }
    }

    pub fn cancel_artifact_envelope_decode(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let active = self
            .envelope_decode_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-envelope.stale-handle"), "unknown or stale envelope decode handle"))?;
        active.cancel.cancel_now();
        Ok(())
    }

    /// 🏗️ Atomically transfers one decoded envelope into the app-owned retained store
    /// initializer. Capacity or factory rejection leaves the completed record in its exact slot.
    pub fn try_begin_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle,cx:&mut semio_framework_job::StepContext<'_>) -> Result<bool, crate::PluginLifecycleFailure> {
        if semio_framework_job::Generation(self.store.generation_now()) != handle.generation {
            self.cancel_artifact_envelope_decode(handle)?;
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-stale-admission"), "store replacement generation changed before initializer admission").into());
        }
        if !self.store_replacement_jobs.can_insert(handle.operation.0) {
            return Ok(false);
        }
        let ticket = self
            .envelope_decode_jobs
            .get(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation && active.state == ActiveArtifactEnvelopeDecodeState::Ready && !active.cancel.is_cancelled_now())
            .and_then(|active| active.completion.ticket())
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-output-not-ready"), "decoded envelope is not ready for retained store initialization"))?;
        if !self.artifact_envelope_decode_worker_owners_are_terminal(handle){let _=self.advance_ready_envelope_decode_worker_owners(handle,cx.retained_grant(),cx)?;return Ok(false);}
        let child_content_generation = self.child_content_generation;
        let mut target = ArtifactStoreReplacementAdmissionTarget::<A, M> { source_identity:(cx.operation(),cx.generation()),jobs: &mut self.store_replacement_jobs, operation: handle.operation, generation: handle.generation, child_content_generation, actor: self.store.local_actor_id().clone(), marker: std::marker::PhantomData };
        match self.envelope_completed_records.try_publish_to(ticket, &mut target) {
            Ok(false) | Err(store::os_store::owned_schema_decode::ArtifactEnvelopeCompletedRecordFault::Contended) => Ok(false),
            Ok(true) => {
                if !self.artifact_envelope_decode_worker_owners_are_terminal(handle){return Err(plugin_sdk_fault("initializer transfer changed its already admitted original decode worker witness").into());}
                let active = self
                    .envelope_decode_jobs
                    .get_mut(handle.operation.0)
                    .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
                    .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-decode-owner"), "decoded envelope owner changed during exact initializer admission"))?;
                if !active.decode_worker_owners_are_terminal() {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-decode-terminal"), "decoded envelope retained an owner after initializer transfer").into());
                }
                active.state = ActiveArtifactEnvelopeDecodeState::Complete;

                Ok(true)
            }
            Err(_) => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-completed-ticket"), "decoded envelope completed-record ticket was stale or invalid").into()),
        }
    }

    /// 🌳️ Pre-admits the exact recursive member count after the retained parent initializer
    /// is terminal and before any member input may enter the candidate transaction.
    pub fn try_begin_owned_document_members(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, expected: usize, expires_at_us: u64) -> Result<bool, Fault> {
        let active = self
            .store_replacement_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| plugin_sdk_fault("owned document member ingress has no matching replacement authority"))?;
        active.begin_members(expected, expires_at_us)
    }

    /// 📥️ Transfers one admitted member-open request into its exact retained ordinal.
    #[expect(clippy::result_large_err, reason = "Rejected admission returns the exact retained member request and identity owner unchanged.")]
    pub fn admit_owned_document_member(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle, ingress: OwnedDocumentMemberIngress) -> Result<(), (Fault, OwnedDocumentMemberIngress)> {
        let Some(active) = self.store_replacement_jobs.get_mut(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation) else {
            return Err((plugin_sdk_fault("owned document member ingress has no matching replacement authority"), ingress));
        };
        active.admit_member(ingress)
    }

    /// 🔒️ Freezes the complete ordinal set so bounded typed opens can begin.
    pub fn seal_owned_document_members(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let active = self
            .store_replacement_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| plugin_sdk_fault("owned document member seal has no matching replacement authority"))?;
        active.seal_members()
    }

    pub fn poll_artifact_store_replacement(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> ArtifactEnvelopeDecodeOperationPoll {
        self.store_replacement_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).map_or(ArtifactEnvelopeDecodeOperationPoll::Fault, |active| match active.state {
            ActiveArtifactStoreReplacementState::Initializing
            | ActiveArtifactStoreReplacementState::AwaitingMembers
            | ActiveArtifactStoreReplacementState::OpeningMembers
            | ActiveArtifactStoreReplacementState::ValidatingClosure
            | ActiveArtifactStoreReplacementState::PreparingCandidateViews
            | ActiveArtifactStoreReplacementState::CandidateReady => ArtifactEnvelopeDecodeOperationPoll::Pending,
            ActiveArtifactStoreReplacementState::ClosingRejectedMember
            | ActiveArtifactStoreReplacementState::RetiringRejectedMembers
            | ActiveArtifactStoreReplacementState::RetiringCommittedMembers
            | ActiveArtifactStoreReplacementState::RetiringCommittedStore
            | ActiveArtifactStoreReplacementState::RetiringRejectedCandidate => ArtifactEnvelopeDecodeOperationPoll::Progress,
            ActiveArtifactStoreReplacementState::Complete if active.committed && active.refusal.is_none() => ArtifactEnvelopeDecodeOperationPoll::Ready,
            ActiveArtifactStoreReplacementState::Complete if matches!(active.refusal, Some(ArtifactStoreReplacementRefusal::Cancelled)) => ArtifactEnvelopeDecodeOperationPoll::Cancelled,
            ActiveArtifactStoreReplacementState::Complete => ArtifactEnvelopeDecodeOperationPoll::Fault,
        })
    }

    /// 🧭️ The exact leg that refused this replacement, minted as a typed fault. Read BEFORE
    /// {@link acknowledge_artifact_store_replacement} retires the operation — after that the
    /// evidence is gone and only the generic collapse remains.
    pub fn artifact_store_replacement_refusal(&self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Option<Fault> {
        self.store_replacement_jobs.get(handle.operation.0).filter(|active| active.operation == handle.operation && active.generation == handle.generation).and_then(|active| active.refusal).map(ArtifactStoreReplacementRefusal::into_fault)
    }

    pub fn acknowledge_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<bool, Fault> {
        let Some(active) = self.store_replacement_jobs.get(handle.operation.0) else { return Ok(false) };
        if active.operation != handle.operation || active.generation != handle.generation || !active.terminal_is_empty() {
            return Ok(false);
        }
        let active = self
            .store_replacement_jobs
            .remove(handle.operation.0)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-terminal-owner"), "terminal store replacement changed before exact acknowledgement"))?;
        drop(active);
        Ok(true)
    }

    pub fn cancel_artifact_store_replacement(&mut self, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let active = self
            .store_replacement_jobs
            .get_mut(handle.operation.0)
            .filter(|active| active.operation == handle.operation && active.generation == handle.generation)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("artifact-store.replacement-stale-handle"), "unknown or stale store replacement handle"))?;
        if !active.committed {
            active.request_cancel();
        }
        Ok(())
    }

    /// 🌱️ Completes an archive's member roster with every child the hydrated candidate parent declares,
    /// the archive omits, and the app derives (`ArtifactApp::genesis_child_pack`) — the content-addressed
    /// members a whole-document load (`Effect::LoadDocument`, which carries pack+spr only) can never
    /// ship. Each minted entry is owner-stamped to the candidate root and takes the next ordinal after the
    /// archived ones; the fixed member and byte authorities are re-checked over the completed roster. A
    /// slot the app cannot derive is left alone, so closure validation still rejects it as `Incomplete`.
    ///
    /// 🔢️ Archived entries are already reversed for ordinal-ordered popping; ordinals stay
    /// `0..n` for them and continue at `n` for every minted member, whatever the pop order.
    pub(super) fn complete_document_archive_genesis(&mut self, active: &mut ActiveDocumentArchiveLoad<A::Snapshot, A::Mutation>, handle: ArtifactEnvelopeDecodeOperationHandle) -> Result<(), Fault> {
        let archive = active.archive.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive member roster owner is absent"))?;
        let candidate = self
            .store_replacement_jobs
            .get(handle.operation.0)
            .filter(|replacement| replacement.operation == handle.operation && replacement.generation == handle.generation)
            .and_then(ActiveArtifactStoreReplacement::candidate_awaiting_members)
            .ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive genesis has no hydrated candidate parent awaiting members"))?;
        let dialect = candidate.envelope().dialect.clone().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive genesis requires the candidate parent's exact dialect"))?;
        let parent = ArtifactRef { artifact_id: candidate.envelope().id.clone(), dialect };
        let snapshot = candidate.snapshot_ref();
        let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, format!("document archive genesis child projection failed: {error}")))?;
        let mut ordinal = archive.members.len();
        let mut payload_bytes = archive.members.iter().fold(archive.parent_spr.len(), |total, entry| total + entry.envelope_pack.len());
        for index in 0..projection.len() {
            let Some((slot, fields)) = projection.get(index) else { break };
            if archive.members.iter().any(|entry| entry.owner.slot == slot && entry.owner.child_id == fields.child_id) {
                continue;
            }
            let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? else { continue };
            let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.to_string(), child_id: fields.child_id.to_string() };
            let expected = ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect: dialect.clone() };
            let envelope_pack = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack))
                .map_err(|error| plugin_sdk_fault(format!("genesis child {slot}/{} envelope: {error}", fields.child_id)))?;
            if ordinal >= protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
                return Err(document_load_fault(DOCUMENT_LOAD_TOO_MANY_MEMBERS_CODE, "document archive genesis exceeds its fixed 1024-member authority").with_param("count", (ordinal + 1).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS.to_string()));
            }
            payload_bytes = payload_bytes.checked_add(envelope_pack.len()).filter(|bytes| *bytes <= protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_TOO_LARGE_CODE, "document archive genesis exceeds its fixed byte authority").with_param("bytes", payload_bytes.saturating_add(envelope_pack.len()).to_string()).with_param("maximum", protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES.to_string()))?;
            archive.members.push(protocol::OwnedDocumentMemberPackEntry {
                ordinal: u32::try_from(ordinal).map_err(|_| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "document archive genesis ordinal exceeds u32"))?,
                reference: protocol::DocumentArchiveArtifactRef { artifact_id: fields.artifact_id.to_string(), artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() },
                owner: protocol::DocumentArchiveOwnerRef {
                    parent: protocol::DocumentArchiveArtifactRef { artifact_id: parent.artifact_id.clone(), artifact_kind: parent.dialect.artifact_kind.clone(), standard: parent.dialect.standard.clone(), subset: parent.dialect.subset.clone() },
                    slot: slot.to_string(),
                    child_id: fields.child_id.to_string(),
                },
                envelope_pack,
            });
            ordinal += 1;
            active.total = active.total.saturating_add(1);
        }
        Ok(())
    }

    /// 🌱️ Mints every derivable composed child onto a live envelope replacement's member roster —
    /// the JSON ingress path ships only the parent envelope, so `ArtifactApp::genesis_child_pack`
    /// must supply the same members `seed_genesis_children` opened at boot.
    pub(super) fn complete_store_replacement_genesis(&mut self, operation_id: u64) -> Result<(), Fault> {
        let active = self.store_replacement_jobs.get(operation_id).ok_or_else(|| plugin_sdk_fault("live store replacement genesis lost its exact replacement authority"))?;
        let handle = ArtifactEnvelopeDecodeOperationHandle { operation: active.operation, generation: active.generation };
        let candidate = active.retained_store.as_ref().ok_or_else(|| plugin_sdk_fault("live store replacement genesis has no hydrated candidate parent"))?;
        let dialect = candidate.envelope().dialect.clone().ok_or_else(|| plugin_sdk_fault("live store replacement genesis requires the candidate parent's exact dialect"))?;
        let parent = ArtifactRef { artifact_id: candidate.envelope().id.clone(), dialect };
        let snapshot = candidate.snapshot_ref();
        let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| plugin_sdk_fault(format!("live store replacement genesis child projection failed: {error}")))?;
        let mut entries = Vec::new();
        let mut ordinal = 0usize;
        for index in 0..projection.len() {
            let Some((slot, fields)) = projection.get(index) else { break };
            let Some(initial_pack) = A::genesis_child_pack(snapshot, slot, fields.child_id).map_err(|error| plugin_sdk_fault(error.into_message()))? else { continue };
            let dialect = ArtifactDialect { artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() };
            let schema = genesis_member_schema::<M>(&dialect)?;
            let owner = store::OwnerRef { parent: parent.clone(), slot: slot.to_string(), child_id: fields.child_id.to_string() };
            let expected = ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect: dialect.clone() };
            let envelope_pack = ::semio_framework_async::poll::resolve_ready(store::os_store::text_format::genesis_member_envelope_pack(schema, &expected, &owner, &initial_pack))
                .map_err(|error| plugin_sdk_fault(format!("genesis child {slot}/{} envelope: {error}", fields.child_id)))?;
            if ordinal >= protocol::DOCUMENT_ARCHIVE_MAXIMUM_MEMBERS {
                return Err(plugin_sdk_fault("live store replacement genesis exceeds its fixed 1024-member authority"));
            }
            entries.push(protocol::OwnedDocumentMemberPackEntry {
                ordinal: u32::try_from(ordinal).map_err(|_| plugin_sdk_fault("live store replacement genesis ordinal exceeds u32"))?,
                reference: protocol::DocumentArchiveArtifactRef { artifact_id: fields.artifact_id.to_string(), artifact_kind: fields.artifact_kind.to_string(), standard: fields.standard.to_string(), subset: fields.subset.to_string() },
                owner: protocol::DocumentArchiveOwnerRef {
                    parent: protocol::DocumentArchiveArtifactRef { artifact_id: parent.artifact_id.clone(), artifact_kind: parent.dialect.artifact_kind.clone(), standard: parent.dialect.standard.clone(), subset: parent.dialect.subset.clone() },
                    slot: slot.to_string(),
                    child_id: fields.child_id.to_string(),
                },
                envelope_pack,
            });
            ordinal += 1;
        }
        let active = self.store_replacement_jobs.get_mut(operation_id).ok_or_else(|| plugin_sdk_fault("live store replacement genesis changed before member admission"))?;
        active.begin_members(entries.len(), u64::MAX)?;
        for entry in entries {
            let mut pending = PendingDocumentArchiveMember::new(entry).map_err(|(fault, _)| fault)?;
            while !pending.fill_one_page(store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES)? {}
            while !matches!(pending.retire_source_step(RetainedCloneGrant { maximum_copy_bytes: store::os_store::owned_schema_decode::OWNED_SCHEMA_DECODE_PAGE_BYTES, ..plugin_demand_grant(pending.source_retirement_demands()) }), RetainedCloneStep::Complete(_)) {}
            let ingress = pending.take_ingress(handle, self.store.local_actor_id().clone()).map_err(|(fault, _)| fault)?;
            active.admit_member(ingress).map_err(|(fault, _)| fault)?;
        }
        active.seal_members()?;
        Ok(())
    }

    /// 🔀️ Merges a decoded read-back into the live document (design §22.22) in the host's own poll turn and requests the
    /// operation's terminal: `Ready` with what it took (`completed == total`) and is ahead by, or the fault that
    /// refused it. Nothing is replaced.
    pub(super) async fn merge_document_archive(&mut self, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let merged = self.merge_decoded_document_archive(operation, identity, cx).await;
        let active = self.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed during its merge"))?;
        match merged {
            Ok((taken, ahead)) => {
                active.completed = taken;
                active.total = taken;
                active.ahead = ahead;
                active.request_terminal(ActiveDocumentArchiveLoadState::Ready);
            }
            Err(fault) => active.request_fault(&fault),
        }
        Ok(())
    }

    /// 🧬️ The merge itself, answering the events taken and the events this program holds that the archive lacks: the
    /// archive's members must be the live ones (a member merge is the member's own, not yet available), its decoded
    /// log joins the store's in one store call — an archive of another document is refused — and an open history edit
    /// hears the base move like after any remote edit.
    async fn merge_decoded_document_archive(&mut self, operation: u64, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(u64, u64), Fault> {
        let live = self.archive_member_entries().await?;
        let active = self.document_archive_loads.get_mut(operation).ok_or_else(|| document_load_fault(DOCUMENT_LOAD_OPERATION_UNKNOWN_CODE, "recursive document archive merge authority changed before its merge"))?;
        let archive = active.archive.as_mut().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive merge input owner is absent"))?;
        if !live.iter().eq(archive.members.iter().rev()) {
            return Err(document_load_fault(DOCUMENT_LOAD_MEMBERS_DIFFER_CODE, "the archive's members differ from this document's live members; a member merge is not available"));
        }
        let pack = std::mem::take(&mut archive.parent_pack);
        let history = active.decoded_history.take().ok_or_else(|| document_load_fault(DOCUMENT_LOAD_FAILED_CODE, "recursive document archive decoded history owner is absent"))?;
        let generation = self.store.generation();
        let merge = match self.store.merge_persisted_history(&pack, history).await {
            Ok(merge) => merge,
            Err(store::VcsError::ValidationFailed(detail)) => return Err(document_load_fault(DOCUMENT_LOAD_OTHER_DOCUMENT_CODE, detail)),
            Err(error) => return Err(error.into_fault()),
        };
        self.cache = None;
        if self.store.generation() != generation {
            self.deliver_base_moved(identity, cx).await?;
        }
        self.follow_derivable_children(identity).await?;
        Ok((u64::try_from(merge.merged).unwrap_or(u64::MAX), u64::try_from(merge.ahead).unwrap_or(u64::MAX)))
    }

    /// ⏳️ A follow pass waits on child retirements and the parent has not been followed since — pending, runnable
    /// work until maintenance returned them (see `child_follow_awaits_retirement`).
    pub(super) fn derivable_follow_awaits_retirement(&self) -> bool {
        self.child_follow_awaits_retirement && self.store.generation_now() != self.followed_parent_generation
    }

    /// ♻️ Whether the next `roots` child-content and `members` child-member retirement generations each find a vacant
    /// slot of their fixed registry.
    fn child_retirements_admit(&self, roots: usize, members: usize) -> bool {
        (1..=roots as u64).all(|offset| self.child_content_generation.checked_add(offset).is_some_and(|generation| self.child_content_retirements.can_insert(generation)))
            && (1..=members as u64).all(|offset| self.child_member_retirement_generation.checked_add(offset).is_some_and(|generation| self.child_member_retirements.can_insert(generation)))
    }

    /// ⏳️ Queries the original fixed retirement slots; funded maintenance makes room.
    pub(super) fn reclaim_child_retirements(&self, roots: usize, members: usize) -> Result<bool, Fault> {
        if roots > ARTIFACT_LIVE_OUTPUT_SLOTS || members > ARTIFACT_LIVE_OUTPUT_SLOTS {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-retirement-span"), "one child publication span exceeds the fixed retirement authority"));
        }
        Ok(self.child_retirements_admit(roots, members))
    }

    /// 🎟️ Admits the next immutable child-content publication once its retirement slot is vacant — reclaimed first when
    /// an older retirement still holds it ([`Self::reclaim_child_retirements`]).
    pub(crate) fn admit_child_content_publication(&mut self) -> Result<u64, Fault> {
        self.admit_child_content_publication_span(1)?;
        Ok(self.child_content_generation + 1)
    }

    /// 🎟️ Admits `count` consecutive child-content publications (see [`Self::admit_child_content_publication`]). Only a
    /// ring of retirements that are every one still borrowed answers the retryable
    /// `interactive-job.child-root-retirement-saturated`.
    pub(super) fn admit_child_content_publication_span(&mut self, count: usize) -> Result<(), Fault> {
        if self.child_content_generation.checked_add(count as u64).is_none() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-generation"), "immutable child-content generation exhausted"));
        }
        if !self.reclaim_child_retirements(count, 0)? {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-retirement-saturated"), "immutable child-content publication waits on retirements that are all still borrowed").with_retryable(true));
        }
        Ok(())
    }

    pub(crate) async fn publish_child_content_member(&mut self, generation: u64, slot: &str, child_id: &str) -> Result<(), Fault> {
        self.publish_member_content(generation, MemberKeyRef::root(slot, child_id)).await
    }

    /// 📰️ Republishes the immutable child-content root with the member at one exact owner edge captured anew.
    pub(super) async fn publish_member_content(&mut self, generation: u64, key: MemberKeyRef<'_>) -> Result<(), Fault> {
        if generation != self.child_content_generation.saturating_add(1) || !self.child_content_retirements.can_insert(generation) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-publication-authority"), "immutable child-content publication lost its exact admitted generation"));
        }
        let entry = self.children.member(key).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.child-root-member"), "immutable child-content publication lost its exact live member"))?;
        let next = self.child_content_root.with_member(key, &entry.reference, &entry.member).await?;
        let previous = std::mem::replace(&mut *self.child_content_root, next);
        if previous.root.as_ref().is_some_and(|root| root.len != 0) {
            self.child_content_retirements.insert_admitted(generation, ChildContentRetirement::new(previous, false));
        }
        self.child_content_generation = generation;
        Ok(())
    }

    /// 📮️ Sends one member's events on the document's backbone under that member's lane: the owner-path text of the
    /// member that owns it (empty for the document itself) and its step there (design §21.9). A member whose owner is
    /// not live has no lane: its events are refused here, never sent under another lane and never dropped.
    pub(super) async fn send_member_lane(&mut self, lane: &MemberKey, payload: Vec<u8>) -> Result<(), Fault> {
        if payload.is_empty() || self.store.backbone_ref().is_none() {
            return Ok(());
        }
        let Some(owner) = self.children.path_of(lane.borrowed()).map(|path| path.owner_text()) else {
            return Err(plugin_sdk_fault(format!("the events of member {}/{} of member {} have no lane on the backbone", lane.slot, lane.child_id, lane.owner)));
        };
        self.store.send_member_mutations(&owner, &lane.slot, &lane.child_id, payload).await.map_err(|error| error.into_fault())
    }

    pub(super) fn peer_roster_slot(generation: u64) -> usize {
        generation as usize % ARTIFACT_LIVE_OUTPUT_SLOTS
    }

    /// 🧭️ The lowest mounted peer roster publication the ordered cursor has already passed. A
    /// publication whose generation is not ahead of `peer_roster_processed_generation` can never be the
    /// rotation's next exact generation again, so without this the maintenance stage answers
    /// `Pending { 0, 0 }` for it forever: it never reaches its outcome slot, `take_presence_outcome`
    /// never answers, and the app's close conjunction — which demands an empty publication registry —
    /// is unreachable. Walking it through the very same validate/outcome ladder lets
    /// `validate_peer_roster_publication` refuse its generation and hand the refusal to its caller.
    pub(super) fn orphaned_peer_roster_generation(&self) -> Option<u64> {
        let processed = self.peer_roster_processed_generation;
        let mut orphaned: Option<u64> = None;
        self.peer_roster_publications.each_id(|generation| {
            if generation <= processed && orphaned.is_none_or(|lowest| generation < lowest) {
                orphaned = Some(generation);
            }
        });
        orphaned
    }

    pub(super) fn validate_peer_roster_publication(&self, seq: u64, generation: u64, cancel: &semio_framework_job::CancelToken) -> Result<ValidatedPeerRosterCommit, Fault> {
        let slot = Self::peer_roster_slot(generation);
        if self.peer_roster_reservations[slot] != Some((generation, seq))
            || generation != self.peer_roster_processed_generation.checked_add(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-generation"), "peer roster publication generation exhausted"))?
            || !self.peer_presence_retirements.can_insert(generation)
            || !self.presence_peer_retirements.can_insert(generation)
            || !self.peer_roster_outcomes.can_insert(generation)
            || !self.presence_store.peer_retirement_available()
            || self.presence_store.retirement_started()
        {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-publication-authority"), "peer roster lost its exact generation, outcome, or retirement admission before publication"));
        }
        if ::semio_framework_async::poll::resolve_ready(cancel.is_cancelled()) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.peer-roster-cancelled"), "peer roster publication was cancelled before atomic commit"));
        }
        Ok(ValidatedPeerRosterCommit { generation })
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the original fixed-capacity input so its caller can retry or retire that exact owner without an extra allocation.")]
    #[expect(clippy::needless_pass_by_value, reason = "This non-Copy token conveys one-use admission authority and must be consumed by the commit, even though its identity fields are scalar.")]
    pub(super) fn publish_peer_roster_candidate_admitted(&mut self, admission: ValidatedPeerRosterCommit, candidate: PeerRosterCandidate<A::Presence>) -> Result<(), PeerRosterCandidate<A::Presence>> {
        let PeerRosterCandidate { seq, generation: candidate_generation, cancel, own_color, now_ms, root, typed } = candidate;
        let ValidatedPeerRosterCommit { generation } = admission;
        let _ = now_ms;
        let typed_retirement = match self.presence_store.publish_peer_commit(typed) {
            Ok(retirement) => retirement,
            Err(typed) => return Err(PeerRosterCandidate { seq, generation: candidate_generation, cancel, own_color, now_ms, root, typed }),
        };
        let previous = std::mem::replace(&mut *self.peer_presence, root);
        if previous.is_empty() {
            drop(previous);
        } else {
            self.peer_presence_retirements.insert_admitted(generation, PeerPresenceRootRetirement::new(previous));
        }
        self.own_color = own_color;
        self.peer_presence_generation = generation;
        if let Some(retirement) = typed_retirement {
            self.presence_peer_retirements.insert_admitted(generation, retirement);
        }
        Ok(())
    }

    fn declared_child_reference(&self, slot: &str, child_id: &str, dialect: &ArtifactDialect) -> Result<ArtifactRef, Fault> {
        let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("child member projection failed: {error}")))?;
        let fields = (0..projection.len()).filter_map(|index| projection.get(index)).find(|(declared, fields)| *declared == slot && fields.child_id == child_id).map(|(_, fields)| fields).ok_or_else(|| plugin_sdk_fault("child member identity is not declared by the current parent"))?;
        if fields.artifact_kind != dialect.artifact_kind || fields.standard != dialect.standard || fields.subset != dialect.subset {
            return Err(plugin_sdk_fault("child member dialect differs from the declared target"));
        }
        Ok(ArtifactRef { artifact_id: fields.artifact_id.to_owned(), dialect: dialect.clone() })
    }

    async fn admit_child_member(&mut self, slot: String, child_id: String, dialect: ArtifactDialect) -> Result<ChildStoreAdmission, Fault> {
        let expected = self.declared_child_reference(&slot, &child_id, &dialect)?;
        let root_index = self.child_content_root.admit_member(MemberKeyRef::root(&slot, &child_id))?;
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(plugin_sdk_fault("owned member admission requires the parent's exact declared dialect"));
        }
        let generation = self.admit_child_content_publication()?;
        let parent_id = self.store.envelope().id.clone();
        let graph = self.composition.graph_mut().await.admit_owns(&parent_id, &slot, &expected.artifact_id).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
        let owner = store::OwnerRef { parent: ArtifactRef { artifact_id: parent_id, dialect: parent_dialect }, slot: slot.clone(), child_id: child_id.clone() };
        let key = MemberKey::root(slot, child_id);
        let member = self.children.admit_identity(key.borrowed())?;
        Ok(ChildStoreAdmission { key, expected, owner, member, graph, root_index, generation })
    }

    async fn prepare_child_member(&self, admission: &ChildStoreAdmission, member: &mut M, restored: bool, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(ChildContentView, Option<usize>), Fault> {
        let owner = member.owner_ref();
        if member.artifact_ref().as_ref() != Some(&admission.expected) || (owner.as_ref() != Some(&admission.owner) && (restored || owner.is_some())) {
            return Err(plugin_sdk_fault("child member does not match its exact admitted identity and ownership"));
        }
        let pin_index = self.pending_child_pins.iter().position(|pin| pin.child_ref.artifact_id == admission.expected.artifact_id);
        if let Some(index) = pin_index {
            if !restored {
                return Err(plugin_sdk_fault("direct child registration cannot consume a queued restore checkpoint"));
            }
            let pin = &self.pending_child_pins[index];
            if pin.child_ref != admission.expected {
                return Err(plugin_sdk_fault("pending child checkpoint belongs to a different exact dialect"));
            }
            let alternative = member.current_alternative_id().await.ok_or_else(|| plugin_sdk_fault("pending child checkpoint has no live alternative authority"))?;
            member.checkout(&pin.checkpoint_id, &alternative, identity).await.map_err(|error| plugin_sdk_fault(error.to_string()))?;
        }
        let root = self.child_content_root.capture_member_admitted(admission.root_index, admission.key.borrowed(), &admission.expected, member).await?;
        Ok((root, pin_index))
    }

    async fn commit_child_member(&mut self, admission: ChildStoreAdmission, mut member: M, root: ChildContentView, pin_index: Option<usize>) {
        member.set_owner(Some(admission.owner.clone())).await;
        self.composition.graph_mut().await.commit_owns_admitted(admission.graph);
        self.children.insert_admitted(admission.member, admission.expected, admission.owner, member);
        let previous = std::mem::replace(&mut *self.child_content_root, root);
        if !previous.is_empty() {
            self.child_content_retirements.insert_admitted(admission.generation, ChildContentRetirement::new(previous, false));
        }
        self.child_content_generation = admission.generation;
        if let Some(index) = pin_index {
            self.pending_child_pins.remove(index);
        }
    }

    #[cfg(test)]
    pub(in super::super) fn test_child_admission_state(&self, abort_generation: u64) -> ChildAdmissionTestState {
        ChildAdmissionTestState {
            children_empty: self.children.is_empty(),
            generation: self.child_content_generation,
            abort_generation: self.child_member_retirement_generation,
            abort_empty: self.child_member_retirements.is_empty(),
            requested_abort_retained: self.child_member_retirements.get(abort_generation).is_some(),
            content_empty: self.child_content_root.is_empty(),
            roots_retiring_empty: self.child_content_retirements.is_empty(),
        }
    }

    /// ♻️ The ONE release site of an admitted typed-operation slot. Every outcome an operation can reach
    /// — settled, faulted, refused, replaced, cancelled — funnels its mounted owner into
    /// [`MountedTypedCommandFullOperationStage::Retiring`], and this is the only code that may remove it
    /// from `tool_operations` and so free its residue class for [`Self::admit_typed_operation_slot`]: one
    /// bounded retirement unit, the terminal-empty witness, then the exact removal. The turn driver
    /// ([`Self::advance_typed_operation_publication_one`]), the cooperative maintenance rotation and the
    /// app close ladder all walk THIS function, so no authority can admit a slot and leave its release to
    /// another (ticket 26/09/02/PUZZLE-3D-END-TO-END wave B8).
    /// 🧾️ The migrated twin of [`Self::dispatch_emit`]'s own command-log append. A typed command's
    /// emit reaches its stores through the mounted publication ladder, which never walks
    /// `dispatch_emit`, so before this every migrated dispatch reached the history panel only as
    /// [`Self::backfill_command_log`]'s anonymous `apply` row — losing the verb, its
    /// declared [`ActionKind`], its own label and its revert target — and a dispatch that published
    /// no durable lane at all (a `Mutation` verb whose reducer emitted nothing, every `View` verb
    /// such as `select`) reached it not at all.
    ///
    /// It runs on the durable RECEIPT, not at retirement, because the operation mints its `Ui` page
    /// between the two and rendering the history body backfills: an edit this row owns would already
    /// be filed under `apply` by the time the slot retired. Only the document lane opens or amends a row: a config-lane
    /// edit is never a history row (design §20.13, L4). A coalesced gesture amends the edit its first dispatch already
    /// logged, so later dispatches append no second row — and they mark that row dirty, so the admitting completion
    /// re-upserts it.
    pub(super) fn record_typed_operation_lane(&mut self, mounted: &mut MountedTypedCommandFullOperation<A>) {
        if mounted.terminal_fault.is_some() {
            return;
        }
        let Some(edit_id) = self.store.envelope().vcs.edits.last().map(|edit| edit.id.clone()) else { return };
        let logged_seq = self.command_log.iter().find(|entry| entry.edit_id.as_deref() == Some(edit_id.as_str())).map(|entry| entry.seq);
        if let Some(seq) = logged_seq {
            mounted.command_logged = true;
            self.history_dirty_sequences.insert(seq);
            return;
        }
        if mounted.command_logged {
            if let Some(entry) = self.command_log.last_mut().filter(|entry| entry.action_id == mounted.verb) {
                entry.edit_id = Some(edit_id);
                self.history_dirty_sequences.insert(entry.seq);
                self.log_generation += 1;
            }
            return;
        }
        mounted.command_logged = true;
        let kind = self.typed_operation_command_kind(&mounted.verb, true);
        let verb = mounted.verb.clone();
        let label = self.verb_label(&verb);
        self.record_command(&verb, kind, label, Some(edit_id), Vec::new(), None);
    }

    /// 📨️ Ends the event's window's open gesture with the fact's reason (the runtime's ONE mapping, design §22.10), then
    /// delivers `event` to the app: its typed answer ([`ArtifactApp::host_event`]) dispatches to the event's window —
    /// addressed through `meta`'s view when it holds that window, else through the last roster — as an ordinary typed
    /// operation that is never a history row of its own. An event the app ignores, or for a window no view holds,
    /// dispatches nothing.
    async fn deliver_host_event(&mut self, event: HostEvent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        self.end_window_gesture(&event);
        let Some(command) = A::host_event(&event) else { return Ok(()) };
        let window_id = event.window_id();
        let view = meta.view_state.as_ref().and_then(|view| view.for_window_instance(window_id)).or_else(|| ViewModel { window_instances: self.host_event_roster.clone(), ..meta.view_state.as_ref()?.clone() }.for_window_instance(window_id));
        let Some(view) = view else { return Ok(()) };
        let window_meta = ActionMeta { view_state: Some(view), ..meta.clone() };
        let verb = A::command_id(&command).await;
        let payload = ::protocol::OpBinary::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.admit_command_wire(verb, &payload, 1).await?;
        self.delivering_host_event = true;
        let result = Box::pin(self.dispatch_typed_command_inner(Box::new(command), admission, &window_meta, identity, cx)).await;
        self.delivering_host_event = false;
        result.map(|_| ())
    }

    /// 🧰️ Delivers [`HostEvent::UtilityChanged`] for every window whose utility `meta`'s view moved since a dispatch
    /// last showed it — the targeted window reads the per-call overlay, every other one the view's map, and a window
    /// the view names no utility for is unknown, never a switch — and remembers the view's window roster.
    pub(super) async fn deliver_utility_host_events(&mut self, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let Some(view) = meta.view_state.as_ref().filter(|_| !self.delivering_host_event) else { return Ok(()) };
        if !view.window_instances.is_empty() && self.host_event_roster != view.window_instances {
            self.host_event_roster = view.window_instances.clone();
            self.host_event_utilities.retain(|window_id, _| view.window_instances.iter().any(|window| window.id == *window_id));
        }
        let mut switched = Vec::new();
        for window in &view.window_instances {
            let utility = if view.window_id.as_deref() == Some(window.id.as_str()) { Some(view.active_utility_id.as_deref()) } else { view.active_utility_by_window_id.get(&window.id).map(|utility| Some(utility.as_str())) };
            let Some(utility) = utility else { continue };
            match self.host_event_utilities.get_mut(&window.id) {
                Some(previous) if previous.as_deref() == utility => {}
                Some(previous) => {
                    let from = std::mem::replace(previous, utility.map(str::to_string));
                    switched.push(HostEvent::UtilityChanged { window_id: window.id.clone(), from, to: utility.map(str::to_string) });
                }
                None => {
                    self.host_event_utilities.insert(window.id.clone(), utility.map(str::to_string));
                }
            }
        }
        for event in switched {
            self.deliver_host_event(event, meta, identity, cx).await?;
        }
        Ok(())
    }

    /// 🌐️ Delivers one instance-wide event, built per window, to every window of `meta`'s view, else of the last roster.
    pub(super) async fn deliver_host_event_to_every_window(&mut self, event: impl Fn(String) -> HostEvent, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        self.end_every_gesture(&event(String::new()));
        let roster = meta.view_state.as_ref().map(|view| view.window_instances.as_slice()).filter(|roster| !roster.is_empty()).unwrap_or(&self.host_event_roster);
        let windows: Vec<String> = roster.iter().map(|window| window.id.clone()).collect();
        for window_id in windows {
            self.deliver_host_event(event(window_id), meta, identity, cx).await?;
        }
        Ok(())
    }

    /// 🌐️ Delivers [`HostEvent::BaseMoved`] to every window once a remote edit moved the base, as the local actor.
    pub(super) async fn deliver_base_moved(&mut self, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<(), Fault> {
        let Some(instance_id) = self.live_runtime_instance_id else { return Ok(()) };
        let meta = ActionMeta { actor: self.store.local_actor_id().0.clone(), instance_id, view_state: None };
        self.deliver_host_event_to_every_window(|window_id| HostEvent::BaseMoved { window_id }, &meta, identity, cx).await
    }

    /// 📨️ `hostEvent{windowId, kind}`: the host-forwarded window fact, delivered at once; an unknown kind or a
    /// missing window is refused.
    pub(super) async fn dispatch_host_event_action(&mut self, args: Option<&DslValue>, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let text = |key: &str| args.and_then(|value| value.get(key)).and_then(DslValue::as_str);
        let event = text(semio_framework::HOST_EVENT_ARG_WINDOW_ID).zip(text(semio_framework::HOST_EVENT_ARG_KIND)).and_then(|(window_id, kind)| HostEvent::forwarded(window_id, kind));
        let Some(event) = event else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("hostEvent.invalid"), "hostEvent requires a windowId and a kind of blur, captureLost or retiring"));
        };
        self.deliver_host_event(event, meta, identity, cx).await?;
        let ui_scope = if self.take_gesture_ended() { UiDirtyScope::Full } else { UiDirtyScope::None };
        Ok(Self::empty_result(semio_framework::HOST_EVENT_ACTION_ID, meta, Vec::new(), Vec::new(), ui_scope).await)
    }

    /// 🧾️ A settled typed operation that published no document lane and no shared-config lane still
    /// owes the history panel one row, on the admitting completion: `dispatch_emit`'s empty
    /// `artifact_mutations` branch files the verb under its declared kind (`View` for `select`,
    /// `Mutation` for a reducer that emitted nothing, `edit_id: None`). Recording that row only from
    /// retirement lands it after [`Self::take_typed_operation_completion`] has built `history_patch`,
    /// so the row surfaced on the next command. Both the completion take and
    /// [`Self::retire_typed_operation_unit`] call this; `command_logged` makes the second a no-op.
    /// Silent for a faulted operation, exactly as `dispatch_emit`'s `Err` path is, and for a `View`
    /// whose only durable emission is the per-window config lane — an orbit tick is not a command.
    pub(super) fn record_settled_typed_operation_command(&mut self, operation_id: u64) {
        if self.host_event_operations.remove(&operation_id) {
            if let Some(operation) = self.tool_operations.get_mut(operation_id) {
                operation.command_logged = true;
            }
            return;
        }
        let logged = {
            let Some(operation) = self.tool_operations.get(operation_id) else { return };
            if operation.command_logged || operation.terminal_fault.is_some() || operation.published_artifact || operation.published_config {
                return;
            }
            let kind = self.typed_operation_command_kind(&operation.verb, false);
            if operation.published_window_config && matches!(kind, ActionKind::View) {
                return;
            }
            (operation.verb.clone(), kind)
        };
        let (verb, kind) = logged;
        if let Some(operation) = self.tool_operations.get_mut(operation_id) {
            operation.command_logged = true;
        }
        self.record_command(&verb, kind, None, None, Vec::new(), None);
    }

    /// 🧾️ The declared [`ActionKind`] one migrated verb logs under — the same three-step resolution
    /// [`Self::dispatch_emit`] runs, with the same last resort: an unresolved verb that reached a
    /// document store is a `Mutation`, one that reached nothing is a `View`.
    fn typed_operation_command_kind(&self, verb: &str, artifact_lane: bool) -> ActionKind {
        match self.invocation_kind {
            Some(kind) => kind,
            None => match self.registry.get(verb).map(|definition| definition.kind) {
                Some(kind) => kind,
                None => match self.registry.get_command(verb) {
                    Some(definition) => definition.kind,
                    None if artifact_lane => ActionKind::Mutation,
                    None => ActionKind::View,
                },
            },
        }
    }

    /// 📏️ Quotes the next retiring typed-operation turn: its private child group's close, one owner hand-off, or the operation's own retirement.
    pub(crate) fn typed_operation_retirement_demands(&self, operation_id: u64, body: usize) -> Result<RetirementDemand, Fault> {
        let operation = self.tool_operations.get(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "typed operation owner changed before its retirement quote"))?;
        if operation.close_fault_pending(){return operation.close_fault_demands(body).map_err(plugin_retirement_fault);}
        if let Some(group) = operation.owned_child_group {
            if self.private_child_groups.get(group).is_some() { return self.private_child_group_operation_close_demands(group, body).map_err(plugin_retirement_fault); }
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if operation.lease_retirement_ready(){return operation.lease_retirement_demands(&self.tool_cancellations,body).map_err(plugin_retirement_fault);}
        operation.retirement_demands(body).map_err(plugin_retirement_fault)
    }

    #[inline(never)]
    pub(crate) fn retire_typed_operation_unit(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        let grant=cx.retained_grant();let demand=self.typed_operation_retirement_demands(operation_id,grant.maximum_copy_bytes)?;
        semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|self.retire_typed_operation_granted(operation_id,same,actual).map(|step|(step,false)))
    }

    fn retire_typed_operation_granted(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>,grant:RetainedCloneGrant)->Result<PluginLifecycleStep,crate::PluginLifecycleFailure>{
        self.request_gesture_cancellation(operation_id);
        let item = RetainedCloneProgress { copied_items: 1, ..Default::default() };
        if self.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::close_fault_pending){return self.tool_operations.get_mut(operation_id).unwrap().close_fault_step(grant).map(|step|PluginLifecycleStep::Progress(step.progress())).map_err(Into::into);}
        if let Some(group) = self.tool_operations.get(operation_id).and_then(|operation| operation.owned_child_group) {
            if self.private_child_groups.get(group).is_some() {
                return Ok(match self.close_private_child_group_operation_step(group,cx,grant)? {
                    semio_framework_job::InteractiveJobCloseStep::Pending { progress } | semio_framework_job::InteractiveJobCloseStep::Complete { progress } => PluginLifecycleStep::Progress(progress),
                    semio_framework_job::InteractiveJobCloseStep::Blocked => PluginLifecycleStep::Blocked { reason: "private child group close awaits its owner" },
                });
            }
            if grant.maximum_items == 0 { return Ok(PluginLifecycleStep::Progress(RetainedCloneProgress::default())); }
            self.tool_operations.get_mut(operation_id).unwrap().owned_child_group = None;
            return Ok(PluginLifecycleStep::Progress(item));
        }
        if self.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::lease_retirement_ready){let step=self.tool_operations.get_mut(operation_id).unwrap().lease_retirement_step(&self.tool_cancellations,grant).map_err(plugin_retirement_fault)?;return Ok(PluginLifecycleStep::Progress(step.progress()));}
        self.record_settled_typed_operation_command(operation_id);
        let operation = self
            .tool_operations
            .get_mut(operation_id)
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "typed operation owner changed during one bounded retirement unit"))?;
        let step = operation.retirement_step(cx)?;
        let PluginLifecycleStep::Complete(progress) = step else { return Ok(step) };
        if !self.tool_operations.get(operation_id).is_some_and(MountedTypedCommandFullOperation::terminal_is_empty) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-terminal"), "typed operation reported Complete without exact terminal emptiness").into());
        }
        let operation =
            self.tool_operations.remove(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "terminal typed operation changed before exact removal"))?;
        self.operation_progress_retired |= operation.progress.is_some();
        drop(operation);
        self.typed_inline_interaction_verbs.retain(|(operation, _)| *operation != operation_id);
        Ok(PluginLifecycleStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }))
    }

    /// ♻️ Drives ONE retiring typed operation for a bounded slice of this turn, the exact twin of the
    /// slice [`Self::drive_typed_operation_worker`] gives a Worker-stage operation — same ladder, same
    /// owner, same `INTERACTIVE_TURN_WORKER_PUMPS`/`INTERACTIVE_TURN_WORKER_WALL_US` bound. Without it a
    /// `Retiring` operation answered `has_runnable_typed_operations` forever while the turn driver advanced nothing
    /// for it: the slot was released only by `maintenance_step` stage 0 of [`MAINTENANCE_STAGES`], one
    /// unit per rotation on the shared live-cleanup job, so a sustained action rate filled all
    /// [`ARTIFACT_LIVE_OUTPUT_SLOTS`] classes and every later action was refused
    /// `interactive-job.typed-operation-capacity`.
    pub(super) fn retire_typed_operation_run(&mut self,operation_id:u64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),crate::PluginLifecycleFailure>{
        let started_us=cx.now_us();
        for _ in 0..INTERACTIVE_TURN_WORKER_PUMPS {
            if !self.tool_operations.get(operation_id).is_some_and(|operation| operation.stage == MountedTypedCommandFullOperationStage::Retiring) {
                return Ok(());
            }
            if self.tool_operations.get(operation_id).is_some_and(|operation|operation.granted_retirement_ready()&&operation.granted_retirement_pending()){
                let grant=crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.close,cx.retained_grant());
                let demand=self.tool_operations.get(operation_id).unwrap().granted_retirement_demands(grant.maximum_copy_bytes)?;let step=semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,actual|{let step=self.tool_operations.get_mut(operation_id).unwrap().granted_retirement_step(same,actual)?;Ok((PluginLifecycleStep::Progress(step.progress()),false))})?;let step=RetainedCloneStep::Progress(step.progress().unwrap_or_default());
                if !step.progress().fits(grant){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed operation retirement exceeded its issuer grant").with_retained_progress(step.progress()).into());}
                let progress=step.progress();if progress==RetainedCloneProgress::default(){return Ok(());}
                let owner=self.tool_operations.get_mut(operation_id).unwrap();assert!(owner.original_retirement_receipt.is_none(),"original mounted Close receipt cannot be overwritten before acknowledgement");owner.original_retirement_receipt=Some((grant,progress));return Ok(());

            }
            let grant = crate::plugin_runtime::original_plugin_turn_grant(self.mounted_policy.close, cx.retained_grant());
            let demand = self.typed_operation_retirement_demands(operation_id, grant.maximum_copy_bytes)?;
            if !plugin_grant_funds(demand, grant) {
                return Ok(());
            }
            match cx.with_retained_loan(grant,|same|self.retire_typed_operation_unit(operation_id,same).map_err(Into::into))? {
                PluginLifecycleStep::Blocked { .. } | PluginLifecycleStep::AwaitingInput { .. } => return Ok(()),
                PluginLifecycleStep::Progress(progress) | PluginLifecycleStep::Complete(progress) => {
                    if !progress.fits(grant) {
                        return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"typed operation retirement exceeded its issuer grant").with_retained_progress(progress).into());
                    }
                    if progress == RetainedCloneProgress::default() {
                        return Ok(());
                    }
                }
            }
            if cx.should_yield()||started_us.is_some_and(|started_us|cx.now_us().is_some_and(|now_us|now_us.saturating_sub(started_us)>=INTERACTIVE_TURN_WORKER_WALL_US)) {
                return Ok(());
            }
        }
        Ok(())
    }

    /// 📐️ Quotes the original registry and its real child snapshot owners; an empty registry owes only its backing, and only once the close has detached the child root that still inserts its retirement.
    fn child_root_retirement_quote(&self, body: usize, closing: bool) -> Result<Option<RetirementDemand>, ValueError> {
        original_child_root_retirement_demands(&self.child_content_retirements, &self.children, Some(&self.child_member_retirements), self.maintenance_child_root_cursor, body, closing)
    }

    /// 📏️ Quotes the next child-root retirement turn; nothing is owed once the registry and its backing are gone.
    pub(super) fn child_root_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(self.child_root_retirement_quote(body, self.close_child_root_detached)?.unwrap_or_default())
    }

    /// 🎟️ Advances one child-root retirement under the caller's unchanged grant.
    pub(super) fn child_root_retirement_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, Fault> {
        self.child_root_retirement_unit(grant, self.close_child_root_detached).map(|step| step.progress())
    }

    /// 🌳️ Supplies the exact live and saved previews to the common original child disposer.
    fn child_root_retirement_unit(&mut self, grant: RetainedCloneGrant, closing: bool) -> Result<RetainedCloneStep, Fault> {
        let member = self.time_travel.member.as_ref().and_then(|member| member.children.as_ref());
        let review = self.time_travel.member.as_ref().and_then(|member| member.review_children.as_ref().map(|review|&review.view));
        advance_original_child_root_retirement(&mut self.child_content_retirements, &mut self.children, Some(&mut self.child_member_retirements), &self.child_content_root, member, review, &mut self.maintenance_child_root_cursor, grant, closing)
    }

    /// 📏️ Quotes the next child-member retirement turn; nothing is owed once the registry and its backing are gone.
    pub(super) fn child_member_retirement_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(self.child_member_retirement_quote(body, self.close_child_root_detached)?.unwrap_or_default())
    }

    /// 🎟️ Advances one child-member retirement under the caller's unchanged grant.
    pub(super) fn child_member_retirement_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, Fault> {
        self.child_member_retirement_unit(grant, self.close_child_root_detached).map(|step| step.progress())
    }

    fn child_member_retirement_quote(&self,body:usize,closing:bool)->Result<Option<RetirementDemand>,ValueError>{
        let Some((_,generation))=self.child_member_retirements.next_id_from(self.child_member_retirement_cursor)else{let release_bytes=self.child_member_retirements.empty_backing_byte_demand().unwrap_or(0);return Ok((closing&&release_bytes!=0).then_some(RetirementDemand{copy_bytes:size_of_val(&self.child_member_retirements),release_bytes,depth:1,..Default::default()}))};let retirement=self.child_member_retirements.get(generation).unwrap();let parent_copy=size_of::<usize>();
        if retirement.terminal_is_empty(){return Ok(Some(RetirementDemand{copy_bytes:parent_copy+size_of::<Option<(u64,ChildMemberRetirement<M>)>>(),depth:1,..Default::default()}))}
        ChildContentRetirement::parent_snapshot_demand(retirement.retirement_demands(body.saturating_sub(parent_copy))?,parent_copy).map(Some)
    }

    fn child_member_retirement_unit(&mut self,grant:RetainedCloneGrant,closing:bool)->Result<RetainedCloneStep,Fault>{
        let empty=RetainedCloneProgress::default();let Some(demand)=self.child_member_retirement_quote(grant.maximum_copy_bytes,closing).map_err(|error|plugin_sdk_fault(error.into_message()))?else{return Ok(RetainedCloneStep::Progress(empty))};
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(empty))}
        let Some((index, generation)) = self.child_member_retirements.next_id_from(self.child_member_retirement_cursor) else {
            let backing=std::mem::take(&mut self.child_member_retirements.slots);self.child_member_retirements.allocation_admitted=false;drop(backing);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..empty}));
        };
        let retirement = self.child_member_retirements.get_mut(generation).expect("exact child-member retirement remains admitted");
        if retirement.entry.as_ref().is_some_and(|entry| !entry.member.snapshot_read_leases_terminal_is_empty()) {
            return Ok(RetainedCloneStep::Progress(empty));
        }
        if retirement.terminal_is_empty(){drop(self.child_member_retirements.remove(generation));self.child_member_retirement_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}))}
        let parent_copy=size_of::<usize>();let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-parent_copy,maximum_depth:grant.maximum_depth-1,..grant};let step=retirement.close_step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,retirement.terminal_is_empty(),"original child member registry").map_err(|error|plugin_sdk_fault(error.into_message()))?;
        let mut receipt=step.progress();if receipt!=empty{self.child_member_retirement_cursor=(index+1)%ARTIFACT_LIVE_OUTPUT_SLOTS;receipt.copied_bytes=receipt.copied_bytes.checked_add(parent_copy).expect("original child registry cursor preflight")}
        let receipt=semio_framework_value::retained_clone::admit_retained_clone_progress(grant,receipt,"original child member registry parent").map_err(|error|plugin_sdk_fault(error.into_message()))?;Ok(RetainedCloneStep::Progress(receipt))
    }

    fn validate_parent_child_restore(&self, slot: &str, child_id: &str, expected: &ArtifactRef) -> Result<(), Fault> {
        let parent_dialect: ArtifactDialect = A::DIALECT.into();
        if self.store.envelope().dialect.as_ref() != Some(&parent_dialect) {
            return Err(plugin_sdk_fault("child restore requires the parent's exact declared dialect"));
        }
        let parent = ArtifactRef { artifact_id: self.store.envelope().id.clone(), dialect: parent_dialect };
        let source = DocumentClosureSourceView::new(self.store.generation_now(), &parent, self.store.snapshot_ref(), &self.children);
        let projection = store::OwnedDocumentClosureSource::child_projection(&source, None).map_err(|error| plugin_sdk_fault(format!("child restore projection failed: {error:?}")))?;
        if !(0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot && fields.child_id == child_id && fields.artifact_id == expected.artifact_id && fields.artifact_kind == expected.dialect.artifact_kind && fields.standard == expected.dialect.standard && fields.subset == expected.dialect.subset) {
            return Err(plugin_sdk_fault("child restore is not declared by the loaded parent snapshot"));
        }
        Ok(())
    }

    /// 🌱️ Restores one exact owned member and publishes its graph, map and snapshot together.
    pub async fn open_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, envelope_pack: &[u8], identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let slot = slot.into();
        let child_id = child_id.into();
        let expected = self.declared_child_reference(&slot, &child_id, &dialect)?;
        self.validate_parent_child_restore(&slot, &child_id, &expected)?;
        let parent_generation = self.store.generation_now();
        let abort_generation = self.child_member_retirement_generation.checked_add(1).ok_or_else(|| plugin_sdk_fault("failed child retirement generation exhausted"))?;
        if !self.reclaim_child_retirements(0, 1)? {
            return Err(plugin_sdk_fault("failed child retirement waits on retirements that are all still borrowed").with_retryable(true));
        }
        let admission = self.admit_child_member(slot, child_id, dialect).await?;
        let mut member = match M::open(&admission.expected, Some(&admission.owner), envelope_pack, self.store.local_actor_id().clone()).await {
            Ok(member) => member,
            Err(error) => {
                assert!(self.children.cancel_admission(&admission.member), "failed child open retains its exact admission until cancellation");
                return Err(plugin_sdk_fault(error.to_string()));
            }
        };
        let prepared = if self.store.generation_now() != parent_generation {
            Err(plugin_sdk_fault("loaded parent changed during child restore"))
        } else {
            match self.validate_parent_child_restore(&admission.key.slot, &admission.key.child_id, &admission.expected) {
                Ok(()) => self.prepare_child_member(&admission, &mut member, true, identity).await,
                Err(fault) => Err(fault),
            }
        };
        let (root, pin_index) = match prepared {
            Ok(prepared) => prepared,
            Err(fault) => {
                assert!(self.children.cancel_admission(&admission.member), "failed child preparation returns its exact map admission");
                self.child_member_retirements.insert_admitted(abort_generation, ChildMemberRetirement::new(ChildMemberEntry { reference: admission.expected, owner: admission.owner, member }));
                self.child_member_retirement_generation = abort_generation;
                return Err(fault);
            }
        };
        self.commit_child_member(admission, member, root, pin_index).await;
        Ok(())
    }

    /// 🔎️ The live child store for `(slot, child_id)`, if adopted/created — the read half of
    /// `open_child`/`register_child`.
    pub async fn child_store(&self, slot: &str, child_id: &str) -> Option<&M> {
        self.children.get(&(slot.to_string(), child_id.to_string())).map(|entry| &entry.member)
    }

    /// 📣️ Announces every live composed member's whole event log on the PARENT's backbone
    /// under that member's own `(slot, child_id)` lane. A composed document owns exactly one
    /// replica endpoint — `ArtifactStore::announce_history` covers the parent lane alone, so
    /// without this a joining replica never learns a single child-lane edit and the two replicas
    /// diverge on the very content a composing plugin keeps in its children (measured on
    /// `🌊️flow`'s `content` child, ticket 26/09/19 `📓️flow.md` §5.3).
    pub(super) async fn announce_member_event_logs(&mut self) -> Result<(), Fault> {
        if self.children.is_empty() {
            return Ok(());
        }
        let lanes: Vec<MemberKey> = self.children.keyed_entries().map(|(key, _)| key.to_key()).collect();
        for lane in lanes {
            let Some(entry) = self.children.member_mut(lane.borrowed()) else { continue };
            let payload = SpaceMember::event_log_payload(&entry.member).await.map_err(|error| error.into_fault())?;
            self.send_member_lane(&lane, payload).await?;
        }
        Ok(())
    }

    /// 📤️ Announces the tail edit of every member a composite gesture just touched, under
    /// that member's own lane. Called from the ONE choke point child edits land at
    /// (`dispatch_emit_group`), so a child-lane edit travels exactly like a parent-lane one.
    pub(super) async fn announce_member_tail_edits(&mut self, lanes: &[(String, String)]) -> Result<(), Fault> {
        if self.store.backbone_ref().is_none() {
            return Ok(());
        }
        for (slot, child_id) in lanes {
            let Some(entry) = self.children.get_mut(&(slot.clone(), child_id.clone())) else { continue };
            let payload = SpaceMember::announce_tail_edit_payload(&mut entry.member).await.map_err(|error| error.into_fault())?;
            self.store.send_member_mutations("", slot, child_id, payload).await.map_err(|error| error.into_fault())?;
        }
        Ok(())
    }

    /// 📥️ Routes every member-addressed backbone message the parent store pumped into the
    /// live member that owns that exact lane, and answers how many lanes folded something. Runs after the
    /// derivable children followed the freshly folded parent, so a lane for the child a parent op just named
    /// finds its member. A lane naming a member the CURRENT parent declares but this replica does not hold is
    /// a fault, never a silent drop: the two replicas would otherwise disagree about the document's own
    /// composition. A lane naming a child the parent no longer declares is a superseded content address (a
    /// peer's lane for the child its own later edit — or ours — replaced), and a lane naming a child the
    /// follow pass has not opened yet because it waits on retirements is the child that pass derives: both
    /// carry nothing the composition does not already hold and are skipped.
    pub(super) async fn fold_member_inbound(&mut self) -> Result<usize, Fault> {
        let inbound = self.store.take_member_inbound();
        if inbound.is_empty() {
            return Ok(0);
        }
        let mut folded = 0usize;
        for (owner, slot, child_id, envelopes) in inbound {
            let lane = MemberPath::under(&owner, slot.as_str(), child_id.as_str()).and_then(|path| self.children.resolve(&path).map(|(key, _)| key));
            let entry = match lane.as_ref() {
                Some(key) => self.children.member_mut(key.borrowed()),
                None => None,
            };
            let Some(entry) = entry else {
                let declared = if owner.is_empty() {
                    let projection = semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(self.store.snapshot_ref()).map_err(|error| plugin_sdk_fault(format!("member lane projection failed: {error}")))?;
                    (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot.as_str() && fields.child_id == child_id.as_str())
                } else {
                    match MemberPath::parse(&owner).and_then(|path| self.children.resolve(&path)) {
                        Some((_, owning)) => {
                            let projection = owning.member.child_restore_projection().map_err(|error| plugin_sdk_fault(format!("member lane projection failed: {error}")))?;
                            (0..projection.len()).filter_map(|index| projection.get(index)).any(|(declared, fields)| declared == slot.as_str() && fields.child_id == child_id.as_str())
                        }
                        None => false,
                    }
                };
                if declared && !self.derivable_follow_awaits_retirement() {
                    let lane = if owner.is_empty() { format!("{slot}/{child_id}") } else { format!("{owner}/{slot}/{child_id}") };
                    return Err(plugin_sdk_fault(format!("backbone member lane {lane} names no live composed member of this replica")));
                }
                continue;
            };
            SpaceMember::ingest_remote_payload(&mut entry.member, &envelopes).await.map_err(|error| error.into_fault())?;
            folded += 1;
        }
        self.cache = None;
        Ok(folded)
    }

    /// 🌱️ Adopts an already-live `M` member into the child-store map directly — the
    /// counterpart `dispatch_emit_group` uses to absorb a `store::os_store::composition_coordinator::GroupReceipt::created_children`
    /// entry (a `ChildGenesis`-minted member has no prior caller-held reference an `open_child`
    /// call could have produced it from) and the general escape hatch for a caller that already
    /// holds a constructed member from elsewhere (e.g. a host-level `SpaceHost`).
    /// 🌱️ Adopts an already-built child store under `(slot, child_id)`. Like [`Self::open_child`]
    /// it MUST seed the ownership edge into `self.composition`'s graph: `dispatch_group`'s phase 1
    /// ownership check (`CompositionGraph::owner_of`) is fail-closed, so a child present in
    /// `self.children` but absent from the graph is rejected as an `OwnershipViolation` the first
    /// time anything dispatches against it. Registering in one place and validating from the other
    /// is precisely the inconsistency this seeds away.
    pub async fn register_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, mut member: M, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), ChildMemberRegistrationError<M>> {
        let admission = match self.admit_child_member(slot.into(), child_id.into(), dialect).await {
            Ok(admission) => admission,
            Err(fault) => return Err(ChildMemberRegistrationError { member: Some(member), fault }),
        };
        let (root, pin_index) = match self.prepare_child_member(&admission, &mut member, false, identity).await {
            Ok(prepared) => prepared,
            Err(fault) => {
                assert!(self.children.cancel_admission(&admission.member), "failed child preparation returns its exact map admission");
                return Err(ChildMemberRegistrationError { member: Some(member), fault });
            }
        };
        self.commit_child_member(admission, member, root, pin_index).await;
        Ok(())
    }

    /// 📌️ Leaves-first half of a composing document's checkpoint: commits every DIRTY
    /// child (a clean child is already pinned at its current checkpoint, and committing it again
    /// would mint an empty checkpoint per parent commit) and returns the `CompositionPin`s the
    /// parent's own about-to-be-created checkpoint must carry.
    ///
    /// A child with no checkpoint at all after committing contributes no pin rather than
    /// aborting the parent's checkpoint: a pin that named nothing would be worse than an absent
    /// one, and the parent's history is still perfectly valid without it.
    /// 📌️ One checkpoint-cascade unit: commits the child at `slot`/`child_id` when it is dirty,
    /// republishes its content root, and answers the pin its current checkpoint contributes.
    pub(super) async fn checkpoint_child_unit(&mut self, message: &str, authors: &[vcs::Author], key: MemberKeyRef<'_>, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<Option<vcs::CompositionPin>, Fault> {
        let publication_generation = self.admit_child_content_publication()?;
        let (reference, checkpoint_id) = {
            let entry = self.children.member_mut(key).ok_or_else(|| plugin_sdk_fault("checkpoint child authority changed during bounded publication"))?;
            if entry.member.is_dirty().await {
                entry.member.commit_checkpoint(message.to_string(), authors.to_vec(), identity).await.map_err(|error| error.into_fault())?;
            }
            (entry.reference.clone(), entry.member.current_checkpoint_id().await)
        };
        self.publish_member_content(publication_generation, key).await?;
        Ok(checkpoint_id.map(|checkpoint_id| vcs::CompositionPin { child_ref: reference, checkpoint_id }))
    }

    /// 📌️ Records `pins` on the checkpoint the parent's dispatch just created. Runs AFTER
    /// that dispatch because the checkpoint does not exist until then, and `composition_pins`
    /// participates in `content_addressed_checkpoint_id`, so the pins must be present on the
    /// envelope that gets persisted.
    pub(super) async fn stamp_checkpoint_composition_pins(&mut self, pins: Vec<vcs::CompositionPin>) -> Result<(), Fault> {
        if pins.is_empty() {
            return Ok(());
        }
        if let Some(checkpoint_id) = self.store.current_checkpoint_id().map(str::to_string) {
            self.store.set_checkpoint_composition_pins(&checkpoint_id, pins).await.map_err(|error| error.into_fault())?;
        }
        Ok(())
    }

    /// ⏮️ Checkout half of the cascade: the pins the parent's now-current checkpoint recorded,
    /// each restored by one [`Self::checkout_child_unit`]. A pin naming a child that is not currently
    /// open is QUEUED (`pending_child_pins`) rather than dropped, so a child adopted later still lands
    /// on its pinned state instead of silently staying at head — see `open_child`.
    pub(super) fn checkout_cascade_pins(&mut self) -> Result<Vec<vcs::CompositionPin>, Fault> {
        let Some(checkpoint_id) = self.store.current_checkpoint_id().map(str::to_string) else { return Ok(Vec::new()) };
        let Some(pins) = self.store.envelope().vcs.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).map(|checkpoint| checkpoint.composition_pins.clone()) else { return Ok(Vec::new()) };
        self.pending_child_pins.clear();
        self.admit_child_content_publication_span(pins.len())?;
        Ok(pins)
    }

    /// ⏮️ One checkout-cascade unit: restores one live child to `pin`, or queues the pin.
    pub(super) async fn checkout_child_unit(&mut self, pin: &vcs::CompositionPin, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), Fault> {
        let child_key = self.children.keyed_entries().find(|(_, entry)| entry.reference.artifact_id == pin.child_ref.artifact_id).map(|(key, _)| key.to_key());
        let Some(key) = child_key else {
            self.pending_child_pins.push(pin.clone());
            return Ok(());
        };
        let publication_generation = self.admit_child_content_publication()?;
        let entry = self.children.member_mut(key.borrowed()).ok_or_else(|| plugin_sdk_fault("checkout child authority changed during bounded publication"))?;
        let alternative_id = entry.member.current_alternative_id().await.unwrap_or_default();
        let _ = entry.member.checkout(&pin.checkpoint_id, &alternative_id, identity).await;
        self.publish_member_content(publication_generation, key.borrowed()).await
    }
}
