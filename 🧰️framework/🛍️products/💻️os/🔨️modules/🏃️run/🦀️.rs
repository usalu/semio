//! 🕸️ Headless computation of an OS studio's workflow — no UI involved. A `SpaceRunner` walks
//! `workflow::Workflow` (the kernel-crate persisted graph — a node IS the app-instance, identified
//! solely by `WorkflowNode::id`) in topological order, drives each node's app through
//! `AppChannelHost` — the exact `protocol::AppCommand`/`AppFrame` binary channel a live UI speaks, so
//! a headless run never needs a UI-mock API — moves `Media` along edges, and skips any node whose
//! inputs, document, and config are all unchanged since the last (sealed) run. Every node's frame
//! script is `Hello → LoadConfig → LoadDocument → MediaIn* → (MediaOut+MediaFingerprint)* →
//! ReadDocument → ReadConfig` (see `SpaceRunner::compute_node`); documents/configs are addressed by
//! their node's own `artifact_ref`/`config_ref` string, never by a separate instance id.
//!
//! 🔒️ W5 Lane A ("non-destructive `SpaceRunner` rework"): a run is READONLY over its source. The
//! `documents`/`configs` maps `SpaceRunner::run` reads are NEVER written back — `ReadDocument`/
//! `ReadConfig`'s mutated bytes (whatever `MediaIn` importing changed) land in a `RunSink` instead
//! (`🔖️RunContext` below), keyed by node id, a deliberately different key space than the source
//! `artifact_ref`/`config_ref` maps so a caller cannot accidentally alias a run-owned path onto a
//! source artifact path the way the old destructive path did (pre-rework: `bin.rs` wrote a node's
//! mutated document/config bytes straight back over `artifacts/<artifact_ref>`/`artifacts/<config_ref>`
//! — the very same paths `SpaceRunner::run` had just read them from). "Pinned" source access for this
//! wave means simply "whatever the source currently is, read-only, at the moment `run` reads it" — a
//! real checkpoint-id-pinned snapshot is later-wave `space::DraftCatalog`/collection-artifact
//! integration work (see `SpaceBundle`'s own doc on `workflow_node_for_app`'s still-path-stem
//! `artifact_ref`/`config_ref`).
//!
//! 🗄️ Memoization is now keyed off the PRIOR SEALED run's own `workflow::RunArtifact.node_records`
//! (`prior_node_records`, read-only), not a side-channel `run/state.json` file — that file and its
//! `RunState`/`NodeRunRecord` types are deleted (there were two parallel memoization mechanisms before
//! this rework; now there is exactly one). A first run (no prior sealed `RunArtifact`) computes every
//! node fresh.

//#region 🔖️Types
use dsl::{from_dsl_value, to_dsl_value};
/// 🎞️ The exact binary channel a live UI speaks — re-exported so an `AppChannelHost` implementor
/// never needs a direct `protocol` dependency just to name these types.
pub use protocol::{AppCommand, AppFrame, CHANNEL_VERSION};
use semio_framework::{media_types_compatible, Media, MediaClass, MediaCompat, MediaError, MediaFingerprint, MediaForm, MediaPayload, MediaType, MediaWireFormat, PackageDescriptor, PluginManifest, PortMultiplicity};
// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the reactor-ABI event/
// effect vocabulary `WasmtimeNodeHost::open`/`run_turn`/`exchange` now speak — the same family of
// names `🎯️targets/🧊️wgpu/🦀️.rs`'s `kernel_runtime` module imports from this same path, for the
// same reason (driving a real turn through `semio_framework_actor::Kernel`); this crate uses
// `Effect::Respond`/`RequestOutcome` (per-command reply correlation) rather than that consumer's
// `Effect::SendMessage`/`MessageEndpoint` (UI-surface push messaging, which `run` has no use for).
use semio_framework::kernel::{AppInstanceId, Effect, Event, PluginInstanceId, QuotaSchema, RequestOutcome};
use semio_framework_actor::ActorId as RuntimeActorId;
use semio_framework_actor::{ActivationEvent, ActorKind, Backpressure, Envelope, Lane, Origin, PackageId, Payload};
use semio_framework_async::{CancelToken, OperationContext, TraceId};
// 🎛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (dedyn-fw-os-guestruntime): trait import needed for
// `.compile`/`.instantiate` method-call syntax now that `runtime` below is the concrete
// `GuestRuntimes` enum, not `dyn GuestRuntime` (a `dyn Trait` receiver resolves its methods without
// the trait in scope; a concrete enum implementing it does not).
use semio_framework_plugin_host::GuestRuntime;
// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the shared
// kernel-activation facade — see `semio-framework-os`'s own `🎠️activation.rs` module doc for why it
// lives in the product's host crate rather than in the wgpu target's `ParallelRuntime`.
use semio_framework_os::activation::{actor_budget_from_turn_budget, NativeKernelRuntime};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, Mutex};
use store::{BlobStore, NoBlobStore};
use workflow::{FinishRunNode, MediaContract, PortFingerprint, RunMutation, RunNodeRecord, RunNodeStatus, RunOutputArtifact, RunParameterValue, SealRun, StartRun, StartRunNode, Workflow, WorkflowEdge, WorkflowNode, WorkflowParameterBinding};

/// 🚧️ A failure computing a studio's workflow headlessly.
#[derive(Debug)]
pub enum RunError {
    UnknownNode(String),
    UnknownPort {
        edge_id: String,
        node_id: String,
        port_id: String,
    },
    Incompatible {
        edge_id: String,
        produced: MediaType,
        accepted: MediaType,
    },
    UnregisteredConversion {
        edge_id: String,
        from: MediaForm,
        to: MediaForm,
    },
    MissingRequiredInput {
        node_id: String,
        port_id: String,
    },
    MultiplicityViolation {
        node_id: String,
        port_id: String,
        count: usize,
    },
    NoConverter {
        class: MediaClass,
        from: MediaForm,
        to: MediaForm,
    },
    Cycle(Vec<String>),
    Host(String),
    Media(MediaError),
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Serde(serde_json::Error),
    MutationApply(protocol::MutationApplyError),
    /// 🛑️ `SpaceRunner`'s `OperationContext.cancel` was cancelled — checked at the top of
    /// `compute_node`, before that node's `open`/`exchange`, so a cancelled run stops before its
    /// NEXT node rather than mid-exchange (an in-flight `exchange` future is not itself
    /// preemptible — same honest limitation `semio-framework-os-services::ComputePool` documents).
    Cancelled,
}

impl std::fmt::Display for RunError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownNode(node) => write!(formatter, "unknown workflow node {node}"),
            Self::UnknownPort { edge_id, node_id, port_id } => write!(formatter, "workflow edge {edge_id} references unknown port `{port_id}` on node `{node_id}`"),
            Self::Incompatible { edge_id, produced, accepted } => write!(formatter, "workflow edge {edge_id} type mismatch: producer offers {produced:?}, consumer accepts {accepted:?}"),
            Self::UnregisteredConversion { edge_id, from, to } => write!(formatter, "workflow edge {edge_id} negotiated a conversion {from:?} -> {to:?} but no converter is registered for it"),
            Self::MissingRequiredInput { node_id, port_id } => write!(formatter, "input port `{port_id}` on node `{node_id}` is required but has no incoming edge"),
            Self::MultiplicityViolation { node_id, port_id, count } => write!(formatter, "input port `{port_id}` on node `{node_id}` accepts at most one connection but has {count}"),
            Self::NoConverter { class, from, to } => write!(formatter, "no media converter registered for {class:?}: {from:?} -> {to:?}"),
            Self::Cycle(nodes) => write!(formatter, "workflow has a cycle (unreachable nodes: {nodes:?})"),
            Self::Host(message) => write!(formatter, "host error: {message}"),
            Self::Media(error) => write!(formatter, "media error: {error}"),
            Self::Io { path, source } => write!(formatter, "io error at {}: {source}", path.display()),
            Self::Serde(error) => write!(formatter, "(de)serialization error: {error}"),
            Self::MutationApply(error) => write!(formatter, "run document rejected an operation: {error}"),
            Self::Cancelled => formatter.write_str("run cancelled"),
        }
    }
}

impl std::error::Error for RunError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Media(error) => Some(error),
            Self::Io { source, .. } => Some(source),
            Self::Serde(error) => Some(error),
            Self::MutationApply(error) => Some(error),
            _ => None,
        }
    }
}

impl From<MediaError> for RunError {
    fn from(error: MediaError) -> Self {
        Self::Media(error)
    }
}

impl From<protocol::MutationApplyError> for RunError {
    fn from(error: protocol::MutationApplyError) -> Self {
        Self::MutationApply(error)
    }
}

impl From<serde_json::Error> for RunError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(error)
    }
}

//#endregion 🔖️Types

//#region 🔖️AppChannelHost
/// 🔌️ The one seam `SpaceRunner` calls through — every concrete plugin host (native wasmtime,
/// browser worker, or an in-process fake for tests) implements this the same way, driving a node
/// through exactly the binary `AppCommand`/`AppFrame` channel a live UI speaks (see
/// `protocol_channel`) — a headless run is never a separate UI-mock API. `open` mints an opaque
/// handle the runner threads back on every later `exchange` call; `exchange` is a single batched
/// duplex round trip (`WasmPluginRuntime::exchange`'s native counterpart) that the caller now
/// `.await`s instead of blocking on (async-first rewrite, ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME, packet terra-directory-and-run) — the guest
/// contract is still exactly one turn at a time per instance, so `SpaceRunner` never issues a second
/// `exchange` for the same `node` handle before the first one's future resolves (see `run`'s
/// strictly sequential per-node loop and `compute_node`'s single `.await` point); a caller that
/// wants concurrency must open distinct instances, never overlap two turns on one. `ctx` carries
/// `SpaceRunner`'s own `OperationContext` (`actor` set to `node`) so a real implementation can
/// honor cancellation/deadline/trace INSIDE one exchange too, not just between them — `compute_node`
/// itself only checks `ctx.cancel` between exchanges (see `RunError::Cancelled`'s own doc).
// 🔇️ `async fn` in a trait normally warns because it hides a `Send` bound a `dyn` caller might
// need — irrelevant here: `AppChannelHost` is used only as a generic bound (`SpaceRunner<H:
// AppChannelHost>`), never as `dyn AppChannelHost` (verified: `grep -rn "dyn AppChannelHost"`
// across the repo has zero hits), so no caller can ever demand a `Send` future regardless of what
// `open`/`exchange` do inside — R3's rule (Send obtained structurally, never a bound) applies to
// generic call sites too, not only `dyn` ones. MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet
// `run-kernel-wiring`): `WasmtimeNodeHost`'s own `open`/`exchange` now genuinely cross real OS
// threads inside one `.await` (`run_turn`'s `Kernel`-dispatched `ShardExecutor` turn) — the "single
// thread, one `block_on`" claim this comment used to make described the OLD stub bodies, not the
// real ones; the reasoning above (no `dyn`, so no forced `Send`) is what actually keeps this legal,
// and it always was.
#[allow(async_fn_in_trait)]
pub trait AppChannelHost {
    /// 🗺️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (W2-A): `artifact_ref`
    /// (the node's own `WorkflowNode.artifact_ref`) is threaded through so a real host can populate
    /// its `InstanceDirectory` at instantiate-app time — a fake/test host is free to ignore it.
    async fn open(&mut self, plugin_id: &str, app_id: &str, artifact_ref: &str) -> Result<u32, RunError>;
    async fn exchange(&mut self, ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError>;
}
//#endregion 🔖️AppChannelHost

//#region 🔖️MediaCache
/// 📦️ Content-addressed cache of exported `Media` values, keyed by `MediaFingerprint`. Lets a
/// downstream dirty node import a clean upstream node's last output without re-instantiating that
/// upstream node at all — the whole point of fingerprint-based incrementality.
pub trait MediaCache {
    fn get(&self, fingerprint: &MediaFingerprint) -> Option<Media>;
    fn put(&mut self, fingerprint: &MediaFingerprint, media: &Media);
}

//#endregion 🔖️MediaCache

//#region 🔖️BlobStore
/// 💾️ Disk-backed `store::BlobStore` under `<studio>/blobs/<hash>` — backs both a `WasmPluginRuntime`'s
/// guest-side `write-blob`/`read-blob` host calls (via `WasmtimeNodeHost` registering it on every
/// runtime it loads) and `media_to_artifact`/`media_from_artifact`'s own resolution of a
/// `MediaPayload::Binary` value's bytes on the way on/off the wire.
pub struct FileBlobStore {
    root: PathBuf,
}

impl FileBlobStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn blob_path(&self, hash: &str) -> PathBuf {
        self.root.join(hash)
    }
}

impl BlobStore for FileBlobStore {
    fn put(&self, bytes: &[u8], media_type: &str) -> Result<store::BlobRef, store::VcsError> {
        let hash = framework_hash::hash_bytes(bytes);
        std::fs::create_dir_all(&self.root).map_err(|error| store::VcsError::Backbone(error.to_string()))?;
        std::fs::write(self.blob_path(&hash), bytes).map_err(|error| store::VcsError::Backbone(error.to_string()))?;
        Ok(store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
    }

    fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, store::VcsError> {
        match std::fs::read(self.blob_path(hash)) {
            Ok(bytes) => Ok(Some(bytes)),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(store::VcsError::Backbone(error.to_string())),
        }
    }

    fn has(&self, hash: &str) -> Result<bool, store::VcsError> {
        Ok(self.blob_path(hash).exists())
    }

    fn delete(&self, hash: &str) -> Result<(), store::VcsError> {
        match std::fs::remove_file(self.blob_path(hash)) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(store::VcsError::Backbone(error.to_string())),
        }
    }
}

/// 🧠️ Process-local `store::BlobStore` — the test-only map cache counterpart for blob bytes, used
/// wherever a full `SpaceBundle` (and its `blobs/` dir) isn't in play, chiefly `SpaceRunner`'s own
/// `FakeHost`-based unit tests.
#[derive(Default)]
pub struct InMemoryBlobStore {
    entries: Mutex<HashMap<String, (Vec<u8>, String)>>,
}

impl BlobStore for InMemoryBlobStore {
    fn put(&self, bytes: &[u8], media_type: &str) -> Result<store::BlobRef, store::VcsError> {
        let hash = framework_hash::hash_bytes(bytes);
        let mut entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        entries.insert(hash.clone(), (bytes.to_vec(), media_type.to_string()));
        Ok(store::BlobRef { hash, size: bytes.len() as u64, media_type: media_type.to_string() })
    }

    fn get(&self, hash: &str) -> Result<Option<Vec<u8>>, store::VcsError> {
        let entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        Ok(entries.get(hash).map(|(bytes, _)| bytes.clone()))
    }

    fn has(&self, hash: &str) -> Result<bool, store::VcsError> {
        let entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        Ok(entries.contains_key(hash))
    }

    fn delete(&self, hash: &str) -> Result<(), store::VcsError> {
        let mut entries = self.entries.lock().map_err(|_| store::VcsError::Backbone("blob store lock poisoned".into()))?;
        entries.remove(hash);
        Ok(())
    }
}
//#endregion 🔖️BlobStore

//#region 🔖️MediaArtifact
/// 🔁️ Lossless bridge from `Media` to the wire-level `(descriptor, data)` byte pair carried by
/// `AppCommand::MediaIn`/`AppFrame::Media` — reuses `semio_framework_plugin::app::MediaArtifactDescriptor`
/// directly (not a hand-mirrored duplicate) so the runner and every guest plugin's
/// `plugin_consume_media`/`plugin_produce_media` glue agree on the shape by construction. A `Binary`
/// payload's bytes never live inline in `Media` (only its content-addressed `blob_hash` does) — this
/// is the one place that boundary is crossed, resolving them through `blob_store` into the wire's
/// inline `data`.
pub fn media_to_artifact<B: BlobStore>(media: &Media, blob_store: &B) -> Result<(Vec<u8>, Vec<u8>), RunError> {
    let (wire, blob_hash, data) = match &media.payload {
        MediaPayload::Structured { schema, json } => (MediaWireFormat::Document { schema: schema.clone() }, None, json.clone().into_bytes()),
        MediaPayload::Binary { format_kind, blob_hash } => {
            let bytes = blob_store.get(blob_hash).map_err(|error| RunError::Host(error.to_string()))?.ok_or_else(|| RunError::Host(format!("blob not found: {blob_hash}")))?;
            (MediaWireFormat::Binary { format_kind: format_kind.clone() }, Some(blob_hash.clone()), bytes)
        }
    };
    let descriptor = semio_framework_plugin::app::MediaArtifactDescriptor { edge_id: None, port_id: None, kind_id: None, media_type: Some(media.media_type), wire, blob_hash };
    let descriptor_value = to_dsl_value(&descriptor).map_err(|error| RunError::Host(error))?;
    Ok((store::pack_rt::encode_wire_value(&descriptor_value), data))
}

/// 🔁️ Inverse of [`media_to_artifact`]. A `Binary` wire artifact's `data` is written into
/// `blob_store` (content-addressed, idempotent) rather than kept inline, mirroring `Media`'s own
/// "binary payloads never carry bytes directly" invariant — the freshly computed hash supersedes
/// whatever `blob_hash` the artifact's own descriptor claimed.
pub fn media_from_artifact<B: BlobStore>(descriptor: &[u8], data: Vec<u8>, blob_store: &B) -> Result<Media, RunError> {
    let value = store::pack_rt::decode_wire_value(descriptor).map_err(|error| RunError::Host(error.to_string()))?;
    let descriptor: semio_framework_plugin::app::MediaArtifactDescriptor = from_dsl_value(value).map_err(|error| RunError::Host(error))?;
    let media_type = descriptor.media_type.ok_or_else(|| RunError::Host("media artifact descriptor is missing media_type".to_string()))?;
    let payload = match descriptor.wire {
        MediaWireFormat::Document { schema } => MediaPayload::Structured { schema, json: String::from_utf8(data).map_err(|error| RunError::Host(error.to_string()))? },
        MediaWireFormat::Binary { format_kind } => {
            let mime = semio_framework::format_descriptor(&format_kind)
                .map_err(|error| RunError::Host(error.to_string()))?
                .ok_or_else(|| RunError::Host(format!("unknown media format kind {format_kind:?}")))?
                .mimes
                .first()
                .cloned()
                .ok_or_else(|| RunError::Host(format!("media format kind {format_kind:?} has no MIME claim")))?;
            let blob_ref = blob_store.put(&data, &mime).map_err(|error| RunError::Host(error.to_string()))?;
            MediaPayload::Binary { format_kind, blob_hash: blob_ref.hash }
        }
    };
    Ok(Media { media_type, payload })
}

/// 🔎️ Which `AppCommand::seq` (if any) an `AppFrame` replies to — `None` for the handful of
/// unsolicited shapes (`DocumentChanged`, `ConfigChanged`) that never carry one. Channel v12
/// (`📡️spr/🧵️channel/🦀️.rs`'s own doc) retires the `Hello`/`Welcome` handshake entirely —
/// lifecycle now arrives through the reactor ABI's `Event::InstanceOpen`/`InstanceClose` — and
/// replaces `UiSection`/`Effects`/`Events` with `UiPatch`/`UiSnapshotEnd`.
fn frame_in_reply_to(frame: &AppFrame) -> Option<u64> {
    match frame {
        AppFrame::Done { in_reply_to } => Some(*in_reply_to),
        AppFrame::Invocation { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Document { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::ContextMenu { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Media { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::MediaFingerprint { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::UiPatch { in_reply_to, .. } => *in_reply_to,
        AppFrame::UiSnapshotEnd { .. } => None,
        AppFrame::Error { in_reply_to, .. } => *in_reply_to,
        AppFrame::DocumentChanged { .. } | AppFrame::ConfigChanged { .. } => None,
        AppFrame::Config { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Emit { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Draft { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Children { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::Ephemeral { .. } => None,
        AppFrame::HistorySnapshot { in_reply_to, .. } => Some(*in_reply_to),
        AppFrame::TransactionProposal { in_reply_to, .. } => Some(*in_reply_to),
        // 🔀️ Transaction phase-2 frames correlate by `txn_id`, not by `AppCommand::seq` — this
        // runner's own frame script never opens a transaction (`compute_node` sends no
        // `TransactionPrepare`/`Commit`/`Rollback`), so these can never actually appear in a
        // `SpaceRunner` batch; listed for exhaustiveness (contract-freeze.md §C8's tag table is
        // frozen — a wildcard arm here would silently swallow a real future addition instead of
        // failing loud the way this match already does for everything else).
        AppFrame::TransactionPrepared { .. } | AppFrame::TransactionCommitted { .. } | AppFrame::TransactionRolledBack { .. } => None,
        // ⚔️ Pushed unsolicited (contract-freeze.md §C8/C9) except as the direct reply to
        // `ResolveConflict`/`ReadConflicts` — this runner's frame script never sends those either, so
        // `in_reply_to` is `None` in every frame this crate actually sees; still correlate correctly
        // (not `None` unconditionally) so a future caller that DOES send them gets the real answer.
        AppFrame::MergeReport { in_reply_to, .. } => *in_reply_to,
        AppFrame::Conflicts { in_reply_to, .. } => *in_reply_to,
    }
}

/// 🔑️ Decodes an `AppFrame::MediaFingerprint::fingerprint`/`MediaFingerprint`'s
/// `store::pack_rt::encode_wire_value`-encoded wire payload back into its plain string (a
/// `MediaFingerprint(String)` newtype serializes transparently, so the wire value is just a string).
fn decode_fingerprint_wire(bytes: &[u8]) -> Result<String, RunError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| RunError::Host(error.to_string()))?;
    value.as_str().map(str::to_string).ok_or_else(|| RunError::Host("media fingerprint wire value was not a string".to_string()))
}

fn app_frame_fault_summary(fault: &[u8]) -> String {
    let fault = dsl::decode_fault_bytes(fault);
    format!("{}: {}", fault.code.0, fault.message)
}

/// 🧾 Formats an `AppFrame::Error`'s trailing `report` (a packed `protocol::DispatchReport`, present
/// whenever `fault.code == "mutation.rejected"` — contract-freeze.md §C8/C9) into a short
/// human-readable `code: message [target]` list, so a rejected dispatch's REAL `mutation.*` messages
/// reach `RunError`'s own text (and, through it, `sink.record(RunMutation::AppendRunLog(..))`'s sealed
/// diagnostics — see `bin.rs::run`'s `Err` branch) instead of only the generic
/// `app_frame_fault_summary` one-liner. Empty for a pre-CHANNEL_VERSION-11 peer or a rejection whose
/// report genuinely carries no messages.
fn dispatch_report_summary(report: &[u8]) -> String {
    if report.is_empty() {
        return String::new();
    }
    let Ok(value) = store::pack_rt::decode_wire_value(report) else { return String::new() };
    let Ok(decoded) = from_dsl_value::<protocol::DispatchReport>(value) else { return String::new() };
    decoded.messages.iter().map(|message| if message.target.is_empty() { format!("{}: {}", message.code.0, message.message) } else { format!("{}: {} [{}]", message.code.0, message.message, message.target.join("/")) }).collect::<Vec<_>>().join("; ")
}

/// 🧾 One rejected frame's full message: `` `app_id` <verb> (fault_code: fault_message)``, plus —
/// whenever the frame's trailing `report` carries real `mutation.*` messages — `` — code: text
/// [target]; ...`` appended (`dispatch_report_summary`). The single call site every
/// `compute_node` `AppFrame::Error` arm shares, so a node's REAL rejection reason (not just the
/// generic `mutation.rejected` fault) reaches `RunError`'s text everywhere a dispatch can be rejected.
fn dispatch_error_message(app_id: &str, verb: &str, fault: &[u8], report: &[u8]) -> String {
    let mut message = format!("`{app_id}` {verb} ({})", app_frame_fault_summary(fault));
    let summary = dispatch_report_summary(report);
    if !summary.is_empty() {
        message.push_str(&format!(" — {summary}"));
    }
    message
}

#[cfg(test)]
fn run_fault_bytes(code: impl Into<String>, message: impl Into<String>) -> Vec<u8> {
    dsl::encode_fault_bytes(&dsl::Fault::new(dsl::FaultOrigin::Os, dsl::FaultCode::new(code.into()), message))
}
//#endregion 🔖️MediaArtifact

//#region 🔖️MediaConverters
/// 🔀️ A pure `Media -> Media` conversion for one `(class, from, to)` triple — e.g. `(ThreeD, Brep,
/// Mesh)` for a CAD tessellator. Registered once per plugin load (Wave 2); this crate ships only the
/// registry mechanism and the `compute_node` call site — no converter bodies.
pub type MediaConvertFn = fn(&Media) -> Result<Media, RunError>;

/// 🗺️ Process-wide converter table — `LazyLock`/`Mutex` because registration happens at plugin-load
/// time (an ordinary function call, not a `SpaceRunner` method) while lookup happens per-input inside
/// `SpaceRunner::run`; both need the same shared table without threading it through every signature.
static MEDIA_CONVERTERS: LazyLock<Mutex<HashMap<(MediaClass, MediaForm, MediaForm), MediaConvertFn>>> = LazyLock::new(|| Mutex::new(HashMap::new()));

/// 🔌️ Registers (or replaces) the converter for one `(class, from, to)` triple.
pub fn register_media_converter(class: MediaClass, from: MediaForm, to: MediaForm, f: MediaConvertFn) {
    MEDIA_CONVERTERS.lock().expect("media converter registry lock poisoned").insert((class, from, to), f);
}

/// 🖼️ Vector→Raster: rasterizes an SVG `Media` payload (as produced by a TwoD×Vector app's
/// `"vector:out"` port — e.g. `draw`'s `2d.drawing`, `MediaPayload::Structured { schema:
/// "2d.drawing", json: <svg text> }`) into a base64 PNG `Media` payload consumable by a TwoD×Raster
/// `"image:in"` port (e.g. `raster`'s `2d.image`) — reuses `semio_framework_os`'s
/// `rasterize_svg_to_png_base64` (the same engine `raster_engine::raster_document_json_from_dwg`'s
/// DWG-import rasterizer already uses), auto-sizing from the SVG's own intrinsic viewBox (`width: 0,
/// height: 0`). The FIRST real `MediaConvertFn` body this registry carries (WORKFLOWS-END-TO-END-
/// TYPED-PORTS Wave 2, draw+raster package) — Brep→Mesh/Design→Kit/Type→Kit remain unregistered
/// until their own owning packages land a body.
fn vector_to_raster(media: &Media) -> Result<Media, RunError> {
    let MediaPayload::Structured { json: svg, .. } = &media.payload else {
        return Err(RunError::Media(MediaError::Payload("vector-to-raster".into(), "expected a Structured (SVG text) payload".into())));
    };
    let png_base64 = semio_framework_os::rasterize_svg_to_png_base64(svg, 0, 0).map_err(RunError::Host)?;
    Ok(Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Raster }, payload: MediaPayload::Structured { schema: "2d.image".into(), json: png_base64 } })
}

/// 🔌️ Registers every framework-builtin media converter this crate ships a real body for — called
/// once at process/composition-root startup (see `bin.rs::run`, right beside where plugins are
/// resolved/loaded). Wave 2 (draw+raster package) adds the first real body (Vector→Raster); later
/// waves add Brep→Mesh/Design→Kit/Type→Kit here as their owning packages land bodies for them.
pub fn register_builtin_converters() {
    register_media_converter(MediaClass::TwoD, MediaForm::Vector, MediaForm::Raster, vector_to_raster);
}

/// 🔎️ Whether a converter is registered for `(class, from, to)` — `validate_edge_kinds` calls this at
/// plan/connect time so a workflow carrying an edge nobody can actually run is rejected before any
/// node ever opens (fail-closed).
fn media_converter_registered(class: MediaClass, from: MediaForm, to: MediaForm) -> bool {
    MEDIA_CONVERTERS.lock().expect("media converter registry lock poisoned").contains_key(&(class, from, to))
}

/// 🔀️ Applies `contract.conversion` (if any) to `media` — identity passthrough when the negotiated
/// contract carries no conversion, looks up + applies the registered `MediaConvertFn` otherwise.
/// `validate_edge_kinds` already rejects a graph carrying an edge whose conversion has no registered
/// converter, so `Err(RunError::NoConverter)` here means the registry changed between validation and
/// this call (e.g. a hot-swapped plugin) — still handled correctly, just not the common path.
pub fn convert_media(contract: &MediaContract, media: Media) -> Result<Media, RunError> {
    match contract.conversion {
        None => Ok(media),
        Some((from, to)) => {
            let class = media.media_type.class;
            let converters = MEDIA_CONVERTERS.lock().expect("media converter registry lock poisoned");
            match converters.get(&(class, from, to)) {
                Some(f) => f(&media),
                None => Err(RunError::NoConverter { class, from, to }),
            }
        }
    }
}
//#endregion 🔖️MediaConverters

//#region 🔖️RunContext
/// ✍️ Write-only sink for everything one `SpaceRunner::run` call produces — the ONLY thing allowed to
/// persist bytes for a node's post-import document/config state or accumulate the run's own
/// `workflow::RunArtifact`. Never reads the source `documents`/`configs` maps `run()` takes, and
/// `run()` never writes through anything OTHER than this — the structural half of "non-destructive":
/// `node_artifacts`/`node_configs` are keyed by node id, a distinct key space from the source
/// `artifact_ref`/`config_ref` strings, so a caller (e.g. `bin.rs`) cannot alias a run-owned write
/// path onto a source artifact path by construction.
///
/// 🔒️ `record` is the ONLY way this crate ever mutates a `workflow::RunArtifact` — it always goes
/// through `workflow::apply_run_operation_checked` (never the raw `workflow::apply_run_operation`),
/// so an operation emitted after `Seal` is rejected here with its typed `RunError::MutationApply`, not silently
/// applied. `SpaceRunner::run` calls `record` for every `NodeStarted`/`NodeFinished`; callers own
/// `Start` (before `run`) and `Seal` (after), since those two carry run-identity/collection-ref fields
/// `SpaceRunner` itself has no business knowing about.
pub struct RunSink {
    pub document: workflow::RunArtifact,
    /// 🧾️ Every operation `record` has successfully applied, in order — `bin.rs` replays this exact
    /// sequence through a real `store::ArtifactStore` to produce persistable pack+spr bytes for
    /// `SpaceBundle::write_run_document` (the same "build an envelope, `Apply`, `snapshot_pack`"
    /// pattern every other document in this codebase persists through).
    pub mutations: Vec<RunMutation>,
    pub node_artifacts: BTreeMap<String, (Vec<u8>, Vec<u8>)>,
    pub node_configs: BTreeMap<String, (Vec<u8>, Vec<u8>)>,
}

impl RunSink {
    pub fn new(document: workflow::RunArtifact) -> Self {
        Self { document, mutations: Vec::new(), node_artifacts: BTreeMap::new(), node_configs: BTreeMap::new() }
    }

    pub async fn record(&mut self, operation: RunMutation) -> Result<(), RunError> {
        self.document = workflow::apply_run_operation_checked(&self.document, operation.clone()).await?;
        self.mutations.push(operation);
        Ok(())
    }

    pub fn write_node_artifact(&mut self, node_id: &str, pack: Vec<u8>, spr: Vec<u8>) {
        self.node_artifacts.insert(node_id.to_string(), (pack, spr));
    }

    pub fn write_node_config(&mut self, node_id: &str, pack: Vec<u8>, spr: Vec<u8>) {
        self.node_configs.insert(node_id.to_string(), (pack, spr));
    }
}

/// 🎛️ Deterministic bytes for one node's bound `RunParameterValue` overlay (sorted by `field_path`) —
/// folded into that node's `config_fingerprint` so `--param` overrides correctly dirty/cache-key a
/// node even though the underlying config pack+spr bytes sent to the app are untouched (this crate is
/// generic over arbitrary apps' config schemas — it has no per-app `ConfigSpec` to safely patch an
/// opaque config pack's fields by `field_path` itself; that requires `semio_framework::ConfigSpec`,
/// which lives one layer up, at the plugin-manifest layer this crate doesn't depend on). Delivering an
/// override into the actual bytes the app receives is deferred — see this wave's final report.
fn node_parameter_overlay_bytes(node_id: &str, bindings: &[WorkflowParameterBinding], parameter_values: &[RunParameterValue]) -> Vec<u8> {
    let mut pairs: Vec<(&str, &str)> =
        bindings.iter().filter(|binding| binding.node_id == node_id).filter_map(|binding| parameter_values.iter().find(|value| value.parameter_id == binding.parameter_id).map(|value| (binding.field_path.as_str(), value.value.as_str()))).collect();
    pairs.sort_unstable();
    let mut bytes = Vec::new();
    for (field_path, value) in pairs {
        bytes.extend_from_slice(field_path.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(value.as_bytes());
        bytes.push(0);
    }
    bytes
}

/// 🔑️ `document`/`config` fingerprints for one node — `config_fingerprint` folds in the node's bound
/// parameter overlay (see `node_parameter_overlay_bytes`) so a `--param` override changes the
/// fingerprint even when the underlying config bytes don't.
fn node_fingerprints(node_id: &str, document_spr: &[u8], config_spr: &[u8], bindings: &[WorkflowParameterBinding], parameter_values: &[RunParameterValue]) -> (String, String) {
    let document_fingerprint = framework_hash::hash_bytes(document_spr);
    let overlay = node_parameter_overlay_bytes(node_id, bindings, parameter_values);
    let config_fingerprint = if overlay.is_empty() { framework_hash::hash_bytes(config_spr) } else { framework_hash::hash_bytes(&[config_spr, &overlay].concat()) };
    (document_fingerprint, config_fingerprint)
}
//#endregion 🔖️RunContext

//#region 🔖️SpaceBundle
/// 📁️ The canonical on-disk shape of a space (`## Design rulings` -> `On-disk space layout` in
/// `.claude/plans/the-final-goal-for-jolly-spindle.md`, rewritten here for W4's storage-layout task —
/// superseding the pre-W4 flat `space.os.pack`/`<artifact_ref>.pack` layout, whose filenames were
/// stale leftovers from the dissolved `OsDocument`):
/// - root: `space.space.pack`+`space.space.spr` — the space manifest slot (the `OsDocument` VCS
///   envelope's binary pack+dsl form — see `semio_framework_os::encode_os_space_payload`; today this
///   still carries a `workflow::WorkflowSnapshot`/`WorkflowMutation` pair per the os-core dissolve's
///   `## The inversion`, not yet a real `space::SpaceSnapshot` manifest — wiring the ROOT slot's
///   actual decoded type to `SpaceSnapshot` is later-wave work, this rewrite is the path/filename
///   convention only, see `read_space_document`/`write_space_document`),
/// - `collections/<collection id>.collection.pack|.spr` — one `space::CollectionSnapshot` per
///   collection (`collection_pack_path`/`collection_spr_path`/`read_collection`/`write_collection` —
///   not yet called by anything in this crate; collections aren't wired into `SpaceRunner` until a
///   later wave, this is the reserved path convention those callers will use),
/// - `artifacts/<artifact id>.pack|.spr` — one pair per document artifact, reusing the exact same
///   "id IS the address" convention `space::artifact_backbone_uri(space_id, artifact_id)` uses one
///   level up (a bundle has no separate `space_id` of its own — the bundle root already IS one space,
///   so there is nothing to reuse `artifact_backbone_uri` itself for beyond this shared convention;
///   see `artifact_pack_path`/`artifact_spr_path`). A workflow node's `artifact_ref`/`config_ref`
///   (`workflow::workflow_node_for_app`, e.g. `"artifacts/<node id>"`/`"config/<node id>"`) is passed
///   straight through as `artifact_id` — two distinct artifact ids per node, so they never collide
///   under `artifacts/`, and nothing on the `workflow` crate side of this addressing scheme needs to
///   change to fit it (`artifact_pack_path`/`config_pack_path` below are thin `artifact_pack_path`
///   aliases kept for callers' existing names),
/// - `blobs/` — content-addressed, space-level (backing a `MediaPayload::Binary` value's bytes — see
///   `FileBlobStore`; unchanged from the pre-W4 layout, already canonical),
/// - `cache/media/` — cross-run shared media-fingerprint cache (renamed from `run/media/`),
/// - `runs/<run id>.run.pack|.spr` — the `workflow::RunArtifact` VCS envelope itself (W5 Lane A);
///   `runs/<run id>/nodes/<node id>.document.pack|.spr`/`.config.pack|.spr` — that run's OWN mirrored
///   copy of each node's post-import document/config bytes (`run_node_artifact_pack_path`/
///   `run_node_config_pack_path` below) — deliberately NOT under `artifacts/`: a run never writes
///   through the same path a node's SOURCE document/config was read from (see this crate's module doc,
///   "non-destructive rework" — this replaces the old `run/state.json`-backed `RunState`, deleted by
///   this rework). This wave uses a single canonical `run id` (`"default"`, minted by `bin.rs`) — one
///   active run slot per bundle; a real multi-run history/listing is later-wave `space::DraftCatalog`
///   integration work.
///
/// Ids only — no paths inside the space document itself — so the bundle stays relocatable and syncs
/// the same way over `file://` or a semio_hub backbone.
pub struct SpaceBundle {
    root: PathBuf,
}

impl SpaceBundle {
    pub fn open(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn space_artifact_pack_path(&self) -> PathBuf {
        self.root.join("space.space.pack")
    }

    pub fn space_artifact_spr_path(&self) -> PathBuf {
        self.root.join("space.space.spr")
    }

    /// 🗂️ Canonical collection path: `collections/<collection id>.collection.pack`.
    pub fn collection_pack_path(&self, collection_id: &str) -> PathBuf {
        self.root.join("collections").join(format!("{collection_id}.collection.pack"))
    }

    pub fn collection_spr_path(&self, collection_id: &str) -> PathBuf {
        self.root.join("collections").join(format!("{collection_id}.collection.spr"))
    }

    /// 🗂️ Canonical artifact path: `artifacts/<artifact_id>.pack` — see this region's own doc comment
    /// for why `artifact_id` is passed straight through unmodified (it may itself contain `/`, e.g. a
    /// workflow node's `artifact_ref`/`config_ref`) rather than requiring a flat basename.
    pub fn artifact_pack_path(&self, artifact_id: &str) -> PathBuf {
        self.root.join("artifacts").join(format!("{artifact_id}.pack"))
    }

    pub fn artifact_spr_path(&self, artifact_id: &str) -> PathBuf {
        self.root.join("artifacts").join(format!("{artifact_id}.spr"))
    }

    /// 🔖️ Config counterpart of `artifact_pack_path` — same alias-over-`artifact_pack_path`
    /// convention (a node's `config_ref` is already e.g. `config/<node id>`).
    pub fn config_pack_path(&self, config_ref: &str) -> PathBuf {
        self.artifact_pack_path(config_ref)
    }

    pub fn config_spr_path(&self, config_ref: &str) -> PathBuf {
        self.artifact_spr_path(config_ref)
    }

    /// 🗂️ Canonical run-document path: `runs/<run id>.run.pack`.
    pub fn run_artifact_pack_path(&self, run_id: &str) -> PathBuf {
        self.root.join("runs").join(format!("{run_id}.run.pack"))
    }

    pub fn run_artifact_spr_path(&self, run_id: &str) -> PathBuf {
        self.root.join("runs").join(format!("{run_id}.run.spr"))
    }

    /// 🗂️ Canonical run-owned node-document path: `runs/<run id>/nodes/<node id>.document.pack` — see
    /// this region's own doc comment for why this is a distinct tree from `artifacts/`.
    pub fn run_node_artifact_pack_path(&self, run_id: &str, node_id: &str) -> PathBuf {
        self.root.join("runs").join(run_id).join("nodes").join(format!("{node_id}.document.pack"))
    }

    pub fn run_node_artifact_spr_path(&self, run_id: &str, node_id: &str) -> PathBuf {
        self.root.join("runs").join(run_id).join("nodes").join(format!("{node_id}.document.spr"))
    }

    pub fn run_node_config_pack_path(&self, run_id: &str, node_id: &str) -> PathBuf {
        self.root.join("runs").join(run_id).join("nodes").join(format!("{node_id}.config.pack"))
    }

    pub fn run_node_config_spr_path(&self, run_id: &str, node_id: &str) -> PathBuf {
        self.root.join("runs").join(run_id).join("nodes").join(format!("{node_id}.config.spr"))
    }

    pub fn media_cache_dir(&self) -> PathBuf {
        self.root.join("cache").join("media")
    }

    pub fn blobs_dir(&self) -> PathBuf {
        self.root.join("blobs")
    }

    /// @emoji 📦️ Reads the studio's pack+spr bytes, matching `read_artifact`/`read_config`'s "empty
    /// spr if never persisted" convention (a bare pack with no history is a valid fresh studio).
    pub fn read_space_document(&self) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.space_artifact_pack_path();
        let pack = std::fs::read(&pack_path).map_err(|source| RunError::Io { path: pack_path, source })?;
        let spr_path = self.space_artifact_spr_path();
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    pub fn write_space_document(&self, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.space_artifact_pack_path();
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.space_artifact_spr_path(), spr).map_err(|source| RunError::Io { path: self.space_artifact_spr_path(), source })
    }

    /// @emoji 📦️ Reads one node's document pack+spr bytes, `(Vec::new(), Vec::new())` if never persisted.
    pub fn read_artifact(&self, artifact_ref: &str) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.artifact_pack_path(artifact_ref);
        let pack = match std::fs::read(&pack_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), Vec::new())),
            Err(source) => return Err(RunError::Io { path: pack_path, source }),
        };
        let spr_path = self.artifact_spr_path(artifact_ref);
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    pub fn write_artifact(&self, artifact_ref: &str, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.artifact_pack_path(artifact_ref);
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.artifact_spr_path(artifact_ref), spr).map_err(|source| RunError::Io { path: self.artifact_spr_path(artifact_ref), source })
    }

    /// @emoji 📦️ Reads one node's config pack+spr bytes — mirrors `read_artifact` exactly (same
    /// "never persisted" fallback, same error shape).
    pub fn read_config(&self, config_ref: &str) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.config_pack_path(config_ref);
        let pack = match std::fs::read(&pack_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), Vec::new())),
            Err(source) => return Err(RunError::Io { path: pack_path, source }),
        };
        let spr_path = self.config_spr_path(config_ref);
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    /// @emoji 📦️ Writes one node's config pack+spr bytes — mirrors `write_artifact` exactly
    /// (directory-created-if-missing, same error shape).
    pub fn write_config(&self, config_ref: &str, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.config_pack_path(config_ref);
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.config_spr_path(config_ref), spr).map_err(|source| RunError::Io { path: self.config_spr_path(config_ref), source })
    }

    /// @emoji 📦️ Reads one collection's pack+spr bytes — mirrors `read_artifact`'s "never persisted"
    /// fallback exactly. 🚧️ Not yet called from this crate's own runner/CLI flow (collections aren't
    /// wired into `SpaceRunner` until a later wave) — this is the reserved canonical path a future
    /// caller writes/reads through, kept symmetric with `read_artifact`/`write_artifact` on purpose.
    pub fn read_collection(&self, collection_id: &str) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.collection_pack_path(collection_id);
        let pack = match std::fs::read(&pack_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), Vec::new())),
            Err(source) => return Err(RunError::Io { path: pack_path, source }),
        };
        let spr_path = self.collection_spr_path(collection_id);
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    pub fn write_collection(&self, collection_id: &str, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.collection_pack_path(collection_id);
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.collection_spr_path(collection_id), spr).map_err(|source| RunError::Io { path: self.collection_spr_path(collection_id), source })
    }

    /// @emoji 📦️ Reads a run's own `workflow::RunArtifact` pack+spr bytes, `(Vec::new(), Vec::new())`
    /// if never persisted (no prior run of this id) — mirrors `read_artifact`'s fallback exactly.
    pub fn read_run_document(&self, run_id: &str) -> Result<(Vec<u8>, Vec<u8>), RunError> {
        let pack_path = self.run_artifact_pack_path(run_id);
        let pack = match std::fs::read(&pack_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok((Vec::new(), Vec::new())),
            Err(source) => return Err(RunError::Io { path: pack_path, source }),
        };
        let spr_path = self.run_artifact_spr_path(run_id);
        let spr = match std::fs::read(&spr_path) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(source) => return Err(RunError::Io { path: spr_path, source }),
        };
        Ok((pack, spr))
    }

    pub fn write_run_document(&self, run_id: &str, pack: &[u8], spr: &[u8]) -> Result<(), RunError> {
        let pack_path = self.run_artifact_pack_path(run_id);
        if let Some(parent) = pack_path.parent() {
            std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
        }
        std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
        std::fs::write(self.run_artifact_spr_path(run_id), spr).map_err(|source| RunError::Io { path: self.run_artifact_spr_path(run_id), source })
    }

    /// @emoji 📦️ Persists every `RunSink::node_artifacts`/`node_configs` entry from one completed run
    /// under `runs/<run id>/nodes/` — the only place this crate ever writes a node's post-import
    /// document/config bytes to disk (never back over `artifacts/<artifact_ref>`/`artifacts/<config_ref>`).
    pub fn write_run_nodes(&self, run_id: &str, sink: &RunSink) -> Result<(), RunError> {
        for (node_id, (pack, spr)) in &sink.node_artifacts {
            let pack_path = self.run_node_artifact_pack_path(run_id, node_id);
            if let Some(parent) = pack_path.parent() {
                std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
            }
            std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
            std::fs::write(self.run_node_artifact_spr_path(run_id, node_id), spr).map_err(|source| RunError::Io { path: self.run_node_artifact_spr_path(run_id, node_id), source })?;
        }
        for (node_id, (pack, spr)) in &sink.node_configs {
            let pack_path = self.run_node_config_pack_path(run_id, node_id);
            if let Some(parent) = pack_path.parent() {
                std::fs::create_dir_all(parent).map_err(|source| RunError::Io { path: parent.to_path_buf(), source })?;
            }
            std::fs::write(&pack_path, pack).map_err(|source| RunError::Io { path: pack_path, source })?;
            std::fs::write(self.run_node_config_spr_path(run_id, node_id), spr).map_err(|source| RunError::Io { path: self.run_node_config_spr_path(run_id, node_id), source })?;
        }
        Ok(())
    }

    pub fn blob_store(&self) -> FileBlobStore {
        FileBlobStore::new(self.blobs_dir())
    }
}
//#endregion 🔖️SpaceBundle

//#region 🔖️Topology
/// 🔢️ Deterministic topological order (Kahn's algorithm, lexicographically-smallest-ready-node-first)
/// over `graph`'s nodes. `Err(RunError::Cycle)` names whichever nodes never became ready —
/// `workflow::validate_workflow` should be called first to reject cycles with a friendlier message;
/// this is the runner's authoritative order once that check has passed.
///
/// 🧷️ Not redundant with `workflow::plan_workflow`: that function only walks the downstream closure
/// of an already-dirty node set to emit `WorkflowDelivery` records, and its own topological helper is
/// private to the kernel crate. The runner needs a total Kahn order over EVERY node (dirty, clean, or
/// edge-less) to decide host-open/execution sequence deterministically — a genuinely different job.
fn topological_order(graph: &Workflow) -> Result<Vec<String>, RunError> {
    let mut indegree: BTreeMap<String, usize> = graph.nodes.iter().map(|node| (node.id.clone(), 0)).collect();
    let mut outgoing: HashMap<String, Vec<String>> = HashMap::new();
    for edge in &graph.edges {
        *indegree.entry(edge.target_node_id.clone()).or_insert(0) += 1;
        outgoing.entry(edge.source_node_id.clone()).or_default().push(edge.target_node_id.clone());
    }
    let mut ready: BTreeSet<String> = indegree.iter().filter(|(_, degree)| **degree == 0).map(|(id, _)| id.clone()).collect();
    let mut order = Vec::with_capacity(graph.nodes.len());
    while let Some(node_id) = ready.iter().next().cloned() {
        ready.remove(&node_id);
        order.push(node_id.clone());
        for next in outgoing.get(&node_id).into_iter().flatten() {
            if let Some(degree) = indegree.get_mut(next) {
                *degree -= 1;
                if *degree == 0 {
                    ready.insert(next.clone());
                }
            }
        }
    }
    if order.len() != graph.nodes.len() {
        let unreached: Vec<String> = graph.nodes.iter().map(|node| node.id.clone()).filter(|id| !order.contains(id)).collect();
        return Err(RunError::Cycle(unreached));
    }
    Ok(order)
}
//#endregion 🔖️Topology

//#region 🔖️EdgeValidation
/// ✅️ Full connect-time edge validation, called by both `plan` (the `--dry` path never opens a
/// single node) and `run`: every edge's endpoints must resolve to real ports; the producer's
/// `MediaType` must be `media_types_compatible` with the consumer's (typed rejection, no more
/// stringly `artifact_kind` equality); a negotiated `contract.conversion` must have a registered
/// converter (fail-closed — an edge nobody can actually run is worse than one caught here at plan
/// time); and every input port's `required`/`multiplicity` constraint must hold across the WHOLE
/// node, not just one edge (so it's checked in a second pass once every edge's incoming count is known).
fn validate_edge_kinds(graph: &Workflow) -> Result<(), RunError> {
    let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut incoming_count: HashMap<(&str, &str), usize> = HashMap::new();

    for edge in &graph.edges {
        let source_node = *node_by_id.get(edge.source_node_id.as_str()).ok_or_else(|| RunError::UnknownNode(edge.source_node_id.clone()))?;
        let target_node = *node_by_id.get(edge.target_node_id.as_str()).ok_or_else(|| RunError::UnknownNode(edge.target_node_id.clone()))?;
        let source_port = source_node.outputs.iter().find(|port| port.id == edge.source_port_id).ok_or_else(|| RunError::UnknownPort { edge_id: edge.id.clone(), node_id: edge.source_node_id.clone(), port_id: edge.source_port_id.clone() })?;
        let target_port = target_node.inputs.iter().find(|port| port.id == edge.target_port_id).ok_or_else(|| RunError::UnknownPort { edge_id: edge.id.clone(), node_id: edge.target_node_id.clone(), port_id: edge.target_port_id.clone() })?;

        if matches!(media_types_compatible(&source_port.spec.media_type, &target_port.spec.media_type), MediaCompat::Reject) {
            return Err(RunError::Incompatible { edge_id: edge.id.clone(), produced: source_port.spec.media_type, accepted: target_port.spec.media_type });
        }

        if let Some((from, to)) = edge.contract.conversion {
            if !media_converter_registered(edge.contract.media_type.class, from, to) {
                return Err(RunError::UnregisteredConversion { edge_id: edge.id.clone(), from, to });
            }
        }

        *incoming_count.entry((edge.target_node_id.as_str(), edge.target_port_id.as_str())).or_insert(0) += 1;
    }

    for node in &graph.nodes {
        for port in &node.inputs {
            let count = incoming_count.get(&(node.id.as_str(), port.id.as_str())).copied().unwrap_or(0);
            if port.spec.required && count == 0 {
                return Err(RunError::MissingRequiredInput { node_id: node.id.clone(), port_id: port.id.clone() });
            }
            if matches!(port.spec.multiplicity, PortMultiplicity::One) && count > 1 {
                return Err(RunError::MultiplicityViolation { node_id: node.id.clone(), port_id: port.id.clone(), count });
            }
        }
    }
    Ok(())
}
//#endregion 🔖️EdgeValidation

//#region 🔖️SpaceRunner
/// 📊️ What actually happened in one `run()` call — which nodes were recomputed and which were left
/// untouched because neither their document, inputs, nor config changed.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RunReport {
    pub recomputed: Vec<String>,
    pub clean: Vec<String>,
}

/// 🩺️ Computes which nodes `SpaceRunner::run` would recompute, without instantiating a single host
/// — the `--dry` plan. Reuses exactly the dirty check `run` applies, so the plan can never drift from
/// what an actual run would do. `documents`/`configs` map a node's `artifact_ref`/`config_ref` string
/// to its current `(pack, spr)` artifact bytes — missing/absent means "never persisted".
/// `prior_node_records` is the PRIOR SEALED `workflow::RunArtifact.node_records`, keyed by node id
/// (empty for a first-ever run — every node then plans as recomputed).
pub fn plan(
    graph: &Workflow,
    documents: &BTreeMap<String, (Vec<u8>, Vec<u8>)>,
    configs: &BTreeMap<String, (Vec<u8>, Vec<u8>)>,
    parameter_values: &[RunParameterValue],
    parameter_bindings: &[WorkflowParameterBinding],
    prior_node_records: &BTreeMap<String, RunNodeRecord>,
) -> Result<RunReport, RunError> {
    validate_edge_kinds(graph)?;
    let order = topological_order(graph)?;
    let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
    let mut incoming: HashMap<&str, Vec<&WorkflowEdge>> = HashMap::new();
    for edge in &graph.edges {
        incoming.entry(edge.target_node_id.as_str()).or_default().push(edge);
    }
    let mut report = RunReport::default();
    for node_id in &order {
        let node = *node_by_id.get(node_id.as_str()).ok_or_else(|| RunError::UnknownNode(node_id.clone()))?;
        let document = documents.get(&node.artifact_ref).cloned().unwrap_or_default();
        let config = configs.get(&node.config_ref).cloned().unwrap_or_default();
        let (document_fingerprint, config_fingerprint) = node_fingerprints(node_id, &document.1, &config.1, parameter_bindings, parameter_values);
        let mut input_fingerprints: BTreeMap<String, String> = BTreeMap::new();
        for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
            let fingerprint = prior_node_records.get(&edge.source_node_id).and_then(|record| record.output_fingerprints.iter().find(|entry| entry.port_id == edge.source_port_id)).map(|entry| entry.fingerprint.clone()).unwrap_or_default();
            input_fingerprints.insert(edge.target_port_id.clone(), fingerprint);
        }
        let dirty = match prior_node_records.get(node_id.as_str()) {
            None => true,
            Some(record) => {
                let previous_inputs: BTreeMap<String, String> = record.input_fingerprints.iter().map(|entry| (entry.port_id.clone(), entry.fingerprint.clone())).collect();
                record.document_fingerprint != document_fingerprint || previous_inputs != input_fingerprints || record.config_fingerprint != config_fingerprint
            }
        };
        if dirty {
            report.recomputed.push(node_id.clone());
        } else {
            report.clean.push(node_id.clone());
        }
    }
    Ok(report)
}

/// 🕸️ Computes one studio's workflow against an `AppChannelHost`. Node dirtiness is decided purely
/// from the PRIOR SEALED run's `workflow::RunNodeRecord`: the document's own fingerprint (did the
/// app's document change since last run — e.g. a UI edit), its resolved input fingerprints (did
/// anything upstream change), and its config's fingerprint (folding in any bound `--param` overlay —
/// see `node_parameter_overlay_bytes`). A clean node is never opened at all; its cached output
/// fingerprints feed straight into its consumers. `run()` is READONLY over `documents`/`configs` — see
/// this crate's module doc — every byte it produces lands in the caller-supplied `RunSink` instead.
// 🔀️ `B` is the pluggable `BlobStore` backend — dedyn-fw-os-misc, R11(a): `BlobStore` has 5 impls
// spread across independently-owned modules/crates (`🏪️store::NoBlobStore`/`FolderEventLogStorage`,
// `🪐️space::TestBlobStore`, this crate's own `FileBlobStore`/`InMemoryBlobStore`) — a genuinely open,
// cross-crate extension point (`🏃️run` cannot name a `🪐️space`/`🏪️store::sync` type without a reverse
// dependency), so `dyn_enum_close!` cannot close it; every caller already supplies a concrete value
// explicitly (`SpaceRunner::new`'s own `blob_store` parameter), so `B` is inferred at every call
// site and the `NoBlobStore` default only matters for a bare `SpaceRunner<H>` mention (none exist
// outside doc comments, verified).
pub struct SpaceRunner<H: AppChannelHost, B: BlobStore + 'static = NoBlobStore> {
    host: H,
    blob_store: Arc<B>,
    /// ⚖️ This run's local/authority `MergePolicy` (contract-freeze.md §C3) — sent as
    /// `AppCommand::SetMergePolicy` right after every node's `Hello`, in the same batched exchange
    /// (see `compute_node`'s doc). `--policy`/config default: `Normal`.
    merge_policy: protocol::MergePolicy,
    /// 🛑️ This run's own cancellation root — `CancelToken::root()` by default (never cancelled
    /// unless a caller holds a clone via `cancel_token()` and calls `.cancel()` on it), a child of
    /// which becomes every `compute_node` call's `OperationContext.cancel`. One token for the whole
    /// run, not one per node: cancelling it stops the run before its NEXT node (see
    /// `RunError::Cancelled`'s doc) without needing a separate per-node cancel wire.
    cancel: CancelToken,
    /// ⏰️ Optional wall-clock deadline (epoch ms) applied to every `OperationContext` this run
    /// builds — `None` (the default) means no deadline, matching every pre-async-rewrite caller's
    /// behavior exactly.
    deadline_ms: Option<u64>,
}

impl<H: AppChannelHost, B: BlobStore + 'static> SpaceRunner<H, B> {
    pub fn new(host: H, blob_store: Arc<B>, merge_policy: protocol::MergePolicy) -> Self {
        Self { host, blob_store, merge_policy, cancel: CancelToken::root(), deadline_ms: None }
    }

    pub fn into_host(self) -> H {
        self.host
    }

    /// 🛑️ A clone of this run's cancellation root — call `.cancel()` on it from another thread (or
    /// a UI cancel button's handler) to stop the run before its next node. `SpaceRunner` itself
    /// never cancels this token.
    pub fn cancel_token(&self) -> CancelToken {
        self.cancel.clone()
    }

    /// ⏰️ Applies a wall-clock deadline (epoch ms) to every `OperationContext` this run builds from
    /// here on. Builder-style: `SpaceRunner::new(..).with_deadline_ms(..)`.
    pub fn with_deadline_ms(mut self, deadline_ms: Option<u64>) -> Self {
        self.deadline_ms = deadline_ms;
        self
    }

    /// 🪪️ Builds one node's `OperationContext`: `actor` is the node's own `AppChannelHost` handle
    /// (so a real host can tell which instance an operation belongs to), `cancel` a child of this
    /// run's root (see the `cancel` field's own doc — cancelling the run cancels every node's
    /// context transitively via `CancelToken::child`'s max-severity fold).
    fn node_ctx(&self, node_handle: u32) -> OperationContext {
        OperationContext { actor: node_handle as u64, generation: 0, trace: TraceId(node_handle as u64), lane: 0, deadline_ms: self.deadline_ms, cancel: self.cancel.child(), capability: None }
    }

    /// 🔌️ Returns `node`'s already-open handle, opening it (`host.open(node.plugin_id, node.app_id)`)
    /// and caching the handle in `live` on first use. Lazy by construction (unlike a plain
    /// `HashMap::entry(..).or_insert(expr)`, which would evaluate `expr` — and so call `host.open` —
    /// unconditionally even when the entry already exists).
    async fn open_node(&mut self, live: &mut HashMap<String, u32>, node: &WorkflowNode) -> Result<u32, RunError> {
        if let Some(handle) = live.get(&node.id) {
            return Ok(*handle);
        }
        let handle = self.host.open(&node.plugin_id, &node.app_id, &node.artifact_ref).await?;
        live.insert(node.id.clone(), handle);
        Ok(handle)
    }

    /// 🎬️ Runs one node's whole frame script — `SetMergePolicy` (this run's `merge_policy`, sent
    /// FIRST so the instance's local/authority policy is established before any other command
    /// reaches it — channel v12 retires `Hello`/`Welcome` entirely, `host.open` is what now
    /// establishes the instance, via `Event::InstanceOpen` on the reactor ABI, not a wire command),
    /// `LoadConfig`, `LoadDocument`, one `MediaIn` per resolved input, one
    /// `MediaOut`+`MediaFingerprint` pair per output port, then `ReadDocument` and finally
    /// `ReadConfig` to persist whatever the imports mutated on either artifact (see this file's
    /// header doc: "importing media is emitting operations") — as a single batched `host.exchange`
    /// call. Returns the node's mutated document bytes, its mutated config bytes, and per output port
    /// the exported `Media` plus its wire fingerprint string.
    async fn compute_node(
        &mut self,
        live: &mut HashMap<String, u32>,
        node: &WorkflowNode,
        document: &(Vec<u8>, Vec<u8>),
        config: &(Vec<u8>, Vec<u8>),
        input_media: &BTreeMap<String, Media>,
    ) -> Result<((Vec<u8>, Vec<u8>), (Vec<u8>, Vec<u8>), BTreeMap<String, (Media, String)>), RunError> {
        // 🛑️ Checked BEFORE `open`/`exchange` — a cancelled run stops before its NEXT node rather
        // than mid-exchange (see `RunError::Cancelled`'s own doc).
        if self.cancel.is_cancelled() {
            return Err(RunError::Cancelled);
        }
        let handle = self.open_node(live, node).await?;
        let ctx = self.node_ctx(handle);

        let mut seq: u64 = 0;
        let mut next_seq = move || {
            seq += 1;
            seq
        };

        let mut commands = Vec::new();

        let set_policy_seq = next_seq();
        commands.push(AppCommand::SetMergePolicy { seq: set_policy_seq, policy: self.merge_policy.as_u8() });

        let load_config_seq = next_seq();
        commands.push(AppCommand::LoadConfig { seq: load_config_seq, pack: config.0.clone(), spr: config.1.clone() });

        let load_document_seq = next_seq();
        commands.push(AppCommand::LoadDocument { seq: load_document_seq, pack: document.0.clone(), spr: document.1.clone() });

        let mut media_in_seqs = Vec::with_capacity(input_media.len());
        for (port, media) in input_media {
            let (descriptor, data) = media_to_artifact(media, self.blob_store.as_ref())?;
            let this_seq = next_seq();
            commands.push(AppCommand::MediaIn { seq: this_seq, port: port.clone(), descriptor, data });
            media_in_seqs.push(this_seq);
        }

        let mut output_seqs = Vec::with_capacity(node.outputs.len());
        for port in &node.outputs {
            let media_out_seq = next_seq();
            commands.push(AppCommand::MediaOut { seq: media_out_seq, port: port.id.clone(), request: Vec::new() });
            let fingerprint_seq = next_seq();
            commands.push(AppCommand::MediaFingerprint { seq: fingerprint_seq, port: port.id.clone() });
            output_seqs.push((port.id.clone(), media_out_seq, fingerprint_seq));
        }

        let read_artifact_seq = next_seq();
        commands.push(AppCommand::ReadDocument { seq: read_artifact_seq });

        let read_config_seq = next_seq();
        commands.push(AppCommand::ReadConfig { seq: read_config_seq });

        let frames = self.host.exchange(&ctx, handle, commands).await?;

        if let Some(AppFrame::Error { fault, report, .. }) = frames.iter().find(|frame| matches!(frame, AppFrame::Error { in_reply_to: None, .. })) {
            return Err(RunError::Host(dispatch_error_message(&node.app_id, "sent an unsolicited rejection", fault, report)));
        }

        let reply_to = |seq: u64| -> Result<&AppFrame, RunError> { frames.iter().find(|frame| frame_in_reply_to(frame) == Some(seq)).ok_or_else(|| RunError::Host(format!("`{}` sent no reply to seq {seq}", node.app_id))) };
        let expect_done = |seq: u64, frame: &AppFrame| -> Result<(), RunError> {
            match frame {
                AppFrame::Done { .. } => Ok(()),
                AppFrame::Error { fault, report, .. } => Err(RunError::Host(dispatch_error_message(&node.app_id, &format!("rejected seq {seq}"), fault, report))),
                other => Err(RunError::Host(format!("`{}` sent an unexpected frame for seq {seq}: {other:?}", node.app_id))),
            }
        };

        expect_done(set_policy_seq, reply_to(set_policy_seq)?)?;
        expect_done(load_config_seq, reply_to(load_config_seq)?)?;
        expect_done(load_document_seq, reply_to(load_document_seq)?)?;
        for this_seq in &media_in_seqs {
            expect_done(*this_seq, reply_to(*this_seq)?)?;
        }

        let mut outputs = BTreeMap::new();
        for (port_id, media_out_seq, fingerprint_seq) in &output_seqs {
            let media = match reply_to(*media_out_seq)? {
                AppFrame::Media { descriptor, data, .. } => media_from_artifact(descriptor, data.clone(), self.blob_store.as_ref())?,
                AppFrame::Error { fault, report, .. } => return Err(RunError::Host(dispatch_error_message(&node.app_id, &format!("failed to produce media on `{port_id}`"), fault, report))),
                other => return Err(RunError::Host(format!("`{}` sent an unexpected frame for media-out `{port_id}`: {other:?}", node.app_id))),
            };
            let fingerprint = match reply_to(*fingerprint_seq)? {
                AppFrame::MediaFingerprint { fingerprint, .. } => decode_fingerprint_wire(fingerprint)?,
                AppFrame::Error { fault, report, .. } => return Err(RunError::Host(dispatch_error_message(&node.app_id, &format!("failed to fingerprint `{port_id}`"), fault, report))),
                other => return Err(RunError::Host(format!("`{}` sent an unexpected frame for media-fingerprint `{port_id}`: {other:?}", node.app_id))),
            };
            outputs.insert(port_id.clone(), (media, fingerprint));
        }

        let mutated_document = match reply_to(read_artifact_seq)? {
            AppFrame::Document { pack, spr, .. } => (pack.clone(), spr.clone()),
            AppFrame::Error { fault, report, .. } => return Err(RunError::Host(dispatch_error_message(&node.app_id, "failed to read its document", fault, report))),
            other => return Err(RunError::Host(format!("`{}` sent an unexpected frame reading its document: {other:?}", node.app_id))),
        };

        let mutated_config = match reply_to(read_config_seq)? {
            AppFrame::Config { pack, spr, .. } => (pack.clone(), spr.clone()),
            AppFrame::Error { fault, report, .. } => return Err(RunError::Host(dispatch_error_message(&node.app_id, "failed to read its config", fault, report))),
            other => return Err(RunError::Host(format!("`{}` sent an unexpected frame reading its config: {other:?}", node.app_id))),
        };

        Ok((mutated_document, mutated_config, outputs))
    }

    /// 🕸️ Runs every dirty node in `graph`'s topological order, importing media across each edge
    /// (applying `convert_media` per edge's negotiated `contract` first). `documents`/`configs` stay
    /// READONLY for the whole call — a mutated document/config's bytes go into `sink` (keyed by node
    /// id), never back into these two source maps (see this crate's module doc, "non-destructive
    /// rework"). `sink.record` is called for every `NodeStarted`/`NodeFinished`; the caller owns
    /// emitting `Start` before and `Seal` after this call.
    pub async fn run(
        &mut self,
        graph: &Workflow,
        documents: &BTreeMap<String, (Vec<u8>, Vec<u8>)>,
        configs: &BTreeMap<String, (Vec<u8>, Vec<u8>)>,
        parameter_values: &[RunParameterValue],
        parameter_bindings: &[WorkflowParameterBinding],
        prior_node_records: &BTreeMap<String, RunNodeRecord>,
        cache: &mut dyn MediaCache,
        sink: &mut RunSink,
    ) -> Result<RunReport, RunError> {
        validate_edge_kinds(graph)?;
        let order = topological_order(graph)?;
        let node_by_id: HashMap<&str, &WorkflowNode> = graph.nodes.iter().map(|node| (node.id.as_str(), node)).collect();
        let mut incoming: HashMap<&str, Vec<&WorkflowEdge>> = HashMap::new();
        for edge in &graph.edges {
            incoming.entry(edge.target_node_id.as_str()).or_default().push(edge);
        }

        let mut report = RunReport::default();
        let mut live: HashMap<String, u32> = HashMap::new();
        // 🧷️ Progressively filled AS this run computes each node, in topological order — the within-run
        // counterpart to `prior_node_records` (which stays fixed: the prior SEALED run's ground truth).
        // A downstream node's input fingerprint must reflect what its upstream producer computed THIS
        // run, not last time — mirrors the old `RunState.nodes`'s dual role (both "prior run" and "just
        // computed this run"), split into two maps here on purpose so "prior sealed run" (read-only
        // memoization ground truth) and "this run's own progress" (write-as-we-go) can never be confused.
        let mut current_run_records: BTreeMap<String, RunNodeRecord> = BTreeMap::new();

        for node_id in &order {
            let node = *node_by_id.get(node_id.as_str()).ok_or_else(|| RunError::UnknownNode(node_id.clone()))?;
            let document = documents.get(&node.artifact_ref).cloned().unwrap_or_default();
            let config = configs.get(&node.config_ref).cloned().unwrap_or_default();
            let (document_fingerprint, config_fingerprint) = node_fingerprints(node_id, &document.1, &config.1, parameter_bindings, parameter_values);

            let mut input_fingerprints: BTreeMap<String, String> = BTreeMap::new();
            for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
                let fingerprint = current_run_records.get(&edge.source_node_id).and_then(|record| record.output_fingerprints.iter().find(|entry| entry.port_id == edge.source_port_id)).map(|entry| entry.fingerprint.clone()).unwrap_or_default();
                input_fingerprints.insert(edge.target_port_id.clone(), fingerprint);
            }

            let previous = prior_node_records.get(node_id.as_str());
            let dirty = match previous {
                None => true,
                Some(record) => {
                    let previous_inputs: BTreeMap<String, String> = record.input_fingerprints.iter().map(|entry| (entry.port_id.clone(), entry.fingerprint.clone())).collect();
                    record.document_fingerprint != document_fingerprint || previous_inputs != input_fingerprints || record.config_fingerprint != config_fingerprint
                }
            };

            if !dirty {
                report.clean.push(node_id.clone());
                // ⚡️ Cache hit: reuse the prior sealed run's own record verbatim (fingerprints/outputs),
                // just relabeled `CacheHit` — this node is never opened, never mutates anything, and its
                // outputs stay exactly what they already were.
                let previous_record = previous.expect("dirty=false implies a prior record exists").clone();
                let node_record = RunNodeRecord { status: RunNodeStatus::CacheHit, ..previous_record };
                current_run_records.insert(node_id.clone(), node_record.clone());
                sink.record(RunMutation::FinishRunNode(FinishRunNode { node_record })).await?;
                continue;
            }
            report.recomputed.push(node_id.clone());
            sink.record(RunMutation::StartRunNode(StartRunNode { node_id: node_id.clone() })).await?;

            let mut input_media: BTreeMap<String, Media> = BTreeMap::new();
            for edge in incoming.get(node_id.as_str()).into_iter().flatten() {
                let fingerprint = MediaFingerprint(input_fingerprints.get(&edge.target_port_id).cloned().unwrap_or_default());
                let media = match cache.get(&fingerprint) {
                    Some(media) => media,
                    None => {
                        // 🩹️ Defensive one-hop fallback (mirrors the pre-`AppChannelHost` runner's own
                        // behavior): a clean upstream node's output should already be in `cache` from a
                        // prior run; reaching here means it genuinely isn't (e.g. an evicted media
                        // cache dir) — recompute the source node directly, WITHOUT recursively
                        // resolving ITS OWN inputs (a clean node's inputs are, by definition, unchanged
                        // since it was last fully computed). Still fully readonly over `documents`/
                        // `configs`: this recompute's own mutated bytes are simply discarded (`_source_*`)
                        // rather than written anywhere, matching "run never writes over source" even in
                        // this fallback path.
                        let source_node = *node_by_id.get(edge.source_node_id.as_str()).ok_or_else(|| RunError::UnknownNode(edge.source_node_id.clone()))?;
                        let source_document = documents.get(&source_node.artifact_ref).cloned().unwrap_or_default();
                        let source_config = configs.get(&source_node.config_ref).cloned().unwrap_or_default();
                        let (_source_document, _source_config, source_outputs) = self.compute_node(&mut live, source_node, &source_document, &source_config, &BTreeMap::new()).await?;
                        let (media, _fresh_fingerprint) = source_outputs.get(&edge.source_port_id).cloned().ok_or_else(|| RunError::Host(format!("upstream node `{}` produced no output on port `{}`", edge.source_node_id, edge.source_port_id)))?;
                        cache.put(&fingerprint, &media);
                        media
                    }
                };
                let converted = convert_media(&edge.contract, media)?;
                input_media.insert(edge.target_port_id.clone(), converted);
            }

            let started_at = std::time::Instant::now();
            let (mutated_document, mutated_config, outputs) = self.compute_node(&mut live, node, &document, &config, &input_media).await?;
            let duration_ms = started_at.elapsed().as_secs_f64() * 1000.0;

            // 🔒️ THE non-destructive rework's write side: mutated document/config bytes go into `sink`'s
            // run-owned area (keyed by node id), never back into `documents`/`configs` — those stay
            // read-only source maps for the whole call.
            sink.write_node_artifact(node_id, mutated_document.0, mutated_document.1);
            sink.write_node_config(node_id, mutated_config.0, mutated_config.1);

            let mut output_fingerprints = Vec::new();
            let mut run_outputs = Vec::new();
            for (port_id, (media, fingerprint)) in &outputs {
                output_fingerprints.push(PortFingerprint { port_id: port_id.clone(), fingerprint: fingerprint.clone() });
                cache.put(&MediaFingerprint(fingerprint.clone()), media);
                // 📤️ The media cache is the one place this crate already persists exported output bytes
                // durably (`<space>/cache/media/<fingerprint>.json`) — `RunOutputArtifact`
                // points at that real, existing location rather than inventing a second one this wave.
                run_outputs.push(RunOutputArtifact { port_id: port_id.clone(), artifact_id: format!("cache/media/{fingerprint}"), path: format!("cache/media/{fingerprint}.json") });
            }
            let node_record = RunNodeRecord {
                node_id: node_id.clone(),
                status: RunNodeStatus::Computed,
                document_fingerprint,
                config_fingerprint,
                input_fingerprints: input_fingerprints.iter().map(|(port_id, fingerprint)| PortFingerprint { port_id: port_id.clone(), fingerprint: fingerprint.clone() }).collect(),
                output_fingerprints,
                outputs: run_outputs,
                duration_ms,
            };
            current_run_records.insert(node_id.clone(), node_record.clone());
            sink.record(RunMutation::FinishRunNode(FinishRunNode { node_record })).await?;
        }

        Ok(report)
    }
}
//#endregion 🔖️SpaceRunner

//#region 🔖️WasmtimeNodeHost
/// 🧩️ Native `AppChannelHost` over `semio-framework-plugin-host`'s `GuestRuntime` — the
/// `WasmPluginRuntime`-based synchronous-exchange design this doc used to describe is gone
/// (`📌️important.md`'s "Replace, never wrap" list); `world actor` exports only `reactor::poll`/
/// `jobs::{start-job,step-job,cancel-job}`/`checkpoint`/`describe`, no per-verb calls and no
/// `exchange` at all. Channel v12 (`📡️spr/🧵️channel/🦀️.rs`) already anticipates this:
/// "lifecycle now arrives through the reactor ABI's `Event::InstanceOpen`/`InstanceClose`" — `open`
/// now instantiates a real `GuestInstance` and submits `Event::InstanceOpen`; `exchange` submits the
/// batch through the retained command-page ingress owner and reads back `Effect::Respond`
/// effects correlated by `req.0 == seq`.
///
/// ✅️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (R1-native-manifest): `load_runtime_recursive` now
/// reads a real `PluginManifest` — via `PackageDescriptor` (packet E1's build-time-emitted
/// `🛂️.descriptor.semio`), NOT via a wasm call. `design-abi.md` §3's freshness test
/// (`descriptor_is_fresh`) already byte-compares each plugin's OWN native `describe()` build against
/// its committed `🛂️.descriptor.semio`, and this ticket's whole premise (2550 installed records, zero
/// runtime resources) forbids re-instantiating a component just to read its manifest — so
/// `descriptor_path_for_plugin` (populated from the registry's `cratePath`, see `📦️bin.rs`) is tried
/// FIRST: zero instantiations. Only a plugin with no committed descriptor yet (most of the 33 today
/// — packet D0 is still landing the emission plumbing) falls through to a hard, honest error; a live
/// `describe()` fallback is DESIGNED (this struct's own doc, `📓️terra-R1-report.md` §1) but not
/// wired here — it needs a `GuestRuntime::describe` seam on `🔌️plugin/🖥️host/🦀️.rs`, a
/// file two sibling packets (P1, T1) are live in right now (lease-requested, not edited).
///
/// 🚧️ **Still a documented gap**: `run_transaction`/`undo_transaction_group`'s `exec`/`plan`
/// closures have a DIFFERENT gap one layer up — full per-command reply correlation and
/// contributed-mutation planning over `execute_turn`'s effects is a real post-turn dispatch loop that
/// belongs with the kernel/scheduler (H1-H4/T1), not this packet — they compile and are correctly
/// typed, but always return a clearly-worded `TransactionError` today.
#[cfg(not(target_arch = "wasm32"))]
// 🔀️ `B: BlobStore + 'static = NoBlobStore` — same open-extension-point reasoning as `SpaceRunner`'s
// own `B` (see its doc comment). `blob_store` here is `#[allow(dead_code)]` (reserved, unused) but
// still a real caller-supplied `new()` parameter, so the same generic treatment applies.
pub struct WasmtimeNodeHost<B: BlobStore + 'static = NoBlobStore> {
    /// 🧬️ Compiles components and (for `load_runtime_recursive`'s own plugin-service instance only)
    /// instantiates them directly — see `kernel`'s own doc for why that ONE call site does not go
    /// through the full kernel-activation facade.
    guest_runtime: Arc<semio_framework_plugin_host::GuestRuntimes>,
    /// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the real kernel this
    /// host now drives instead of minting `RuntimeActorId`s ad hoc — see `semio-framework-os`'s
    /// `🎠️activation.rs` module doc. `open`'s real per-app-instance actor uses `kernel.activate`
    /// (mints the id, pins a shard, hands the `GuestInstance` to that shard's own `ShardExecutor`
    /// thread); `load_runtime_recursive`'s per-plugin router-registration instance uses only
    /// `kernel.kernel_mut().activate` (id-minting/bookkeeping, no shard handoff) because
    /// `PluginInstanceHandle` still calls `GuestRuntime::execute_turn` directly on its own
    /// `GuestInstance` — see that field's own doc for why those are genuinely two different shapes.
    kernel: NativeKernelRuntime,
    plugin_path_for_plugin: HashMap<String, PathBuf>,
    /// 📦️ `plugin_id -> its committed 🛂️.descriptor.semio path` (registry `cratePath`-derived, see
    /// `📦️bin.rs`'s `resolve_descriptor_paths`) — `load_runtime_recursive`'s FIRST, zero-instantiation
    /// manifest source (this struct's own doc). A plugin absent from this map, or whose file does not
    /// exist on disk yet, has no committed descriptor — `load_runtime_recursive` fails loudly for it
    /// rather than falling back to a live `describe()` call (not wired here — see this struct's doc).
    descriptor_path_for_plugin: HashMap<String, PathBuf>,
    /// 🧬️ Content-addressed compiled-component cache, one entry per plugin id — mirrors
    /// `SharedWasmtimeEngine`'s own on-disk `.cwasm` cache one level up (in-process, per-host-instance).
    compiled_for_plugin: HashMap<String, semio_framework_plugin_host::CompiledHandle>,
    /// 🗺️ Every plugin id this host has successfully loaded a manifest for — see this struct's own
    /// doc for why `load_runtime_recursive` cannot populate this yet.
    manifests: HashMap<String, PluginManifest>,
    /// 🚧️ Reserved for the future post-turn effect dispatcher (`Effect::BlobLoad`/`BlobWrite`) — the
    /// OLD synchronous `register_host_blob_store`/`HostState.blob_store` wiring is gone with
    /// `HostState` itself; nothing calls into this yet.
    #[allow(dead_code)]
    blob_store: Arc<B>,
    /// 🌉️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION (D3): shared
    /// across every plugin this host lazily loads, so any loaded plugin's routed `io-run`/`io-sniff`
    /// job dispatch can reach any OTHER loaded plugin's registry — see `IoRouter`'s own doc comment.
    io_router: Arc<semio_framework_plugin_host::IoRouter>,
    /// 🕸️ PLUGIN-DEPENDENCIES-ARTIFACT-CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS (W2-A): every loaded
    /// plugin's manifest, dependency-validated — `runtime_for` registers into this BEFORE a
    /// plugin's own routers, and only after every declared dependency has itself been loaded (see
    /// `load_runtime_recursive`).
    plugin_graph: Arc<semio_framework_plugin_host::PluginGraph>,
    /// 🎯️ Every loaded plugin's `contributor.list-artifact-mutations` roster, merged.
    mutation_router: Arc<semio_framework_plugin_host::ArtifactMutationRouter>,
    /// 💡️ Contributor-aware artifact inference router.
    inference_router: Arc<semio_framework_plugin_host::ArtifactInferenceRouter>,
    /// 🗺️ `ArtifactRef -> (plugin_id, instance_id, artifact_kind)`, populated by `open` at
    /// instantiate-app time.
    instance_directory: Arc<semio_framework_plugin_host::InstanceDirectory>,
    /// 🎯️ Drives contract §5 transactions over this host's own loaded instances — see
    /// `run_transaction`.
    transaction_coordinator: Arc<semio_framework_plugin_host::HostTransactionCoordinator>,
    /// 🚪️👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (W1-A): every loaded plugin's
    /// viewer/editor surfaces — see `resolve_open_artifact`/`set_default_app`/`clear_default_app`.
    app_router: Arc<semio_framework_plugin_host::AppRouter>,
    /// 🎚️ Folded `OpeningPreferences` snapshot — event-sourced (contract §4): every write goes
    /// through `apply_opening_config_mutation` on a typed `OpeningConfigMutation`, never a direct
    /// field mutation. Persisted local-only for now (no `ConfigStore`/disk binding wired yet — see
    /// `set_default_app`'s doc); a real multi-session deployment would fold this from a durable op
    /// log at boot instead of starting empty every run.
    opening_preferences: semio_framework_plugin_host::opening_config::OpeningPreferences,
    /// 🧬️ `open`'s real body mints the returned handle from this counter — see that method.
    next_handle: u32,
    /// ✅️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): `Kernel::activate`
    /// takes a caller-supplied `plugin_ordinal: u16`, one per DISTINCT plugin id (not one per
    /// activation — the SAME convention `🎯️targets/🧊️wgpu/🦀️.rs`'s own `kernel_runtime::
    /// plugin_ordinal` uses), so both `load_runtime_recursive`'s plugin-service actor and `open`'s
    /// per-app-instance actors for the SAME plugin share one ordinal. Replaces the old raw
    /// `next_actor_ordinal: u64` counter, which minted every actor id ad hoc outside the kernel.
    plugin_ordinals: HashMap<String, u16>,
    /// 🗺️ `handle -> (plugin_id, instance_id)` — the SAME addressing scheme `WasmPluginRuntime` used
    /// (one runtime hosting many `create_app`-minted instance ids); the live `GuestInstance` for
    /// `(plugin_id, instance_id)` lives in `guest_instances` below, since `design-abi.md §4`'s actor
    /// model gives each app instance its OWN `GuestInstance`/`RuntimeActorId` now, not a shared store.
    instances: HashMap<u32, (String, u32)>,
    /// ✅️ `handle -> ActorId` for every instance `open` has activated through `kernel` — `run_turn`'s
    /// own lookup key for `Kernel::submit`/`tick_and_dispatch`/`complete`.
    instance_actors: HashMap<u32, RuntimeActorId>,
    /// 🧬️ Not populated yet — `open`'s own `GuestInstance` is owned by its pinned `ShardExecutor`
    /// thread now (see `kernel`'s own doc), not held here; this map is reserved for a future
    /// suspend/resume/checkpoint path that needs to read a live instance's state back out.
    #[allow(dead_code)]
    guest_instances: HashMap<(String, u32), semio_framework_plugin_host::GuestInstance>,
    /// ⏱️ Monotonic milliseconds this host's own `Kernel::tick` calls are stamped with — same
    /// purity-respecting clock source as `🎯️targets/🧊️wgpu/🦀️.rs`'s `KernelThreadState::now_ms`
    /// (`Kernel` itself takes no clock, per `🎭️actor`'s own rule), incremented once per `run_turn`.
    now_ms: u64,
    /// 🔢️ `Envelope.seq` source for this host's own turns — distinct
    /// from `SpaceRunner::compute_node`'s own per-node `next_seq` closure (that one numbers
    /// `AppCommand`s within a single batched `exchange` call; this one numbers kernel envelopes).
    next_turn_seq: u64,
    retained_command_closes: semio_framework::kernel::CommandDriverRegistry<1>,
    rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry<1>,
}

/// ⛽️ One generous constant turn/instantiate budget, shared by `load_runtime_recursive`'s
/// plugin-service instantiate call and `run_turn`'s real per-node turns — until the DRR scheduler
/// threads a real per-lane one through (same honestly-flagged gap `🎯️targets/🧊️wgpu/🦀️.rs`'s own
/// `kernel_runtime::TURN_BUDGET` already documents on that consumer).
#[cfg(not(target_arch = "wasm32"))]
const NODE_TURN_BUDGET: semio_framework::kernel::Budget = semio_framework::kernel::Budget { fuel: 10_000_000, deadline_ms: 5_000, max_effects: 256, max_patch_bytes: 1 << 20, max_frames: 256 };

#[cfg(not(target_arch = "wasm32"))]
impl<B: BlobStore + 'static> WasmtimeNodeHost<B> {
    /// 🗺️ `plugin_path_for_plugin` maps a plugin id (`WorkflowNode::plugin_id`, the same id
    /// `PLUGIN_WASM_ARTIFACTS`' first tuple element names) to the compiled `.wasm` component path the
    /// dev shell build already produces under `framework/os/dev/plugin-modules/<plugin id>/`.
    /// `descriptor_path_for_plugin` maps the SAME plugin id to its committed `🛂️.descriptor.semio`
    /// (registry `cratePath`-derived — see `📦️bin.rs`'s `resolve_descriptor_paths`); absent entries
    /// simply have no committed descriptor yet (this struct's own doc).
    /// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): `async` now — it
    /// builds `kernel` (`NativeKernelRuntime::new`, itself `async` since `Kernel::new`/`Kernel::
    /// activate` are, per the crate-wide universal-async decree, O1). `shard_count: 1`: this host's
    /// own contract is strictly sequential, one node/turn at a time (`AppChannelHost`'s own doc:
    /// "the guest contract is still exactly one turn at a time per instance ... a caller that wants
    /// concurrency must open distinct instances, never overlap two turns on one" — and
    /// `SpaceRunner::run`'s node loop never opens two instances concurrently either), so a second
    /// shard thread would sit idle. The type stays general (same `shard_count` parameter
    /// `ParallelRuntime::new` takes) for a future caller that does want more.
    pub async fn new(plugin_path_for_plugin: HashMap<String, PathBuf>, descriptor_path_for_plugin: HashMap<String, PathBuf>, blob_store: Arc<B>) -> Self {
        let guest_runtime = Arc::new(semio_framework_plugin_host::GuestRuntimes::Owned(semio_framework_plugin_host::OwnedRuntime::new()));
        let kernel = NativeKernelRuntime::new(guest_runtime.clone(), 1, 0, 64).await;
        Self {
            guest_runtime,
            kernel,
            plugin_path_for_plugin,
            descriptor_path_for_plugin,
            compiled_for_plugin: HashMap::new(),
            manifests: HashMap::new(),
            blob_store,
            io_router: Arc::new(semio_framework_plugin_host::IoRouter::new()),
            plugin_graph: Arc::new(semio_framework_plugin_host::PluginGraph::new()),
            mutation_router: Arc::new(semio_framework_plugin_host::ArtifactMutationRouter::new()),
            inference_router: Arc::new(semio_framework_plugin_host::ArtifactInferenceRouter::new()),
            instance_directory: Arc::new(semio_framework_plugin_host::InstanceDirectory::new()),
            transaction_coordinator: Arc::new(semio_framework_plugin_host::HostTransactionCoordinator::new()),
            app_router: Arc::new(semio_framework_plugin_host::AppRouter::new()),
            opening_preferences: semio_framework_plugin_host::opening_config::OpeningPreferences::default(),
            next_handle: 1,
            plugin_ordinals: HashMap::new(),
            instances: HashMap::new(),
            instance_actors: HashMap::new(),
            guest_instances: HashMap::new(),
            now_ms: 0,
            next_turn_seq: 1,
            retained_command_closes: semio_framework::kernel::CommandDriverRegistry::new(),
            rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry::new(),
        }
    }

    /// 🔢️ One ordinal per DISTINCT plugin id — see `plugin_ordinals`' own doc.
    fn plugin_ordinal(&mut self, plugin_id: &str) -> u16 {
        let next = self.plugin_ordinals.len() as u16;
        *self.plugin_ordinals.entry(plugin_id.to_string()).or_insert(next)
    }

    /// 🎬️ `Envelope` sequence source — see `next_turn_seq`'s own field doc.
    fn take_turn_seq(&mut self) -> u64 {
        let seq = self.next_turn_seq;
        self.next_turn_seq += 1;
        seq
    }

    /// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): drives one batch of
    /// `events` for `actor`/`instance` genuinely through the kernel — `Kernel::submit` → tick-and-
    /// dispatch to `actor`'s own pinned shard → wait for its `ShardOutcome` → `Kernel::complete` —
    /// mirroring `🎯️targets/🧊️wgpu/🦀️.rs`'s own `KernelThreadState::run_turn` almost line for
    /// line (that file's own doc explains each step; this is the same mechanism, minus UI-surface
    /// reconciliation, which `run` has no use for). `RUN_TURN_OUTCOME_TIMEOUT`/`NODE_TURN_BUDGET` are
    /// this host's own constants, same values `load_runtime_recursive` already used for its own
    /// plugin-service instantiate budget, now shared with the real per-node turn path too.
    async fn run_turn(&mut self, actor: RuntimeActorId, events: Vec<Event>) -> Result<semio_framework::kernel::TurnResult, RunError> {
        const RUN_TURN_OUTCOME_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
        let node_budget = actor_budget_from_turn_budget(NODE_TURN_BUDGET, Lane::Background).await;
        let mut envelopes = Vec::with_capacity(events.len().max(1));
        for event in &events {
            let seq = self.take_turn_seq();
            envelopes.push(Envelope { to: actor, from: Origin::Kernel, lane: Lane::Background, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload: Payload::Event { bytes: serde_json::to_vec(event).map_err(RunError::Serde)? } });
        }
        for envelope in &envelopes {
            if !matches!(self.kernel.submit(envelope).await, Backpressure::Accept) {
                return Err(RunError::Host(format!("kernel: run_turn submit for actor {} was not Accept-ed (mailbox pressure) — a single sequential run should never see this", actor.0)));
            }
        }
        let mut turn_result: Option<semio_framework::kernel::TurnResult> = None;
        let mut fault: Option<String> = None;
        loop {
            self.now_ms += 1;
            let decision = self.kernel.tick_and_dispatch(self.now_ms, move |_actor| node_budget).await;
            if decision.run.is_empty() {
                break;
            }
            let outcomes = self.kernel.wait_for_outcomes(decision.run.len(), RUN_TURN_OUTCOME_TIMEOUT);
            if outcomes.len() < decision.run.len() {
                return Err(RunError::Host("kernel: shard produced no outcome for this turn (timed out)".to_string()));
            }
            for outcome in &outcomes {
                match outcome {
                    semio_framework_plugin_host::shard::ShardOutcome::Turn { actor: reported, result } => {
                        let _ = self.kernel.complete(RuntimeActorId(*reported), result, 0, 0, self.now_ms).await;
                        if *reported == actor.0 {
                            let status = match &result.status {
                                semio_framework_actor::TurnStatus::Idle => semio_framework::kernel::TurnStatus::Idle,
                                semio_framework_actor::TurnStatus::MoreWork => semio_framework::kernel::TurnStatus::MoreWork,
                                semio_framework_actor::TurnStatus::CheckpointReady { checkpoint } => semio_framework::kernel::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() },
                                semio_framework_actor::TurnStatus::Faulted { detail } => semio_framework::kernel::TurnStatus::Faulted(detail.clone()),
                                status => return Err(RunError::Host(format!("unexpected reactor turn status: {status:?}"))),
                            };
                            turn_result = Some(semio_framework::kernel::TurnResult {
                                ui_patches: semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&result.ui_patches, *reported)
                                    .map_err(|error| RunError::Host(format!("kernel: decode ui patch transport: {error}")))?
                                    .take_owner()
                                    .map_err(|_| RunError::Host("kernel: turn patch transport lease lost its exact owner".to_string()))?,
                                effects: serde_json::from_slice(&result.effects).map_err(RunError::Serde)?,
                                presence: Vec::new(),
                                next_wake: result.next_wake,
                                status,
                                fuel_used: result.usage.fuel,
                                command_ingress: serde_json::from_slice(&result.command_ingress).map_err(RunError::Serde)?,
                            });
                        } else {
                            let _ = semio_framework::kernel::close_ui_turn_patch_transport_session_one(*reported);
                            let _ = semio_framework::kernel::close_ui_turn_patch_transport_one();
                        }
                    }
                    semio_framework_plugin_host::shard::ShardOutcome::Fault { actor: reported, message } => {
                        let faulted = semio_framework::kernel::TurnResult {
                            ui_patches: semio_framework::kernel::UiTurnPatches::default(),
                            effects: Vec::new(),
                            presence: Vec::new(),
                            next_wake: None,
                            status: semio_framework::kernel::TurnStatus::Faulted(message.clone().into_bytes()),
                            fuel_used: 0,
                            command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
                        };
                        let _ = self.kernel.complete(RuntimeActorId(*reported), &faulted, 0, 0, self.now_ms).await;
                        if *reported == actor.0 {
                            fault = Some(message.clone());
                        }
                    }
                    // 🚧️ `run`'s own turns never send `Suspend`/`Resume`/`Cancel` payloads — any
                    // other `ShardOutcome` variant reaching here belongs to a DIFFERENT actor `Kernel::
                    // tick` happened to grant in the same call (DRR is global even with one shard),
                    // same "silently ignored, never aborts an otherwise-successful turn" policy the
                    // wgpu target's own `run_turn` documents for the identical reason.
                    _ => {}
                }
            }
        }
        if let Some(message) = fault {
            return Err(RunError::Host(message));
        }
        match turn_result {
            Some(result) => Ok(result),
            None => Err(RunError::Host("kernel: shard produced no outcome for this turn".to_string())),
        }
    }

    /// 📊️ `(plugins loaded so far, distinct route keys)` — surfaced by the dev-boot smoke test so a
    /// zero-plugin/zero-key router (the router silently doing nothing) is visible, not just "boot
    /// didn't crash." `IoRouter::stats` only errors on a poisoned lock — a diagnostic-only stat line
    /// degrades to `(0, 0)` rather than panicking a whole run over it.
    pub fn io_router_stats(&self) -> (usize, usize) {
        self.io_router.stats().unwrap_or((0, 0))
    }

    pub fn plugin_graph(&self) -> &semio_framework_plugin_host::PluginGraph {
        &self.plugin_graph
    }

    pub fn mutation_router(&self) -> &semio_framework_plugin_host::ArtifactMutationRouter {
        &self.mutation_router
    }

    pub fn inference_router(&self) -> &semio_framework_plugin_host::ArtifactInferenceRouter {
        &self.inference_router
    }

    pub fn instance_directory(&self) -> &semio_framework_plugin_host::InstanceDirectory {
        &self.instance_directory
    }

    pub fn app_router(&self) -> &semio_framework_plugin_host::AppRouter {
        &self.app_router
    }

    async fn manifest_for(&mut self, plugin_id: &str) -> Result<&PluginManifest, RunError> {
        let mut loading = Vec::new();
        self.load_runtime_recursive(plugin_id, &mut loading).await?;
        Ok(self.manifests.get(plugin_id).expect("just loaded or already present"))
    }

    /// 🕸️ Scout-2 §3's binding requirement: a dependency is loaded (recursively) before its
    /// dependent, and `PluginGraph::register` validates the whole graph (missing dependency,
    /// version mismatch, cycle — contract §4 rule 5) before this plugin's own routers see it.
    /// `loading` guards against a cycle that only becomes visible once a manifest is actually read
    /// (unlike `PluginGraph`'s own cycle check, which only fires once every member is registered).
    ///
    /// ✅️ R1-native-manifest: reads `plugin_id`'s manifest from `descriptor_path_for_plugin`'s
    /// committed `🛂️.descriptor.semio` (packet E1's `describe_component` emitter output, decoded the
    /// SAME way `🔌️plugin/📇️describe/📦️packages/🦀️rust/🦀️.rs`'s own `describe_component` decodes
    /// a live `describe()` return: `store::pack_rt::decode_wire_value` then `dsl::from_dsl_value`) —
    /// zero wasm instantiations (this struct's own doc). No live-`describe()` fallback yet — that
    /// needs a `GuestRuntime` seam on `🔌️plugin/🖥️host/🦀️.rs`, out of `path_scope` this
    /// packet (lease-requested, see `📓️terra-R1-report.md`) — so a plugin with no committed
    /// descriptor fails loudly here instead of silently instantiating to compensate.
    fn read_committed_descriptor(&self, plugin_id: &str) -> Result<PackageDescriptor, RunError> {
        let Some(path) = self.descriptor_path_for_plugin.get(plugin_id) else {
            return Err(RunError::Host(format!(
                "plugin `{plugin_id}` has no committed `🛂️.descriptor.semio` registered (regenerate the registry: `bun nx run @semio-tech/plugin-registry:generate`) and no live `describe()` fallback is wired yet (see `WasmtimeNodeHost`'s own doc)"
            )));
        };
        if !path.is_file() {
            return Err(RunError::Host(format!(
                "plugin `{plugin_id}` has no committed descriptor at `{}` yet (packet D0 is still landing per-plugin emission) and no live `describe()` fallback is wired yet (see `WasmtimeNodeHost`'s own doc)",
                path.display()
            )));
        }
        let bytes = std::fs::read(path).map_err(|error| RunError::Io { path: path.clone(), source: error })?;
        let decoded = store::pack_rt::decode_wire_value(&bytes).map_err(|error| RunError::Host(format!("plugin `{plugin_id}`: decoding `{}` as a pack: {error}", path.display())))?;
        from_dsl_value(decoded).map_err(|error| RunError::Host(format!("plugin `{plugin_id}`: decoding `{}` as a PackageDescriptor: {error}", path.display())))
    }

    /// ✅️ R1-native-manifest: the real recursive-load shape the OLD gap's own doc comment spelled
    /// out — compile the component, decode its committed descriptor (`read_committed_descriptor`,
    /// zero instantiations), recurse over `manifest.dependencies` BEFORE this plugin's own routers
    /// see it (contract §4 rule 5's load-order requirement), instantiate ONE `GuestInstance` for
    /// post-turn job dispatch (`PluginInstanceHandle`'s own doc — `io-run`/`io-sniff`/`infer`, never
    /// this turn's own `execute_turn`), register with every router in the SAME order the OLD
    /// `WasmPluginRuntime`-backed version did, then the SAME `owned_surface_gaps` hard-gate.
    async fn load_runtime_recursive(&mut self, plugin_id: &str, loading: &mut Vec<String>) -> Result<(), RunError> {
        if self.manifests.contains_key(plugin_id) {
            return Ok(());
        }
        if loading.contains(&plugin_id.to_string()) {
            loading.push(plugin_id.to_string());
            return Err(RunError::Host(format!("plugin dependency cycle while loading: {}", loading.join(" -> "))));
        }
        loading.push(plugin_id.to_string());
        let path = self.plugin_path_for_plugin.get(plugin_id).cloned().ok_or_else(|| RunError::Host(format!("no compiled program registered for plugin `{plugin_id}`")))?;
        if !self.compiled_for_plugin.contains_key(plugin_id) {
            let bytes = std::fs::read(&path).map_err(|error| RunError::Io { path: path.clone(), source: error })?;
            let package = semio_framework_plugin_host::PackageRef {
                package: semio_framework_plugin_host::PackageId(plugin_id.to_string()),
                hash: semio_framework_plugin_host::PackageHash(framework_hash::hash_bytes(&bytes).into_bytes().try_into().unwrap_or([0u8; 32])),
            };
            let compiled = self.guest_runtime.compile(&package, &bytes).map_err(|error| RunError::Host(error.to_string()))?;
            self.compiled_for_plugin.insert(plugin_id.to_string(), compiled);
        }

        let descriptor = self.read_committed_descriptor(plugin_id)?;
        let manifest = descriptor.manifest;

        for dependency in &manifest.dependencies {
            Box::pin(self.load_runtime_recursive(&dependency.plugin_id, loading)).await?;
        }

        let compiled = self.compiled_for_plugin.get(plugin_id).cloned().expect("just compiled or already present above");
        // 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the ad hoc
        // `RuntimeActorId::new(0, 0, self.next_actor_ordinal as u32, 0)` counter this used to mint
        // from is GONE — `Kernel::activate` mints the id and pins a shard for real now (this is the
        // literal "🏃️run ... mints its own RuntimeActorIds" bypass this packet's brief names).
        // `kernel.kernel_mut().activate` (NOT the full `kernel.activate` facade below) deliberately:
        // this instance's `GuestInstance` stays HELD BY `PluginInstanceHandle`, which calls
        // `GuestRuntime::execute_turn` on it directly for post-turn job dispatch
        // (`io_router`/`inference_router`) — routing that through `ShardExecutor` ownership (what the
        // full facade does) would take the instance away from `PluginInstanceHandle` entirely, a much
        // bigger change than this packet's own `path_scope` (`🔌️plugin/🖥️host/**`'s `io_router`
        // internals are not owned here) — see `kernel`'s own struct-field doc.
        let plugin_ordinal = self.plugin_ordinal(plugin_id);
        let kind = ActorKind::PluginApp { plugin: PackageId(plugin_id.to_string()), app_id: "<plugin-runtime>".to_string(), instance_id: 0 };
        let actor = self.kernel.kernel_mut().activate(PackageId(plugin_id.to_string()), plugin_ordinal, kind, Lane::Background, None, ActivationEvent::Manual).await;
        // ⚖️ `GuestRuntime::instantiate`'s `Budget` is `semio_framework::kernel::Budget` (the
        // `reactor::poll`-shaped one — fuel/deadline_ms/max_effects/max_patch_bytes/max_frames), NOT
        // `semio_framework_actor::Budget` (a different, later-wave shape by the same name — see
        // `🎠️kernel/🦀️.rs` vs `🎭️actor/🦀️.rs`, both named `Budget`). Literal
        // mirrors the sibling MCP gateway's own one-off-instantiate budget
        // (`🌉️mcp/🏠️workspace/🦀️.rs`'s `activate_plugin_instance`), the closest real
        // precedent for "compile once, instantiate once, no live turn loop yet".
        let instance = self.guest_runtime.instantiate(&compiled, actor, &[], &NODE_TURN_BUDGET).map_err(|error| RunError::Host(format!("plugin `{plugin_id}`: instantiate: {error}")))?;
        let handle = Arc::new(semio_framework_plugin_host::PluginInstanceHandle::new(actor, Arc::clone(&self.guest_runtime), instance));

        let artifact_dialect_entries: Vec<_> = descriptor.contributions.composer_entries.iter().map(|entry| (entry.writes.clone(), entry.reads.clone())).collect();
        // 🕳️ `ContributionSet.io_entries` (owner/counterpart/direction) carries no `fidelity`/
        // `sniffs` — `io_schema::IoEntryDescriptor`'s fields (a DIFFERENT type of the same name in
        // `🚪️io/🧬️schema/🦀️component.rs`) the descriptor schema does not emit yet. `&[]` here is
        // honest, not a shortcut: fabricating a fidelity/sniffs value would be a real correctness
        // lie, and `note`'s own committed descriptor has zero `io_entries` either way, so this gap
        // is invisible on today's only fully-wired smoke path. A future packet must either widen
        // `ContributionSet.io_entries` with those two fields at emission time (E1/A2) or resolve
        // them from `IoFidelityDeclaration` elsewhere before this can be more than `&[]`.
        self.io_router.register_plugin(plugin_id, Arc::clone(&handle), &artifact_dialect_entries, &[]).map_err(|error| RunError::Host(error.to_string()))?;

        let mut mutation_roster: Vec<semio_framework_plugin_host::HostMutationRosterEntry> = descriptor
            .contributions
            .mutation_services
            .iter()
            .map(|service| semio_framework_plugin_host::HostMutationRosterEntry {
                mutation_id: service.mutation_id.clone(),
                verb: service.semantics.verb.clone(),
                entity: service.semantics.entity.clone(),
                kind: service.semantics.kind.clone(),
                record: service.semantics.record.clone(),
                contributor: None,
                artifact_kind: None,
            })
            .collect();
        for contribution in &descriptor.contributions.artifact_contributions {
            for mutation in &contribution.mutations {
                mutation_roster.push(semio_framework_plugin_host::HostMutationRosterEntry {
                    mutation_id: mutation.mutation_id.clone(),
                    verb: mutation.semantics.verb.clone(),
                    entity: mutation.semantics.entity.clone(),
                    kind: mutation.semantics.kind.clone(),
                    record: mutation.semantics.record.clone(),
                    contributor: Some(plugin_id.to_string()),
                    artifact_kind: Some(contribution.artifact_kind.clone()),
                });
            }
        }
        self.mutation_router.register_roster_with_runtime(plugin_id, &manifest.dependencies, Arc::clone(&handle), mutation_roster).map_err(|error| RunError::Host(error.to_string()))?;

        let inference_roster: Vec<serde_json::Value> = descriptor
            .contributions
            .inference_services
            .iter()
            .chain(descriptor.contributions.artifact_contributions.iter().flat_map(|contribution| contribution.inferences.iter()))
            .map(serde_json::to_value)
            .collect::<Result<Vec<_>, _>>()
            .map_err(RunError::Serde)?;
        let inference_wire_bytes = serde_json::to_vec(&inference_roster).map_err(RunError::Serde)?;
        self.inference_router.register_plugin(plugin_id, &manifest.dependencies, Arc::clone(&handle), &inference_wire_bytes).map_err(|error| RunError::Host(error.to_string()))?;

        self.plugin_graph.register(manifest.clone()).map_err(|error| RunError::Host(error.to_string()))?;
        self.app_router.register_manifest(plugin_id, &manifest).map_err(|fault| RunError::Host(format!("{}: {}", fault.code.0, fault.message)))?;

        let gaps = self.app_router.owned_surface_gaps();
        if !gaps.is_empty() {
            return Err(RunError::Host(format!("plugin `{plugin_id}` loaded but left {} owned-surface gap(s): {}", gaps.len(), gaps.iter().map(|fault| format!("{}: {}", fault.code.0, fault.message)).collect::<Vec<_>>().join("; "))));
        }

        self.manifests.insert(plugin_id.to_string(), manifest);
        Ok(())
    }

    /// ✂️ Contract §4.5: refused while any OTHER loaded plugin still depends on `plugin_id`.
    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), RunError> {
        self.plugin_graph.guard_unload(plugin_id).map_err(|error| RunError::Host(error.to_string()))?;
        self.io_router.unregister_plugin(plugin_id).map_err(|error| RunError::Host(error.to_string()))?;
        self.mutation_router.unregister_plugin(plugin_id).map_err(|error| RunError::Host(error.to_string()))?;
        self.inference_router.unregister_plugin(plugin_id).map_err(|error| RunError::Host(error.to_string()))?;
        self.app_router.unregister_plugin(plugin_id);
        self.plugin_graph.unregister(plugin_id).map_err(|error| RunError::Host(error.to_string()))?;
        self.manifests.remove(plugin_id);
        self.compiled_for_plugin.remove(plugin_id);
        Ok(())
    }

    /// 🩹️ Contract §4.5: re-validates the WHOLE graph with `plugin_id` replaced (so a version bump
    /// that would break a live dependent is rejected before anything swaps), then atomically replaces
    /// the registered manifest and every router entry for it.
    ///
    /// 🚧️ Same gap as `load_runtime_recursive` — always fails until a manifest decoder exists.
    pub fn hot_reload_plugin(&mut self, plugin_id: &str) -> Result<(), RunError> {
        self.manifests.remove(plugin_id);
        self.compiled_for_plugin.remove(plugin_id);
        self.manifest_for(plugin_id).map(|_| ())
    }

    /// 🎯️ Contract §5 end to end: `initiator_handle` must already have proposed (its own
    /// `dispatch_emit` stashed a `TransactionProposalDraft` instead of applying — the caller drains
    /// it via `exchange` and passes `local_ops`/`description`/`foreign` straight through here).
    ///
    /// 🚧️ `exec`/`plan` always return `TransactionError` today — this struct's own doc comment
    /// explains why (a real per-command reply correlation / contributed-mutation-plan dispatch over
    /// `GuestRuntime::execute_turn`'s effects needs a post-turn dispatch loop this packet does not
    /// build). Kept as a real, correctly-typed call into `HostTransactionCoordinator::run_transaction`
    /// (unlike deleting the method) so `TransactionCoordinator`'s own resolution/gating logic — which
    /// IS real and IS tested (`host_transaction_coordinator_tests`) — stays reachable from here the
    /// moment the two closures below get real bodies.
    pub fn run_transaction(&self, initiator_handle: u32, local_ops: Vec<Vec<u8>>, description: String, foreign: Vec<protocol::ForeignStep>) -> Result<semio_framework_plugin_host::TransactionOutcome, semio_framework_plugin_host::TransactionError> {
        let (plugin_id, instance_id) = self.instances.get(&initiator_handle).cloned().ok_or_else(|| semio_framework_plugin_host::TransactionError::rejected("transaction.unknown-target", format!("unknown node handle {initiator_handle}")))?;
        let initiator = semio_framework_plugin_host::TransactionMember { plugin_id, instance_id };
        self.transaction_coordinator.run_transaction(
            &self.instance_directory,
            &self.mutation_router,
            |plugin_id, _instance_id, _command| {
                Err(semio_framework_plugin_host::TransactionError::rejected(
                    "transaction.not-wired",
                    format!("plugin `{plugin_id}`: exchange has no world-actor equivalent yet — needs a post-turn effect-dispatch loop over GuestRuntime::execute_turn, not built in this packet (see 📓️terra-B1b-host-complete-report.md)"),
                ))
            },
            |contributor, _artifact_kind, _mutation_id, _member, _payload| {
                Err(semio_framework_plugin_host::TransactionError::rejected("transaction.not-wired", format!("contributor `{contributor}`: artifact-mutation-plan has no world-actor equivalent yet — see 📓️terra-B1b-host-complete-report.md")))
            },
            initiator,
            local_ops,
            description,
            foreign,
        )
    }

    /// 🚧️ Same gap as `run_transaction` — see its own doc comment.
    pub fn undo_transaction_group(&self, members: &[semio_framework_plugin_host::TransactionMember], group_id: &str) {
        self.transaction_coordinator.undo_group(
            |plugin_id, _instance_id, _command| {
                Err(semio_framework_plugin_host::TransactionError::rejected("transaction.not-wired", format!("plugin `{plugin_id}`: exchange has no world-actor equivalent yet — see 📓️terra-B1b-host-complete-report.md")))
            },
            members,
            group_id,
        )
    }

    //#region 🔖️OpeningCommands
    /// 🚪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (W1-A), contract §3: the host-level
    /// implementation of `os.open-artifact`. Empty `plugin_id`/`app_id` means "ask `OpeningResolver`";
    /// both set means the caller already knows which surface it wants (validated, echoed back
    /// unchanged — the SDK-side `relay_open_artifact` in `semio-framework-plugin`'s
    /// `OpeningCommandRelay` region runs the SAME shape of validation before the shell relay; this is
    /// the host's own independent check, reusing its exact `opening.*` fault-code vocabulary so a
    /// caller sees one consistent error grammar on either side of the wire). Returns
    /// `semio_framework::Fault` directly (not `RunError`) so the frozen fault codes (`surface.*` from
    /// `OpeningResolver`, `opening.*` here) reach the caller verbatim instead of being flattened into
    /// an opaque string — `exchange` below re-encodes it onto the wire unchanged. This is a HOST-LEVEL
    /// operation (which plugin/app to instantiate) rather than an already-open-instance operation, so
    /// — like `run_transaction` above — it is a direct method rather than only reachable through
    /// `exchange`'s per-`node` interface; `exchange` also intercepts the wire `AppCommand::OpenArtifact`
    /// and delegates here.
    pub fn resolve_open_artifact(&self, artifact_ref: &str, role_wire: u8, plugin_id: &str, app_id: &str) -> Result<semio_framework::AppRef, semio_framework::Fault> {
        let role = opening_role_from_wire(role_wire)?;
        let (dialect, artifact_role) = semio_framework::parse_surface_app_id(artifact_ref).map_err(|error| semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("opening.invalid-artifact-ref"), error))?;
        if role != artifact_role {
            return Err(semio_framework::Fault::new(
                semio_framework::FaultOrigin::Os,
                semio_framework::FaultCode::new("opening.role-mismatch"),
                format!("artifact ref {artifact_ref} declares {} but the command declares {}", artifact_role.as_str(), role.as_str()),
            ));
        }
        match (plugin_id.is_empty(), app_id.is_empty()) {
            (true, true) => {
                let user_default = self.opening_preferences.defaults.iter().find(|entry| entry.dialect == dialect && entry.role == role).map(|entry| &entry.app);
                semio_framework_plugin_host::OpeningResolver::resolve(&self.app_router, &dialect, role, user_default)
            }
            (false, false) => Ok(semio_framework::AppRef { plugin_id: plugin_id.to_string(), app_id: app_id.to_string() }),
            _ => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("opening.partial-app-ref"), "plugin_id and app_id must either both be empty or both be set")),
        }
    }

    /// ✏️ Contract §3's `os.set-default-viewer`/`os.set-default-editor`: pins `app` as the default
    /// `role` surface for `(artifact_kind, standard, subset)` — refuses to pin a surface the
    /// `AppRouter` does not actually know about (a stale/typo'd `AppRef` would otherwise silently
    /// pin to nothing, since `OpeningResolver` already falls through past any default not present in
    /// the router; refusing up front is a better failure than a default that quietly never applies).
    /// Applies through the SAME event-sourced `OpeningConfigMutation`/`apply_opening_config_mutation`
    /// the schema facet's own `MutationDiff` impl defines (contract §4: "never a mutable map") — never
    /// a direct field write onto `self.opening_preferences`.
    pub fn set_default_app(&mut self, artifact_kind: &str, standard: &str, subset: &str, role_wire: u8, plugin_id: &str, app_id: &str) -> Result<(), semio_framework::Fault> {
        let role = opening_role_from_wire(role_wire)?;
        let dialect = semio_framework::ArtifactDialect { artifact_kind: artifact_kind.to_string(), standard: standard.to_string(), subset: subset.to_string() };
        let app = semio_framework::AppRef { plugin_id: plugin_id.to_string(), app_id: app_id.to_string() };
        if !self.app_router.surfaces_for(&dialect, role).contains(&app) {
            return Err(semio_framework::Fault::new(
                semio_framework::FaultOrigin::Os,
                semio_framework::FaultCode::new("opening.invalid-app-ref"),
                format!("`{}` is not a registered {} surface for `{}`", app.app_id, role.as_str(), dialect.to_coordinate()),
            ));
        }
        let mutation = semio_framework_plugin_host::opening_config::mutations::set_default_app::set_default_app(dialect, role, app);
        semio_framework_plugin_host::opening_config::apply_opening_config_mutation(&mut self.opening_preferences, &mutation).map_err(|error| semio_framework::Fault::from(error.to_string()))?;
        Ok(())
    }

    /// 🧹 Contract §3's `os.clear-default-app`: unpins whatever default is set for
    /// `(artifact_kind, standard, subset, role)`, if any — infallible past role validation (clearing
    /// an already-absent default is a no-op, matching `ClearDefaultApp`'s own diff), same
    /// event-sourced apply path as `set_default_app`.
    pub fn clear_default_app(&mut self, artifact_kind: &str, standard: &str, subset: &str, role_wire: u8) -> Result<(), semio_framework::Fault> {
        let role = opening_role_from_wire(role_wire)?;
        let dialect = semio_framework::ArtifactDialect { artifact_kind: artifact_kind.to_string(), standard: standard.to_string(), subset: subset.to_string() };
        let mutation = semio_framework_plugin_host::opening_config::mutations::clear_default_app::clear_default_app(dialect, role);
        semio_framework_plugin_host::opening_config::apply_opening_config_mutation(&mut self.opening_preferences, &mutation).map_err(|error| semio_framework::Fault::from(error.to_string()))?;
        Ok(())
    }
    //#endregion 🔖️OpeningCommands
}

/// 🔤️ Wire `role: u8` -> `AppRole` (`0` Viewer, `1` Editor, contract §3) — the host-side twin of the
/// SDK guest's `opening_role` in `semio-framework-plugin`'s `OpeningCommandRelay` region; same fault
/// code (`opening.invalid-role`) on both sides of the wire.
fn opening_role_from_wire(role_wire: u8) -> Result<semio_framework::AppRole, semio_framework::Fault> {
    match role_wire {
        0 => Ok(semio_framework::AppRole::Viewer),
        1 => Ok(semio_framework::AppRole::Editor),
        other => Err(semio_framework::Fault::new(semio_framework::FaultOrigin::Os, semio_framework::FaultCode::new("opening.invalid-role"), format!("opening role {other} must be 0 (viewer) or 1 (editor)"))),
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<B: BlobStore + 'static> AppChannelHost for WasmtimeNodeHost<B> {
    /// ✅️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the real body —
    /// `manifest_for` (real for any plugin with a committed descriptor since R1-native-manifest;
    /// this method's own doc used to say "manifest_for always errors today", which stopped being
    /// true once that packet landed), `kernel.activate` (mints the `ActorId`, pins a shard,
    /// instantiates, hands the `GuestInstance` to that shard's own `ShardExecutor` thread), then
    /// `run_turn` submits `Event::InstanceOpen` through the kernel (channel v12's own doc:
    /// "lifecycle now arrives through ... `Event::InstanceOpen`") and drives it to completion exactly
    /// like a real per-node turn — this is the SAME `run_turn` `exchange` uses below, not a separate
    /// path. `artifact_ref` binding into `instance_directory` (this struct's own field doc) is a
    /// SEPARATE, still-unwired concern this packet does not touch (out of scope: it needs the
    /// manifest's own app entry's `io.document_schema`, not anything kernel-shaped).
    async fn open(&mut self, plugin_id: &str, app_id: &str, _artifact_ref: &str) -> Result<u32, RunError> {
        self.manifest_for(plugin_id).await?;
        let compiled = self.compiled_for_plugin.get(plugin_id).cloned().expect("manifest_for just compiled or already present");
        let plugin_ordinal = self.plugin_ordinal(plugin_id);
        let instance_handle = self.next_handle;
        self.next_handle += 1;
        let kind = ActorKind::PluginApp { plugin: PackageId(plugin_id.to_string()), app_id: app_id.to_string(), instance_id: instance_handle };
        let actor = self.kernel.activate(PackageId(plugin_id.to_string()), plugin_ordinal, kind, Lane::Background, None, ActivationEvent::Manual, &compiled, &[], &NODE_TURN_BUDGET).await.map_err(RunError::Host)?;
        self.instances.insert(instance_handle, (plugin_id.to_string(), instance_handle));
        self.instance_actors.insert(instance_handle, actor);
        let open_event = Event::InstanceOpen {
            instance: PluginInstanceId(instance_handle.to_string()),
            app_id: AppInstanceId(app_id.to_string()),
            actor: "local".to_string(),
            config: Vec::new(),
            assets: Vec::new(),
            capabilities: Vec::new(),
            quotas: QuotaSchema::default(),
        };
        let _ = self.run_turn(actor, vec![open_event]).await?;
        Ok(instance_handle)
    }

    /// 🚪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET (W1-A) contract §3: intercepts the
    /// three OS-level `AppCommand`s (`OpenArtifact`/`SetDefaultApp`/`ClearDefaultApp`) BEFORE the
    /// generic per-instance forward below — they are host-level (which plugin/app to resolve, or how
    /// to fold `OpeningPreferences`), never something a guest's `VcsArtifactApp` could handle, and a
    /// guest given one of these bytes would just fail to decode it as any of ITS OWN typed commands.
    /// `node` still selects which loaded runtime the REMAINING (non-opening) commands in the same
    /// batch forward to — resolved once opening frames are peeled off, so a batch of pure opening
    /// commands never needs a live instance at all. Simplification, documented: opening-command
    /// frames are appended BEFORE any passthrough guest frames rather than interleaved at their
    /// original position — fine while a real caller sends either an opening batch or a document batch,
    /// never both mixed in one call (today's only caller, the SDK's `OpeningCommandRelay`, never mixes
    /// them).
    async fn exchange(&mut self, _ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError> {
        let mut frames = Vec::new();
        let mut passthrough = Vec::new();
        for command in commands {
            match command {
                AppCommand::OpenArtifact { seq, artifact_ref, role, plugin_id, app_id } => {
                    frames.push(match self.resolve_open_artifact(&artifact_ref, role, &plugin_id, &app_id) {
                        Ok(_resolved) => AppFrame::Done { in_reply_to: seq },
                        Err(fault) => AppFrame::Error { in_reply_to: Some(seq), fault: dsl::encode_fault_bytes(&fault), report: Vec::new() },
                    });
                }
                AppCommand::SetDefaultApp { seq, artifact_kind, standard, subset, role, plugin_id, app_id } => {
                    frames.push(match self.set_default_app(&artifact_kind, &standard, &subset, role, &plugin_id, &app_id) {
                        Ok(()) => AppFrame::Done { in_reply_to: seq },
                        Err(fault) => AppFrame::Error { in_reply_to: Some(seq), fault: dsl::encode_fault_bytes(&fault), report: Vec::new() },
                    });
                }
                AppCommand::ClearDefaultApp { seq, artifact_kind, standard, subset, role } => {
                    frames.push(match self.clear_default_app(&artifact_kind, &standard, &subset, role) {
                        Ok(()) => AppFrame::Done { in_reply_to: seq },
                        Err(fault) => AppFrame::Error { in_reply_to: Some(seq), fault: dsl::encode_fault_bytes(&fault), report: Vec::new() },
                    });
                }
                other => passthrough.push(other),
            }
        }
        if !passthrough.is_empty() {
            if !self.rejected_command_builds.terminal_is_empty() {
                let (complete, _, _) = self.rejected_command_builds.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                return Err(RunError::Host(if complete { "rejected command-build owner reached terminal empty; retry the exchange".to_string() } else { "rejected command-build owner closed one exact page; retry the exchange".to_string() }));
            }
            if !self.retained_command_closes.terminal_is_empty() {
                let (complete, _, _) = self.retained_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                return Err(RunError::Host(if complete {
                    "previous cancelled command owner reached terminal empty; retry the exchange".to_string()
                } else {
                    "previous cancelled command owner is closing one exact page; retry the exchange".to_string()
                }));
            }
            if !persistent_command_completion_port_ready() {
                return Err(RunError::Host("persistent command completion submit/poll/cancel authority is not admitted".to_string()));
            }
            let operation = self.take_turn_seq();
            if !self.retained_command_closes.can_insert(operation) || !self.rejected_command_builds.can_insert(operation) {
                return Err(RunError::Host("retained command close registry is saturated or collided".to_string()));
            }
            // ✅️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet `run-kernel-wiring`): the real body
            // this comment used to describe as future work — `node` now DOES resolve, since `open`
            // (above) genuinely activates an instance through the kernel. Each passthrough
            // `AppCommand` becomes a retained fixed-page owner. One page is lowered per reactor turn;
            // the next command remains untouched until the guest acknowledges the current terminal.
            // `TurnResult.effects` is then scanned for `Effect::Respond{req, result}`,
            // decoding `RequestOutcome::Ok(bytes)` as the guest's own `protocol::AppFrame` reply and
            // `Err(bytes)` as `AppFrame::Error` — exactly the shape this comment always described.
            let actor = *self.instance_actors.get(&node).ok_or_else(|| RunError::Host(format!("unknown node handle {node}")))?;
            let mut envelopes = semio_framework::kernel::CommandEnvelopeSet::try_new().map_err(|fault| RunError::Host(fault.to_string()))?;
            for command in &passthrough {
                let seq = app_command_seq(command);
                let command = protocol::encode_app_command(command).await.map_err(|fault| RunError::Host(fault.to_string()))?;
                if let Err((fault, rejected)) = envelopes.try_push(semio_framework::kernel::CommandEnvelope { instance: node, seq, command }) {
                    self.rejected_command_builds.insert_admitted(operation, semio_framework::kernel::RejectedCommandBuild::new(envelopes, rejected));
                    return Err(RunError::Host(fault.to_string()));
                }
            }
            let generation = self.take_turn_seq();
            let batch = match semio_framework::kernel::CommandBatch::try_new(generation, envelopes) {
                Ok(batch) => batch,
                Err((fault, owners)) => {
                    self.rejected_command_builds.insert_admitted(operation, semio_framework::kernel::RejectedCommandBuild::from_admitted(owners));
                    return Err(RunError::Host(fault.to_string()));
                }
            };
            self.retained_command_closes.insert_admitted(operation, generation, semio_framework::kernel::CommandBatchDriver::new(operation, batch));
            let mut effects = Vec::new();
            loop {
                let events = match self.retained_command_closes.with_driver_mut(operation, generation, |driver| driver.next_page()).map_err(|fault| RunError::Host(fault.to_string()))?.map_err(|fault| RunError::Host(fault.to_string()))? {
                    Some((cursor, bytes)) => vec![Event::CommandIngressPage { cursor, bytes }],
                    None => vec![Event::Wake],
                };
                self.retained_command_closes.prepare_suspend(operation, generation).map_err(|fault| RunError::Host(fault.to_string()))?;
                let turn = self.run_turn(actor, events).await?;
                self.retained_command_closes.resume(operation, generation).map_err(|fault| RunError::Host(fault.to_string()))?;
                let progress = self
                    .retained_command_closes
                    .with_driver_mut(operation, generation, |driver| driver.observe(&turn.command_ingress, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES))
                    .map_err(|fault| RunError::Host(fault.to_string()))?
                    .map_err(|fault| RunError::Host(fault.to_string()))?;
                effects.extend(turn.effects);
                match progress {
                    semio_framework::kernel::CommandBatchProgress::Complete => {
                        self.retained_command_closes.remove_terminal(operation, generation).map_err(|fault| RunError::Host(fault.to_string()))?;
                        break;
                    }
                    semio_framework::kernel::CommandBatchProgress::Faulted => {
                        self.retained_command_closes.begin_close(operation, generation).map_err(|fault| RunError::Host(fault.to_string()))?;
                        let (complete, _, _) = self.retained_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                        return Err(RunError::Host(if complete { "command ingress faulted after terminal exact-owner cleanup".to_string() } else { "command ingress faulted; retained owner closed one exact page and awaits retry".to_string() }));
                    }
                    semio_framework::kernel::CommandBatchProgress::PageReady | semio_framework::kernel::CommandBatchProgress::Waiting => {}
                }
            }
            for effect in effects {
                if let Effect::Respond { req, result } = effect {
                    match result {
                        RequestOutcome::Ok(bytes) => match protocol::decode_app_frame(&bytes).await {
                            Ok(frame) => frames.push(frame),
                            Err(error) => frames.push(AppFrame::Error { in_reply_to: Some(req.0), fault: dsl::encode_fault_bytes(&semio_framework::Fault::from(error.to_string())), report: Vec::new() }),
                        },
                        RequestOutcome::Err(bytes) => frames.push(AppFrame::Error { in_reply_to: Some(req.0), fault: bytes, report: Vec::new() }),
                    }
                }
            }
        }
        Ok(frames)
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn persistent_command_completion_port_ready() -> bool {
    false
}

/// 🔢️ Every `AppCommand` variant's leading `seq: u64` field, used verbatim as the kernel envelope's
/// own command-ingress sequence (see `exchange`'s own passthrough doc for why reusing it — rather
/// than minting a fresh kernel-side sequence and maintaining a translation table back to it — is
/// sound: one `exchange` batch's commands already carry distinct seqs, minted by `SpaceRunner::
/// compute_node`'s own local `next_seq` closure, and `Effect::Respond{req}` correlates directly
/// against whichever seq the retained command envelope carried). An exhaustive match, deliberately —
/// a future `AppCommand` variant missing here is a compile error, not a silent seq-of-zero.
#[cfg(not(target_arch = "wasm32"))]
fn app_command_seq(command: &AppCommand) -> u64 {
    match command {
        AppCommand::ConfigCommand { seq, .. }
        | AppCommand::Command { seq, .. }
        | AppCommand::CommandText { seq, .. }
        | AppCommand::ContextMenu { seq, .. }
        | AppCommand::ArtifactCommand { seq, .. }
        | AppCommand::ApplyEnvelopes { seq, .. }
        | AppCommand::LoadDocument { seq, .. }
        | AppCommand::ReadDocument { seq }
        | AppCommand::LoadConfig { seq, .. }
        | AppCommand::ReadConfig { seq }
        | AppCommand::MediaIn { seq, .. }
        | AppCommand::MediaOut { seq, .. }
        | AppCommand::MediaFingerprint { seq, .. }
        | AppCommand::PureCommand { seq, .. }
        | AppCommand::LoadChildren { seq, .. }
        | AppCommand::ReadChildren { seq }
        | AppCommand::ReadHistory { seq }
        | AppCommand::TransactionPrepare { seq, .. }
        | AppCommand::TransactionCommit { seq, .. }
        | AppCommand::TransactionRollback { seq, .. }
        | AppCommand::TransactionUndo { seq, .. }
        | AppCommand::TransactionRedo { seq, .. }
        | AppCommand::OpenArtifact { seq, .. }
        | AppCommand::SetDefaultApp { seq, .. }
        | AppCommand::ClearDefaultApp { seq, .. }
        | AppCommand::SetMergePolicy { seq, .. }
        | AppCommand::ResolveConflict { seq, .. }
        | AppCommand::ReadConflicts { seq }
        | AppCommand::Presence { seq, .. } => *seq,
    }
}
//#endregion 🔖️WasmtimeNodeHost

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework::{MediaPortDirection, MediaPortSpec};
    use workflow::{placeholder_media_contract, WorkflowMediaPort, WORKFLOW_SCHEMA};

    //#region 🔖️TestMediaCache
    /// 🧪️ Test-local map-backed `MediaCache` for exercising cached runner outputs.
    #[derive(Default)]
    struct TestMediaCache {
        entries: HashMap<String, Media>,
    }

    impl MediaCache for TestMediaCache {
        fn get(&self, fingerprint: &MediaFingerprint) -> Option<Media> {
            self.entries.get(&fingerprint.0).cloned()
        }

        fn put(&mut self, fingerprint: &MediaFingerprint, media: &Media) {
            self.entries.insert(fingerprint.0.clone(), media.clone());
        }
    }
    //#endregion 🔖️TestMediaCache

    /// 🧪️ A fake `AppChannelHost` for tests: no wasm at all, just a per-instance document/config, a
    /// fixed structured output per port, and an in-process `InMemoryBlobStore` — enough to interpret
    /// the exact `AppCommand`/`AppFrame` frame script `SpaceRunner::compute_node` sends, so
    /// `SpaceRunner`'s dirty/clean bookkeeping can be exercised without a real program.
    /// 🧪️ Outputs are keyed by app id, not by handle — a real app's export is a function of its
    /// document/logic, not of the ephemeral instance handle a host happens to mint this call, and a
    /// node genuinely does get re-opened (a fresh handle) on every dirty re-run.
    #[derive(Default)]
    struct FakeHost {
        documents: HashMap<u32, (Vec<u8>, Vec<u8>)>,
        configs: HashMap<u32, (Vec<u8>, Vec<u8>)>,
        handle_app: HashMap<u32, String>,
        outputs: HashMap<(String, String), Media>,
        blob_store: InMemoryBlobStore,
        next: u32,
        imported: Vec<(u32, String, Media)>,
    }

    impl FakeHost {
        fn set_output(&mut self, app_id: &str, port: &str, json: &str) {
            self.outputs.insert((app_id.to_string(), port.to_string()), Media { media_type: fake_media_type(), payload: MediaPayload::Structured { schema: "test".into(), json: json.into() } });
        }
    }

    fn fake_media_type() -> MediaType {
        MediaType { class: MediaClass::Data, form: MediaForm::Value }
    }

    impl AppChannelHost for FakeHost {
        async fn open(&mut self, _plugin_id: &str, app_id: &str, _artifact_ref: &str) -> Result<u32, RunError> {
            self.next += 1;
            self.handle_app.insert(self.next, app_id.to_string());
            Ok(self.next)
        }

        async fn exchange(&mut self, _ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError> {
            let app_id = self.handle_app.get(&node).cloned().unwrap_or_default();
            let mut frames = Vec::new();
            for command in commands {
                match command {
                    // 🧬️ Channel v12 retires `Hello`/`Welcome` — `open` (above) is what now
                    // establishes the instance, matching the reactor ABI's `Event::InstanceOpen`.
                    AppCommand::LoadConfig { seq, pack, spr } => {
                        self.configs.insert(node, (pack, spr));
                        frames.push(AppFrame::Done { in_reply_to: seq });
                    }
                    AppCommand::LoadDocument { seq, pack, spr } => {
                        self.documents.insert(node, (pack, spr));
                        frames.push(AppFrame::Done { in_reply_to: seq });
                    }
                    AppCommand::MediaIn { seq, port, descriptor, data } => match media_from_artifact(&descriptor, data, &self.blob_store) {
                        Ok(media) => {
                            self.imported.push((node, port, media));
                            frames.push(AppFrame::Done { in_reply_to: seq });
                        }
                        Err(error) => frames.push(AppFrame::Error { in_reply_to: Some(seq), fault: run_fault_bytes("handler", error.to_string()), report: Vec::new() }),
                    },
                    AppCommand::MediaOut { seq, port, .. } => match self.outputs.get(&(app_id.clone(), port.clone())) {
                        Some(media) => match media_to_artifact(media, &self.blob_store) {
                            Ok((descriptor, data)) => frames.push(AppFrame::Media { in_reply_to: seq, port, descriptor, data }),
                            Err(error) => frames.push(AppFrame::Error { in_reply_to: Some(seq), fault: run_fault_bytes("handler", error.to_string()), report: Vec::new() }),
                        },
                        None => frames.push(AppFrame::Error { in_reply_to: Some(seq), fault: run_fault_bytes("handler", "no output"), report: Vec::new() }),
                    },
                    AppCommand::MediaFingerprint { seq, port } => match self.outputs.get(&(app_id.clone(), port)) {
                        Some(media) => {
                            let fingerprint = MediaFingerprint::of(media);
                            // 🩹️ Pre-existing bug fixed in passing: `store::pack_rt::encode_wire_value` takes
                            // `&dsl::DslValue`, not `&serde_json::Value` — `decode_fingerprint_wire` (this
                            // file) already expects the `DslValue`-transparent-string encoding `to_dsl_value`
                            // produces for a `MediaFingerprint(String)` newtype.
                            let value = to_dsl_value(&fingerprint).unwrap_or(dsl::DslValue::Null);
                            frames.push(AppFrame::MediaFingerprint { in_reply_to: seq, port: String::new(), fingerprint: store::pack_rt::encode_wire_value(&value) });
                        }
                        None => frames.push(AppFrame::Error { in_reply_to: Some(seq), fault: run_fault_bytes("handler", "no output"), report: Vec::new() }),
                    },
                    AppCommand::ReadDocument { seq } => {
                        let (pack, spr) = self.documents.get(&node).cloned().unwrap_or_default();
                        frames.push(AppFrame::Document { in_reply_to: seq, pack, spr, ops: String::new() });
                    }
                    AppCommand::ReadConfig { seq } => {
                        let (pack, spr) = self.configs.get(&node).cloned().unwrap_or_default();
                        frames.push(AppFrame::Config { in_reply_to: seq, pack, spr, ops: String::new() });
                    }
                    AppCommand::SetMergePolicy { seq, .. } => {
                        frames.push(AppFrame::Done { in_reply_to: seq });
                    }
                    _ => {}
                }
            }
            Ok(frames)
        }
    }

    fn media_port(node_id: &str, spec_id: &str, direction: MediaPortDirection, kind_id: &str, multiplicity: PortMultiplicity, required: bool) -> WorkflowMediaPort {
        let direction_word = match direction {
            MediaPortDirection::In => "in",
            MediaPortDirection::Out => "out",
        };
        WorkflowMediaPort { id: format!("{node_id}:{spec_id}:{direction_word}"), spec: MediaPortSpec { id: spec_id.into(), label: spec_id.into(), direction, media_type: fake_media_type(), kind_id: Some(kind_id.into()), required, multiplicity } }
    }

    fn workflow_node(id: &str, outputs: Vec<WorkflowMediaPort>, inputs: Vec<WorkflowMediaPort>) -> WorkflowNode {
        WorkflowNode {
            id: id.into(),
            plugin_id: "program".into(),
            app_id: format!("app-{id}"),
            label: id.into(),
            yields: String::new(),
            artifact_ref: format!("artifacts/{id}"),
            config_ref: format!("config/{id}"),
            x: 0.0,
            y: 0.0,
            width: 220.0,
            height: 100.0,
            inputs,
            outputs,
        }
    }

    fn two_node_graph() -> Workflow {
        let source = workflow_node("node-a", vec![media_port("node-a", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
        let target = workflow_node("node-b", Vec::new(), vec![media_port("node-b", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, true)]);
        let edge =
            WorkflowEdge { id: "edge-1".into(), source_node_id: "node-a".into(), source_port_id: "node-a:out:out".into(), target_node_id: "node-b".into(), target_port_id: "node-b:in:in".into(), contract: placeholder_media_contract("data.value") };
        Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![source, target], edges: vec![edge] }
    }

    fn empty_documents(graph: &Workflow) -> BTreeMap<String, (Vec<u8>, Vec<u8>)> {
        graph.nodes.iter().map(|node| (node.artifact_ref.clone(), (Vec::new(), Vec::new()))).collect()
    }

    fn empty_configs(graph: &Workflow) -> BTreeMap<String, (Vec<u8>, Vec<u8>)> {
        graph.nodes.iter().map(|node| (node.config_ref.clone(), (Vec::new(), Vec::new()))).collect()
    }

    /// 🧪️ A fresh, unsealed `RunSink` with `Start` already recorded — every test below emits
    /// `NodeStarted`/`NodeFinished` through `SpaceRunner::run` on top of this, then (where memoization
    /// across two runs matters) seals it and extracts `prior_node_records_from` for the second `run()`.
    async fn fresh_sink() -> RunSink {
        let mut sink = RunSink::new(workflow::empty_run_document());
        sink.record(RunMutation::StartRun(StartRun {
            workflow_ref: "test.workflow".into(),
            workflow_checkpoint_id: String::new(),
            input_collection_ref: String::new(),
            input_snapshot_id: String::new(),
            parameter_values: Vec::new(),
            output_collection_ref: String::new(),
            trigger: workflow::RunTrigger::Manual { actor: "test".into() },
        }))
        .await.expect("Start on a fresh sink always applies");
        sink
    }

    #[semio_framework_async_macros::async_test]
    async fn run_sink_preserves_typed_admission_rejection_without_recording_it() {
        let mut sink = fresh_sink().await;
        let document = sink.document.clone();
        let mutations = sink.mutations.clone();
        let duplicate = RunMutation::StartRun(StartRun {
            workflow_ref: "test.workflow".into(), workflow_checkpoint_id: String::new(), input_collection_ref: String::new(), input_snapshot_id: String::new(), parameter_values: Vec::new(), output_collection_ref: String::new(), trigger: workflow::RunTrigger::Manual { actor: "test".into() },
        });
        let error = sink.record(duplicate).await.expect_err("a second Start must be rejected");
        match error {
            RunError::MutationApply(error) => {
                assert_eq!(error.code, "mutation.apply.conflicting-target");
                assert_eq!(error.target, vec!["status"]);
            }
            other => panic!("expected typed mutation rejection, got {other:?}"),
        }
        assert_eq!(sink.document, document);
        assert_eq!(sink.mutations, mutations);
    }

    /// 🧪️ `workflow::RunArtifact.node_records`, keyed by node id — the shape `SpaceRunner::run`'s
    /// `prior_node_records` parameter takes, built from a (test-)sealed prior run's document.
    fn prior_node_records_from(document: &workflow::RunArtifact) -> BTreeMap<String, RunNodeRecord> {
        document.node_records.iter().map(|record| (record.node_id.clone(), record.clone())).collect()
    }

    #[test]
    fn topological_order_respects_edges() {
        let graph = two_node_graph();
        let order = topological_order(&graph).expect("acyclic");
        assert_eq!(order, vec!["node-a".to_string(), "node-b".to_string()]);
    }

    #[test]
    fn detects_cycles() {
        let mut graph = two_node_graph();
        graph.edges.push(WorkflowEdge { id: "edge-2".into(), source_node_id: "node-b".into(), source_port_id: "b-out".into(), target_node_id: "node-a".into(), target_port_id: "a-in".into(), contract: placeholder_media_contract("data.value") });
        assert!(matches!(topological_order(&graph), Err(RunError::Cycle(_))));
    }

    #[semio_framework_async_macros::async_test]
    async fn first_run_recomputes_every_node_second_run_is_a_no_operation() {
        let graph = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-node-a", "node-a:out:out", "\"hello\"");
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
        let mut cache = TestMediaCache::default();
        let documents = empty_documents(&graph);
        let configs = empty_configs(&graph);

        let mut sink_1 = fresh_sink().await;
        let report_1 = runner.run(&graph, &documents, &configs, &[], &[], &BTreeMap::new(), &mut cache, &mut sink_1).await.expect("first run");
        assert_eq!(report_1.recomputed, vec!["node-a".to_string(), "node-b".to_string()]);
        assert!(report_1.clean.is_empty());
        sink_1.record(RunMutation::SealRun(SealRun { status: workflow::RunStatus::Succeeded })).await.expect("seal first run");

        // 🔒️ Non-destructive: the SOURCE `documents`/`configs` maps `run()` read are byte-identical to
        // what was passed in — nothing was written back into them (the load-bearing proof for this wave).
        assert_eq!(documents, empty_documents(&graph), "run() must never mutate its source documents map");
        assert_eq!(configs, empty_configs(&graph), "run() must never mutate its source configs map");

        let prior = prior_node_records_from(&sink_1.document);
        let mut sink_2 = fresh_sink().await;
        let report_2 = runner.run(&graph, &documents, &configs, &[], &[], &prior, &mut cache, &mut sink_2).await.expect("second run");
        assert!(report_2.recomputed.is_empty(), "unchanged documents must not re-trigger recompute: {:?}", report_2.recomputed);
        assert_eq!(report_2.clean, vec!["node-a".to_string(), "node-b".to_string()]);
        assert!(sink_2.document.node_records.iter().all(|record| record.status == RunNodeStatus::CacheHit), "every node on the second run must be a CacheHit: {:?}", sink_2.document.node_records);
    }

    #[semio_framework_async_macros::async_test]
    async fn editing_upstream_document_dirties_downstream_only_through_the_wire() {
        let graph = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-node-a", "node-a:out:out", "\"hello\"");
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
        let mut cache = TestMediaCache::default();
        let documents = empty_documents(&graph);
        let configs = empty_configs(&graph);
        let mut sink_1 = fresh_sink().await;
        runner.run(&graph, &documents, &configs, &[], &[], &BTreeMap::new(), &mut cache, &mut sink_1).await.expect("first run");
        sink_1.record(RunMutation::SealRun(SealRun { status: workflow::RunStatus::Succeeded })).await.expect("seal first run");

        let mut documents_2 = documents.clone();
        // 🧮️ Fingerprints are still `.spr`-bytes hashes (see `node_fingerprints`) — editing the op log
        // (`spr`), not the pack, is what must dirty the node. `documents` (the first run's SOURCE map)
        // is untouched by `run()` itself — this test edits its OWN local copy to simulate a live UI edit
        // landing on the source between runs.
        documents_2.insert("artifacts/node-a".to_string(), (Vec::new(), b"edited".to_vec()));
        let prior = prior_node_records_from(&sink_1.document);
        let mut sink_2 = fresh_sink().await;
        let report_2 = runner.run(&graph, &documents_2, &configs, &[], &[], &prior, &mut cache, &mut sink_2).await.expect("second run");
        assert_eq!(report_2.recomputed, vec!["node-a".to_string()], "node-a's own document changed, so node-a must recompute");
        assert_eq!(report_2.clean, vec!["node-b".to_string()], "node-a's FakeHost output is fixed, so its output fingerprint is unchanged — node-b must stay clean (the early-cutoff this whole design exists for)");
    }

    /// 🧪️ Changing a node's own effective config — document and resolved inputs held constant — must
    /// dirty exactly that node on the very next `plan()`/`run()`, mirroring
    /// `editing_upstream_document_dirties_downstream_only_through_the_wire`'s shape but on the config
    /// dimension instead of the document one.
    #[semio_framework_async_macros::async_test]
    async fn changing_a_nodes_config_alone_dirties_it_without_touching_document_or_inputs() {
        let graph = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-node-a", "node-a:out:out", "\"hello\"");
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
        let mut cache = TestMediaCache::default();
        let documents = empty_documents(&graph);
        let configs_1 = empty_configs(&graph);
        let mut sink_1 = fresh_sink().await;
        runner.run(&graph, &documents, &configs_1, &[], &[], &BTreeMap::new(), &mut cache, &mut sink_1).await.expect("first run");
        sink_1.record(RunMutation::SealRun(SealRun { status: workflow::RunStatus::Succeeded })).await.expect("seal first run");
        let prior = prior_node_records_from(&sink_1.document);

        let plan_unchanged = plan(&graph, &documents, &configs_1, &[], &[], &prior).expect("plan with unchanged config");
        assert!(plan_unchanged.recomputed.is_empty(), "nothing changed, plan must report every node clean: {:?}", plan_unchanged.recomputed);

        let mut configs_2 = configs_1.clone();
        configs_2.insert("config/node-a".to_string(), (Vec::new(), b"threshold=2".to_vec()));
        let plan_changed = plan(&graph, &documents, &configs_2, &[], &[], &prior).expect("plan with changed config");
        assert_eq!(plan_changed.recomputed, vec!["node-a".to_string()], "only node-a's own config changed, so only node-a should be recomputed by the plan");

        let mut sink_2 = fresh_sink().await;
        let report_2 = runner.run(&graph, &documents, &configs_2, &[], &[], &prior, &mut cache, &mut sink_2).await.expect("second run with changed config");
        assert_eq!(report_2.recomputed, vec!["node-a".to_string()], "node-a's config changed, so node-a must recompute even though its document and inputs did not");
        assert_eq!(report_2.clean, vec!["node-b".to_string()], "node-a's FakeHost output is fixed regardless of config, so node-b must stay clean");
    }

    /// 🧪️ A `--param` override bound onto a node's config field must dirty that node purely through the
    /// fingerprint overlay (`node_parameter_overlay_bytes`) even though the raw config `.spr` bytes this
    /// crate sends the app are byte-identical — see this crate's module doc on why the override isn't
    /// patched into the opaque config bytes directly.
    #[semio_framework_async_macros::async_test]
    async fn parameter_overlay_alone_dirties_its_bound_node_without_changing_raw_config_bytes() {
        let graph = two_node_graph();
        let mut host = FakeHost::default();
        host.set_output("app-node-a", "node-a:out:out", "\"hello\"");
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
        let mut cache = TestMediaCache::default();
        let documents = empty_documents(&graph);
        let configs = empty_configs(&graph);
        let bindings = vec![WorkflowParameterBinding { parameter_id: "p1".into(), node_id: "node-a".into(), field_path: "/threshold".into() }];

        let mut sink_1 = fresh_sink().await;
        runner.run(&graph, &documents, &configs, &[], &bindings, &BTreeMap::new(), &mut cache, &mut sink_1).await.expect("first run");
        sink_1.record(RunMutation::SealRun(SealRun { status: workflow::RunStatus::Succeeded })).await.expect("seal first run");
        let prior = prior_node_records_from(&sink_1.document);

        let plan_unchanged = plan(&graph, &documents, &configs, &[], &bindings, &prior).expect("plan with no parameter values yet");
        assert!(plan_unchanged.recomputed.is_empty(), "no bound parameter value yet — nothing should be dirty: {:?}", plan_unchanged.recomputed);

        let parameter_values = vec![RunParameterValue { parameter_id: "p1".into(), value: "42".into() }];
        let plan_changed = plan(&graph, &documents, &configs, &parameter_values, &bindings, &prior).expect("plan with a parameter value bound");
        assert_eq!(plan_changed.recomputed, vec!["node-a".to_string()], "the bound node's fingerprint must change purely from the parameter overlay: {:?}", plan_changed);

        let mut sink_2 = fresh_sink().await;
        let report_2 = runner.run(&graph, &documents, &configs, &parameter_values, &bindings, &prior, &mut cache, &mut sink_2).await.expect("second run with a param override");
        assert_eq!(report_2.recomputed, vec!["node-a".to_string()]);
        assert_eq!(sink_2.node_configs.get("node-a"), Some(&configs["config/node-a"]), "raw config bytes sent to the app are untouched by the overlay — only the fingerprint changes");
    }

    //#region 🔖️ExchangeOrderingTests
    /// 🧪️ `AppChannelHost::exchange`'s own doc promises no second `exchange` for the same `node`
    /// handle overlaps the first. `SpaceRunner` enforces that structurally today (it owns `H` and
    /// calls through `&mut self`, so two `exchange` futures against the SAME owned host could never
    /// even be POLLED concurrently — the borrow checker forbids it). `RecorderHost` proves the
    /// DETECTOR below actually catches overlap when nothing prevents it, so
    /// `space_runner_never_overlaps_exchange_for_the_same_node_across_a_real_run`'s "never overlaps"
    /// isn't a vacuous pass: it shares its bookkeeping through an `Rc<RefCell<_>>`, so cloning it
    /// (unlike sharing one owned `H`) genuinely CAN be driven concurrently, and `exchange` yields
    /// once mid-call (`semio_framework_async::yield_once`) so an interleaving executor has a real
    /// chance to expose overlap.
    #[derive(Default)]
    struct RecorderState {
        next_handle: u32,
        in_flight: std::collections::HashSet<u32>,
        overlap_detected: bool,
        completed_in_order: Vec<u32>,
    }

    #[derive(Clone, Default)]
    struct RecorderHost(std::rc::Rc<std::cell::RefCell<RecorderState>>);

    /// 🧪️ Every `AppCommand` variant `RecorderHost`'s own test graphs ever send (no media ports —
    /// see `two_independent_solo_nodes`) mapped to the SPECIFIC `AppFrame` reply `compute_node`
    /// expects for it — `ReadDocument`/`ReadConfig` each demand their own typed frame
    /// (`AppFrame::Document`/`AppFrame::Config`), not a generic `Done` (see `compute_node`'s own
    /// `reply_to`/pattern-match).
    fn reply_for(command: &AppCommand) -> AppFrame {
        match command {
            AppCommand::SetMergePolicy { seq, .. } => AppFrame::Done { in_reply_to: *seq },
            AppCommand::LoadConfig { seq, .. } => AppFrame::Done { in_reply_to: *seq },
            AppCommand::LoadDocument { seq, .. } => AppFrame::Done { in_reply_to: *seq },
            AppCommand::ReadDocument { seq } => AppFrame::Document { in_reply_to: *seq, pack: Vec::new(), spr: Vec::new(), ops: String::new() },
            AppCommand::ReadConfig { seq } => AppFrame::Config { in_reply_to: *seq, pack: Vec::new(), spr: Vec::new(), ops: String::new() },
            other => panic!("RecorderHost's test graphs never send {other:?}"),
        }
    }

    impl AppChannelHost for RecorderHost {
        async fn open(&mut self, _plugin_id: &str, _app_id: &str, _artifact_ref: &str) -> Result<u32, RunError> {
            let mut state = self.0.borrow_mut();
            state.next_handle += 1;
            Ok(state.next_handle)
        }

        async fn exchange(&mut self, _ctx: &OperationContext, node: u32, commands: Vec<AppCommand>) -> Result<Vec<AppFrame>, RunError> {
            {
                let mut state = self.0.borrow_mut();
                if !state.in_flight.insert(node) {
                    state.overlap_detected = true;
                }
            }
            semio_framework_async::yield_once().await;
            let frames = commands.iter().map(reply_for).collect();
            {
                let mut state = self.0.borrow_mut();
                state.in_flight.remove(&node);
                state.completed_in_order.push(node);
            }
            Ok(frames)
        }
    }

    fn two_independent_solo_nodes() -> Workflow {
        Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![workflow_node("node-a", Vec::new(), Vec::new()), workflow_node("node-b", Vec::new(), Vec::new())], edges: Vec::new() }
    }

    fn root_ctx() -> OperationContext {
        OperationContext { actor: 0, generation: 0, trace: TraceId(0), lane: 0, deadline_ms: None, cancel: CancelToken::root(), capability: None }
    }

    /// 🧪️ Sanity check for `RecorderHost` itself: two callers racing the SAME node id, with nothing
    /// serializing them (unlike `SpaceRunner`, which owns its host exclusively), DO overlap once
    /// interleaved — proves the detector the next test relies on would actually catch a real
    /// regression rather than passing no matter what.
    #[test]
    fn recorder_host_detects_genuine_overlap_when_nothing_prevents_it() {
        let recorder = RecorderHost::default();
        let mut a = recorder.clone();
        let mut b = recorder.clone();
        let ctx = root_ctx();
        let fut_a = a.exchange(&ctx, 7, vec![AppCommand::ReadDocument { seq: 1 }]);
        let fut_b = b.exchange(&ctx, 7, vec![AppCommand::ReadDocument { seq: 2 }]);
        let (_a, _b) = semio_framework_async::block_on(semio_framework_async::join2(fut_a, fut_b));
        assert!(recorder.0.borrow().overlap_detected, "two callers racing the same node id with no ownership guard must overlap");
    }

    /// 🧪️ The actual ordering property this ticket asks for: `SpaceRunner::run`'s own call pattern
    /// (strictly sequential, one node at a time, in topological order) never overlaps two `exchange`
    /// calls for the same node — even against `RecorderHost`'s yield point, which would expose an
    /// accidental `join`/`spawn` introduced by a later refactor as `overlap_detected`.
    #[semio_framework_async_macros::async_test]
    async fn space_runner_never_overlaps_exchange_for_the_same_node_across_a_real_run() {
        let graph = two_independent_solo_nodes();
        let host = RecorderHost::default();
        let recorder = host.clone();
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
        let mut cache = TestMediaCache::default();
        let documents = empty_documents(&graph);
        let configs = empty_configs(&graph);
        let mut sink = fresh_sink().await;

        runner.run(&graph, &documents, &configs, &[], &[], &BTreeMap::new(), &mut cache, &mut sink).await.expect("both solo nodes compute cleanly against RecorderHost");

        let state = recorder.0.borrow();
        assert!(!state.overlap_detected, "SpaceRunner must never issue two exchange calls for the same node concurrently");
        assert_eq!(state.completed_in_order, vec![1, 2], "node-a (handle 1) completes strictly before node-b (handle 2) — sequential topological order preserved");
    }

    /// 🧪️ `compute_node` checks `self.cancel` BEFORE `open`/`exchange` (see `RunError::Cancelled`'s
    /// doc) — cancelling `SpaceRunner`'s own token stops the run before its NEXT node. Node-a's own
    /// `exchange` still completes (cancellation is checked between nodes, not preemptible mid-call —
    /// the same honest limitation `semio-framework-os-services::ComputePool` documents), so this
    /// asserts node-b specifically never runs.
    #[semio_framework_async_macros::async_test]
    async fn cancelling_the_run_token_stops_the_run_before_the_next_node() {
        let graph = two_independent_solo_nodes();
        let host = RecorderHost::default();
        let recorder = host.clone();
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
        let cancel = runner.cancel_token();
        let mut cache = TestMediaCache::default();
        let documents = empty_documents(&graph);
        let configs = empty_configs(&graph);
        let mut sink = fresh_sink().await;

        // 🛑️ Cancel BEFORE the run even starts — deterministic, no real sleep/race needed: the very
        // first `compute_node` call (node-a) must already observe `Cancelled`.
        cancel.cancel();
        let result = runner.run(&graph, &documents, &configs, &[], &[], &BTreeMap::new(), &mut cache, &mut sink).await;
        assert!(matches!(result, Err(RunError::Cancelled)), "a run cancelled before its first node must fail with RunError::Cancelled, got {result:?}");
        assert!(recorder.0.borrow().completed_in_order.is_empty(), "no node's exchange should run once the token is cancelled before the run starts");
    }
    //#endregion 🔖️ExchangeOrderingTests

    #[semio_framework_async_macros::async_test]
    async fn rejects_incompatible_edge_media_types() {
        let mut graph = two_node_graph();
        graph.nodes[1].inputs[0].spec.media_type = MediaType { class: MediaClass::Text, form: MediaForm::Document };
        let host = FakeHost::default();
        let mut runner = SpaceRunner::new(host, Arc::new(InMemoryBlobStore::default()), protocol::MergePolicy::default());
        let mut cache = TestMediaCache::default();
        let documents = empty_documents(&graph);
        let configs = empty_configs(&graph);
        let mut sink = fresh_sink().await;
        let result = runner.run(&graph, &documents, &configs, &[], &[], &BTreeMap::new(), &mut cache, &mut sink).await;
        assert!(matches!(result, Err(RunError::Incompatible { .. })));
    }

    #[test]
    fn validate_rejects_missing_required_input() {
        let node = workflow_node("solo", Vec::new(), vec![media_port("solo", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, true)]);
        let graph = Workflow { schema: WORKFLOW_SCHEMA.into(), nodes: vec![node], edges: Vec::new() };
        assert!(matches!(validate_edge_kinds(&graph), Err(RunError::MissingRequiredInput { .. })));
    }

    #[test]
    fn validate_rejects_multiplicity_one_input_with_two_incoming_edges() {
        let source_a = workflow_node("src-a", vec![media_port("src-a", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
        let source_b = workflow_node("src-b", vec![media_port("src-b", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
        let target = workflow_node("target", Vec::new(), vec![media_port("target", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, false)]);
        let graph = Workflow {
            schema: WORKFLOW_SCHEMA.into(),
            nodes: vec![source_a, source_b, target],
            edges: vec![
                WorkflowEdge { id: "e1".into(), source_node_id: "src-a".into(), source_port_id: "src-a:out:out".into(), target_node_id: "target".into(), target_port_id: "target:in:in".into(), contract: placeholder_media_contract("data.value") },
                WorkflowEdge { id: "e2".into(), source_node_id: "src-b".into(), source_port_id: "src-b:out:out".into(), target_node_id: "target".into(), target_port_id: "target:in:in".into(), contract: placeholder_media_contract("data.value") },
            ],
        };
        assert!(matches!(validate_edge_kinds(&graph), Err(RunError::MultiplicityViolation { .. })));
    }

    #[test]
    fn validate_rejects_unregistered_conversion() {
        let source = workflow_node("src", vec![media_port("src", "out", MediaPortDirection::Out, "data.value", PortMultiplicity::One, true)], Vec::new());
        let target = workflow_node("dst", Vec::new(), vec![media_port("dst", "in", MediaPortDirection::In, "data.value", PortMultiplicity::One, false)]);
        let mut contract = placeholder_media_contract("data.value");
        // 🧪️ A conversion form pair this test file never registers a converter for — distinct from
        // any pair the `media_converter_registry_*` tests below register, so the global
        // `MEDIA_CONVERTERS` table (shared across all tests in this process) can't race with this one.
        contract.conversion = Some((MediaForm::Trinity, MediaForm::Dag));
        let graph = Workflow {
            schema: WORKFLOW_SCHEMA.into(),
            nodes: vec![source, target],
            edges: vec![WorkflowEdge { id: "e1".into(), source_node_id: "src".into(), source_port_id: "src:out:out".into(), target_node_id: "dst".into(), target_port_id: "dst:in:in".into(), contract }],
        };
        assert!(matches!(validate_edge_kinds(&graph), Err(RunError::UnregisteredConversion { .. })));
    }

    #[test]
    fn convert_media_is_identity_when_contract_has_no_conversion() {
        let contract = placeholder_media_contract("data.value");
        let media = Media { media_type: fake_media_type(), payload: MediaPayload::Structured { schema: "test".into(), json: "\"hi\"".into() } };
        let converted = convert_media(&contract, media.clone()).expect("identity conversion never fails");
        assert_eq!(converted, media);
    }

    #[test]
    fn media_converter_registry_applies_registered_converter() {
        // 🧪️ A `(class, from, to)` triple unique to this test (see `validate_rejects_unregistered_
        // conversion`'s note on the shared global table).
        register_media_converter(MediaClass::Kit, MediaForm::Design, MediaForm::Sequence, |media| Ok(Media { media_type: media.media_type, payload: MediaPayload::Structured { schema: "converted".into(), json: "\"converted\"".into() } }));
        let mut contract = placeholder_media_contract("kit.design");
        contract.media_type = MediaType { class: MediaClass::Kit, form: MediaForm::Sequence };
        contract.conversion = Some((MediaForm::Design, MediaForm::Sequence));
        let media = Media { media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Design }, payload: MediaPayload::Structured { schema: "design".into(), json: "\"raw\"".into() } };
        let converted = convert_media(&contract, media).expect("registered converter applies");
        assert_eq!(converted.payload, MediaPayload::Structured { schema: "converted".into(), json: "\"converted\"".into() });
    }

    #[test]
    fn vector_to_raster_rasterizes_svg_to_a_2d_image_media() {
        let svg = Media {
            media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
            payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 4 4" width="4" height="4"><rect width="4" height="4" fill="#ff0000"/></svg>"##.into() },
        };
        let raster = vector_to_raster(&svg).expect("rasterizes real svg text");
        assert_eq!(raster.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Raster });
        let MediaPayload::Structured { schema, json } = raster.payload else { panic!("expected structured payload") };
        assert_eq!(schema, "2d.image");
        assert!(!json.is_empty(), "png_base64 must be non-empty for real svg content");
    }

    #[test]
    fn vector_to_raster_rejects_non_structured_payload() {
        let media = Media { media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector }, payload: MediaPayload::Binary { format_kind: "png".into(), blob_hash: "hash".into() } };
        assert!(vector_to_raster(&media).is_err());
    }

    #[test]
    fn register_builtin_converters_wires_vector_to_raster_through_convert_media() {
        register_builtin_converters();
        let mut contract = placeholder_media_contract("2d.drawing");
        contract.media_type = MediaType { class: MediaClass::TwoD, form: MediaForm::Raster };
        contract.conversion = Some((MediaForm::Vector, MediaForm::Raster));
        let media = Media {
            media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Vector },
            payload: MediaPayload::Structured { schema: "2d.drawing".into(), json: r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 2 2" width="2" height="2"><rect width="2" height="2" fill="#00ff00"/></svg>"##.into() },
        };
        let converted = convert_media(&contract, media).expect("builtin vector->raster converter is registered");
        assert_eq!(converted.media_type, MediaType { class: MediaClass::TwoD, form: MediaForm::Raster });
    }

    //#region 🔖️NativeManifestSmoke
    /// 🧭️ Walks up from `CARGO_MANIFEST_DIR` looking for `nx.json` — the SAME strategy
    /// `📦️bin.rs`'s own `find_repo_root` uses, duplicated here (not `include!`d — the bin crate's
    /// own doc explains a `[[bin]]` target does not share the lib's module tree).
    fn test_repo_root() -> PathBuf {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        loop {
            if dir.join("nx.json").is_file() {
                return dir;
            }
            assert!(dir.pop(), "walked past the filesystem root looking for nx.json");
        }
    }

    /// 🧪️ R1-native-manifest runtime evidence: `🗒️note` is the one plugin with a committed
    /// `🛂️.descriptor.semio` verified sha256-identical to a real native `describe()` build
    /// (`design-abi.md` §3's `descriptor_is_fresh` gate; `📓️terra-D0-note-describe1.txt`) — every
    /// other one of the 33 plugin ids has none yet (`📓️status.md`'s W3 report: descriptor emission
    /// is fleet-wide blocked on a pre-existing capability-claim rule, packet D0). Tries every
    /// location a compiled `note` wasm could plausibly be: the canonical `target/wasm32-wasip2/`
    /// dirs `resolve_plugin_paths` (`📦️bin.rs`) uses at real CLI runtime, THEN this ticket's own
    /// scratch build (`CARGO_TARGET_DIR=<ticket>/🎯️target-r1 cargo build -p semio-s-plugin-note
    /// --target wasm32-wasip2`, run once to produce this evidence — see `📓️terra-R1-report.md`).
    /// SKIPS rather than fails if none exist (a fresh checkout with neither built yet) — it exists
    /// to demonstrate the wired path against real bytes when they are present, not to assert any
    /// particular build artifact's existence.
    #[test]
    fn note_plugin_manifest_loads_from_its_committed_descriptor() {
        let repo_root = test_repo_root();
        let descriptor_path = repo_root.join("✏️s/🔌️plugins/🗒️note/🛂️.descriptor.semio");
        assert!(descriptor_path.is_file(), "committed note descriptor missing at {}", descriptor_path.display());

        let candidate_wasm_paths = [
            repo_root.join("target/wasm32-wasip2/debug/semio_s_plugin_note.wasm"),
            repo_root.join("target/wasm32-wasip2/wasm-release/semio_s_plugin_note.wasm"),
            repo_root.join(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target-r1/wasm32-wasip2/debug/semio_s_plugin_note.wasm"),
        ];
        let Some(wasm_path) = candidate_wasm_paths.into_iter().find(|path| path.is_file()) else {
            eprintln!("[DEBUG] note_plugin_manifest_loads_from_its_committed_descriptor: SKIPPED — no compiled note wasm in any candidate location");
            return;
        };

        let mut plugin_paths = HashMap::new();
        plugin_paths.insert("note".to_string(), wasm_path);
        let mut descriptor_paths = HashMap::new();
        descriptor_paths.insert("note".to_string(), descriptor_path);
        // 🚫️async: this `#[test] fn` body is a sanctioned executor entry point (R4 clause 5) — the
        // one thread root driving `WasmtimeNodeHost::new`/`manifest_for`, both `async fn` now that
        // they build/drive the real `NativeKernelRuntime` (packet `run-kernel-wiring`). Same
        // `semio_framework_async::block_on` convention every other test in this module already uses
        // (see this crate's own `Cargo.toml` doc comment on why: a plain single-poll executor, not
        // tokio).
        let mut host = semio_framework_async::block_on(WasmtimeNodeHost::new(plugin_paths, descriptor_paths, Arc::new(InMemoryBlobStore::default())));

        let manifest = semio_framework_async::block_on(host.manifest_for("note")).expect("note must load natively from its committed descriptor, zero live describe() calls");
        assert_eq!(manifest.plugin_id, "note");
        assert!(!manifest.apps.is_empty(), "note's real manifest declares at least one app");
        assert!(manifest.dependencies.is_empty(), "note's committed descriptor declares zero PluginManifest.dependencies");

        let (routed_plugins, _routes) = host.io_router_stats();
        assert_eq!(routed_plugins, 1, "note must be the one plugin registered with the io router after this load");
        assert!(host.plugin_graph().is_registered("note").unwrap_or(false), "note must be registered in the plugin graph");
        assert!(host.app_router().owned_surface_gaps().is_empty(), "note's own panels leave no viewer/editor surface gap");
    }
    //#endregion 🔖️NativeManifestSmoke
}
//#endregion 🔖️Tests
