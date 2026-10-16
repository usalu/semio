//! 🏭️ HubInferenceRuntimeV1: the hub inference authority and its frozen-identity helpers.

use super::*;
use semio_hub_inference_runtime_contract::{DOCUMENT_GATE_CAPACITY, GisMapApprovalCommitErrorV1, GisMapApprovalCommitRequestV1, GisMapApprovalCommitterV1, GisMapApprovalIngressAuthorityV1, GisMapApprovalOutcomeV1, GisMapApprovalUndoCommitReceiptV1, GisMapApprovalUndoCommitRequestV1, GisMapPreparedUndoCommandV1, InferenceMapBaseV1, InferenceOperationAdmissionV1, InferenceProposalV1, InferenceRouteErrorV1, InferenceRunKeyV1, RetainedInferenceRunSupervisorV1, RetainedInferenceRunV1, WORK_UNIT_LIMIT, approval_actor, approval_mutation_id, approval_stamp_v1, document_key, fresh_now_ms, gis_map_storage_refusal, inference_budget};

/// 🏃️ The hub's only inference authority: frozen binding, ledger, gates, retained cancellation.
pub struct HubInferenceRuntimeV1 {
    pub binding: Arc<VerifiedGisMapArtifactBindingV1>,
    pub ledger: Arc<InferenceJobLedgerV1>,
    supervisor: RetainedInferenceRunSupervisorV1,
    document_gates: Mutex<HashMap<String, Arc<tokio::sync::Mutex<()>>>>,
    pub committer: Arc<dyn GisMapApprovalCommitterV1>,
    pub(super) proposals: Arc<dyn MapInferenceProposalBindingV1>,
    #[cfg(feature = "integration-fixtures")]
    pub(super) checkpoint_test_gate: Mutex<Option<Arc<InferenceCheckpointTestGateV1>>>,
}

impl HubInferenceRuntimeV1 {
    /// 🧊️ Binds one process-lifetime runtime to an already-verified GIS Map selection and the registered proposal binding.
    pub fn new(binding: Arc<VerifiedGisMapArtifactBindingV1>, ledger: Arc<InferenceJobLedgerV1>, committer: Arc<dyn GisMapApprovalCommitterV1>, proposals: Arc<dyn MapInferenceProposalBindingV1>) -> Self {
        Self {
            binding,
            ledger,
            supervisor: RetainedInferenceRunSupervisorV1::new(),
            document_gates: Mutex::new(HashMap::new()),
            committer,
            proposals,
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

    pub fn proposals(&self) -> &Arc<dyn MapInferenceProposalBindingV1> {
        &self.proposals
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

    pub(super) fn install_operation<F, Fut>(&self, receipt: &semio_hub_inference_ledger::InferenceLedgerJobRowV1, identity: &InferenceIdentityV1, now_ms: u64, control: Arc<InferenceOperationControlV1>, task: F) -> Result<InferenceOperationAdmissionV1, InferenceRouteErrorV1>
    where
        F: FnOnce(semio_hub_inference_ledger::InferenceRunClaimV1, oneshot::Receiver<()>, OwnedSemaphorePermit) -> Fut,
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
        let _ = identity;
        let mut failure: Option<InferenceRouteErrorV1> = None;
        let run = self.proposals.infer(base.pack.as_slice(), job_id, &inference_budget(), &mut |completed| {
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
        let run = match run {
            Ok(run) => run,
            Err(MapBindingRefusalV1::Execution) => return Err(failure.unwrap_or(InferenceRouteErrorV1::Invalid)),
            Err(refusal) => return Err(refusal.into()),
        };
        let proposal = InferencePrivateBytesV1::new(run.proposal, PROPOSAL_MAX_BYTES)?;
        let proposal_hash = sha256(proposal.as_slice());
        Ok(InferenceProposalV1 { result: InferencePrivateBytesV1::new(run.result, RESULT_MAX_BYTES)?, proposal, proposal_hash })
    }

    /// ✍️ Rebuilds the sole `CreateRegion` and its inverse server-side and stamps one canonical envelope.
    pub fn server_stamped_command(&self, identity: &InferenceIdentityV1, job_id: &str, base: &InferenceMapBaseV1, proposal_hash: &str, document_clock: Option<protocol::HybridLogicalTimestamp>, now_ms: u64) -> Result<InferencePrivateBytesV1, InferenceRouteErrorV1> {
        let effect = self.proposals.approval_effect(base.pack.as_slice(), job_id, &inference_budget())?;
        if sha256(&effect.forward) != proposal_hash {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let mutation_id = approval_mutation_id(job_id, proposal_hash);
        let scope = DocumentScope::new(identity.space_id.clone(), identity.document_id.clone());
        let bytes = encode_server_stamped_command_v1(&CanonicalInferenceCommandPartsV1 {
            mutation_id: &mutation_id,
            document_id: &document_key(&scope),
            actor: &approval_actor(identity),
            diff_schema: GIS_DOCUMENT_SCHEMA,
            diff_payload: &effect.forward,
            inverse_schema: GIS_DOCUMENT_SCHEMA,
            inverse_payload: &effect.inverse,
            timestamp: approval_stamp_v1(document_clock, now_ms),
        })?;
        Ok(InferencePrivateBytesV1::new(bytes, semio_hub_inference::command::COMMAND_MAX_BYTES)?)
    }

    /// ↩️ Rebuilds the original fixed-three work and stamps only its exact typed inverse.
    pub(super) fn server_stamped_undo_command(&self, target: &semio_hub_inference_ledger::GisMapApprovalUndoTargetV1, idempotency_key: &str, base: &InferenceMapBaseV1, document_clock: Option<protocol::HybridLogicalTimestamp>, now_ms: u64) -> Result<GisMapPreparedUndoCommandV1, InferenceRouteErrorV1> {
        if sha256(target.original_command.as_slice()) != target.original_command_hash {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let original = CanonicalInferenceCommandV1::decode(target.original_command.as_slice()).map_err(|_| InferenceRouteErrorV1::Conflict)?;
        let effect = self.proposals.undo_effect(base.pack.as_slice(), original.inverse_payload(), &target.original_job_id)?;
        let original_forward = effect.original_forward;
        let original_inverse = effect.original_inverse;
        let actor = format!("user:{}#session:{}", target.user_id, target.session_id);
        if target.original_mutation_id != approval_mutation_id(&target.original_job_id, &sha256(&original_forward))
            || !original.matches_fixed_three_parent(&target.original_mutation_id, &document_key(&target.scope), &actor, &original_forward, &original_inverse)
        {
            return Err(InferenceRouteErrorV1::Conflict);
        }
        let diff = effect.forward;
        let inverse = effect.inverse;
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
        Ok(GisMapPreparedUndoCommandV1 { operation_id, proposal_hash, mutation_id, command_hash, command: InferencePrivateBytesV1::new(bytes, semio_hub_inference::command::COMMAND_MAX_BYTES)? })
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
        commit_prepared_approval(&self.committer, self.proposals.as_ref(), identity, job_id, proposal_hash, command, base, deadline_ms, now_ms, document_write, ingress).await
    }

    pub(super) async fn commit_undo(
        &self,
        target: &semio_hub_inference_ledger::GisMapApprovalUndoTargetV1,
        idempotency_key: &str,
        prepared: &GisMapPreparedUndoCommandV1,
        base: &InferenceMapBaseV1,
        deadline_ms: u64,
        now_ms: u64,
        document_write: Arc<tokio::sync::Mutex<()>>,
        ingress: Arc<dyn GisMapApprovalIngressAuthorityV1>,
    ) -> Result<GisMapApprovalUndoCommitReceiptV1, InferenceRouteErrorV1> {
        let composed_children = base.composed_children(self.proposals.as_ref())?;
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
pub fn compare_frozen_identity(frozen: &semio_hub_inference::schema::InferenceBindingIdentityV1, identity: &InferenceIdentityV1, scope: &DocumentScope, base: &InferenceMapBaseV1) -> Result<(), InferenceRouteErrorV1> {
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
    proposals: &dyn MapInferenceProposalBindingV1,
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
    let composed_children = base.composed_children(proposals)?;
    if composed_children.as_slice() != proposals.commit_members() {
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

/// 🔒️ The private input and authorities retained until the blocking worker is joined.
pub(super) struct RetainedInferenceRunOwnerV1 {
    pub(super) runtime: Arc<HubInferenceRuntimeV1>,
    pub(super) directory: Arc<semio_hub_directory_dispatch::HubDirectories>,
    pub(super) rebootstrap: Arc<semio_hub_lag_rebootstrap::VerifiedRebootstrapSource>,
    pub(super) gate: Arc<tokio::sync::Mutex<()>>,
    pub(super) scope: DocumentScope,
    pub(super) identity: InferenceIdentityV1,
    pub(super) job_id: String,
    pub(super) expires_at_ms: u64,
    pub(super) claim: semio_hub_inference_ledger::InferenceRunClaimV1,
    pub(super) base: InferenceMapBaseV1,
    pub(super) control: Arc<InferenceOperationControlV1>,
    pub(super) _permit: OwnedSemaphorePermit,
}
