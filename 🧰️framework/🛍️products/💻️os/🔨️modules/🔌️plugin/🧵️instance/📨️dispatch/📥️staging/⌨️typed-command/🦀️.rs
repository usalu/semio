//! 🏂️ Staged `typed-command` members of the `instance-dispatch` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! ⌨️ Typed command admission and start: roots capture, wire admission, import media and configuration commands.
use super::*;
use super::docs_instance_root::artifact_handle_of;
use super::docs_instance_root::viewer_read_only_fault;
use super::docs_maintenance_ladder::removed;

impl<'ctx, A: ArtifactApp, M: SpaceMember + MemberFactory + 'static, X: InstanceExtension> DispatchCtx<'ctx, A, M, X> {
    pub(crate) async fn dispatch_typed_command_inner(&mut self, command: Box<A::Command>, admission: AdmittedToolCommand, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if self.tools.live_runtime_instance_id != Some(meta.instance_id) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.live-instance"), format!("typed command '{}' does not belong to the mounted live app instance", admission.verb)));
        }
        self.media.require_complete_tool_operation_pipeline(&admission)?;
        self.deliver_utility_host_events(meta, identity, cx).await?;
        let verb = A::command_id(&command).await.to_string();
        if admission.verb != verb {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.command-identity"), format!("exact admitted command '{}' decoded as '{verb}'", admission.verb)));
        }
        if let Some(fault) = self.ingest.document_loading_refusal(&*self.tools, &verb, X::admitted_while_loading(&verb)) {
            return Err(fault);
        }
        let Some(operation_id) = self.media.admit_typed_operation_slot(&*self.pending, &*self.tools) else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.typed-operation-capacity"), "every fixed typed-operation and segmented-output slot already owns a live operation"));
        };
        if self.host_events.delivering_host_event {
            self.host_events.host_event_operations.insert(operation_id.0);
        }
        if let Some(tag) = self.tool_machines.ingress.take() {
            self.tool_machines.bind(operation_id.0, tag);
        }
        if let QualifiedToolProof::AppOwned(registration) = &admission.proof {
            {
                let ordinary = (registration.latest_wins_target)(command.as_ref()).is_none();
                let disposer = (registration.latest_wins_command_disposer)()
                    .and_then(|owner| owner.downcast::<Box<dyn ArtifactOwnedDisposer<A::Command>>>().ok())
                    .map(|owner| *owner)
                    .ok_or_else(|| plugin_sdk_fault("latest-wins target requires its exact compiler-owned command retirement factory"))?;
                let revision = self.stores.store.content_revision_now();
                let generation = semio_framework_job::Generation(self.stores.store.generation_now());
                let base_revision = semio_framework_job::RevisionId(u64::from_be_bytes(revision[..8].try_into().expect("revision lane width")));
                let operation = semio_framework_job::Operation::new(operation_id, base_revision, generation, operation_id.0);
                self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = Some(operation_id.0);
                self.tools.latest_wins_commands.insert_admitted(
                    operation_id.0,
                    PendingLatestWinsCommand {
                        command: ManuallyDrop::new(Some(command)),
                        disposer,
                        admission: Some(admission),
                        meta: meta.clone(),
                        operation,
                        source_identity:(cx.operation(),cx.generation()),
                        revision,
                        lease: None,
                        key_copy: None,
                        key: None,
                        retired_key: None,
                        lookup_started: false,
                        accepted: ordinary,
                        restarting: false,
                        closing: false,
                        receipt:None,
                        fault:ManuallyDrop::new(None),fault_retirement:None,
                    },
                );
                self.tools.latest_wins_order.push(operation_id.0).expect("exact FIFO command slot was pre-admitted");
                let mut started = ToolOperations::<A>::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
                started.output = DslValue::Object(vec![("operationId".into(), DslValue::String(operation_id.0.to_string())), ("generation".into(), DslValue::String(generation.0.to_string()))]);
                return Ok(started);
            }
        }
        Err(plugin_sdk_fault("typed admission requires its exact retained command owner"))
    }

    /// 📸️ The roots one typed command runs against, captured once and identically for the shell lane
    /// that mounts the operation ([`Self::start_typed_command_operation`]) and for the agent lane's prepare
    /// phase that only previews it ([`Self::preview_addressed_action`]) — so the two lanes can never run
    /// the same verb against different state.
    pub(super) async fn capture_typed_command_roots(&mut self, command: &A::Command, meta: &ActionMeta) -> Result<TypedCommandRoots<A>, Fault> {
        self.as_render().refresh_cache().await?;
        let (snapshot, config, history) = self.history.command_cache_inputs();
        let canonical_base_revision = self.stores.store.content_revision();
        let window_config_authority = self.stores.window_config_store.capture(meta.view_state.as_ref()).await?;
        let targeted_window_transient_view = match A::retained_window_transient_target(command) {
            Some((window_id, expected_kind)) => {
                let view = meta.view_state.as_ref().ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an exact ViewModel roster"))?;
                let window = view.window_instances.iter().find(|window| window.id == window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture requires an attached window instance"))?;
                if window.window_kind_id != expected_kind {
                    return Err(plugin_sdk_fault("targeted window transient capture does not match the command owner's expected window kind"));
                }
                Some(view.for_window_instance(window_id).ok_or_else(|| plugin_sdk_fault("targeted window transient capture lost its attached window instance"))?)
            }
            None => None,
        };
        let window_transient_authority = self.stores.window_transient_store.capture(targeted_window_transient_view.as_ref().or(meta.view_state.as_ref()))?;
        Ok(TypedCommandRoots {
            snapshot,
            config,
            history,
            children: std::sync::Arc::new(ChildContentView::clone(&self.child_composition.child_content_root)),
            draft_snapshot: self.stores.draft_store.snapshot_root(),
            interaction_state: self.stores.interaction_store.snapshot_root(),
            interaction_hover: self.interaction.interaction_hover.clone(),
            presence_peers: self.stores.presence_store.peers_root(),
            transient: self.stores.transient_store.current_root(),
            peer_presence: std::sync::Arc::clone(&self.roster.peer_presence),
            canonical_base_revision,
            base_revision: semio_framework_job::RevisionId(u64::from_be_bytes(canonical_base_revision[..8].try_into().expect("revision lane width"))),
            generation: semio_framework_job::Generation(self.stores.store.generation()),
            config_generation: self.stores.config_store.generation(),
            draft_generation: self.stores.draft_store.generation(),
            presence_generation: self.stores.presence_store.generation().await,
            transient_generation: self.stores.transient_store.generation().await,
            window_config_authority,
            window_transient_authority,
        })
    }

    /// 🧪️ Observes actual capture authority without exposing production dispatch preparation.
    #[cfg(test)]
    pub(crate) async fn test_capture_presence_roots(&mut self, command: &A::Command, meta: &ActionMeta) -> Result<(std::sync::Arc<PeerPresenceRoot>, std::sync::Arc<store::os_store::presence::PresencePeersRoot<A::Presence>>), Fault> {
        let TypedCommandRoots { peer_presence, presence_peers, .. } = self.capture_typed_command_roots(command, meta).await?;
        Ok((peer_presence, presence_peers))
    }

    /// 🧪️ Mounts an exact typed admission so laws can observe the boundary before producer completion.
    #[cfg(test)]
    pub(crate) async fn test_mount_typed_command(&mut self, command: A::Command, verb: &str, meta: &ActionMeta, identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        let payload = ::protocol::OpBinary::encode_op(&command).map_err(|error| error.into_fault())?;
        let admission = self.tools.admit_command_wire(&*self.stores, verb, &payload, 1).await?;
        self.dispatch_typed_command_inner(Box::new(command), admission, meta, identity, cx).await
    }

    /// 🧵️ Captures immutable roots and mounts one admitted worker turn. Ephemeral producers
    /// execute inside their app-owned job and publish through its declared completion lanes.
    pub(super) async fn start_typed_command_operation(
        &mut self,
        command: Box<A::Command>,
        admission: AdmittedToolCommand,
        meta: &ActionMeta,
        operation_id: semio_framework_job::OperationId,
        cancellation_lease: ToolCancellationLease,
     cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        assert!(self.tools.tool_operations.can_insert(operation_id.0), "typed worker handoff retains its exact empty operation slot");
        assert_eq!(self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS], Some(operation_id.0), "worker handoff owns its exact pending reservation");
        let verb = admission.verb.clone();
        let TypedCommandRoots {
            snapshot,
            config,
            history,
            children,
            draft_snapshot,
            interaction_state,
            interaction_hover,
            presence_peers,
            transient,
            peer_presence,
            canonical_base_revision,
            base_revision,
            generation,
            config_generation,
            draft_generation,
            presence_generation,
            transient_generation,
            window_config_authority,
            window_transient_authority,
        } = self.capture_typed_command_roots(command.as_ref(), meta).await?;
        let parent_document_id = self.stores.store.envelope().id.clone();
        let seed_handle = artifact_handle_of(&format!("{}/{}/{verb}/{}", meta.instance_id, self.tools.tool_job_controller_id, base_revision.0)).await;
        let operation = semio_framework_job::Operation::new(operation_id, base_revision, generation, (seed_handle.0 as u64) ^ ((seed_handle.0 >> 64) as u64));
        assert_eq!(cancellation_lease.key.base_revision, base_revision, "fresh worker receives a revision-rebound cancellation lease");
        assert_eq!(cancellation_lease.key.generation, generation, "fresh worker receives a generation-rebound cancellation lease");
        let payload_schema_id = admission.proof.schema_id();
        let qualified_key = admission.proof.key();
        let publication_lanes = admission.proof.publication_lanes();
        let completion = ArtifactToolCompletion::<A>::new();
        let mut identity_observer = |_: semio_framework_value::native_encoding::NativeEncodeProgress| !cancellation_lease.token.is_cancelled_now() && !cancellation_lease.publication_claim.is_cancelled();
        let identity = semio_framework_value::NativeEncodeControl::new(admission.proof.contract().max_output_bytes, &mut identity_observer).pause().map_err(|error| plugin_sdk_fault(error.to_string()))?;
        drop(identity_observer);
        completion.install_authoring_identity(identity, &cancellation_lease)?;
        let retained_wire_admission = admission.wire_admission.clone();
        let app_owned_retained_route = matches!(&admission.proof, QualifiedToolProof::AppOwned(_));
        let mut retained_wire = Some(admission.raw_wire);
        let raw_input = if app_owned_retained_route { ArtifactToolRawInput::transferred_to_factory() } else { ArtifactToolRawInput::new(retained_wire.take().expect("generic retained wire owner remains mounted")) };
        let output_chunks = ArtifactOutputChunks::new(admission.proof.contract().max_output_bytes);
        let mut request_snapshot_read = if app_owned_retained_route { Some(self.stores.store.snapshot_read().map_err(FaultFrom::into_fault)?) } else { None };
        let mut request_window_config = if app_owned_retained_route { window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())? } else { None };
        let owned_context = if app_owned_retained_route {
            let presence_local = self.stores.presence_store.local_read().map_err(Fault::from)?;
            let gesture = self.as_render().admit_gesture_slot(operation_id.0, &canonical_base_revision, meta,artifact_owned_tool_birth_grant(),artifact_owned_tool_handoff_grant())?;
            Some(ArtifactOwnedContextHandle::admit(
            ArtifactOwnedToolJobContext::new(
                meta.instance_id,
                meta.view_state.clone(),
                canonical_base_revision,
                draft_generation,
                transient_generation,
                ArtifactOwnedToolJobSnapshots {
                    children: std::sync::Arc::clone(&children),
                    draft: std::sync::Arc::clone(&draft_snapshot),
                    transient: std::sync::Arc::clone(&transient),
                    window_config: window_config_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                    window_transient: window_transient_authority.as_ref().map(|authority| authority.snapshot.try_duplicate()).transpose().map_err(|error| error.into_fault())?,
                },
            )
            .with_source_issuers(&self.stores.draft_store, &self.stores.transient_store)
            .with_presence(presence_local, std::sync::Arc::clone(&presence_peers), self.stores.presence_store.peer_retirement_factory())
            .with_tool_run(self.tool_runs.view_for(meta.view_state.as_ref().and_then(|view| view.window_id.as_deref())))
            .with_provisional(self.tool_machines.provisional_values(), self.tool_machines.provisional_generation())
            .with_gesture(gesture),
            artifact_owned_tool_birth_grant(),
        ).0) } else { None };
        let operation_spec = match admission.proof.clone() {
            QualifiedToolProof::Bounded(_) => {
                let job = TypedCommandFullOperationJob::<A> {
                    operation: None,
                    parent_document_id: Some(parent_document_id.clone()),
                    command: Some(std::sync::Arc::from(command)),
                    snapshot: Some(snapshot),
                    config: Some(config),
                    history: Some(history),
                    children: Some(std::sync::Arc::clone(&children)),
                    draft: Some(draft_snapshot),
                    interaction_state: Some(interaction_state),
                    interaction_hover: Some(std::sync::Arc::new(interaction_hover)),
                    peer_presence: Some(peer_presence),
                    presence_local: Some(self.stores.presence_store.local_read().map_err(Fault::from)?),
                    presence_peers: Some(presence_peers),
                    transient: Some(transient),
                    contract: admission.proof.contract(),
                    decoded_items: admission.decoded_items,
                    phase: TypedCommandFullOperationPhase::Prepare,
                    prepare_cursor: 0,
                    reducer_cursor: 0,
                    output: Some(completion.clone()),
                    state: [0; 3],
                    fault_message: b"",
                    pending: None,
                    outcomes: crate::tool_contract::ReservedJobOutcomes::new(),
                    closing: false,
                };
                semio_framework::ToolOperationSpec::new(self.tools.tool_job_controller_id.clone(), verb.clone(), payload_schema_id, job, operation)
            }
            QualifiedToolProof::AppOwned(_) => A::build_tool_job(ArtifactOwnedToolJobRequest {
                retained:self.mounted_policy.maintenance,
                command,
                raw_wire: raw_input.clone(),
                operation,
                controller_id: qualified_key.controller_id,
                tool_id: qualified_key.tool_id,
                payload_schema_id,
                contract: admission.proof.contract(),
                decoded_items: admission.decoded_items,
                app_instance_id: meta.instance_id,
                parent_document_id,
                canonical_base_revision,
                authoring_seed: self.stores.authoring_seed(&meta.actor),
                snapshot,
                snapshot_read: request_snapshot_read.take().expect("app-owned route retains its request snapshot read"),
                config,
                window_config: request_window_config.take(),
                history,
                interaction_state,
                interaction_hover: std::sync::Arc::new(interaction_hover),
                context: owned_context.expect("app-owned route retains its original admitted context"),
                instance_operation_owner: self.tools.instance_operation_owner.clone(),
                output_chunks: output_chunks.clone(),
                completion: completion.clone(),
            })
            .await?
            .ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.missing-owned-builder"), format!("app-owned tool '{verb}' registered a factory but supplied no exact payload builder")))?,
            QualifiedToolProof::FrameworkOwned(_) => {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.framework-route"), format!("framework-owned tool '{verb}' cannot enter the application typed-command reducer")));
            }
        };
        let dispatch = if app_owned_retained_route {
            self.tools.tool_jobs
                .dispatch_wire_retained_with_spec(&retained_wire_admission, retained_wire.take().expect("app-owned retained wire owner transfers once"), None, operation_spec)
                .map_err(|rejected| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), rejected.error.to_string()))?
        } else {
            self.tools.tool_jobs.dispatch(operation_spec).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?
        };
        let session=None;let session_rejected=Some(semio_framework_plugin_ownership::reserved_job_session::RefusedWorkerSource::pending(dispatch.job,(cx.operation(),cx.generation())));
        self.tools.tool_operations.insert_admitted(
            operation_id.0,
            MountedTypedCommandFullOperation {
                verb: verb.clone(),
                meta: meta.clone(),
                operation,
                source_identity:(cx.operation(),cx.generation()),
                canonical_revision: canonical_base_revision,
                artifact_generation: generation.0,
                config_generation,
                draft_generation,
                presence_generation,
                transient_generation,
                window_config_authority,
                window_transient_authority,
                publication_lanes,
                session,
                session_rejected,
                reserved_producer: None,
                completion: Some(completion),
                completion_retirement:None,
                publication_retirement:None,
                output_retirement:None,raw_input: (!app_owned_retained_route).then_some(raw_input),
                output_chunks: Some(output_chunks),
                cancellation_lease: Some(cancellation_lease),
                identity: None,
                identity_progress: semio_framework_value::native_encoding::NativeEncodeProgress { completed: 0, total: 0, owned_bytes: 0 },
                worker_semantic_pending: false,
                worker_outcome_pending: false,
                worker_fault_capture: None,
                terminal_seen: false,
                worker_resume_pending: false,
                publication: None,
                pending_artifact_publication: None, pending_publication_outcome:PendingPublicationOutcome::new(),pending_window_config_receipt:None,cancellation_retirement:None,
                pending_child_publication: None,
                owned_child_group: None,
                owned_child_committed: false,
                owned_child_result_pending: false,
                captured_child_content: Some(std::sync::Arc::clone(&children)),
                captured_child_content_generation: self.child_composition.child_content_generation,
                result_page: None,
                result_page_presented: false,
                result_sequence: 0,
                publication_progress: 0,
                publication_checkpoint: None,
                publication_ownership_progress: None,
                original_retirement_receipt:None,
                actor_capture: None,
                publication_attempt: 0,
                ui_pending: false,
                progress: None,
                progress_pending: false,
                user_cancel_requested: false,
                published_artifact: false,
                published_config: false,
                published_window_config: false,
                command_logged: false,
                interaction_revalidated: false,
                terminal_fault: None,
                retained_close_fault: None, retained_close_fault_retirement:None,retained_close_fault_refusal:None,
                stage: MountedTypedCommandFullOperationStage::Worker,
            },
        );
        self.tools.typed_operation_reservations[operation_id.0 as usize % ARTIFACT_LIVE_OUTPUT_SLOTS] = None;
        let mut started = ToolOperations::<A>::empty_result(&verb, meta, Vec::new(), Vec::new(), UiDirtyScope::None).await;
        started.output = DslValue::Object(vec![("operationId".into(), DslValue::String(operation_id.0.to_string())), ("generation".into(), DslValue::String(generation.0.to_string()))]);
        Ok(started)
    }

    /// 🎯️ B1: the shared body behind both `dispatch_typed_command` (wire bytes, decoded above)
    /// 🧪️ Test-only typed-value helper. Production callers must enter with an exact
    /// owner-qualified retained raw-page admission and cannot infer identity from a bare command.
    pub fn dispatch_typed<'b, 'context>(mut self, command: A::Command, meta: &'ctx ActionMeta, identity: &'ctx mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>,cx:&'ctx mut semio_framework_job::StepContext<'context>) -> impl Future<Output = Result<InvocationResult, Fault>> + 'ctx + use<'ctx, 'b, 'context, A, M, X> {
        let command = Box::new(command);
        async move {
            let log_generation_before = self.history.log_generation;
            let verb = A::command_id(command.as_ref()).await;
            let payload = ::protocol::OpBinary::encode_op(command.as_ref()).map_err(|error| error.into_fault())?;
            let admission = self.tools.admit_command_wire(&*self.stores, verb, &payload, 1).await?;
            let result = self.dispatch_typed_command_inner(command, admission, meta, identity, cx).await?;
            Ok(self.as_render().finish_recorded(log_generation_before, verb, result).await)
        }
    }

    /// 🕰️ The actual body of `PluginApp::import_media` — see `dispatch_action`'s doc.
    ///
    /// 🔒️ Contract §2.3 clause 1 — `import` is not a string action (see `VIEWER_REJECTED_ACTION_IDS`'s
    /// own doc), so it is guarded here instead of `dispatch_action`.
    pub(super) async fn dispatch_import_media(&mut self, port: &str, media: Media, meta: &ActionMeta, cx:&mut semio_framework_job::StepContext<'_>) -> Result<InvocationResult, Fault> {
        if A::ROLE == AppRole::Viewer {
            return Err(viewer_read_only_fault("import"));
        }
        let proof = self.tools.qualified_tool_proof(&*self.stores, "import-media")?;
        let key = proof.key();
        let admission = self.tools.tool_jobs.admit_exact_wire(key.controller_id, key.tool_id, proof.schema_id(), &[]).map_err(|error| Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.dispatch"), error.to_string()))?;
        if !proof.admits::<A>(&admission) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.factory-identity"), "media import resolved outside its exact controller/owner/factory/tool/schema proof"));
        }
        let (authority,source)=self.media.pending_media_import_permit.take().ok_or_else(||plugin_sdk_fault("media source requires its original pre-admitted import permit"))?;
        if (source.0,source.1)!=(cx.operation(),cx.generation()){*self.media.pending_media_import_permit=Some((authority,source));return Err(plugin_sdk_fault("media import requires its unchanged original caller"))}
        let (job, completion) = match self.as_render().build_artifact_reserved_media_job(port, media, meta).await { Ok(producer) => producer, Err(fault) => { self.tools.finish_reserved_emit_producer(authority); return Err(fault); } };
        let raw = Vec::new();
        self.dispatch_reserved_original_producer("import-media",&raw,1,job,completion,meta,Some(admission),authority, cx).await
    }

    /// 🧮️ B1: dispatches a binary-encoded `store::os_store::store_schema::ArtifactCommand<A::ConfigMutation>` against
    /// the config store — real work for `AppCommand::ConfigCommand` (replaces the deleted
    /// `apply_config_bytes` whole-record-replace legacy path).
    pub(super) async fn dispatch_config_command_inner(&mut self,command_bytes:&mut Vec<u8>,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.pending.receive_configuration_failure(cx)?{return Ok(None)}
        if self.pending.pending_configuration_permit.is_some()&&self.pending.pending_configuration_completion.is_some(){return self.finish_configuration_command(meta,identity,cx).await}
        let proof=self.tools.qualified_tool_proof(&*self.stores, "configuration-binary")?;let key=proof.key();
        let admission=match self.tools.tool_jobs.admit_exact_wire(key.controller_id,key.tool_id,proof.schema_id(),command_bytes){Ok(admission)=>admission,Err(source)=>{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Admission(source));self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}};
        if self.pending.pending_configuration_permit.is_none(){let Some(authority)=self.admit_reserved_emit_producer("configuration-binary",meta,cx)?else{return Ok(None)};self.pending.pending_configuration_permit=Some(authority);self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));return Ok(None)}
        if self.pending.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration admission requires its unchanged original receiving caller").into())}
        type Completion<M>=std::sync::Mutex<Option<Result<ArtifactCommand<M>,semio_framework_job::OriginalTurnFailure>>>;
        let bytes=semio_framework_value::retirement::shared::shared_retirement_allocation_bytes::<Completion<A::ConfigMutation>>();let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes{return Ok(None)}
        let decoded=std::sync::Arc::new(std::sync::Mutex::new(None));cx.consume_retained(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}).expect("exact original configuration completion preflight remains exclusive");cx.consume_fuel(1);
        self.pending.pending_configuration_identity=Some((cx.operation(),cx.generation()));
        self.pending.pending_configuration_completion=Some(crate::reserved_jobs::ConfigurationDecodeRetirement{original:ManuallyDrop::new(Some(decoded.clone())),payload:None});
        let job=FrameworkConfigurationBinaryJob::<A>{raw:std::mem::take(command_bytes),cursor:0,stage:FrameworkConfigurationBinaryStage::Envelope,operation:None,decoded:ManuallyDrop::new(Some(decoded)),decoded_close:None,receiving:ManuallyDrop::new(None),native:semio_framework_job::OriginalNativeDecodeSource::empty(semio_framework_job::OriginalNativeDecodePolicy{maximum_owned_bytes:crate::reserved_jobs::FRAMEWORK_CONFIGURATION_NATIVE_OWNED_BYTES}),failure:ManuallyDrop::new(None),state:[0;8],pending:None,outcomes:crate::tool_contract::ReservedJobOutcomes::new(),closing:false};
        let authority=self.pending.pending_configuration_permit.take().unwrap();
        let permit=match self.tools.run_framework_reserved_job(&self.mounted_policy, &mut *self.stores, "configuration-binary", &[], 1, job, meta, Some(admission), authority, cx).await{Ok(permit)=>permit,Err((source,permit))=>{self.pending.pending_configuration_permit=Some(permit);*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}};
        self.pending.pending_configuration_permit=Some(permit);
        let command=self.pending.pending_configuration_completion.as_ref().unwrap().original.as_ref().unwrap().lock().unwrap_or_else(std::sync::PoisonError::into_inner).take().ok_or_else(||Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.missing-output"),"configuration binary job completed without a decoded command"))?;
        match command{Ok(command)=>*self.pending.pending_configuration_command=Some(command),Err(source)=>{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Original(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}}
        self.finish_configuration_command(meta,identity,cx).await
    }

    async fn finish_configuration_command(&mut self,meta:&ActionMeta,identity:&mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<InvocationResult>,semio_framework_job::OriginalTurnFailure>{
        if self.pending.pending_configuration_identity!=Some((cx.operation(),cx.generation())){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"configuration commit requires its unchanged original caller").into())}
        if let Some(owner)=self.pending.pending_configuration_completion.as_mut(){
            let demand=if owner.terminal_is_empty(){RetirementDemand{depth:1,..Default::default()}}else{owner.demand(cx.retained_grant().maximum_copy_bytes)?};
            let terminal=owner.terminal_is_empty();let mut removed=false;
            semio_framework_job::close_original_retained_step(cx,demand,|grant|{if terminal{removed=true;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))}else{owner.step(grant)}})?;
            if removed{self.pending.pending_configuration_completion.take();}return Ok(None)
        }
        let permit=self.pending.pending_configuration_permit.as_ref().expect("configuration commit retains its admitted permit");self.stores.validate_framework_reserved_commit(X::reserved_action_refusal(&*self.time_travel, &*self.tool_runs, "configuration-binary"), "configuration-binary", permit).await?;
        if permit.is_cancelled().await{return Err(Fault::new(FaultOrigin::Framework,FaultCode::new("interactive-job.cancelled"),"configuration binary was cancelled before config-store commit").into())}
        let grant=cx.retained_grant();if cx.should_yield()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(None)}
        let command=self.pending.pending_configuration_command.take().expect("configuration commit retains its original typed command");cx.consume_retained(RetainedCloneProgress{copied_items:1,..Default::default()})?;cx.consume_fuel(1);
        let dispatched=self.stores.config_store.dispatch(command,identity).await;let permit=self.pending.pending_configuration_permit.take().unwrap();self.tools.finish_reserved_emit_producer(permit);if let Err(source)=dispatched{*self.pending.pending_configuration_failure=Some(crate::reserved_jobs::OriginalConfigurationFailure::Store(source));let _=self.pending.receive_configuration_failure(cx)?;return Ok(None)}self.pending.pending_configuration_identity=None;self.history.cache=None;Ok(Some(ToolOperations::<A>::empty_result("configCommand", meta, Vec::new(), Vec::new(), UiDirtyScope::Full).await))
    }
}
