//! 🎬️ The mutation protocol — packet `P6-actions-policy`, `📋️master.md` §3.3, exact frame sequence
//! from `📓️luna-channel-audit.md`'s "Action Adapter Contract" section. Drives an app instance through
//! [`ArtifactChannel`] — a narrow port THIS packet defines (not the real channel's
//! `AppCommand`/`AppFrame`, which live in the peer ticket's exclusive `📡️spr/🧵️channel` territory).
//! Fields that would require decoding a peer-owned binary format (`OpBinary`, the packed `HistoryPatch`/
//! `DispatchReport` wire) are typed as the ALREADY-DECODED shape this adapter needs; translating from
//! the real channel's actual wire bytes into these shapes — resolving `(capability_id, input JSON)`
//! through the real `command_from_action` bridge, hydrating the real document/config/draft packs,
//! decoding the real `HistoryPatch`/op stream — is P7's job when it implements [`ArtifactChannel`] for
//! real (`🌉️mcp/🏠️workspace`); see this packet's report §"what P7 must implement" for the exact
//! contract. `MockArtifactChannel` is a fully-scripted in-memory artifact store THIS CRATE'S OWN
//! TESTS drive against — it is `#[cfg(test)]` and reachable from no production construction path;
//! a live gateway with no `--folder`/`--hub` binding runs on [`UnboundArtifactChannel`], which
//! answers every command with the typed `workspace.unbound` fault instead of a scripted success.

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;
use semio_framework_os_mcp_catalog::catalog::{CapabilityDefinition, Catalog};
use semio_framework_os_mcp_consent::approval_coordinator::{ApprovalCoordinator, ApprovalRequest, ApprovalResolution};
use semio_framework_os_mcp_foundation::audit::{AgentAuditEvent, AuditDecision, AuditSink, AuditSinks, ClientInfo, SENSITIVE_KEYS, hash_input, redact_input};
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_framework_os_mcp_foundation::handles::{Attachment, HandleKind, HandleTable, IdempotencyStore, SessionHandle, mint_id};
use semio_framework_os_mcp_foundation::reports::{InvocationReport, InvocationStatus, PreparedActionReport, RevisionStamp};
use semio_framework_os_mcp_inference_hub::hub_wire::is_hub_inference_route;
use semio_framework_os_mcp_port::channel_port::{ActivationScope, AppCommand, AppFrame, ArtifactChannel, HistoryUndoPort, HubInferenceApprovalUndoMemberV1, InferCommand, InferenceOutcome, InvokeRequest, MutationOrigin, NO_CHANGE_WARNING, PreparedActionRecord, PreparedOps, SagaMember, SagaMemberResult, SagaRecord, SagaReport, UndoMember, UndoRecord, UndoRedoReport, declared_effective_input, map_fault, omitted_revision_arguments, revision_guard_required, unacknowledged_relay_error};
use semio_framework_os_mcp_port::policy::{AgentPrincipal, ApprovalGate, AutoApprovePolicy, PolicyEngine};
use semio_framework_os_mcp_routing::routing_and_shell_routed::ArtifactChannels;
use semio_framework_os_mcp_schema::schema::{compile_validator, validate};

//#region 🔖️ActionAdapter

/// 🗳️ What one parked approval settled to inside `invoke_uncached` — the adapter's own narrow view
/// of [`ApprovalResolution`], flattened so the caller's error/audit branch stays one match.
enum SettledApproval {
    Approved,
    Denied { channel: &'static str, note: Option<String> },
    Unreachable { details: serde_json::Value },
    Cancelled,
}

struct AuditContext<'a> {
    pub invocation_id: &'a str,
    pub principal: &'a AgentPrincipal,
    pub session: &'a SessionHandle,
    pub capability_id: &'a str,
    raw_input: &'a serde_json::Value,
}

/// 🎬️ The mutation protocol's home — `📋️master.md` §3.3's Observe→Prepare→Preview→Approve→
/// Commit→Verify→Undo/Redo lifecycle, over an [`ArtifactChannel`]. One adapter is shared (behind
/// `Arc`) across every tool call in a process, matching P1b's own "one shared `McpServer`" model
/// (`📓️terra-P1b-report.md` §7.2) — nothing here is per-connection-session-aware yet either.
pub struct ActionAdapter {
    pub channel: Mutex<Box<ArtifactChannels>>,
    pub handles: Arc<HandleTable>,
    pub idempotency: Arc<IdempotencyStore>,
    pub audit: Arc<AuditSinks>,
    pub policy: PolicyEngine,
    pub client: ClientInfo,
    invocation_counter: AtomicU64,
    history_undo_port: Mutex<Option<Arc<dyn HistoryUndoPort>>>,
    approvals: Mutex<Option<Arc<ApprovalCoordinator>>>,
}

pub(crate) const INSTANCE_BUSY_MAX_ATTEMPTS: u32 = 3;

impl ActionAdapter {
    pub fn new(channel: Box<ArtifactChannels>, handles: Arc<HandleTable>, idempotency: Arc<IdempotencyStore>, audit: Arc<AuditSinks>, auto_approve: AutoApprovePolicy, client: ClientInfo) -> Self {
        let policy = PolicyEngine::new(handles.clone(), auto_approve);
        Self { channel: Mutex::new(channel), handles, idempotency, audit, policy, client, invocation_counter: AtomicU64::new(0), history_undo_port: Mutex::new(None), approvals: Mutex::new(None) }
    }

    //#region 💡️Inference
    /// ⚖️ The one shared decision layer this adapter already owns — handed out (never copied) so a
    /// facet outside the mutation protocol enforces the SAME granted-scope set against the SAME
    /// `HandleTable`, rather than standing up a second, disjoint policy engine of its own.
    pub fn policy(&self) -> &PolicyEngine {
        &self.policy
    }

    /// 🎫️ The one shared handle table, for a facet that mints or resolves session-owned handles.
    pub fn handles(&self) -> &Arc<HandleTable> {
        &self.handles
    }

    /// 💡️ Runs one plugin-declared inference over the SAME [`ArtifactChannel`] every mutation
    /// already travels — the general execution route that replaces `channel.not-wired` for every
    /// declared inference service, not just the one hub-backed GIS Map job. This adapter adds no
    /// interpretation of its own: it serialises the command onto the channel and maps the reply
    /// frame, so a guest rejection surfaces as that guest's own [`Fault`] through [`map_fault`] and
    /// a wrong-shaped reply is a loud `Internal`, never a fabricated value.
    pub fn run_inference(&self, instance: u32, command: InferCommand) -> Result<InferenceOutcome, GatewayError> {
        let requested_schema = command.inference_schema.clone();
        let mut channel = self.channel.lock().expect("artifact channel lock poisoned");
        let frames = channel.exchange(instance, vec![AppCommand::Infer(command)]).map_err(|fault| map_fault(&fault))?;
        drop(channel);
        match frames.into_iter().next() {
            Some(AppFrame::Inferred { inference_schema, complete, payload }) => Ok(InferenceOutcome { inference_schema, complete, payload }),
            Some(AppFrame::Error(fault)) => Err(map_fault(&fault)),
            Some(other) => Err(GatewayError::new(GatewayErrorCode::Internal, format!("`{requested_schema}` answered with {other:?} instead of an inference result"))),
            None => Err(GatewayError::new(GatewayErrorCode::Internal, format!("`{requested_schema}` produced no reply frame"))),
        }
    }

    /// 🔌 Binds the sole workspace-owned remote history implementation before serving tools.
    pub fn bind_history_undo_port(&self, port: Arc<dyn HistoryUndoPort>) {
        *self.history_undo_port.lock().expect("history undo port lock poisoned") = Some(port);
    }

    /// 🔌️ Activates the session `instance` addresses under `scope` — see [`ArtifactChannel::activate`].
    pub fn activate_session(&self, instance: u32, scope: &ActivationScope) -> Result<(), GatewayError> {
        self.channel.lock().expect("artifact channel lock poisoned").activate(instance, scope).map_err(|fault| map_fault(&fault))
    }

    /// 📖️ The LIVE document of the guest session `instance` addresses, straight off this adapter's
    /// own channel — the same instance every `action_prepare`/`action_invoke` on that plugin drives.
    ///
    /// 🧊️ `artifact_snapshot` needs exactly this. Reading the persisted row instead answered the
    /// bytes as they were when the artifact was created, so a snapshot taken after a committed
    /// mutation was byte-identical to one taken before it and an agent following the shipped
    /// `mutate-safely` prompt ("re-read the artifact and confirm the change landed") could not
    /// (`📓️ce1-client-e2e-pinning-and-puzzle-bound.md` §8 gap 4).
    pub fn read_session_artifact(&self, instance: u32) -> Result<(Vec<u8>, Vec<u8>), GatewayError> {
        match self.exchange_one(instance, AppCommand::ReadArtifact)? {
            AppFrame::Artifact { pack, spr } => Ok((pack, spr)),
            other => Err(GatewayError::new(GatewayErrorCode::Internal, format!("instance {instance} answered ReadArtifact with {other:?}"))),
        }
    }

    /// ⛩️ Binds the human-in-the-loop resolution chain a parked approval is offered through
    /// (`🛡️policy::ApprovalCoordinator`). Unbound, a destructive capability still parks a handle and
    /// still answers `APPROVAL_REQUIRED` — it just says so without having asked anyone, which is the
    /// honest headless tier, never a silent proceed.
    pub fn bind_approval_coordinator(&self, coordinator: Arc<ApprovalCoordinator>) {
        *self.approvals.lock().expect("approval coordinator lock poisoned") = Some(coordinator);
    }

    pub fn approval_coordinator(&self) -> Option<Arc<ApprovalCoordinator>> {
        self.approvals.lock().expect("approval coordinator lock poisoned").clone()
    }

    /// 🪪 Mints one session-private undo token from a Hub receipt without retaining mutation bytes.
    pub fn retain_hub_inference_approval_undo(
        &self,
        session: &SessionHandle,
        hub_origin: &str,
        scope: &semio_framework_os_kernel::os_directory::DocumentScope,
        route: &str,
        handle:&semio_framework_value::DslValue,
        now_ms: u64,
    ) -> Result<String, GatewayError> {
        semio_framework_os_kernel::os_directory::client::document_http::CompiledDocumentHttpPortV1::validate_payload(handle,16*1024).map_err(|_|GatewayError::new(GatewayErrorCode::InputInvalid,"invalid owner history value"))?;
        if hub_origin.is_empty() || hub_origin.len()>2048 || !is_hub_inference_route(route) || !matches!(handle,semio_framework_value::DslValue::Object(_)) || semio_framework_pack_json::to_json_string(handle).len()>16*1024 {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid,"invalid durable owner history locator"));
        }
        let identity=format!("semio.mcp.owner-history/v1\0{}\0{}\0{}\0{}\0{}\0{}",session.0,hub_origin,route,scope.space_id,scope.document_id,semio_framework_pack_json::to_json_string(handle));
        let idempotency_key=framework_hash::hash_bytes(identity.as_bytes())[..32].to_owned();
        let member=UndoMember::HubInferenceApproval(HubInferenceApprovalUndoMemberV1 {hub_origin:hub_origin.into(),route:route.into(),space_id:scope.space_id.clone(),document_id:scope.document_id.clone(),idempotency_key,payload:handle.clone()});
        let payload = serde_json::to_value(UndoRecord { members: vec![member] }).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        Ok(self.handles.mint(HandleKind::Undo, session.clone(), Attachment::Other { label: "hub-inference-approval".into() }, payload, now_ms))
    }
    //#endregion 💡️Inference

    fn next_invocation_id(&self) -> String {
        format!("inv_{}", self.invocation_counter.fetch_add(1, Ordering::Relaxed))
    }

    fn exchange_one(&self, instance: u32, command: AppCommand) -> Result<AppFrame, GatewayError> {
        let result = self.channel.lock().expect("artifact channel lock poisoned").exchange(instance, vec![command]);
        match result {
            Ok(mut frames) => match frames.pop() {
                Some(AppFrame::Error(fault)) => Err(map_fault(&fault)),
                Some(frame) => Ok(frame),
                None => Err(GatewayError::new(GatewayErrorCode::Internal, "artifact channel returned no frame for one command")),
            },
            Err(fault) => Err(map_fault(&fault)),
        }
    }

    fn record_audit(&self, ctx: AuditContext<'_>, decision: AuditDecision, revision_before: Option<RevisionStamp>, revision_after: Option<RevisionStamp>, outcome: &str, error: Option<GatewayError>, undo_token: Option<String>, now_ms: u64) {
        let event = AgentAuditEvent {
            invocation_id: ctx.invocation_id.to_string(),
            ts_ms: now_ms,
            principal: ctx.principal.id.clone(),
            session: ctx.session.0.clone(),
            capability: ctx.capability_id.to_string(),
            input_hash: hash_input(ctx.raw_input),
            input_redacted: redact_input(ctx.raw_input, SENSITIVE_KEYS),
            decision,
            preview_hash: None,
            txn_id: None,
            edit_ids: Vec::new(),
            revision_before,
            revision_after,
            outcome: outcome.to_string(),
            error,
            duration_ms: 0,
            undo_token,
            client: self.client.clone(),
        };
        let _ = self.audit.append(&event);
    }

    /// ↩️ Bounded retry against `transaction.instance-busy` — a fresh `txn_id` per attempt, since the
    /// occupying transaction (if genuinely a different in-flight caller) has no reason to clear on our
    /// retrying with the same id. Any OTHER mapped error (e.g. `REVISION_CONFLICT` from a concurrent
    /// generation bump) bubbles immediately, no retry.
    fn transaction_prepare_with_retry(&self, instance: u32, ops: PreparedOps, origin: MutationOrigin, now_ms: u64) -> Result<String, GatewayError> {
        let mut last_error = None;
        for _ in 0..INSTANCE_BUSY_MAX_ATTEMPTS {
            let txn_id = mint_id(HandleKind::Transaction, now_ms);
            match self.exchange_one(instance, AppCommand::TransactionPrepare { txn_id: txn_id.clone(), ops: ops.clone(), origin: origin.clone() }) {
                Ok(AppFrame::TransactionPrepared { txn_id }) => return Ok(txn_id),
                Ok(other) => return Err(GatewayError::new(GatewayErrorCode::Internal, format!("unexpected frame from TransactionPrepare: {other:?}"))),
                Err(error) if error.code == GatewayErrorCode::PreconditionFailed => last_error = Some(error),
                Err(error) => return Err(error),
            }
        }
        Err(last_error.unwrap_or_else(|| GatewayError::new(GatewayErrorCode::PreconditionFailed, "instance busy, retries exhausted")))
    }

    //#region 🔖️PrepareAndPreview
    /// 🧾️ `📋️master.md` §3.3 Prepare + Preview, combined into one call (matches `PreparedActionReport`
    /// carrying the preview inline): validate → resolve capability → policy → capture baseline
    /// revision (Observe) → dry-run `PureCommand` (Preview) → mint `prep_`.
    pub fn prepare(&self, catalog: &Catalog, principal: &AgentPrincipal, session: &SessionHandle, capability_id: &str, input: serde_json::Value, instance: u32, now_ms: u64) -> Result<PreparedActionReport, GatewayError> {
        let invocation_id = self.next_invocation_id();

        let capability = match catalog.get(capability_id) {
            Some(capability) => capability,
            None => {
                let error = GatewayError::new(GatewayErrorCode::NotFound, format!("unknown capability: {capability_id}"));
                self.record_audit(AuditContext { invocation_id: &invocation_id, principal, session, capability_id, raw_input: &input }, AuditDecision::Denied { code: error.code }, None, None, "not_found", Some(error.clone()), None, now_ms);
                return Err(error);
            }
        };

        let input = declared_effective_input(capability, input);
        if let Ok(validator) = compile_validator(&capability.input_schema) {
            if let Err(validation_error) = validate(&validator, &input) {
                let error = GatewayError::new(GatewayErrorCode::InputInvalid, validation_error);
                self.record_audit(AuditContext { invocation_id: &invocation_id, principal, session, capability_id, raw_input: &input }, AuditDecision::Denied { code: error.code }, None, None, "input_invalid", Some(error.clone()), None, now_ms);
                return Err(error);
            }
        }

        if let Err(error) = self.policy.authorize_scopes(principal, capability) {
            self.record_audit(AuditContext { invocation_id: &invocation_id, principal, session, capability_id, raw_input: &input }, AuditDecision::Denied { code: error.code }, None, None, "permission_denied", Some(error.clone()), None, now_ms);
            return Err(error);
        }

        let baseline = match self.exchange_one(instance, AppCommand::ReadHistory)? {
            AppFrame::HistorySnapshot(revision) => revision,
            other => return Err(GatewayError::new(GatewayErrorCode::Internal, format!("unexpected frame from ReadHistory: {other:?}"))),
        };

        let (ops, warnings) = match self.exchange_one(instance, AppCommand::PureCommand { capability_id: capability_id.to_string(), input: input.clone() })? {
            AppFrame::Emit { ops, warnings } => (ops, warnings),
            other => return Err(GatewayError::new(GatewayErrorCode::Internal, format!("unexpected frame from PureCommand: {other:?}"))),
        };

        let record = PreparedActionRecord { capability_id: capability_id.to_string(), input: input.clone(), instance, baseline: baseline.clone(), ops: ops.clone(), principal_id: principal.id.clone() };
        let payload = serde_json::to_value(&record).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        let prepared_handle = self.handles.mint(HandleKind::Prepared, session.clone(), Attachment::Capability { capability_id: capability_id.to_string() }, payload, now_ms);

        self.record_audit(AuditContext { invocation_id: &invocation_id, principal, session, capability_id, raw_input: &input }, AuditDecision::Allowed, Some(baseline.clone()), None, "prepared", None, None, now_ms);

        Ok(PreparedActionReport {
            prepared_handle,
            capability_id: capability_id.to_string(),
            expected_revision: Some(baseline),
            preview: serde_json::json!({ "opsCount": ops.op_counts(), "warnings": warnings }),
            expires_at_ms: now_ms.saturating_add(HandleKind::Prepared.default_ttl_ms()),
        })
    }
    //#endregion 🔖️PrepareAndPreview

    //#region 🔖️Cancel
    /// 🛑️ Drops a prepared handle — job-class cancellation (`Effect::CancelJob`) is P7's concern once
    /// background jobs actually run in this crate; nothing mints a `job_` handle here yet.
    pub fn cancel(&self, session: &SessionHandle, prepared_handle: &str, now_ms: u64) -> Result<(), GatewayError> {
        let record = self.handles.resolve(prepared_handle, session, now_ms)?;
        if record.kind != HandleKind::Prepared {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "handle is not a prepared-action handle"));
        }
        self.handles.revoke(prepared_handle);
        Ok(())
    }
    //#endregion 🔖️Cancel

    //#region 🔖️Invoke
    /// 🚀️ `action.invoke` = prepare (if not already prepared) + Approve + Commit + Verify — the
    /// public entry point. When `idempotencyKey` is supplied, the IDEMPOTENCY LOOKUP RUNS FIRST,
    /// before any prepared-handle resolution: `IdempotencyStore::get_or_insert_with`'s `compute`
    /// closure (which resolves the handle, checks policy, checks the revision, and commits) only runs
    /// on a genuine cache MISS. A cache HIT therefore returns the stored `InvocationReport`
    /// (`replayed: true`) without ever touching the prepared handle again — this is deliberate: a
    /// `prep_` handle is one-shot (revoked the instant its first `invoke` completes, §🔖️PrepareAndPreview/
    /// this region), so a replay that re-resolved it would always fail with `NOT_FOUND` (the bug a
    /// post-unblock review caught, see `📓️terra-P6-report.md` "post-unblock fixes"). The idempotency
    /// key, not the prepared handle, is the source of truth for what a replay returns.
    pub fn invoke(&self, catalog: &Catalog, principal: &AgentPrincipal, session: &SessionHandle, request: InvokeRequest, instance: u32, now_ms: u64) -> Result<InvocationReport, GatewayError> {
        match &request.idempotency_key {
            Some(key) => {
                let failure: std::cell::RefCell<Option<GatewayError>> = std::cell::RefCell::new(None);
                let placeholder_capability_id = request.capability_id.clone().or_else(|| request.prepared_handle.clone()).unwrap_or_default();
                let report = self.idempotency.get_or_insert_with(&principal.id, key, now_ms, || match self.invoke_uncached(catalog, principal, session, &request, instance, now_ms) {
                    Ok(report) => report,
                    Err(error) => {
                        let failed = InvocationReport {
                            invocation_id: self.next_invocation_id(),
                            capability_id: placeholder_capability_id.clone(),
                            status: InvocationStatus::Failed,
                            affected_resources: Vec::new(),
                            revision_before: None,
                            revision_after: None,
                            diff_uri: None,
                            warnings: vec![error.message.clone()],
                            undo_token: None,
                            postconditions: Vec::new(),
                            replayed: false,
                        };
                        *failure.borrow_mut() = Some(error);
                        failed
                    }
                });
                match failure.into_inner() {
                    Some(error) => Err(error),
                    None => Ok(report),
                }
            }
            None => self.invoke_uncached(catalog, principal, session, &request, instance, now_ms),
        }
    }

    /// 🚀️ The real, always-fresh invocation logic — resolve/prepare → Approve → revision check →
    /// Commit → Verify → audit → revoke the one-shot `prep_` handle. Never called twice for the SAME
    /// idempotency key within its TTL window (`invoke`'s own job); called directly when no
    /// `idempotencyKey` was supplied at all.
    fn invoke_uncached(&self, catalog: &Catalog, principal: &AgentPrincipal, session: &SessionHandle, request: &InvokeRequest, instance: u32, now_ms: u64) -> Result<InvocationReport, GatewayError> {
        let invocation_id = self.next_invocation_id();

        let (prep_handle_id, record): (Option<String>, PreparedActionRecord) = if let Some(handle) = &request.prepared_handle {
            let resolved = self.handles.resolve(handle, session, now_ms)?;
            if resolved.kind != HandleKind::Prepared {
                return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "handle is not a prepared-action handle"));
            }
            let stored: PreparedActionRecord = serde_json::from_value(resolved.payload).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
            (Some(handle.clone()), stored)
        } else {
            let capability_id = request.capability_id.clone().ok_or_else(|| GatewayError::new(GatewayErrorCode::InputInvalid, "capabilityId or preparedActionHandle is required"))?;
            let input = request.input.clone().unwrap_or_else(|| serde_json::json!({}));
            if request.expected_revision.is_none() {
                let omitted = catalog.get(&capability_id).filter(|capability| self.policy.authorize_scopes(principal, capability).is_ok()).map(|capability| omitted_revision_arguments(capability, &input)).unwrap_or_default();
                if !omitted.is_empty() {
                    let error = revision_guard_required(&capability_id, &omitted);
                    self.record_audit(AuditContext { invocation_id: &invocation_id, principal, session, capability_id: &capability_id, raw_input: &input }, AuditDecision::Denied { code: error.code }, None, None, "revision_guard_required", Some(error.clone()), None, now_ms);
                    return Err(error);
                }
            }
            let prepared = self.prepare(catalog, principal, session, &capability_id, input, instance, now_ms)?;
            let resolved = self.handles.resolve(&prepared.prepared_handle, session, now_ms)?;
            let stored: PreparedActionRecord = serde_json::from_value(resolved.payload).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
            (Some(prepared.prepared_handle), stored)
        };

        let capability = catalog.get(&record.capability_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("capability {} no longer exists in the catalog", record.capability_id)))?;

        let diff_summary = serde_json::json!({ "capabilityId": record.capability_id, "opsCount": record.ops.op_counts() });
        if let ApprovalGate::Required { approval_handle } = self.policy.gate_approval(principal, capability, diff_summary.clone(), request.approval_handle.as_deref(), session, now_ms) {
            let (error, outcome) = match self.settle_approval(principal, capability, session, &approval_handle, &diff_summary, now_ms) {
                SettledApproval::Approved => (None, "approved"),
                SettledApproval::Denied { channel, note } => (
                    Some(
                        GatewayError::new(GatewayErrorCode::PermissionDenied, format!("a human denied approval for capability {} over the {channel} channel", capability.id))
                            .with_details(serde_json::json!({ "approvalHandle": approval_handle, "channel": channel, "note": note })),
                    ),
                    "approval_denied",
                ),
                SettledApproval::Unreachable { details } => (
                    Some(
                        GatewayError::new(GatewayErrorCode::ApprovalRequired, format!("capability {} requires a human approval that nothing attached to this gateway can give", capability.id))
                            .with_details(details),
                    ),
                    "approval_required",
                ),
                SettledApproval::Cancelled => (
                    Some(GatewayError::new(GatewayErrorCode::Cancelled, format!("the call to capability {} was cancelled while it awaited a human approval", capability.id)).with_details(serde_json::json!({ "approvalHandle": approval_handle }))),
                    "cancelled",
                ),
            };
            if let Some(error) = error {
                self.record_audit(
                    AuditContext { invocation_id: &invocation_id, principal, session, capability_id: &record.capability_id, raw_input: &record.input },
                    AuditDecision::Denied { code: error.code },
                    Some(record.baseline.clone()),
                    None,
                    outcome,
                    Some(error.clone()),
                    None,
                    now_ms,
                );
                return Err(error);
            }
            self.record_audit(
                AuditContext { invocation_id: &invocation_id, principal, session, capability_id: &record.capability_id, raw_input: &record.input },
                AuditDecision::Allowed,
                Some(record.baseline.clone()),
                None,
                outcome,
                None,
                None,
                now_ms,
            );
        }

        let expected = request.expected_revision.clone().unwrap_or_else(|| record.baseline.clone());
        let current = match self.exchange_one(record.instance, AppCommand::ReadHistory)? {
            AppFrame::HistorySnapshot(revision) => revision,
            other => return Err(GatewayError::new(GatewayErrorCode::Internal, format!("unexpected frame from ReadHistory: {other:?}"))),
        };
        if current != expected {
            let error = GatewayError::new(GatewayErrorCode::RevisionConflict, format!("expected revision cursor {} but current is {}", expected.cursor, current.cursor)).with_details(serde_json::json!({ "expected": expected, "actual": current }));
            self.record_audit(
                AuditContext { invocation_id: &invocation_id, principal, session, capability_id: &record.capability_id, raw_input: &record.input },
                AuditDecision::Denied { code: error.code },
                Some(current.clone()),
                None,
                "revision_conflict",
                Some(error.clone()),
                None,
                now_ms,
            );
            return Err(error);
        }

        if record.ops.is_empty() {
            let report = InvocationReport {
                invocation_id: invocation_id.clone(),
                capability_id: record.capability_id.clone(),
                status: InvocationStatus::Succeeded,
                affected_resources: Vec::new(),
                revision_before: Some(current.clone()),
                revision_after: Some(current.clone()),
                diff_uri: None,
                warnings: vec![NO_CHANGE_WARNING.to_string()],
                undo_token: None,
                postconditions: Vec::new(),
                replayed: false,
            };
            self.record_audit(
                AuditContext { invocation_id: &invocation_id, principal, session, capability_id: &record.capability_id, raw_input: &record.input },
                AuditDecision::Allowed,
                Some(current.clone()),
                Some(current),
                "no_change",
                None,
                None,
                now_ms,
            );
            if let Some(handle) = prep_handle_id {
                self.handles.revoke(&handle);
            }
            return Ok(report);
        }

        let effects_writes: Vec<String> = capability.effects.writes.iter().map(|selector| selector.0.clone()).collect();
        let origin = MutationOrigin::Agent { principal: principal.id.clone(), invocation_id: invocation_id.clone() };
        let commit_result = match self.transaction_prepare_with_retry(record.instance, record.ops.clone(), origin, now_ms) {
            Ok(txn_id) => match self.exchange_one(record.instance, AppCommand::TransactionCommit { txn_id: txn_id.clone() }) {
                Ok(AppFrame::TransactionCommitted { edit_id, relay: Some(relay), .. }) if !relay.acknowledged => {
                    let _ = self.exchange_one(record.instance, AppCommand::TransactionUndo { group_id: txn_id.clone() });
                    Err(unacknowledged_relay_error(&edit_id, &relay))
                }
                Ok(AppFrame::TransactionCommitted { edit_id, relay, .. }) => {
                    let after = match self.exchange_one(record.instance, AppCommand::ReadHistory) {
                        Ok(AppFrame::HistorySnapshot(revision)) => revision,
                        _ => current.clone(),
                    };
                    let undo_record = UndoRecord { members: vec![UndoMember::LocalGuestTransaction { instance: record.instance, transaction_id: txn_id.clone() }] };
                    let undo_token = self.handles.mint(HandleKind::Undo, session.clone(), Attachment::Capability { capability_id: record.capability_id.clone() }, serde_json::to_value(&undo_record).unwrap_or_default(), now_ms);
                    Ok(InvocationReport {
                        invocation_id: invocation_id.clone(),
                        capability_id: record.capability_id.clone(),
                        status: InvocationStatus::Succeeded,
                        affected_resources: effects_writes,
                        revision_before: Some(current.clone()),
                        revision_after: Some(after),
                        diff_uri: None,
                        warnings: Vec::new(),
                        undo_token: Some(undo_token),
                        postconditions: std::iter::once(format!("edit:{edit_id}")).chain(relay.map(|_| "relay:acknowledged".to_string())).collect(),
                        replayed: false,
                    })
                }
                Ok(other) => Err(GatewayError::new(GatewayErrorCode::Internal, format!("unexpected frame from TransactionCommit: {other:?}"))),
                Err(error) => {
                    let _ = self.exchange_one(record.instance, AppCommand::TransactionRollback { txn_id: txn_id.clone() });
                    Err(error)
                }
            },
            Err(error) => Err(error),
        };

        match &commit_result {
            Ok(report) => self.record_audit(
                AuditContext { invocation_id: &invocation_id, principal, session, capability_id: &record.capability_id, raw_input: &record.input },
                AuditDecision::Allowed,
                Some(current.clone()),
                report.revision_after.clone(),
                "succeeded",
                None,
                report.undo_token.clone(),
                now_ms,
            ),
            Err(error) => self.record_audit(
                AuditContext { invocation_id: &invocation_id, principal, session, capability_id: &record.capability_id, raw_input: &record.input },
                AuditDecision::Denied { code: error.code },
                Some(current.clone()),
                None,
                "failed",
                Some(error.clone()),
                None,
                now_ms,
            ),
        }

        if let Some(handle) = prep_handle_id {
            self.handles.revoke(&handle);
        }

        commit_result
    }
    //#endregion 🔖️Invoke

    //#region 🔖️ApprovalResolution
    pub fn resolve_approval(&self, session: &SessionHandle, approval_handle: &str, approve: bool, now_ms: u64) -> Result<String, GatewayError> {
        self.policy.resolve_approval(session, approval_handle, approve, now_ms)
    }

    /// ⛩️ Offers the freshly-parked `approval_handle` to the bound [`ApprovalCoordinator`] and, on a
    /// human yes, consumes the gate for THIS invocation: the pending record is decided through the
    /// same `PolicyEngine::resolve_approval` a replayed `approvalHandle` goes through, and the fresh
    /// decided handle is immediately re-gated so the proceed path is the identical code a replay
    /// takes. No coordinator bound, or nothing attached to ask, is `Unreachable` — never a proceed.
    fn settle_approval(&self, principal: &AgentPrincipal, capability: &CapabilityDefinition, session: &SessionHandle, approval_handle: &str, diff_summary: &serde_json::Value, now_ms: u64) -> SettledApproval {
        let Some(coordinator) = self.approval_coordinator() else {
            return SettledApproval::Unreachable {
                details: serde_json::json!({
                    "approvalHandle": approval_handle,
                    "channels": { "elicitation": "no approval coordinator is bound to this gateway", "shell": "no approval coordinator is bound to this gateway" },
                    "remedy": "this build wires the coordinator in `run_stdio`/`run_http`; a server constructed without one can never resolve an approval",
                }),
            };
        };
        let capability_id = capability.id.to_string();
        let request = ApprovalRequest {
            approval_handle,
            capability_id: &capability_id,
            capability_title: &capability.title,
            capability_description: &capability.description,
            artifact_kind: capability.artifact_kind.as_deref(),
            principal_id: &principal.id,
            diff_summary,
        };
        match coordinator.resolve(&request) {
            ApprovalResolution::Approved { channel } => match self.policy.resolve_approval(session, approval_handle, true, now_ms) {
                Ok(decided) => match self.policy.gate_approval(principal, capability, diff_summary.clone(), Some(&decided), session, now_ms) {
                    ApprovalGate::Proceed => SettledApproval::Approved,
                    ApprovalGate::Required { .. } => SettledApproval::Unreachable {
                        details: serde_json::json!({ "approvalHandle": approval_handle, "channel": channel.as_str(), "reason": "the decided approval handle did not satisfy its own gate" }),
                    },
                },
                Err(error) => SettledApproval::Unreachable { details: serde_json::json!({ "approvalHandle": approval_handle, "channel": channel.as_str(), "reason": error.message }) },
            },
            ApprovalResolution::Denied { channel, note } => {
                let _ = self.policy.resolve_approval(session, approval_handle, false, now_ms);
                SettledApproval::Denied { channel: channel.as_str(), note }
            }
            ApprovalResolution::Unreachable { details } => SettledApproval::Unreachable { details },
            ApprovalResolution::Cancelled => {
                let _ = self.policy.resolve_approval(session, approval_handle, false, now_ms);
                SettledApproval::Cancelled
            }
        }
    }
    //#endregion 🔖️ApprovalResolution

    //#region 🔖️Saga
    /// 🪢️ Binds several already-`prepare`d handles (potentially across different instances/artifacts)
    /// into one `txn_` saga handle — `📋️master.md` §3.3 "Multi-provider".
    pub fn transaction_begin(&self, session: &SessionHandle, prepared_handles: &[String], now_ms: u64) -> Result<String, GatewayError> {
        if prepared_handles.is_empty() {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "transaction.begin requires at least one prepared handle"));
        }
        let mut members = Vec::with_capacity(prepared_handles.len());
        for handle in prepared_handles {
            let resolved = self.handles.resolve(handle, session, now_ms)?;
            if resolved.kind != HandleKind::Prepared {
                return Err(GatewayError::new(GatewayErrorCode::InputInvalid, format!("{handle} is not a prepared-action handle")));
            }
            let record: PreparedActionRecord = serde_json::from_value(resolved.payload).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
            members.push(SagaMember { prepared_handle: handle.clone(), capability_id: record.capability_id, instance: record.instance, ops: record.ops });
        }
        let payload = serde_json::to_value(&SagaRecord { members }).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        Ok(self.handles.mint(HandleKind::Transaction, session.clone(), Attachment::Other { label: "saga".into() }, payload, now_ms))
    }

    pub fn transaction_rollback(&self, session: &SessionHandle, saga_handle: &str, now_ms: u64) -> Result<(), GatewayError> {
        let record = self.handles.resolve(saga_handle, session, now_ms)?;
        if record.kind != HandleKind::Transaction {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "handle is not a saga transaction handle"));
        }
        self.handles.revoke(saga_handle);
        Ok(())
    }

    /// 🪢️ Phase 1: prepare every member in discovery order, rolling back already-prepared members
    /// (reverse order) on any rejection. Phase 2: commit in REVERSE discovery order; on any commit
    /// failure, compensate every already-committed member via `TransactionUndo` and roll back any
    /// still-only-prepared member — `COMPENSATION_FAILED` iff compensation ITSELF fails for at least
    /// one member (`📓️luna-channel-audit.md` §3, `HostTransactionCoordinator::run_transaction`
    /// reimplemented against this packet's own port, since the real coordinator lives in the peer
    /// ticket's `🔌️plugin/🖥️host` territory).
    pub fn transaction_commit(&self, principal: &AgentPrincipal, session: &SessionHandle, saga_handle: &str, now_ms: u64) -> Result<SagaReport, GatewayError> {
        let record = self.handles.resolve(saga_handle, session, now_ms)?;
        if record.kind != HandleKind::Transaction {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "handle is not a saga transaction handle"));
        }
        let saga: SagaRecord = serde_json::from_value(record.payload).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;

        let mut prepared_txn_ids: Vec<(usize, String)> = Vec::new();
        for (index, member) in saga.members.iter().enumerate() {
            if member.ops.is_empty() {
                continue;
            }
            let origin = MutationOrigin::Agent { principal: principal.id.clone(), invocation_id: format!("{saga_handle}-{index}") };
            match self.transaction_prepare_with_retry(member.instance, member.ops.clone(), origin, now_ms) {
                Ok(txn_id) => prepared_txn_ids.push((index, txn_id)),
                Err(error) => {
                    for (rollback_index, txn_id) in prepared_txn_ids.iter().rev() {
                        let _ = self.exchange_one(saga.members[*rollback_index].instance, AppCommand::TransactionRollback { txn_id: txn_id.clone() });
                    }
                    self.handles.revoke(saga_handle);
                    return Err(error);
                }
            }
        }

        let mut committed: Vec<(usize, String, String)> = Vec::new();
        let mut commit_error: Option<GatewayError> = None;
        for (index, txn_id) in prepared_txn_ids.iter().rev() {
            match self.exchange_one(saga.members[*index].instance, AppCommand::TransactionCommit { txn_id: txn_id.clone() }) {
                Ok(AppFrame::TransactionCommitted { edit_id, relay: Some(relay), .. }) if !relay.acknowledged => {
                    commit_error = Some(unacknowledged_relay_error(&edit_id, &relay));
                    committed.push((*index, txn_id.clone(), edit_id));
                    break;
                }
                Ok(AppFrame::TransactionCommitted { edit_id, .. }) => committed.push((*index, txn_id.clone(), edit_id)),
                Ok(other) => {
                    commit_error = Some(GatewayError::new(GatewayErrorCode::Internal, format!("unexpected frame from TransactionCommit: {other:?}")));
                    break;
                }
                Err(error) => {
                    commit_error = Some(error);
                    break;
                }
            }
        }

        if let Some(error) = commit_error {
            let mut compensation_failed = false;
            for (index, txn_id, _edit_id) in &committed {
                if self.exchange_one(saga.members[*index].instance, AppCommand::TransactionUndo { group_id: txn_id.clone() }).is_err() {
                    compensation_failed = true;
                }
            }
            let committed_indices: std::collections::BTreeSet<usize> = committed.iter().map(|(index, ..)| *index).collect();
            for (index, txn_id) in &prepared_txn_ids {
                if !committed_indices.contains(index) {
                    let _ = self.exchange_one(saga.members[*index].instance, AppCommand::TransactionRollback { txn_id: txn_id.clone() });
                }
            }
            self.handles.revoke(saga_handle);
            if compensation_failed {
                return Err(
                    GatewayError::new(GatewayErrorCode::CompensationFailed, "compensation of already-committed saga members failed; manual recovery required").with_details(serde_json::json!({ "originalError": error.to_tool_error_payload() }))
                );
            }
            return Err(error);
        }

        self.handles.revoke(saga_handle);
        let undo_record = UndoRecord {
            members: committed
                .iter()
                .map(|(index, txn_id, _)| UndoMember::LocalGuestTransaction { instance: saga.members[*index].instance, transaction_id: txn_id.clone() })
                .collect(),
        };
        let undo_token = self.handles.mint(HandleKind::Undo, session.clone(), Attachment::Other { label: "saga".into() }, serde_json::to_value(&undo_record).unwrap_or_default(), now_ms);

        let mut ordered = committed;
        ordered.sort_by_key(|(index, ..)| *index);
        let members: Vec<SagaMemberResult> =
            ordered.into_iter().map(|(index, _, edit_id)| SagaMemberResult { prepared_handle: saga.members[index].prepared_handle.clone(), capability_id: saga.members[index].capability_id.clone(), edit_id }).collect();
        Ok(SagaReport { transaction_handle: saga_handle.to_string(), members, undo_token })
    }
    //#endregion 🔖️Saga

    //#region 🔖️UndoRedo
    fn fan_out(&self, session: &SessionHandle, undo_token: &str, now_ms: u64, redo: bool) -> Result<UndoRedoReport, GatewayError> {
        let record = self.handles.resolve(undo_token, session, now_ms)?;
        if record.kind != HandleKind::Undo {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "handle is not an undo handle"));
        }
        let undo: UndoRecord = serde_json::from_value(record.payload).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        let mut warnings = Vec::new();
        for member in &undo.members {
            let result = match member {
                UndoMember::LocalGuestTransaction { instance, transaction_id } => match self.exchange_one(
                    *instance,
                    if redo { AppCommand::TransactionRedo { group_id: transaction_id.clone() } } else { AppCommand::TransactionUndo { group_id: transaction_id.clone() } },
                ) {
                    Ok(AppFrame::TransactionRedone { group_id }) if redo && group_id == *transaction_id => Ok(()),
                    Ok(AppFrame::TransactionUndone { group_id }) if !redo && group_id == *transaction_id => Ok(()),
                    Ok(frame) => Err((
                        format!("local transaction {transaction_id} on instance {instance}"),
                        GatewayError::new(GatewayErrorCode::Internal, format!("unexpected history frame: {frame:?}")),
                    )),
                    Err(error) => Err((format!("local transaction {transaction_id} on instance {instance}"), error)),
                },
                UndoMember::HubInferenceApproval(remote) if redo => Err((
                    format!("remote approval route {}", remote.route),
                    GatewayError::new(GatewayErrorCode::SideEffectRejected, "remote approval redo requires a new authenticated approval"),
                )),
                UndoMember::HubInferenceApproval(remote) => {
                    let port = self.history_undo_port.lock().expect("history undo port lock poisoned").clone();
                    port.ok_or_else(|| GatewayError::new(GatewayErrorCode::PluginUnavailable, "Hub durable history port is unavailable").retryable())
                        .and_then(|port| port.undo_hub_inference_approval(remote))
                        .map_err(|error| (format!("remote approval route {}", remote.route), error))
                }
            };
            if let Err((label, error)) = result {
                warnings.push(format!("member {label} failed: {}", error.message));
            }
        }
        if !undo.members.is_empty() && warnings.len() == undo.members.len() {
            return Err(GatewayError::new(GatewayErrorCode::SideEffectRejected, "undo/redo failed for every member").with_details(serde_json::json!({ "warnings": warnings })));
        }
        Ok(UndoRedoReport { undo_token: undo_token.to_string(), members: undo.members.len(), warnings })
    }

    /// ↩️ `history.undo` — `TransactionUndo{group_id}` fanned to every member this token covers,
    /// best-effort (a per-member failure is a warning, not a hard error, unless EVERY member fails).
    /// The handle stays resolvable afterward (never revoked) so a symmetric `history.redo` can follow.
    pub fn history_undo(&self, session: &SessionHandle, undo_token: &str, now_ms: u64) -> Result<UndoRedoReport, GatewayError> {
        self.fan_out(session, undo_token, now_ms, false)
    }

    pub fn history_redo(&self, session: &SessionHandle, undo_token: &str, now_ms: u64) -> Result<UndoRedoReport, GatewayError> {
        self.fan_out(session, undo_token, now_ms, true)
    }
    //#endregion 🔖️UndoRedo
}

//#endregion 🔖️ActionAdapter

//#region 🧪️Tests
//#endregion 🧪️Tests
