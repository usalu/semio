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
//! contract. [`MockArtifactChannel`] is a fully-scripted in-memory artifact store this crate's own
//! tests (and, until P7 lands, the live binary) drive against.

use crate::audit::{hash_input, redact_input, AgentAuditEvent, AuditDecision, AuditSink, AuditSinks, ClientInfo, SENSITIVE_KEYS};
use crate::catalog::Catalog;
use crate::errors::{GatewayError, GatewayErrorCode};
use crate::handles::{mint_id, Attachment, HandleKind, HandleTable, IdempotencyStore, SessionHandle};
use crate::policy::{AgentPrincipal, ApprovalGate, AutoApprovePolicy, PolicyEngine};
use crate::schema::{InvocationReport, InvocationStatus, PreparedActionReport, RevisionStamp};
use crate::workspace::ArtifactChannels;
use semio_framework_dispatch_macros::dyn_enum;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

//#region 🔖️Port
/// 📦️ One op payload set, split by store lane — matches `📓️luna-channel-audit.md` §4's recommended
/// `TransactionPrepare.prepared_ops` shape (`Vec<Vec<u8>>`, one element per complete op payload,
/// never a single stream). `Emit` returns this from `PureCommand`; `TransactionPrepare` sends the
/// SAME bytes back unmodified — this adapter never decodes an individual op's contents.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PreparedOps {
    pub document: Vec<Vec<u8>>,
    pub config: Vec<Vec<u8>>,
    pub draft: Vec<Vec<u8>>,
}

impl PreparedOps {
    fn op_counts(&self) -> serde_json::Value {
        serde_json::json!({ "document": self.document.len(), "config": self.config.len(), "draft": self.draft.len() })
    }
}

/// ✍️ `MutationOrigin::Agent` — the real channel's `origin` field on `TransactionPrepare`
/// (`📋️master.md` §3.3: `origin: MutationOrigin::Agent{..}`); this packet's own minimal mirror,
/// carrying just enough for the real channel's `Origin::Agent` (peer ticket packet A1, leased) to be
/// constructed FROM by P7's real `ArtifactChannel` implementation.
#[derive(Clone, Debug, PartialEq)]
pub enum MutationOrigin {
    Agent { principal: String, invocation_id: String },
}

/// 📤️ Commands this adapter sends — a deliberate, minimal subset of the real channel's `AppCommand`
/// (`📓️luna-channel-audit.md` §9: every variant here survives channel v12). No `seq`/batching: this
/// port's contract is ONE command per [`ArtifactChannel::exchange`] call, always exactly one reply
/// frame — simpler to implement and test than the real channel's batched duplex; P7's real
/// implementation is responsible for wrapping each call in a one-element batch (or its own
/// seq/in_reply_to bookkeeping) against the real channel underneath.
#[derive(Clone, Debug, PartialEq)]
pub enum AppCommand {
    ReadHistory,
    PureCommand { capability_id: String, input: serde_json::Value },
    TransactionPrepare { txn_id: String, ops: PreparedOps, label: String, origin: MutationOrigin },
    TransactionCommit { txn_id: String },
    TransactionRollback { txn_id: String },
    TransactionUndo { group_id: String },
    TransactionRedo { group_id: String },
}

/// 📥️ Replies — [`AppFrame::Error`] is a COMMAND-level (business) failure (e.g. generation-mismatch,
/// instance-busy); a hard transport-level failure (no such instance, dead connection) is
/// `Err(Fault)` at the [`ArtifactChannel::exchange`] boundary instead.
#[derive(Clone, Debug, PartialEq)]
pub enum AppFrame {
    HistorySnapshot(RevisionStamp),
    Emit { ops: PreparedOps, warnings: Vec<String> },
    TransactionPrepared { txn_id: String },
    TransactionCommitted { txn_id: String, edit_id: String },
    TransactionRolledBack { txn_id: String },
    TransactionUndone { group_id: String },
    TransactionRedone { group_id: String },
    Error(Fault),
}

/// ⚠️ This port's own minimal `Fault` — `code` mirrors the real channel's fault code strings
/// (`"transaction.generation-mismatch"`, `"transaction.instance-busy"`, `"mutation.rejected"`,
/// `"viewer.read-only"`, `"capability-denied"`) verbatim, per `📋️master.md` §3.3's Fault code table,
/// plus `"budget.exceeded"` (quota exhaustion, this crate's own addition, mapped to
/// `GatewayErrorCode::BudgetExceeded`).
#[derive(Clone, Debug, PartialEq)]
pub struct Fault {
    pub code: String,
    pub message: String,
}

/// 🔌️ The narrow port `ActionAdapter` drives — this packet's brief §3.1 names this exact shape.
/// `instance` is an opaque handle a real `InstanceDirectory` (P7) would resolve from a
/// `(plugin_id, app_id, artifact_ref)` triple; until P7 lands, every capability in this crate's live
/// binary targets a single default instance (`0`), documented at the call site in the module root.
// 🔀️ dedyn-fw-os-misc, O1/R11: closed 2-implementor set (`MockArtifactChannel` here,
// `🏠️workspace::PluginArtifactChannel`) — `#[dyn_enum]` here + `dyn_enum_close!` at `🏠️workspace`'s
// `ArtifactChannels` (the module both implementors are jointly nameable from) closes it into an enum
// instead of `Box<dyn ArtifactChannel>`.
#[dyn_enum]
pub trait ArtifactChannel: Send {
    fn exchange(&mut self, instance: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, Fault>;
}

/// 🧯️ `Fault.code` → `GatewayErrorCode` — `📋️master.md` §3.3's Fault code table, plus this crate's
/// own `"budget.exceeded"` addition (`GatewayErrorCode::BudgetExceeded`, retryable). An unrecognised
/// code is `Internal` (never silently swallowed).
fn map_fault(fault: &Fault) -> GatewayError {
    match fault.code.as_str() {
        "viewer.read-only" | "capability-denied" => GatewayError::new(GatewayErrorCode::PermissionDenied, fault.message.clone()),
        "mutation.rejected" => GatewayError::new(GatewayErrorCode::SideEffectRejected, fault.message.clone()),
        "transaction.generation-mismatch" => GatewayError::new(GatewayErrorCode::RevisionConflict, fault.message.clone()),
        "transaction.instance-busy" => GatewayError::new(GatewayErrorCode::PreconditionFailed, fault.message.clone()).retryable(),
        "budget.exceeded" => GatewayError::new(GatewayErrorCode::BudgetExceeded, fault.message.clone()).retryable(),
        _ => GatewayError::new(GatewayErrorCode::Internal, fault.message.clone()),
    }
}
//#endregion 🔖️Port

//#region 🔖️MockArtifactChannel
struct MockInstanceState {
    artifact_id: String,
    generation: u64,
    head_edit_id: u64,
    pending: Option<String>,
    prepared: BTreeMap<String, (PreparedOps, u64)>,
    force_budget_exceeded: bool,
    force_commit_fault: Option<Fault>,
    force_undo_fails: bool,
}

impl MockInstanceState {
    fn new(instance: u32) -> Self {
        Self { artifact_id: format!("mock-artifact-{instance}"), generation: 0, head_edit_id: 0, pending: None, prepared: BTreeMap::new(), force_budget_exceeded: false, force_commit_fault: None, force_undo_fails: false }
    }

    fn revision(&self) -> RevisionStamp {
        RevisionStamp { artifact_id: self.artifact_id.clone(), head_edit_id: format!("edit-{}", self.head_edit_id), cursor: format!("gen-{}", self.generation) }
    }

    fn handle(&mut self, command: AppCommand) -> AppFrame {
        match command {
            AppCommand::ReadHistory => AppFrame::HistorySnapshot(self.revision()),
            AppCommand::PureCommand { capability_id, input } => {
                let payload = serde_json::to_vec(&serde_json::json!({ "capabilityId": capability_id, "input": input })).unwrap_or_default();
                AppFrame::Emit { ops: PreparedOps { document: vec![payload], config: Vec::new(), draft: Vec::new() }, warnings: Vec::new() }
            }
            AppCommand::TransactionPrepare { txn_id, ops, .. } => {
                if self.force_budget_exceeded {
                    self.force_budget_exceeded = false;
                    return AppFrame::Error(Fault { code: "budget.exceeded".into(), message: "capability budget exhausted".into() });
                }
                if self.pending.is_some() {
                    return AppFrame::Error(Fault { code: "transaction.instance-busy".into(), message: "already has a pending transaction".into() });
                }
                self.pending = Some(txn_id.clone());
                self.prepared.insert(txn_id.clone(), (ops, self.generation));
                AppFrame::TransactionPrepared { txn_id }
            }
            AppCommand::TransactionCommit { txn_id } => {
                if let Some(fault) = self.force_commit_fault.take() {
                    self.pending = None;
                    self.prepared.remove(&txn_id);
                    return AppFrame::Error(fault);
                }
                match self.prepared.get(&txn_id).map(|(_, base_generation)| *base_generation) {
                    None => AppFrame::Error(Fault { code: "transaction.generation-mismatch".into(), message: format!("no prepared transaction {txn_id}") }),
                    Some(base_generation) => {
                        if base_generation != self.generation {
                            self.prepared.remove(&txn_id);
                            self.pending = None;
                            AppFrame::Error(Fault { code: "transaction.generation-mismatch".into(), message: format!("base generation {base_generation} no longer matches current generation {}", self.generation) })
                        } else {
                            self.generation += 1;
                            self.head_edit_id += 1;
                            let edit_id = format!("edit-{}", self.head_edit_id);
                            self.prepared.remove(&txn_id);
                            self.pending = None;
                            AppFrame::TransactionCommitted { txn_id, edit_id }
                        }
                    }
                }
            }
            AppCommand::TransactionRollback { txn_id } => {
                self.prepared.remove(&txn_id);
                if self.pending.as_deref() == Some(txn_id.as_str()) {
                    self.pending = None;
                }
                AppFrame::TransactionRolledBack { txn_id }
            }
            AppCommand::TransactionUndo { group_id } => {
                if self.force_undo_fails {
                    self.force_undo_fails = false;
                    return AppFrame::Error(Fault { code: "mutation.rejected".into(), message: "undo rejected".into() });
                }
                self.generation += 1;
                AppFrame::TransactionUndone { group_id }
            }
            AppCommand::TransactionRedo { group_id } => {
                self.generation += 1;
                AppFrame::TransactionRedone { group_id }
            }
        }
    }
}

struct MockChannelState {
    instances: BTreeMap<u32, MockInstanceState>,
    log: Vec<(u32, AppCommand)>,
}

/// 🧪️ A fully-scripted in-memory artifact store — every generation/revision/prepared-transaction
/// invariant is REAL (not stubbed): `TransactionCommit` genuinely checks the generation captured at
/// `TransactionPrepare` time, a second `TransactionPrepare` on a still-pending instance genuinely
/// returns `transaction.instance-busy`. `Clone` shares the SAME underlying state (`Arc<Mutex<..>>`) —
/// a test keeps one handle for scripting/assertions while handing a clone to `ActionAdapter` (which
/// takes ownership of a `Box<ArtifactChannels>` (was `Box<dyn ArtifactChannel>`, see the
/// `dyn_enum_close!` note above the trait).
#[derive(Clone)]
pub struct MockArtifactChannel {
    state: Arc<Mutex<MockChannelState>>,
}

impl MockArtifactChannel {
    pub fn new() -> Self {
        Self { state: Arc::new(Mutex::new(MockChannelState { instances: BTreeMap::new(), log: Vec::new() })) }
    }

    /// 📜️ Every command sent, in order, tagged with its target instance — the "assert on the recorded
    /// frame log, not just the error" tool the brief's §4 requires.
    pub fn frame_log(&self) -> Vec<(u32, AppCommand)> {
        self.state.lock().expect("mock channel lock poisoned").log.clone()
    }

    fn with_instance(&self, instance: u32, mutate: impl FnOnce(&mut MockInstanceState)) {
        let mut state = self.state.lock().expect("mock channel lock poisoned");
        let instance_state = state.instances.entry(instance).or_insert_with(|| MockInstanceState::new(instance));
        mutate(instance_state);
    }

    /// 🏃️ Simulates a concurrent edit landing on `instance` between this adapter's own prepare and
    /// commit — the next `TransactionCommit` against a transaction prepared before this call now sees
    /// a stale `base_generation` and returns `transaction.generation-mismatch`.
    pub fn bump_generation(&self, instance: u32) {
        self.with_instance(instance, |state| state.generation += 1);
    }

    pub fn force_budget_exceeded(&self, instance: u32) {
        self.with_instance(instance, |state| state.force_budget_exceeded = true);
    }

    pub fn force_commit_fault(&self, instance: u32, fault: Fault) {
        self.with_instance(instance, |state| state.force_commit_fault = Some(fault));
    }

    pub fn force_undo_fails(&self, instance: u32) {
        self.with_instance(instance, |state| state.force_undo_fails = true);
    }
}

impl Default for MockArtifactChannel {
    fn default() -> Self {
        Self::new()
    }
}

impl ArtifactChannel for MockArtifactChannel {
    fn exchange(&mut self, instance: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, Fault> {
        let mut state = self.state.lock().expect("mock channel lock poisoned");
        let mut frames = Vec::with_capacity(commands.len());
        for command in commands {
            state.log.push((instance, command.clone()));
            let instance_state = state.instances.entry(instance).or_insert_with(|| MockInstanceState::new(instance));
            frames.push(instance_state.handle(command));
        }
        Ok(frames)
    }
}
//#endregion 🔖️MockArtifactChannel

//#region 🔖️InternalRecords
/// 🎫️ The `prep_` handle payload — everything `invoke`/`transaction.begin` need to resume a prepared
/// action without re-running prepare/preview.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct PreparedActionRecord {
    capability_id: String,
    input: serde_json::Value,
    instance: u32,
    baseline: RevisionStamp,
    ops: PreparedOps,
    principal_id: String,
}

/// 🎫️ The `txn_` (saga) handle payload — an ordered list of already-prepared members bound together
/// by `transaction.begin`.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct SagaMember {
    prepared_handle: String,
    capability_id: String,
    instance: u32,
    ops: PreparedOps,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SagaRecord {
    members: Vec<SagaMember>,
}

/// 🎫️ The `undo_` handle payload — every low-level channel `txn_id` (paired with its instance) a
/// committed invocation or saga touched; `history.undo`/`history.redo` fan `TransactionUndo`/
/// `TransactionRedo` out to every member, best-effort, exactly like the real
/// `HostTransactionCoordinator::undo_group`/`redo_group` (`📓️luna-channel-audit.md` §6).
#[derive(Clone, Debug, Serialize, Deserialize)]
struct UndoMember {
    instance: u32,
    txn_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UndoRecord {
    members: Vec<UndoMember>,
}
//#endregion 🔖️InternalRecords

//#region 🔖️PublicReports
/// 📨️ `action.invoke`'s combined input — `preparedActionHandle` XOR `(capabilityId, input)`, an
/// optional caller-asserted `expectedRevision` (defaults to the baseline `prepare` captured),
/// `idempotencyKey`, and an `approvalHandle` resuming a previously-required approval gate.
#[derive(Clone, Debug, Default)]
pub struct InvokeRequest {
    pub prepared_handle: Option<String>,
    pub capability_id: Option<String>,
    pub input: Option<serde_json::Value>,
    pub expected_revision: Option<RevisionStamp>,
    pub idempotency_key: Option<String>,
    pub approval_handle: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SagaMemberResult {
    pub prepared_handle: String,
    pub capability_id: String,
    pub edit_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SagaReport {
    pub transaction_handle: String,
    pub members: Vec<SagaMemberResult>,
    pub undo_token: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoRedoReport {
    pub undo_token: String,
    pub members: usize,
    pub warnings: Vec<String>,
}
//#endregion 🔖️PublicReports

//#region 🔖️ActionAdapter
struct AuditContext<'a> {
    invocation_id: &'a str,
    principal: &'a AgentPrincipal,
    session: &'a SessionHandle,
    capability_id: &'a str,
    raw_input: &'a serde_json::Value,
}

/// 🎬️ The mutation protocol's home — `📋️master.md` §3.3's Observe→Prepare→Preview→Approve→
/// Commit→Verify→Undo/Redo lifecycle, over an [`ArtifactChannel`]. One adapter is shared (behind
/// `Arc`) across every tool call in a process, matching P1b's own "one shared `McpServer`" model
/// (`📓️terra-P1b-report.md` §7.2) — nothing here is per-connection-session-aware yet either.
pub struct ActionAdapter {
    channel: Mutex<Box<ArtifactChannels>>,
    handles: Arc<HandleTable>,
    idempotency: Arc<IdempotencyStore>,
    audit: Arc<AuditSinks>,
    policy: PolicyEngine,
    client: ClientInfo,
    invocation_counter: AtomicU64,
}

const INSTANCE_BUSY_MAX_ATTEMPTS: u32 = 3;

impl ActionAdapter {
    pub fn new(channel: Box<ArtifactChannels>, handles: Arc<HandleTable>, idempotency: Arc<IdempotencyStore>, audit: Arc<AuditSinks>, auto_approve: AutoApprovePolicy, client: ClientInfo) -> Self {
        let policy = PolicyEngine::new(handles.clone(), auto_approve);
        Self { channel: Mutex::new(channel), handles, idempotency, audit, policy, client, invocation_counter: AtomicU64::new(0) }
    }

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
    fn transaction_prepare_with_retry(&self, instance: u32, ops: PreparedOps, label: &str, origin: MutationOrigin, now_ms: u64) -> Result<String, GatewayError> {
        let mut last_error = None;
        for _ in 0..INSTANCE_BUSY_MAX_ATTEMPTS {
            let txn_id = mint_id(HandleKind::Transaction, now_ms);
            match self.exchange_one(instance, AppCommand::TransactionPrepare { txn_id: txn_id.clone(), ops: ops.clone(), label: label.to_string(), origin: origin.clone() }) {
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

        if let Ok(validator) = crate::schema::compile_validator(&capability.input_schema) {
            if let Err(validation_error) = crate::schema::validate(&validator, &input) {
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
            let prepared = self.prepare(catalog, principal, session, &capability_id, input, instance, now_ms)?;
            let resolved = self.handles.resolve(&prepared.prepared_handle, session, now_ms)?;
            let stored: PreparedActionRecord = serde_json::from_value(resolved.payload).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
            (Some(prepared.prepared_handle), stored)
        };

        let capability = catalog.get(&record.capability_id).ok_or_else(|| GatewayError::new(GatewayErrorCode::NotFound, format!("capability {} no longer exists in the catalog", record.capability_id)))?;

        let diff_summary = serde_json::json!({ "capabilityId": record.capability_id, "opsCount": record.ops.op_counts() });
        match self.policy.gate_approval(principal, capability, diff_summary, request.approval_handle.as_deref(), session, now_ms) {
            ApprovalGate::Required { approval_handle } => {
                let error = GatewayError::new(GatewayErrorCode::ApprovalRequired, format!("capability {} requires approval before it can be invoked", capability.id)).with_details(serde_json::json!({ "approvalHandle": approval_handle }));
                self.record_audit(
                    AuditContext { invocation_id: &invocation_id, principal, session, capability_id: &record.capability_id, raw_input: &record.input },
                    AuditDecision::Denied { code: error.code },
                    Some(record.baseline.clone()),
                    None,
                    "approval_required",
                    Some(error.clone()),
                    None,
                    now_ms,
                );
                return Err(error);
            }
            ApprovalGate::Proceed => {}
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

        let effects_writes: Vec<String> = capability.effects.writes.iter().map(|selector| selector.0.clone()).collect();
        let origin = MutationOrigin::Agent { principal: principal.id.clone(), invocation_id: invocation_id.clone() };
        let commit_result = match self.transaction_prepare_with_retry(record.instance, record.ops.clone(), &format!("agent invoke {}", record.capability_id), origin, now_ms) {
            Ok(txn_id) => match self.exchange_one(record.instance, AppCommand::TransactionCommit { txn_id: txn_id.clone() }) {
                Ok(AppFrame::TransactionCommitted { edit_id, .. }) => {
                    let after = match self.exchange_one(record.instance, AppCommand::ReadHistory) {
                        Ok(AppFrame::HistorySnapshot(revision)) => revision,
                        _ => current.clone(),
                    };
                    let undo_record = UndoRecord { members: vec![UndoMember { instance: record.instance, txn_id: txn_id.clone() }] };
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
                        postconditions: vec![format!("edit:{edit_id}")],
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
            let origin = MutationOrigin::Agent { principal: principal.id.clone(), invocation_id: format!("{saga_handle}-{index}") };
            match self.transaction_prepare_with_retry(member.instance, member.ops.clone(), &format!("saga {saga_handle} member {index}"), origin, now_ms) {
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
        let undo_record = UndoRecord { members: committed.iter().map(|(index, txn_id, _)| UndoMember { instance: saga.members[*index].instance, txn_id: txn_id.clone() }).collect() };
        let undo_token = self.handles.mint(HandleKind::Undo, session.clone(), Attachment::Other { label: "saga".into() }, serde_json::to_value(&undo_record).unwrap_or_default(), now_ms);

        let mut ordered = committed;
        ordered.sort_by_key(|(index, ..)| *index);
        let members: Vec<SagaMemberResult> =
            ordered.into_iter().map(|(index, _, edit_id)| SagaMemberResult { prepared_handle: saga.members[index].prepared_handle.clone(), capability_id: saga.members[index].capability_id.clone(), edit_id }).collect();
        Ok(SagaReport { transaction_handle: saga_handle.to_string(), members, undo_token })
    }
    //#endregion 🔖️Saga

    //#region 🔖️UndoRedo
    fn fan_out(&self, session: &SessionHandle, undo_token: &str, now_ms: u64, build: impl Fn(String) -> AppCommand) -> Result<UndoRedoReport, GatewayError> {
        let record = self.handles.resolve(undo_token, session, now_ms)?;
        if record.kind != HandleKind::Undo {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "handle is not an undo handle"));
        }
        let undo: UndoRecord = serde_json::from_value(record.payload).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        let mut warnings = Vec::new();
        for member in &undo.members {
            if let Err(error) = self.exchange_one(member.instance, build(member.txn_id.clone())) {
                warnings.push(format!("member {} (instance {}) failed: {}", member.txn_id, member.instance, error.message));
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
        self.fan_out(session, undo_token, now_ms, |txn_id| AppCommand::TransactionUndo { group_id: txn_id })
    }

    pub fn history_redo(&self, session: &SessionHandle, undo_token: &str, now_ms: u64) -> Result<UndoRedoReport, GatewayError> {
        self.fan_out(session, undo_token, now_ms, |txn_id| AppCommand::TransactionRedo { group_id: txn_id })
    }
    //#endregion 🔖️UndoRedo
}
//#endregion 🔖️ActionAdapter

//#region 🧪️Tests
#[cfg(test)]
mod quick {
    use super::*;

    use crate::audit::InMemoryAuditSink;
    use crate::catalog::{compile, CapabilityDefinition, CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, Catalog, ToolExposure};
    use crate::fixtures;
    use semio_framework::manifest::kernel;
    use semio_framework::manifest::{ApprovalMode, CapabilityEffects, CapabilityExecution, CapabilityPolicy, ResourceSelector};
    use semio_framework::{Locale, Terminology};

    fn synthetic_capability(id: &str, scopes: &[&str], approval: ApprovalMode, destructive: bool) -> CapabilityDefinition {
        CapabilityDefinition {
            id: CapabilityRef(id.to_string()),
            version: 1,
            owner: CapabilityOwner::Gateway,
            kind: CapabilityKind::Mutation,
            title: id.to_string(),
            description: String::new(),
            artifact_kind: None,
            use_when: Vec::new(),
            input_schema: serde_json::json!({"type": "object"}),
            output_schema: serde_json::json!({"type": "object"}),
            effects: CapabilityEffects { destructive, writes: vec![ResourceSelector::new("artifact:{self}")], ..Default::default() },
            policy: CapabilityPolicy { scopes: scopes.iter().map(|scope| kernel::CapabilityId(scope.to_string())).collect(), approval },
            execution: CapabilityExecution::default(),
            exposure: ToolExposure::CatalogOnly,
            presentation: CapabilityPresentation { icon_id: None, category: None, keys: None, in_palette: false, args: Vec::new() },
            examples: Vec::new(),
            source: CapabilitySource::Gateway,
        }
    }

    fn single_capability_catalog(capability: CapabilityDefinition) -> Catalog {
        Catalog { hash: "test".into(), entries: vec![capability] }
    }

    fn two_capability_catalog(a: CapabilityDefinition, b: CapabilityDefinition) -> Catalog {
        let mut entries = vec![a, b];
        entries.sort_by(|left, right| left.id.as_str().cmp(right.id.as_str()));
        Catalog { hash: "test".into(), entries }
    }

    fn real_fixture_catalog() -> Catalog {
        compile(&fixtures::note_and_cad_source(), Locale::En, Terminology::Native).expect("fixture catalog compiles")
    }

    // 🔀️ dedyn-fw-os-misc: returns `Arc<AuditSinks>` (was `Arc<InMemoryAuditSink>`) — ONE shared
    // `Arc`, same instance `ActionAdapter` holds, so events it records via the trait method are
    // visible through this same handle. `InMemoryAuditSink::events()` is inherent (not part of the
    // `AuditSink` trait), so a caller wanting it back matches the `AuditSinks::InMemory` variant —
    // see `assert_events` below.
    fn harness(auto_approve: AutoApprovePolicy) -> (ActionAdapter, MockArtifactChannel, Arc<HandleTable>, Arc<AuditSinks>) {
        let channel = MockArtifactChannel::new();
        let handles = Arc::new(HandleTable::new());
        let idempotency = Arc::new(IdempotencyStore::new());
        let audit = Arc::new(AuditSinks::InMemory(InMemoryAuditSink::new()));
        let adapter = ActionAdapter::new(Box::new(ArtifactChannels::Mock(channel.clone())), handles.clone(), idempotency, audit.clone(), auto_approve, ClientInfo { name: "test".into(), version: "0".into() });
        (adapter, channel, handles, audit)
    }

    // 🔀️ dedyn-fw-os-misc: extracts the recorded events from a harness `Arc<AuditSinks>` known (by
    // every caller in this test module) to be the `InMemory` variant.
    fn assert_events(audit: &AuditSinks) -> Vec<AgentAuditEvent> {
        match audit {
            AuditSinks::InMemory(sink) => sink.events(),
            AuditSinks::File(_) => panic!("test harness always constructs AuditSinks::InMemory"),
        }
    }

    fn principal(scopes: &[&str]) -> AgentPrincipal {
        AgentPrincipal::from_scope_names("agent:test", "test agent", &scopes.iter().map(|scope| scope.to_string()).collect::<Vec<_>>(), None)
    }

    //#region 🔖️PreviewVsCommit
    #[test]
    fn preview_ops_and_the_ops_actually_committed_are_the_same_bytes() {
        let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);

        let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
        let preview_ops_count = prepared.preview["opsCount"]["document"].as_u64().unwrap();

        let report = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap();
        assert_eq!(report.status, InvocationStatus::Succeeded);

        // The exact PureCommand op payload (preview) and the exact TransactionPrepare op payload
        // (commit) sent over the wire must be byte-identical — asserted on the recorded frame log, not
        // on internal state. `MockArtifactChannel::handle` derives its `PureCommand` Emit bytes
        // deterministically from `(capability_id, input)`, so reconstructing the expected bytes here
        // (rather than reading them back off a response, which the command-only frame log does not
        // carry) is exact, not approximate.
        let log = channel.frame_log();
        assert!(log.iter().any(|(_, command)| matches!(command, AppCommand::PureCommand { .. })), "a PureCommand was sent during preview");
        let expected_op = serde_json::to_vec(&serde_json::json!({ "capabilityId": "cad.editor.translateSelection", "input": {"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]} })).unwrap();
        let prepare_ops: Vec<Vec<u8>> = log
            .iter()
            .find_map(|(_, command)| match command {
                AppCommand::TransactionPrepare { ops, .. } => Some(ops.document.clone()),
                _ => None,
            })
            .expect("a TransactionPrepare was sent");
        assert_eq!(prepare_ops.len() as u64, preview_ops_count);
        assert_eq!(prepare_ops, vec![expected_op], "the op bytes committed must be byte-identical to what PureCommand emitted during preview");
    }
    //#endregion 🔖️PreviewVsCommit

    //#region 🔖️RevisionConflict
    #[test]
    fn stale_expected_revision_is_a_revision_conflict_with_no_mutation_sent() {
        let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);

        let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();

        // Simulate a concurrent edit landing between prepare and invoke.
        channel.bump_generation(0);
        let before_invoke = channel.frame_log().len();

        let stale = RevisionStamp { artifact_id: "mock-artifact-0".into(), head_edit_id: "edit-0".into(), cursor: "gen-0".into() };
        let error = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), expected_revision: Some(stale), ..Default::default() }, 0, 1).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::RevisionConflict);

        let log = channel.frame_log();
        // Only a ReadHistory (a pure read) may have been sent after the staleness became detectable —
        // no TransactionPrepare/TransactionCommit anywhere in the whole log.
        assert!(!log.iter().any(|(_, command)| matches!(command, AppCommand::TransactionPrepare { .. } | AppCommand::TransactionCommit { .. })), "a mutation command was sent despite a stale expectedRevision: {log:?}");
        assert!(log.len() > before_invoke, "invoke must have issued at least the ReadHistory recheck");
    }
    //#endregion 🔖️RevisionConflict

    //#region 🔖️Idempotency
    #[test]
    fn idempotent_replay_performs_exactly_one_mutation() {
        let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);

        let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
        let request = InvokeRequest { prepared_handle: Some(prepared.prepared_handle), idempotency_key: Some("key-1".into()), ..Default::default() };

        let first = adapter.invoke(&catalog, &principal, &session, request.clone(), 0, 1).unwrap();
        assert!(!first.replayed);
        let commits_after_first = channel.frame_log().iter().filter(|(_, command)| matches!(command, AppCommand::TransactionCommit { .. })).count();
        assert_eq!(commits_after_first, 1);

        // A replay with a DIFFERENT prepared handle in the request would normally re-resolve, but the
        // idempotency key alone must short-circuit before any channel command is sent a second time.
        let second = adapter.invoke(&catalog, &principal, &session, InvokeRequest { idempotency_key: Some("key-1".into()), ..request }, 0, 2).unwrap();
        assert!(second.replayed);
        assert_eq!(second.invocation_id, first.invocation_id);
        let commits_after_second = channel.frame_log().iter().filter(|(_, command)| matches!(command, AppCommand::TransactionCommit { .. })).count();
        assert_eq!(commits_after_second, 1, "a replayed idempotency key must not perform a second mutation");
    }
    //#endregion 🔖️Idempotency

    //#region 🔖️UndoRoundTrip
    #[test]
    fn undo_token_round_trips_through_history_undo_and_redo() {
        let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);

        let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
        let report = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap();
        let undo_token = report.undo_token.expect("a committed invocation mints an undo token");

        let undo_report = adapter.history_undo(&session, &undo_token, 2).unwrap();
        assert_eq!(undo_report.members, 1);
        assert!(undo_report.warnings.is_empty());
        assert!(channel.frame_log().iter().any(|(_, command)| matches!(command, AppCommand::TransactionUndo { .. })));

        let redo_report = adapter.history_redo(&session, &undo_token, 3).unwrap();
        assert_eq!(redo_report.members, 1);
        assert!(channel.frame_log().iter().any(|(_, command)| matches!(command, AppCommand::TransactionRedo { .. })));
    }
    //#endregion 🔖️UndoRoundTrip

    //#region 🔖️ApprovalGateBlocksThenProceeds
    #[test]
    fn approval_gate_blocks_a_destructive_capability_without_approval_and_proceeds_with_it() {
        let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let capability = synthetic_capability("gateway.destructiveThing", &[], ApprovalMode::Always, true);
        let catalog = single_capability_catalog(capability);
        let session = SessionHandle::new("sess_1");
        let principal = principal(&[]);

        let prepared = adapter.prepare(&catalog, &principal, &session, "gateway.destructiveThing", serde_json::json!({}), 0, 0).unwrap();

        let blocked = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle.clone()), ..Default::default() }, 0, 1).unwrap_err();
        assert_eq!(blocked.code, GatewayErrorCode::ApprovalRequired);
        let approval_handle = blocked.details["approvalHandle"].as_str().unwrap().to_string();

        let approved_handle = adapter.resolve_approval(&session, &approval_handle, true, 2).unwrap();
        let succeeded = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), approval_handle: Some(approved_handle), ..Default::default() }, 0, 3).unwrap();
        assert_eq!(succeeded.status, InvocationStatus::Succeeded);
    }
    //#endregion 🔖️ApprovalGateBlocksThenProceeds

    //#region 🔖️ScopeDenialAudited
    #[test]
    fn a_capability_whose_scopes_exceed_the_principals_is_permission_denied_and_audited() {
        let (adapter, _channel, _handles, audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&[]); // no scopes granted at all

        let error = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::PermissionDenied);

        let events = assert_events(&audit);
        assert!(events.iter().any(|event| matches!(&event.decision, AuditDecision::Denied { code } if *code == GatewayErrorCode::PermissionDenied) && event.capability == "cad.editor.translateSelection"));
    }
    //#endregion 🔖️ScopeDenialAudited

    //#region 🔖️Cancel
    #[test]
    fn cancel_drops_a_prepared_handle() {
        let (adapter, _channel, handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);

        let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
        assert_eq!(handles.len(), 1);
        adapter.cancel(&session, &prepared.prepared_handle, 1).unwrap();
        assert_eq!(handles.len(), 0);

        let error = adapter.cancel(&session, &prepared.prepared_handle, 2).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::NotFound);
    }
    //#endregion 🔖️Cancel

    //#region 🔖️InstanceBusy
    #[test]
    fn instance_busy_retries_then_precondition_failed() {
        let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);

        // Occupy instance 0 with an externally-pending transaction that never clears.
        let _ = channel
            .clone()
            .exchange(0, vec![AppCommand::TransactionPrepare { txn_id: "external".into(), ops: PreparedOps::default(), label: "external".into(), origin: MutationOrigin::Agent { principal: "someone-else".into(), invocation_id: "x".into() } }]);

        let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
        let error = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::PreconditionFailed);

        let attempts = channel.frame_log().iter().filter(|(_, command)| matches!(command, AppCommand::TransactionPrepare { .. })).count();
        assert_eq!(attempts as u32, INSTANCE_BUSY_MAX_ATTEMPTS + 1, "one external prepare plus every retry attempt");
    }
    //#endregion 🔖️InstanceBusy

    //#region 🔖️GenerationMismatch
    #[test]
    fn a_concurrent_edit_between_prepare_and_commit_is_a_revision_conflict() {
        let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);

        let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
        // Force the LOW-LEVEL commit to see a stale base_generation without going through the upfront
        // expectedRevision recheck (which used the freshly re-read current revision as `expected`) —
        // bump AFTER our own ReadHistory recheck would run, by scripting a commit-time fault directly.
        channel.force_commit_fault(0, Fault { code: "transaction.generation-mismatch".into(), message: "base generation stale".into() });

        let error = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::RevisionConflict);
    }
    //#endregion 🔖️GenerationMismatch

    //#region 🔖️BudgetExceeded
    #[test]
    fn budget_exceeded_fault_maps_to_budget_exceeded_code() {
        let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);

        let prepared = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": 1.0, "dy": 0.0, "dz": 0.0, "objectIds": ["a"]}), 0, 0).unwrap();
        channel.force_budget_exceeded(0);
        let error = adapter.invoke(&catalog, &principal, &session, InvokeRequest { prepared_handle: Some(prepared.prepared_handle), ..Default::default() }, 0, 1).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::BudgetExceeded);
        assert!(error.retryable);
    }
    //#endregion 🔖️BudgetExceeded

    //#region 🔖️FaultMapping
    #[test]
    fn every_fault_code_maps_to_the_right_gateway_error_code() {
        let cases = [
            ("viewer.read-only", GatewayErrorCode::PermissionDenied),
            ("capability-denied", GatewayErrorCode::PermissionDenied),
            ("mutation.rejected", GatewayErrorCode::SideEffectRejected),
            ("transaction.generation-mismatch", GatewayErrorCode::RevisionConflict),
            ("transaction.instance-busy", GatewayErrorCode::PreconditionFailed),
            ("budget.exceeded", GatewayErrorCode::BudgetExceeded),
            ("some.unrecognised.code", GatewayErrorCode::Internal),
        ];
        for (code, expected) in cases {
            let mapped = map_fault(&Fault { code: code.to_string(), message: "x".into() });
            assert_eq!(mapped.code, expected, "fault code {code} mapped to {:?}, expected {:?}", mapped.code, expected);
        }
    }
    //#endregion 🔖️FaultMapping

    //#region 🔖️SagaCompensation
    #[test]
    fn saga_commits_in_reverse_discovery_order_and_compensates_on_failure() {
        let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let capability_a = synthetic_capability("gateway.memberA", &[], ApprovalMode::Never, false);
        let capability_b = synthetic_capability("gateway.memberB", &[], ApprovalMode::Never, false);
        let catalog = two_capability_catalog(capability_a, capability_b);
        let session = SessionHandle::new("sess_1");
        let principal = principal(&[]);

        let prepared_a = adapter.prepare(&catalog, &principal, &session, "gateway.memberA", serde_json::json!({}), 0, 0).unwrap();
        let prepared_b = adapter.prepare(&catalog, &principal, &session, "gateway.memberB", serde_json::json!({}), 1, 0).unwrap();

        // Member A (instance 0) will fail its commit; its undo (compensation) must be attempted after
        // member B (instance 1, committed first since commit runs in REVERSE discovery order) succeeds.
        channel.force_commit_fault(0, Fault { code: "mutation.rejected".into(), message: "A rejected".into() });

        let saga_handle = adapter.transaction_begin(&session, &[prepared_a.prepared_handle, prepared_b.prepared_handle], 1).unwrap();
        let error = adapter.transaction_commit(&principal, &session, &saga_handle, 2).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::SideEffectRejected);

        let log = channel.frame_log();
        let commit_order: Vec<u32> = log.iter().filter_map(|(instance, command)| matches!(command, AppCommand::TransactionCommit { .. }).then_some(*instance)).collect();
        assert_eq!(commit_order, vec![1, 0], "commit must run in reverse discovery order (B=instance 1 before A=instance 0)");
        assert!(log.iter().any(|(instance, command)| *instance == 1 && matches!(command, AppCommand::TransactionUndo { .. })), "member B (already committed) must be compensated via TransactionUndo");
    }

    #[test]
    fn compensation_failure_itself_is_reported_as_compensation_failed() {
        let (adapter, channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let capability_a = synthetic_capability("gateway.memberA", &[], ApprovalMode::Never, false);
        let capability_b = synthetic_capability("gateway.memberB", &[], ApprovalMode::Never, false);
        let catalog = two_capability_catalog(capability_a, capability_b);
        let session = SessionHandle::new("sess_1");
        let principal = principal(&[]);

        let prepared_a = adapter.prepare(&catalog, &principal, &session, "gateway.memberA", serde_json::json!({}), 0, 0).unwrap();
        let prepared_b = adapter.prepare(&catalog, &principal, &session, "gateway.memberB", serde_json::json!({}), 1, 0).unwrap();

        channel.force_commit_fault(0, Fault { code: "mutation.rejected".into(), message: "A rejected".into() });
        channel.force_undo_fails(1); // compensating the already-committed member B also fails

        let saga_handle = adapter.transaction_begin(&session, &[prepared_a.prepared_handle, prepared_b.prepared_handle], 1).unwrap();
        let error = adapter.transaction_commit(&principal, &session, &saga_handle, 2).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::CompensationFailed);
    }
    //#endregion 🔖️SagaCompensation

    //#region 🔖️Misc
    #[test]
    fn unknown_capability_id_is_not_found() {
        let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);
        let error = adapter.prepare(&catalog, &principal, &session, "no.such.capability", serde_json::json!({}), 0, 0).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::NotFound);
    }

    /// 🐛️ post-unblock fix: this test originally sent `{}` and asserted `INPUT_INVALID`, on the wrong
    /// assumption that translateSelection's `dx`/`dy`/`dz`/`objectIds` args are required. They are
    /// not — `🧫️fixtures/🦀️component.rs`'s `string_array_arg`/`number_arg` helpers never call
    /// `ActionArgDef::required()`, so the compiled `input_schema` has no `"required"` array at all,
    /// and `{}` is genuinely schema-valid (confirmed directly by the repo-owned validator built from
    /// this exact capability's `input_schema` reports `is_valid(&json!({}))  == true`). The test
    /// encoded the wrong expectation, not a defect in `prepare`'s validation — `prepare` itself was
    /// already correctly invoking the validator (confirmed: the same validator correctly rejects a
    /// wrong-typed `dx` and an `additionalProperties:false`-violating unknown field). Fixed by
    /// asserting against genuinely-invalid input instead of an incorrectly-assumed-required field.
    #[test]
    fn invalid_input_against_the_capabilitys_schema_is_input_invalid() {
        let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);
        // `dx` is declared `ArgSchema::Number` — a string value violates the schema's `type: "number"`.
        let error = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"dx": "not a number"}), 0, 0).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::InputInvalid);
    }

    /// 🐛️ post-unblock fix, second half: the schema's `additionalProperties: false` is the OTHER real
    /// enforcement point translateSelection's own args never exercise (none of them are required) —
    /// an unrecognised field must still be rejected.
    #[test]
    fn an_unrecognised_field_against_the_capabilitys_schema_is_input_invalid() {
        let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);
        let error = adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({"notARealArg": 1}), 0, 0).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::InputInvalid);
    }

    /// ✅️ The flip side of the fix above, made explicit rather than left implicit: `{}` genuinely IS
    /// valid input for translateSelection (no arg is required in this fixture), so `prepare` must
    /// succeed for it — pinning down the exact behaviour the two tests above now correctly assume.
    #[test]
    fn empty_input_is_valid_for_a_capability_with_no_required_args() {
        let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let catalog = real_fixture_catalog();
        let session = SessionHandle::new("sess_1");
        let principal = principal(&["artifact.write"]);
        adapter.prepare(&catalog, &principal, &session, "cad.editor.translateSelection", serde_json::json!({}), 0, 0).expect("no arg is required, so {} must be accepted");
    }

    #[test]
    fn transaction_begin_requires_at_least_one_prepared_handle() {
        let (adapter, _channel, _handles, _audit) = harness(AutoApprovePolicy::Never);
        let session = SessionHandle::new("sess_1");
        let error = adapter.transaction_begin(&session, &[], 0).unwrap_err();
        assert_eq!(error.code, GatewayErrorCode::InputInvalid);
    }
    //#endregion 🔖️Misc
}
//#endregion 🧪️Tests
