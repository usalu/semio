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

use crate::audit::{hash_input, redact_input, AgentAuditEvent, AuditDecision, AuditSink, AuditSinks, ClientInfo, SENSITIVE_KEYS};
use crate::catalog::Catalog;
use crate::errors::{GatewayError, GatewayErrorCode};
use crate::handles::{mint_id, Attachment, HandleKind, HandleTable, IdempotencyStore, SessionHandle};
use crate::policy::{AgentPrincipal, ApprovalCoordinator, ApprovalGate, ApprovalRequest, ApprovalResolution, AutoApprovePolicy, PolicyEngine};
use crate::schema::{InvocationReport, InvocationStatus, PreparedActionReport, RevisionStamp};
use crate::workspace::ArtifactChannels;
use semio_framework_dispatch_macros::dyn_enum;
use semio_framework_os_kernel::{DslValue, FromValue, ToValue, ValueError};
use serde::{Deserialize, Serialize};
#[cfg(test)]
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

/// 🌉️ `serde_json::Value` ↔ `DslValue` bridge, built on `🌱️value/🦀️.rs`'s own infallible
/// `From<&DslValue>`/`From<&serde_json::Value>` impls — shared by every field in this file typed
/// `serde_json::Value`.
fn json_value_to_dsl(value: &serde_json::Value) -> DslValue {
    DslValue::from(value)
}

/// 🌉️ See [`json_value_to_dsl`] — the `FromValue` direction, infallible.
fn dsl_to_json_value(value: DslValue) -> Result<serde_json::Value, ValueError> {
    Ok(serde_json::Value::from(value))
}

//#region 🔖️Port
/// 📦️ One op payload set, split by store lane — matches `📓️luna-channel-audit.md` §4's recommended
/// `TransactionPrepare.prepared_ops` shape (`Vec<Vec<u8>>`, one element per complete op payload,
/// never a single stream). `Emit` returns this from `PureCommand`; `TransactionPrepare` sends the
/// SAME bytes back unmodified — this adapter never decodes an individual op's contents.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
    Infer(InferCommand),
    /// 📤️ Export one artifact through the owning app's own media OUT port — `document`/`document_spr`
    /// are the artifact's real pack bytes, loaded into the guest before the port is read. Mirrors the
    /// `LoadDocument` → `MediaOut` pair `🏃️run`'s workflow executor already drives; this port adds no
    /// export-specific wire of its own.
    ExportMedia { port: String, document: Vec<u8>, document_spr: Vec<u8> },
    /// 🆕️ Read the guest's own freshly-opened (genesis) document — how a plugin-typed artifact is
    /// created without this host ever knowing that plugin's schema.
    ReadArtifact,
}

/// 💡️ One plugin-declared inference execution — the `Infer` command's whole payload, kept as its own
/// struct so the variant stays readable and so a caller that already resolved a declared inference
/// row (`crate::inference::DeclaredInference`) can build it field-for-field. `plugin_id` is the
/// plugin whose guest actually owns the route (the row's `contributor`, which equals `owner` for an
/// owner-authored inference): `RoutingArtifactChannel` reads it directly rather than decoding a
/// plugin out of `instance`, because an inference carries no capability id. Every remaining field is
/// exactly what `semio_framework_plugin_host::ArtifactInferenceRouter::infer`'s own request wire
/// carries — this port never invents one of its own.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct InferCommand {
    pub plugin_id: String,
    pub artifact_kind: String,
    pub inference_schema: String,
    pub revision: u64,
    pub generation: u64,
    pub cancellation_id: String,
    pub work_units: u64,
    pub canonical_payload: Vec<u8>,
    /// 🔗️ The artifact this inference is being run ON, and its canonical pair. An inference over an
    /// artifact is not expressible as a hand-typed payload — nobody types 4 096 bitmap cells into a
    /// tool call — so the caller names an artifact and this command carries the document the
    /// guest reads its own snapshot out of. Empty for an inference whose published contract
    /// declares no artifact binding (`semio_framework::InferenceArtifactBinding`), which is the
    /// only case where a bare `canonical_payload` is the whole request.
    pub artifact_id: String,
    pub artifact_document: Option<ArtifactDocumentBinding>,
    pub cancel: InferenceCancel,
}

/// 🛑️ The caller's live cancellation for one inference run. `inference_run` binds it to its job, so
/// `job_cancel` and `notifications/cancelled` stop the in-flight guest job through the plugin host's
/// cold relay instead of waiting for the solve to finish. Two handles compare equal when they are in
/// the same state — the token itself has no identity a frame log could print.
#[derive(Clone)]
pub struct InferenceCancel(pub semio_framework_async::CancelToken);

impl Default for InferenceCancel {
    fn default() -> Self {
        Self(semio_framework_async::CancelToken::root_now())
    }
}

impl std::fmt::Debug for InferenceCancel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "InferenceCancel(cancelled: {})", self.0.is_cancelled_now())
    }
}

impl PartialEq for InferenceCancel {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_cancelled_now() == other.0.is_cancelled_now()
    }
}

/// 🧭️ One phase of activating a plugin session and reading its document — `SessionActivationPhaseV1`
/// in `🏠️workspace/🧬️schema`. The session's component is resolved, read, content-hashed, loaded or
/// compiled, its guest opened, then its document is read. Every job that activates a session
/// (`inference_run`'s artifact binding, `artifact_create`) reports these, and a session that already
/// passed a phase skips it rather than reporting it again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActivationPhase {
    ResolvingComponent,
    ReadingComponent,
    HashingComponent,
    LoadingCompiledCode,
    CompilingComponent,
    OpeningGuest,
    ReadingDocument,
}

impl ActivationPhase {
    pub const ALL: [ActivationPhase; 7] = [Self::ResolvingComponent, Self::ReadingComponent, Self::HashingComponent, Self::LoadingCompiledCode, Self::CompilingComponent, Self::OpeningGuest, Self::ReadingDocument];

    /// 🪪️ The schema's kebab-case id.
    pub fn id(self) -> &'static str {
        match self {
            Self::ResolvingComponent => "resolving-component",
            Self::ReadingComponent => "reading-component",
            Self::HashingComponent => "hashing-component",
            Self::LoadingCompiledCode => "loading-compiled-code",
            Self::CompilingComponent => "compiling-component",
            Self::OpeningGuest => "opening-guest",
            Self::ReadingDocument => "reading-document",
        }
    }

    /// 📈️ Where the phase starts within an activation, `0..1`; each job maps it onto its own span.
    pub fn position(self) -> f64 {
        Self::ALL.iter().position(|phase| *phase == self).map_or(0.0, |index| index as f64 / Self::ALL.len() as f64)
    }

    /// 📏️ Where `fraction` of this phase lands within an activation, `0..1`.
    pub fn at(self, fraction: f64) -> f64 {
        self.position() + fraction.clamp(0.0, 1.0) / Self::ALL.len() as f64
    }
}

/// 🏷️ The fault code an activation stopped by its caller's cancel carries; [`map_fault`] answers it
/// as `CANCELLED`.
pub const ACTIVATION_CANCELLED_FAULT_CODE: &str = "job.cancelled";

/// 🎚️ Progress and cancellation for one session activation. Every interruptible phase boundary is a
/// [`ActivationScope::checkpoint`]; a phase with measurable size (hashing) reports its fraction.
pub struct ActivationScope {
    cancel: semio_framework_async::CancelToken,
    report: Box<dyn Fn(ActivationPhase, f64) + Send + Sync>,
}

impl ActivationScope {
    pub fn new(cancel: semio_framework_async::CancelToken, report: impl Fn(ActivationPhase, f64) + Send + Sync + 'static) -> Self {
        Self { cancel, report: Box::new(report) }
    }

    /// 🫥️ An activation nobody watches or cancels — what an ordinary command exchange opens with.
    pub fn detached() -> Self {
        Self::new(semio_framework_async::CancelToken::root_now(), |_, _| {})
    }

    pub fn cancel(&self) -> &semio_framework_async::CancelToken {
        &self.cancel
    }

    /// ▶️ Reports entry into `phase`, then checks the caller's cancel.
    pub fn enter(&self, phase: ActivationPhase) -> Result<(), Fault> {
        (self.report)(phase, 0.0);
        self.checkpoint()
    }

    /// 📏️ Reports `fraction` (0..=1) of `phase` done, then checks the caller's cancel.
    pub fn advance(&self, phase: ActivationPhase, fraction: f64) -> Result<(), Fault> {
        (self.report)(phase, fraction.clamp(0.0, 1.0));
        self.checkpoint()
    }

    pub fn checkpoint(&self) -> Result<(), Fault> {
        if self.cancel.is_cancelled_now() {
            return Err(Fault { code: ACTIVATION_CANCELLED_FAULT_CODE.to_string(), message: "the caller cancelled while its plugin session was being activated".to_string() });
        }
        Ok(())
    }
}

/// 📦️ One artifact's canonical `pack`/`spr` pair on its way to a guest inference — host-opaque
/// bytes, carried verbatim and encoded only at the boundary the plugin's own contract names.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ArtifactDocumentBinding {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
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
    /// 💡️ The guest's own inference result, exactly as its `artifact-infer` job returned it:
    /// `payload` is the result wire's `canonicalPayload` bytes and `complete` its `complete` flag.
    /// Never a host-synthesised value — a guest that refuses answers [`AppFrame::Error`] instead.
    Inferred { inference_schema: String, complete: bool, payload: Vec<u8> },
    /// 📤️ The guest's own exported media for one OUT port, exactly as `AppFrame::Media` carried it:
    /// `descriptor` is the guest's packed media descriptor, `data` the exported bytes.
    Exported { port: String, descriptor: Vec<u8>, data: Vec<u8> },
    /// 🆕️ The guest's own document pack+spr bytes, host-opaque.
    Artifact { pack: Vec<u8>, spr: Vec<u8> },
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

/// 💡️ What [`ActionAdapter::run_inference`] hands back — the guest's own result payload and its
/// `complete` flag, plus the `inferenceSchema` the guest echoed (asserted equal to the requested one
/// by the real router before it ever reaches here).
#[derive(Clone, Debug, PartialEq)]
pub struct InferenceOutcome {
    pub inference_schema: String,
    pub complete: bool,
    pub payload: Vec<u8>,
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

    /// 🔌️ Brings `instance`'s session up to where its next exchange runs without opening anything:
    /// the component compiled, the guest open. Reports each phase to `scope` and stops at the next
    /// interruptible boundary once `scope` is cancelled, leaving no half-open guest behind.
    fn activate(&mut self, instance: u32, scope: &ActivationScope) -> Result<(), Fault>;
}

/// ↩️ Private durable-history port; only a Hub-bound workspace implements the remote member.
pub trait HistoryUndoPort: Send + Sync {
    fn undo_hub_gis_map_approval(&self, member: &HubGisMapApprovalUndoMemberV1) -> Result<(), GatewayError>;
}

/// 🧯️ `Fault.code` → `GatewayErrorCode` — `📋️master.md` §3.3's Fault code table, plus this crate's
/// own `"budget.exceeded"` addition (`GatewayErrorCode::BudgetExceeded`, retryable) and W8's
/// `"capability.not-found"`/`"plugin.unavailable"` (`🏠️workspace/🦀️.rs`'s `RoutingArtifactChannel`
/// — a caller-supplied bad capability id vs. a plugin that genuinely cannot be reached right now).
/// An unrecognised code is `Internal` (never silently swallowed).
fn map_fault(fault: &Fault) -> GatewayError {
    match fault.code.as_str() {
        "viewer.read-only" | "capability-denied" => GatewayError::new(GatewayErrorCode::PermissionDenied, fault.message.clone()),
        "mutation.rejected" => GatewayError::new(GatewayErrorCode::SideEffectRejected, fault.message.clone()),
        "transaction.generation-mismatch" => GatewayError::new(GatewayErrorCode::RevisionConflict, fault.message.clone()),
        "transaction.instance-busy" => GatewayError::new(GatewayErrorCode::PreconditionFailed, fault.message.clone()).retryable(),
        "budget.exceeded" => GatewayError::new(GatewayErrorCode::BudgetExceeded, fault.message.clone()).retryable(),
        "capability.not-found" => GatewayError::new(GatewayErrorCode::NotFound, fault.message.clone()),
        "plugin.unavailable" | "workspace.unbound" => GatewayError::new(GatewayErrorCode::PluginUnavailable, fault.message.clone()).retryable(),
        ACTIVATION_CANCELLED_FAULT_CODE => GatewayError::new(GatewayErrorCode::Cancelled, fault.message.clone()),
        _ => GatewayError::new(GatewayErrorCode::Internal, fault.message.clone()),
    }
}
//#endregion 🔖️Port

//#region 🔖️UnboundArtifactChannel
/// 🚫️ The channel a gateway with NO `--folder`/`--hub` binding runs on: every command answers the
/// same typed, retryable `workspace.unbound` fault naming exactly which flag closes the gap, so a
/// mutation-protocol call against an unbound session can never be mistaken for a real edit. It
/// replaces the scripted [`MockArtifactChannel`] that used to occupy this slot in the live binary —
/// a caller could not tell a mock commit from a real one from the result alone
/// (`📓️g7-mcp-agent-and-collaboration-audit.md` §6 P0.3).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UnboundArtifactChannel;

/// 🏷️ The fault code every unbound exchange carries — mapped by [`map_fault`] onto a retryable
/// `PLUGIN_UNAVAILABLE`, the same shape `🗿️artifact`'s own tier-1 gate already answers with.
pub const WORKSPACE_UNBOUND_FAULT_CODE: &str = "workspace.unbound";

/// 📝️ The one message every unbound surface repeats verbatim — one sentence, naming both bindings.
pub const WORKSPACE_UNBOUND_MESSAGE: &str =
    "no workspace is bound to this session — start the gateway with --folder <dir> or --hub <url> --space <id> before invoking a capability";

impl UnboundArtifactChannel {
    pub fn fault() -> Fault {
        Fault { code: WORKSPACE_UNBOUND_FAULT_CODE.to_string(), message: WORKSPACE_UNBOUND_MESSAGE.to_string() }
    }
}

impl ArtifactChannel for UnboundArtifactChannel {
    fn exchange(&mut self, _instance: u32, _commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, Fault> {
        Err(Self::fault())
    }

    fn activate(&mut self, _instance: u32, _scope: &ActivationScope) -> Result<(), Fault> {
        Err(Self::fault())
    }
}
//#endregion 🔖️UnboundArtifactChannel

//#region 🔖️MockArtifactChannel
#[cfg(test)]
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

#[cfg(test)]
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
            // 💡️ Scripted exactly like every other arm of this in-memory store: an inference on an
            // instance whose generation has moved past the request's is a real staleness rejection
            // (the same `validate_commit` rule the real router enforces); otherwise the request's own
            // canonical payload comes back as the result payload, so a test can assert the request
            // really travelled the port without this double ever claiming to have computed anything.
            AppCommand::Infer(command) => {
                if command.generation != self.generation {
                    return AppFrame::Error(Fault { code: "transaction.generation-mismatch".into(), message: format!("inference base generation {} no longer matches current generation {}", command.generation, self.generation) });
                }
                AppFrame::Inferred { inference_schema: command.inference_schema, complete: true, payload: command.canonical_payload }
            }
            // 📤️🆕️ This double owns no guest, so it can neither run a media port nor mint a plugin's
            // genesis document. It says so with the SAME typed fault a real unreachable plugin uses,
            // never a synthesised export or an empty pack that a caller could mistake for real bytes.
            AppCommand::ExportMedia { port, .. } => AppFrame::Error(Fault { code: "plugin.unavailable".into(), message: format!("the scripted in-memory channel has no guest to read media port `{port}` from — bind a workspace with --folder/--hub") }),
            AppCommand::ReadArtifact => AppFrame::Error(Fault { code: "plugin.unavailable".into(), message: "the scripted in-memory channel has no guest to read a genesis document from — bind a workspace with --folder/--hub".into() }),
        }
    }
}

#[cfg(test)]
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
#[cfg(test)]
#[derive(Clone)]
pub struct MockArtifactChannel {
    state: Arc<Mutex<MockChannelState>>,
}

#[cfg(test)]
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

#[cfg(test)]
impl Default for MockArtifactChannel {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
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

    fn activate(&mut self, _instance: u32, scope: &ActivationScope) -> Result<(), Fault> {
        scope.checkpoint()
    }
}
//#endregion 🔖️MockArtifactChannel

//#region 🔖️InternalRecords
/// 🎫️ The `prep_` handle payload — everything `invoke`/`transaction.begin` need to resume a prepared
/// action without re-running prepare/preview.
#[derive(Clone, Debug, Serialize, Deserialize, ToValue, FromValue)]
struct PreparedActionRecord {
    capability_id: String,
    #[value(serialize_with = "json_value_to_dsl", deserialize_with = "dsl_to_json_value")]
    input: serde_json::Value,
    instance: u32,
    baseline: RevisionStamp,
    ops: PreparedOps,
    principal_id: String,
}

/// 🎫️ The `txn_` (saga) handle payload — an ordered list of already-prepared members bound together
/// by `transaction.begin`.
#[derive(Clone, Debug, Serialize, Deserialize, ToValue, FromValue)]
struct SagaMember {
    prepared_handle: String,
    capability_id: String,
    instance: u32,
    ops: PreparedOps,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToValue, FromValue)]
struct SagaRecord {
    members: Vec<SagaMember>,
}

/// 🎫️ The `undo_` handle payload — every low-level channel `txn_id` (paired with its instance) a
/// committed invocation or saga touched; `history.undo`/`history.redo` fan `TransactionUndo`/
/// `TransactionRedo` out to every member, best-effort, exactly like the real
/// `HostTransactionCoordinator::undo_group`/`redo_group` (`📓️luna-channel-audit.md` §6).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", content = "member", rename_all = "kebab-case", deny_unknown_fields)]
enum UndoMember {
    LocalGuestTransaction { instance: u32, transaction_id: String },
    HubGisMapApproval(HubGisMapApprovalUndoMemberV1),
}

/// 🌐 Exact server-owned remote history member; it carries a locator/frontier, never inverse bytes.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct HubGisMapApprovalUndoMemberV1 {
    pub hub_origin: String,
    pub space_id: String,
    pub document_id: String,
    pub target_id: String,
    pub idempotency_key: String,
    pub expected_current: semio_framework_os_kernel::os_directory::CheckpointPublicationFrontierV1,
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

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SagaMemberResult {
    pub prepared_handle: String,
    pub capability_id: String,
    pub edit_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct SagaReport {
    pub transaction_handle: String,
    pub members: Vec<SagaMemberResult>,
    pub undo_token: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct UndoRedoReport {
    pub undo_token: String,
    pub members: usize,
    pub warnings: Vec<String>,
}
//#endregion 🔖️PublicReports

//#region 🔖️ActionAdapter
/// 🗳️ What one parked approval settled to inside `invoke_uncached` — the adapter's own narrow view
/// of [`ApprovalResolution`], flattened so the caller's error/audit branch stays one match.
enum SettledApproval {
    Approved,
    Denied { channel: &'static str, note: Option<String> },
    Unreachable { details: serde_json::Value },
}

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
    history_undo_port: Mutex<Option<Arc<dyn HistoryUndoPort>>>,
    approvals: Mutex<Option<Arc<ApprovalCoordinator>>>,
}

const INSTANCE_BUSY_MAX_ATTEMPTS: u32 = 3;

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

    fn approval_coordinator(&self) -> Option<Arc<ApprovalCoordinator>> {
        self.approvals.lock().expect("approval coordinator lock poisoned").clone()
    }

    /// 🪪 Mints one session-private undo token from a Hub receipt without retaining mutation bytes.
    pub fn retain_hub_gis_map_approval_undo(
        &self,
        session: &SessionHandle,
        hub_origin: &str,
        scope: &semio_framework_os_kernel::os_directory::DocumentScope,
        handle: &semio_framework_os_kernel::os_directory::GisMapApprovalUndoHandleV1,
        now_ms: u64,
    ) -> Result<String, GatewayError> {
        if hub_origin.is_empty()
            || hub_origin.len() > 2048
            || handle.target_id.len() != 32
            || !handle.target_id.bytes().all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
            || !handle.expected_current.validate()
            || handle.expected_current.document_id != scope.document_id
        {
            return Err(GatewayError::new(GatewayErrorCode::InputInvalid, "invalid durable GIS approval undo authority"));
        }
        let identity = format!("semio.mcp.hub-gis-map-approval-undo/v1\0{}\0{}\0{}\0{}\0{}", session.0, hub_origin, scope.space_id, scope.document_id, handle.target_id);
        let idempotency_key = framework_hash::hash_bytes(identity.as_bytes())[..32].to_owned();
        let member = UndoMember::HubGisMapApproval(HubGisMapApprovalUndoMemberV1 {
            hub_origin: hub_origin.to_owned(),
            space_id: scope.space_id.clone(),
            document_id: scope.document_id.clone(),
            target_id: handle.target_id.clone(),
            idempotency_key,
            expected_current: handle.expected_current.clone(),
        });
        let payload = serde_json::to_value(UndoRecord { members: vec![member] }).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
        Ok(self.handles.mint(HandleKind::Undo, session.clone(), Attachment::Other { label: "hub-gis-map-approval".into() }, payload, now_ms))
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

        let effects_writes: Vec<String> = capability.effects.writes.iter().map(|selector| selector.0.clone()).collect();
        let origin = MutationOrigin::Agent { principal: principal.id.clone(), invocation_id: invocation_id.clone() };
        let commit_result = match self.transaction_prepare_with_retry(record.instance, record.ops.clone(), &format!("agent invoke {}", record.capability_id), origin, now_ms) {
            Ok(txn_id) => match self.exchange_one(record.instance, AppCommand::TransactionCommit { txn_id: txn_id.clone() }) {
                Ok(AppFrame::TransactionCommitted { edit_id, .. }) => {
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

    /// ⛩️ Offers the freshly-parked `approval_handle` to the bound [`ApprovalCoordinator`] and, on a
    /// human yes, consumes the gate for THIS invocation: the pending record is decided through the
    /// same `PolicyEngine::resolve_approval` a replayed `approvalHandle` goes through, and the fresh
    /// decided handle is immediately re-gated so the proceed path is the identical code a replay
    /// takes. No coordinator bound, or nothing attached to ask, is `Unreachable` — never a proceed.
    fn settle_approval(&self, principal: &AgentPrincipal, capability: &crate::catalog::CapabilityDefinition, session: &SessionHandle, approval_handle: &str, diff_summary: &serde_json::Value, now_ms: u64) -> SettledApproval {
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
                UndoMember::HubGisMapApproval(remote) if redo => Err((
                    format!("Hub GIS approval target {}", remote.target_id),
                    GatewayError::new(GatewayErrorCode::SideEffectRejected, "durable Hub GIS approval redo requires a new authenticated approval"),
                )),
                UndoMember::HubGisMapApproval(remote) => {
                    let port = self.history_undo_port.lock().expect("history undo port lock poisoned").clone();
                    port.ok_or_else(|| GatewayError::new(GatewayErrorCode::PluginUnavailable, "Hub durable history port is unavailable").retryable())
                        .and_then(|port| port.undo_hub_gis_map_approval(remote))
                        .map_err(|error| (format!("Hub GIS approval target {}", remote.target_id), error))
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
#[cfg(test)]
#[path = "🧪️tests/🔬️quick/🦀️.rs"]
mod quick;
//#endregion 🧪️Tests
