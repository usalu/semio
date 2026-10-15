//! 🏭️ HubInferenceRuntimeV1: the hub inference authority and its frozen-identity helpers.

use super::*;

/// 🏃️ The hub's only inference authority: frozen binding, ledger, gates, retained cancellation.
pub struct HubInferenceRuntimeV1 {
    pub(super) binding: Arc<VerifiedGisMapArtifactBindingV1>,
    pub(super) ledger: Arc<InferenceJobLedgerV1>,
    supervisor: RetainedInferenceRunSupervisorV1,
    document_gates: Mutex<HashMap<String, Arc<tokio::sync::Mutex<()>>>>,
    pub(super) committer: Arc<dyn GisMapApprovalCommitterV1>,
    #[cfg(feature = "integration-fixtures")]
    pub(super) checkpoint_test_gate: Mutex<Option<Arc<InferenceCheckpointTestGateV1>>>,
}

impl HubInferenceRuntimeV1 {
    /// 🧊️ Binds one process-lifetime runtime to an already-verified GIS Map selection.
    pub fn new(binding: Arc<VerifiedGisMapArtifactBindingV1>, ledger: Arc<InferenceJobLedgerV1>, committer: Arc<dyn GisMapApprovalCommitterV1>) -> Self {
        Self {
            binding,
            ledger,
            supervisor: RetainedInferenceRunSupervisorV1::new(),
            document_gates: Mutex::new(HashMap::new()),
            committer,
            #[cfg(feature = "integration-fixtures")]
            checkpoint_test_gate: Mutex::new(None),
        }
    }

    pub fn binding(&self) -> &Arc<VerifiedGisMapArtifactBindingV1> {
        &self.binding
    }

    pub fn ledger(&self) -> &Arc<InferenceJobLedgerV1> {
        &self.ledger
    }

    pub fn committer(&self) -> &Arc<dyn GisMapApprovalCommitterV1> {
        &self.committer
    }

    /// 🧪️ Installs one exact pause at the real native codec checkpoint for physical race laws.
    #[cfg(feature = "integration-fixtures")]
    pub fn install_checkpoint_test_gate(&self, gate: Arc<InferenceCheckpointTestGateV1>) -> Result<(), InferenceRouteErrorV1> {
        let mut selected = self.checkpoint_test_gate.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if selected.is_some() {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        *selected = Some(gate);
        Ok(())
    }

    /// 🧪️ Reaps finished workers and returns the exact retained task count.
    #[cfg(feature = "integration-fixtures")]
    pub async fn retained_operation_count_for_test(&self) -> Result<usize, InferenceRouteErrorV1> {
        self.reap_finished_operations().await?;
        self.supervisor.operations.lock().map(|operations| operations.len()).map_err(|_| InferenceRouteErrorV1::Storage)
    }

    /// 🧹️ Stops admission, signals and joins every retained worker, then closes approval owners.
    pub async fn close(&self) -> Result<(), GisMapApprovalCommitErrorV1> {
        self.supervisor.signal_and_join_operations().await.map_err(|error| gis_map_storage_refusal(&error))?;
        self.committer.close().await
    }

    /// 🚧️ Returns the one per-`DocumentScope` async gate every checked phase serializes on.
    pub fn document_gate(&self, scope: &DocumentScope) -> Result<Arc<tokio::sync::Mutex<()>>, InferenceRouteErrorV1> {
        let key = document_key(scope);
        let mut gates = self.document_gates.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if let Some(gate) = gates.get(&key) {
            return Ok(gate.clone());
        }
        if gates.len() >= DOCUMENT_GATE_CAPACITY {
            gates.retain(|_, gate| Arc::strong_count(gate) > 1);
        }
        if gates.len() >= DOCUMENT_GATE_CAPACITY {
            return Err(InferenceRouteErrorV1::Capacity);
        }
        let gate = Arc::new(tokio::sync::Mutex::new(()));
        gates.insert(key, gate.clone());
        Ok(gate)
    }

    pub(super) async fn reap_finished_operations(&self) -> Result<(), InferenceRouteErrorV1> {
        self.supervisor.reap_finished_operations().await
    }

    pub(super) fn install_operation<F, Fut>(&self, receipt: &super::super::sqlite::InferenceLedgerJobRowV1, identity: &InferenceIdentityV1, now_ms: u64, control: Arc<InferenceOperationControlV1>, task: F) -> Result<InferenceOperationAdmissionV1, InferenceRouteErrorV1>
    where
        F: FnOnce(super::super::sqlite::InferenceRunClaimV1, oneshot::Receiver<()>, OwnedSemaphorePermit) -> Fut,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut operations = self.supervisor.operations.lock().map_err(|_| InferenceRouteErrorV1::Storage)?;
        if let Some(operation) = operations.get(&receipt.job_id) {
            return if operation.key.same_run(&receipt.job_id, &receipt.identity_digest) { Ok(InferenceOperationAdmissionV1::Existing) } else { Err(InferenceRouteErrorV1::Conflict) };
        }
        if self.supervisor.closing.load(Ordering::Acquire) {
            return Err(InferenceRouteErrorV1::Unavailable);
        }
        let permit = self.supervisor.capacity.clone().try_acquire_owned().map_err(|_| InferenceRouteErrorV1::Capacity)?;
        let Some(claim) = self.ledger.start(&receipt.job_id, identity, now_ms)? else {
            return Ok(InferenceOperationAdmissionV1::Unclaimed);
        };
        let key = InferenceRunKeyV1 { job_id: receipt.job_id.clone(), identity_digest: receipt.identity_digest.clone(), run_epoch: claim.run_epoch };
        let (start, ready) = oneshot::channel();
        let handle = tokio::spawn(task(claim, ready, permit));
        operations.insert(receipt.job_id.clone(), RetainedInferenceRunV1 { key, control, task: handle });
        Ok(InferenceOperationAdmissionV1::Installed(start))
    }

    /// 🛑️ Interrupts only the exact retained claim already authenticated by its private identity.
    pub(super) fn interrupt_operation(&self, job_id: &str, identity_digest: &str) -> Result<(), InferenceRouteErrorV1> {
        self.supervisor.interrupt_operation(job_id, identity_digest)
    }

    pub(super) fn fresh_now_ms(&self) -> Result<u64, InferenceRouteErrorV1> {
        fresh_now_ms()
    }

    /// 🔍️ Rejects any drift between the frozen job identity and the current server-materialized base.
    pub fn compare_frozen(&self, identity: &InferenceIdentityV1, scope: &DocumentScope, base: &InferenceMapBaseV1) -> Result<(), InferenceRouteErrorV1> {
        compare_frozen_identity(&self.binding.identity(), identity, scope, base)
    }

    /// 🧮️ Runs the frozen native GIS service under the claim's control and derives its sole proposal.
    pub fn infer(
        &self,
        identity: &InferenceIdentityV1,
        job_id: &str,
        base: &InferenceMapBaseV1,
        control: &InferenceOperationControlV1,
        checkpoint: &mut dyn FnMut(u64, u64) -> Result<(), InferenceRouteErrorV1>,
    ) -> Result<InferenceProposalV1, InferenceRouteErrorV1> {
        use directory::FromValue;
        use semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::inferences::GisMapInference;
        let _ = identity;
        let budgets = semio_framework_plugin::WireArtifactInferenceBudget { allocation_bytes: ALLOCATION_BYTES, work_units: WORK_UNIT_LIMIT, recursion_depth: RECURSION_DEPTH };
        let request = semio_framework_plugin::ArtifactInferenceExecutionRequest {
            policy: b"gis-map-v1",
            budgets: &budgets,
            cancellation_id: job_id,
            previous_state: None,
            requested_cache_mode: semio_framework_plugin::WireArtifactInferenceCacheMode::Cold,
            canonical_payload: base.pack.as_slice(),
            dependencies: &[],
        };
        let mut failure: Option<InferenceRouteErrorV1> = None;
        let execution = semio_s_artifact_gis_gismap::infer_gis_map_controlled(&request, &mut |completed| {
            if let Err(error) = control.checkpoint(completed) {
                failure = Some(error.into());
                return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("hub.inference.interrupted", "bounded inference interrupted"));
            }
            if let Err(error) = checkpoint(completed, WORK_UNIT_LIMIT) {
                failure = Some(error);
                return Err(semio_framework_plugin::ArtifactInferenceExecutionError::new("hub.inference.interrupted", "bounded inference interrupted"));
            }
            Ok(())
        });
        let execution = match execution {
            Ok(execution) => execution,
            Err(_) => return Err(failure.unwrap_or(InferenceRouteErrorV1::Invalid)),
        };
        let snapshot = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let inference = GisMapInference::from_value(directory::pack_rt::decode_wire_value(&execution.canonical_payload).map_err(|_| InferenceRouteErrorV1::Invalid)?).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let mutation = inference.bounds_proposal(&snapshot, job_id).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let proposal_bytes = semio_framework_pack_json::to_json_string(&mutation).into_bytes();
        let proposal = InferencePrivateBytesV1::new(proposal_bytes, PROPOSAL_MAX_BYTES)?;
        let proposal_hash = sha256(proposal.as_slice());
        Ok(InferenceProposalV1 { result: InferencePrivateBytesV1::new(execution.canonical_payload, RESULT_MAX_BYTES)?, proposal, proposal_hash })
    }

    /// ✍️ Rebuilds the sole `CreateRegion` and its inverse server-side and stamps one canonical envelope.
    pub fn server_stamped_command(&self, identity: &InferenceIdentityV1, job_id: &str, base: &InferenceMapBaseV1, proposal_hash: &str, document_clock: Option<protocol::HybridLogicalTimestamp>, now_ms: u64) -> Result<InferencePrivateBytesV1, InferenceRouteErrorV1> {
        use semio_s_artifact_gis_gismap::mutations::inverse_gis_map_mutation;
        let (snapshot, inference) = deterministic_map_inference(base, job_id)?;
        let mutation = inference.bounds_proposal(&snapshot, job_id).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let proposal_bytes = semio_framework_pack_json::to_json_string(&mutation).into_bytes();
        if sha256(&proposal_bytes) != proposal_hash {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let inverse = inverse_gis_map_mutation(&snapshot, &mutation);
        let inverse_bytes = semio_framework_pack_json::to_json_string(&inverse).into_bytes();
        let mutation_id = approval_mutation_id(job_id, proposal_hash);
        let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
        let bytes = encode_server_stamped_command_v1(&CanonicalInferenceCommandPartsV1 {
            mutation_id: &mutation_id,
            document_id: &document_key(&scope),
            actor: &approval_actor(identity),
            diff_schema: GIS_DOCUMENT_SCHEMA,
            diff_payload: &proposal_bytes,
            inverse_schema: GIS_DOCUMENT_SCHEMA,
            inverse_payload: &inverse_bytes,
            timestamp: approval_stamp_v1(document_clock, now_ms),
        })?;
        Ok(InferencePrivateBytesV1::new(bytes, super::super::command::COMMAND_MAX_BYTES)?)
    }

    /// ↩️ Rebuilds the original fixed-three work and stamps only its exact typed inverse.
    pub(super) fn server_stamped_undo_command(&self, target: &super::super::sqlite::GisMapApprovalUndoTargetV1, idempotency_key: &str, base: &InferenceMapBaseV1, document_clock: Option<protocol::HybridLogicalTimestamp>, now_ms: u64) -> Result<GisMapPreparedUndoCommandV1, InferenceRouteErrorV1> {
        use directory::Inference as _;
        use semio_s_artifact_gis_gismap::mutations::GisMapMutation;
        use semio_s_artifact_gis_gismap::standards::v1::subsets::any::io::text::mutations::apply_gis_map_mutation;
        use semio_s_artifact_gis_gismap::standards::v1::subsets::any::schema::inferences::GisMapInference;

        if sha256(target.original_command.as_slice()) != target.original_command_hash {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let original = CanonicalInferenceCommandV1::decode(target.original_command.as_slice()).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let inverse_text = std::str::from_utf8(original.inverse_payload()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let inverses = semio_framework_pack_json::from_json_str::<Vec<GisMapMutation>>(inverse_text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        if inverses.len() != 1 {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let current = <semio_s_artifact_gis_gismap::GisMapSnapshot as pack::snapshot::ArtifactPack>::decode_pack(base.pack.as_slice()).map_err(|_| InferenceRouteErrorV1::Invalid)?;
        let mut before = current.clone();
        apply_gis_map_mutation(&mut before, &inverses[0]).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let work = GisMapInference::infer(&before).create_region_group_work(&before, &target.original_job_id).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let original_forward = semio_framework_pack_json::to_json_string(&work.parent).into_bytes();
        let original_inverse = semio_framework_pack_json::to_json_string(&work.parent_inverse).into_bytes();
        let actor = format!("user:{}#session:{}", target.user_id, target.session_id);
        if target.original_mutation_id != approval_mutation_id(&target.original_job_id, &sha256(&original_forward))
            || !original.matches_fixed_three_parent(&target.original_mutation_id, &document_key(&target.scope), &actor, &original_forward, &original_inverse)
        {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let diff = semio_framework_pack_json::to_json_string(&inverses[0]).into_bytes();
        let inverse = semio_framework_pack_json::to_json_string(&vec![work.parent]).into_bytes();
        let operation_id = sha256(format!("semio.hub.gis-map-approval-undo-operation/v1\0{}\0{idempotency_key}", target.target_id).as_bytes())[..32].to_owned();
        let proposal_hash = sha256(&diff);
        let mutation_id = approval_mutation_id(&operation_id, &proposal_hash);
        let bytes = encode_server_stamped_command_v1(&CanonicalInferenceCommandPartsV1 {
            mutation_id: &mutation_id,
            document_id: &document_key(&target.scope),
            actor: &actor,
            diff_schema: GIS_DOCUMENT_SCHEMA,
            diff_payload: &diff,
            inverse_schema: GIS_DOCUMENT_SCHEMA,
            inverse_payload: &inverse,
            timestamp: approval_stamp_v1(document_clock, now_ms),
        })?;
        let command_hash = sha256(&bytes);
        Ok(GisMapPreparedUndoCommandV1 { operation_id, proposal_hash, mutation_id, command_hash, command: InferencePrivateBytesV1::new(bytes, super::super::command::COMMAND_MAX_BYTES)? })
    }

    /// 🧾️ Hands the prepared envelope to the composition transaction and reconciles only its witness.
    pub async fn commit_approval(
        &self,
        identity: &InferenceIdentityV1,
        job_id: &str,
        proposal_hash: &str,
        command: &InferencePrivateBytesV1,
        base: &InferenceMapBaseV1,
        deadline_ms: u64,
        now_ms: u64,
        document_write: Arc<tokio::sync::Mutex<()>>,
        ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
    ) -> Result<GisMapApprovalOutcomeV1, InferenceRouteErrorV1> {
        commit_prepared_approval(&self.committer, identity, job_id, proposal_hash, command, base, deadline_ms, now_ms, document_write, ingress).await
    }

    pub(super) async fn commit_undo(
        &self,
        target: &super::super::sqlite::GisMapApprovalUndoTargetV1,
        idempotency_key: &str,
        prepared: &GisMapPreparedUndoCommandV1,
        base: &InferenceMapBaseV1,
        deadline_ms: u64,
        now_ms: u64,
        document_write: Arc<tokio::sync::Mutex<()>>,
        ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
    ) -> Result<GisMapApprovalUndoCommitReceiptV1, InferenceRouteErrorV1> {
        let composed_children = base.composed_children()?;
        let actor = format!("user:{}#session:{}", target.user_id, target.session_id);
        self.committer
            .undo(GisMapApprovalUndoCommitRequestV1 {
                composed_children: &composed_children,
                scope: &target.scope,
                actor: &actor,
                target,
                idempotency_key,
                mutation_id: &prepared.mutation_id,
                command_hash: &prepared.command_hash,
                operation_id: &prepared.operation_id,
                proposal_hash: &prepared.proposal_hash,
                command: prepared.command.as_slice(),
                base,
                deadline_ms,
                now_ms,
                document_write,
                ingress,
            })
            .await
            .map_err(|error| match error {
                GisMapApprovalCommitErrorV1::Unavailable => InferenceRouteErrorV1::CommitUnavailable,
                GisMapApprovalCommitErrorV1::Rejected => InferenceRouteErrorV1::Denied,
                GisMapApprovalCommitErrorV1::Conflict => InferenceRouteErrorV1::Conflict,
                GisMapApprovalCommitErrorV1::Capacity => InferenceRouteErrorV1::Capacity,
                GisMapApprovalCommitErrorV1::Storage => InferenceRouteErrorV1::Storage,
            })
    }
}

/// 🔍️ Compares one frozen binding plus scope, document, frontier and base-pack digest, exactly.
pub fn compare_frozen_identity(frozen: &super::super::schema::InferenceBindingIdentityV1, identity: &InferenceIdentityV1, scope: &DocumentScope, base: &InferenceMapBaseV1) -> Result<(), InferenceRouteErrorV1> {
    if identity.binding != *frozen {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    identity.validate()?;
    if identity.space_id != scope.space_id
        || identity.document_id != scope.document_id
        || identity.descriptor_digest != base.descriptor_digest
        || identity.head_ordinal != base.frontier.head_edit_ordinal
        || identity.head_edit_id != base.frontier.head_edit_id
        || identity.last_commit_seq != base.frontier.last_commit_seq
        || identity.input_hash != base.digest()
    {
        return Err(InferenceRouteErrorV1::Conflict);
    }
    Ok(())
}

/// 🧾️ Publishes one prepared envelope and reconciles the outbox only against its real WAL witness.
#[allow(clippy::too_many_arguments)]
pub async fn commit_prepared_approval(
    committer: &Arc<dyn GisMapApprovalCommitterV1>,
    identity: &InferenceIdentityV1,
    job_id: &str,
    proposal_hash: &str,
    command: &InferencePrivateBytesV1,
    base: &InferenceMapBaseV1,
    deadline_ms: u64,
    now_ms: u64,
    document_write: Arc<tokio::sync::Mutex<()>>,
    ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
) -> Result<GisMapApprovalOutcomeV1, InferenceRouteErrorV1> {
    let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
    if ingress.scope() != &scope || ingress.user_id() != identity.user_id || ingress.session_id() != identity.session_id || ingress.authorization_generation() != identity.authorization_generation {
        return Err(InferenceRouteErrorV1::Denied);
    }
    let composed_children = base.composed_children()?;
    if composed_children.as_slice() != ["gismap-drawing", "gismap-value"] {
        return Err(InferenceRouteErrorV1::CommitUnavailable);
    }
    let receipt = committer
        .commit(GisMapApprovalCommitRequestV1 {
            composed_children: &composed_children,
            scope: &scope,
            actor: &approval_actor(identity),
            mutation_id: &approval_mutation_id(job_id, proposal_hash),
            command_hash: &sha256(command.as_slice()),
            job_id,
            proposal_hash,
            command: command.as_slice(),
            base,
            base_frontier: &base.frontier,
            deadline_ms,
            now_ms,
            document_write,
            ingress,
        })
        .await
        .map_err(|error| match error {
            GisMapApprovalCommitErrorV1::Unavailable => InferenceRouteErrorV1::CommitUnavailable,
            GisMapApprovalCommitErrorV1::Rejected => InferenceRouteErrorV1::Denied,
            GisMapApprovalCommitErrorV1::Conflict => InferenceRouteErrorV1::Conflict,
            GisMapApprovalCommitErrorV1::Capacity => InferenceRouteErrorV1::Capacity,
            GisMapApprovalCommitErrorV1::Storage => InferenceRouteErrorV1::Storage,
        })?;
    Ok(GisMapApprovalOutcomeV1 { applied: receipt.applied, undo: receipt.undo })
}
