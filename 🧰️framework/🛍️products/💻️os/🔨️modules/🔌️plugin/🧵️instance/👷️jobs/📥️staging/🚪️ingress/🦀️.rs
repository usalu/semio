//! 🔧️ Staged slice of the `ingress` band of the mounted artifact instance: its `instance-jobs` members (WP6 step S0 of the instance family).
//!
//! 🚪️ Ingress of the mounted artifact instance: framework-reserved job driving, constructors, store-constructor units, envelope ingress and decode, document archive genesis and merge, child members and packs.
use super::*;
use crate::tool_jobs::time_travel;

impl<A: ArtifactApp> ToolOperations<A> {
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

    /// 🧷️ Clears only the original reserved publication identity without changing a foreign slot.
    pub(crate) fn release_reserved_emit_slot(&mut self, operation: u64) {
        let reservation = &mut self.typed_operation_reservations[operation as usize % ARTIFACT_LIVE_OUTPUT_SLOTS];
        if *reservation == Some(operation) { *reservation = None; }
    }

    /// 🏁️ Returns a reserved producer's original slot and keyed cancellation authority after ordinary completion.
    pub(crate) fn finish_reserved_emit_producer(&mut self, permit: FrameworkReservedCommitPermit) {
        self.release_reserved_emit_slot(permit.operation.operation.0);
        permit.finish();
    }

    /// 🛡️ Keeps producer admission failures tied to the original reserved identity.
    pub(crate) async fn run_framework_reserved_job<J: semio_framework_job::InteractiveJob + Send + 'static>(
        &mut self, mounted_policy: &crate::MountedOwnerPolicyV1, stores: &mut InstanceStores<A>, verb: &str, raw: &[u8], decoded_items: usize, job: J, meta: &ActionMeta,
        supplied_admission: Option<semio_framework::ToolWireAdmission>,
        supplied_authority: FrameworkReservedCommitPermit,
     cx:&mut semio_framework_job::StepContext<'_>) -> Result<FrameworkReservedCommitPermit, (semio_framework_job::OriginalTurnFailure,FrameworkReservedCommitPermit)> {
        let mut permit=supplied_authority;
        match self.run_framework_reserved_job_inner(mounted_policy, stores, verb, raw, decoded_items, job, meta, supplied_admission, &permit, cx).await {
            Ok(()) => Ok(permit),
            Err(fault) => { permit.lease.cancel(); Err((fault,permit)) },
        }
    }

    pub(crate) async fn run_framework_reserved_job_inner<J: semio_framework_job::InteractiveJob + Send + 'static>(&mut self, mounted_policy: &crate::MountedOwnerPolicyV1, stores: &mut InstanceStores<A>,verb:&str,raw:&[u8],decoded_items:usize,job:J,meta:&ActionMeta,supplied_admission:Option<semio_framework::ToolWireAdmission>,supplied_authority:&FrameworkReservedCommitPermit, cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        if supplied_authority.lease.keyed && (self.typed_operation_reservations[supplied_authority.operation.operation.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS]!=Some(supplied_authority.operation.operation.0)||supplied_authority.lease.key.operation_id!=supplied_authority.operation.operation){return Err(plugin_sdk_fault("reserved producer lost its original keyed slot admission").into());}
        self.drive_framework_reserved_worker(mounted_policy, stores, verb, raw, decoded_items, job, meta, supplied_admission, supplied_authority.operation, supplied_authority.lease.cancel_token(), false, cx).await
    }

    pub(crate) async fn drive_framework_reserved_worker<J: semio_framework_job::InteractiveJob + Send + 'static>(
        &mut self, mounted_policy: &crate::MountedOwnerPolicyV1, stores: &mut InstanceStores<A>,
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
        let proof = self.qualified_tool_proof(stores, verb)?;
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
        let step_grant = mounted_policy.preparation;
        let close_grant = mounted_policy.close;
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
        let live_revision_bytes = stores.store.content_revision();
        let live_revision = semio_framework_job::RevisionId(u64::from_be_bytes(live_revision_bytes[..8].try_into().expect("revision lane width")));
        let live_generation = semio_framework_job::Generation(stores.store.generation());
        if !matches!(semio_framework_job::validate_commit(&operation, live_revision, live_generation), semio_framework_job::CommitValidation::Accepted) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.stale"), format!("framework route '{verb}' completed against a stale revision or generation"))).into());
        }
        Ok(())
    }

    pub(crate) fn qualified_tool_proof(&self, stores: &InstanceStores<A>, verb: &str) -> Result<QualifiedToolProof, Fault> {
        if let Some(lane) = stores.unsupported_publication_contracts.get(verb) {
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

    pub(crate) fn qualified_host_configuration_tool_proof(&self, stores: &InstanceStores<A>, verb: &str) -> Result<QualifiedToolProof, Fault> {
        if let Some(lane) = stores.unsupported_publication_contracts.get(verb) {
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
    pub(crate) fn require_ui_safe_declaration(&self, verb: &str) -> Result<(), Fault> {
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

    pub(crate) async fn admit_command_wire_with_proof(&self, proof: QualifiedToolProof, verb: &str, payload: &[u8], decoded_items: usize) -> Result<AdmittedToolCommand, Fault> {
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

    pub(crate) async fn admit_command_wire(&self, stores: &InstanceStores<A>, verb: &str, payload: &[u8], decoded_items: usize) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let proof = self.qualified_tool_proof(stores, verb)?;
        self.admit_command_wire_with_proof(proof, verb, payload, decoded_items).await
    }

    pub(crate) async fn admit_command_json_with_proof(&self, proof: QualifiedToolProof, verb: &str, args: Option<&DslValue>) -> Result<AdmittedToolCommand, Fault> {
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

    pub(crate) async fn admit_host_configuration_json(&self, stores: &InstanceStores<A>, verb: &str, args: Option<&DslValue>) -> Result<AdmittedToolCommand, Fault> {
        self.require_ui_safe_declaration(verb)?;
        let proof = self.qualified_host_configuration_tool_proof(stores, verb)?;
        self.admit_command_json_with_proof(proof, verb, args).await
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
    pub(crate) fn record_settled_typed_operation_command(&mut self, history: &mut CommandHistory<A>, host_events: &mut HostEvents, operation_id: u64) {
        if host_events.host_event_operations.remove(&operation_id) {
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
            let kind = self.typed_operation_command_kind(host_events, &operation.verb, false);
            if operation.published_window_config && matches!(kind, ActionKind::View) {
                return;
            }
            (operation.verb.clone(), kind)
        };
        let (verb, kind) = logged;
        if let Some(operation) = self.tool_operations.get_mut(operation_id) {
            operation.command_logged = true;
        }
        self.record_command(history, &verb, kind, None, None, Vec::new(), None);
    }

    /// 🧾️ The declared [`ActionKind`] one migrated verb logs under — the same three-step resolution
    /// [`Self::dispatch_emit`] runs, with the same last resort: an unresolved verb that reached a
    /// document store is a `Mutation`, one that reached nothing is a `View`.
    pub(crate) fn typed_operation_command_kind(&self, host_events: &HostEvents, verb: &str, artifact_lane: bool) -> ActionKind {
        match host_events.invocation_kind {
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
    pub(crate) fn typed_operation_retirement_demands<M: SpaceMember + MemberFactory + 'static>(&self, child_composition: &ChildComposition<M>, operation_id: u64, body: usize) -> Result<RetirementDemand, Fault> {
        let operation = self.tool_operations.get(operation_id).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-retirement-authority"), "typed operation owner changed before its retirement quote"))?;
        if operation.close_fault_pending(){return operation.close_fault_demands(body).map_err(plugin_retirement_fault);}
        if let Some(group) = operation.owned_child_group {
            if child_composition.private_child_groups.get(group).is_some() { return child_composition.private_child_group_operation_close_demands(group, body).map_err(plugin_retirement_fault); }
            return Ok(RetirementDemand { depth: 1, ..Default::default() });
        }
        if operation.lease_retirement_ready(){return operation.lease_retirement_demands(&self.tool_cancellations,body).map_err(plugin_retirement_fault);}
        operation.retirement_demands(body).map_err(plugin_retirement_fault)
    }
}
