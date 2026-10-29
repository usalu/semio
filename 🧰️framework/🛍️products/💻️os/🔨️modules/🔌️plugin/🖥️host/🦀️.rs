//! 🛡️ Sandboxed wasmtime component plugin host with capability-gated imports.

#[cfg(not(target_arch = "wasm32"))]
#[path = "🎠️activation/🦀️.rs"]
pub mod activation;
#[cfg(not(target_arch = "wasm32"))]
#[path = "🎁️package-assets/🦀️.rs"]
pub mod package_assets;

use semio_framework_plugin_guest_contract::PluginHostError;
use semio_framework::FaultFrom;
pub use semio_framework_actor::{PackageHash, PackageId};
use semio_framework_value_derive::FromValue;
use std::collections::HashMap;

// 🗑️ `PLUGIN_FUEL_BUDGET` is gone — `design-runtime.md` §1's `Budget.fuel`/`⚖️LaneDefaults` (per-lane,
// per-turn, threaded through every `GuestRuntime::execute_turn`/`step_job` call) replaces the single
// process-wide constant this used to be.

//#region 📈️RuntimeMetricsPublisher
/// 📈️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (T1): native-side sampling + 2Hz cadence gate for bus
/// topic `os.runtime.metrics` — `semio_framework_actor::KernelMetrics`'s own doc comment: "the host
/// publishes this as bus topic `os.runtime.metrics` at 2Hz." Wraps `Kernel::runtime_metrics_snapshot`
/// (pure, clock-injected) and overlays the one field the pure crate cannot compute itself:
/// `ShardMetricsSample.metrics.heartbeat_age_ms`, from each shard's own
/// `ShardTransport::heartbeat()` reading (`🎭️actor/🦀️.rs`'s `ShardTransport` trait — this
/// file owns the transports, the pure crate does not).
///
/// 🚧️ GAP (see `📓️terra-T1-report.md` `## honest gaps`): nothing in this codebase yet drives a live
/// `semio_framework_actor::Kernel` on a native thread — `Kernel::new` has zero call sites outside
/// that crate's own tests and its wasm-only `KernelHost` glue (`grep -rn "Kernel::new(" --include
/// "*.rs" .`, checked at packet start). This publisher is therefore wired as the exact call the
/// future kernel-thread owner (H1-H4) makes each pump; it is unit-tested end-to-end against a real
/// `Kernel`, but nothing currently invokes it at runtime, and no subscriber-fanout for
/// `Effect::PublishEvent`/`Effect::Subscribe` exists anywhere in this file to hand the payload to
/// (same `grep` found zero delivery call sites, native or web).
pub struct RuntimeMetricsPublisher {
    last_published_ms: Option<u64>,
}

impl RuntimeMetricsPublisher {
    /// 🚫️async: E1 pure constructor consumed by `impl Default` (external trait, sync-only) — R9.
    pub fn new() -> Self {
        Self { last_published_ms: None }
    }

    /// 📡️ Samples `kernel` and returns the pack-encoded `os.runtime.metrics` payload
    /// (`semio_framework_actor::RuntimeMetricsSnapshot::pack_encode`) when the 2Hz interval elapsed,
    /// else `None`. `shard_heartbeats` maps each live `ShardId` to its transport's last
    /// `ShardTransport::heartbeat()` reading — the overlay `Kernel::shard_metrics_samples` cannot do
    /// itself (see this struct's doc comment).
    ///
    /// 🚫️async: R10 residue shape 2 — a future is consumed by a single `.await`; awaited once
    /// here instead of once inside the loop and again (moved) at `pack_encode` below.
    pub async fn maybe_sample(&mut self, kernel: &semio_framework_actor::Kernel, now_ms: u64, shard_heartbeats: &HashMap<semio_framework_actor::ShardId, u64>) -> Option<Vec<u8>> {
        if !semio_framework_actor::runtime_metrics_due(self.last_published_ms, now_ms).await {
            return None;
        }
        self.last_published_ms = Some(now_ms);
        let mut snapshot = kernel.runtime_metrics_snapshot(now_ms).await;
        for shard in &mut snapshot.shards {
            if let Some(&last_beat) = shard_heartbeats.get(&shard.shard) {
                shard.metrics.heartbeat_age_ms = now_ms.saturating_sub(last_beat) as u32;
            }
        }
        let mut out = Vec::new();
        snapshot.pack_encode(&mut out).await;
        Some(out)
    }
}

impl Default for RuntimeMetricsPublisher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️runtime-metrics-publisher/🦀️.rs"]
mod runtime_metrics_publisher_tests;
//#endregion 📈️RuntimeMetricsPublisher

//#region 🔖️ArtifactSession
/// 📦️ Opaque pack triple for one artifact lane (document / config / draft). Typed `ArtifactStore`
/// still lives in guest `VcsArtifactApp` until `AppCommand::PureCommand` + host `dispatch` land;
/// the host already mirrors these bytes as the authority seam.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionLanePack {
    pub pack: Vec<u8>,
    pub spr: Vec<u8>,
    pub ops: String,
    /// 🧾 Binary op packs from guest `AppFrame::Emit` awaiting host typed apply.
    pub pending_binary_ops: Vec<u8>,
}

impl SessionLanePack {
    /// 🏗️ Empty lane (no snapshot yet).
    pub async fn empty() -> Self {
        Self::default()
    }

    /// 🏗️ From a full pack+spr+ops snapshot.
    pub async fn from_files(pack: Vec<u8>, spr: Vec<u8>, ops: String) -> Self {
        Self { pack, spr, ops, pending_binary_ops: Vec::new() }
    }

    /// 📥 Replaces this lane's opaque snapshot.
    pub async fn adopt(&mut self, pack: Vec<u8>, spr: Vec<u8>, ops: String) {
        self.pack = pack;
        self.spr = spr;
        self.ops = ops;
        self.pending_binary_ops.clear();
    }

    /// 🧾 Applies guest `AppFrame::Emit` op bytes onto this lane via `ArtifactCodec` when `schema` is set.
    pub async fn apply_emit_ops(&mut self, schema: Option<&str>, ops: Vec<u8>, identity:&mut semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) {
        if ops.is_empty() {
            return;
        }
        let Some(schema) = schema.filter(|s| !s.is_empty()) else {
            self.pending_binary_ops = ops;
            return;
        };
        let Ok(Some(codec)) = store::os_store::codec_registry::document_codec(schema).await else {
            self.pending_binary_ops = ops;
            return;
        };
        if self.pack.is_empty() && self.spr.is_empty() {
            self.pending_binary_ops = ops;
            return;
        }
        let Ok(reference) = codec.reference() else {
            self.pending_binary_ops = ops;
            return;
        };
        match (reference.apply_ops_binary)(&self.pack, &self.spr, &ops,identity).await {
            Ok((pack, spr, ops_text)) => {
                self.pack = pack;
                self.spr = spr;
                self.ops = ops_text;
                self.pending_binary_ops.clear();
            }
            Err(_error) => {
                self.pending_binary_ops = ops;
            }
        }
    }

    /// 📭 True when no pack bytes have been adopted.
    pub async fn is_empty(&self) -> bool {
        self.pack.is_empty() && self.spr.is_empty()
    }
}

/// 🧾 Host-owned per-instance document authority: opaque document/config/draft packs plus generation
/// counters. The plugin-wide {@link EngineCache} lives on `HostState` (WIT `engine-derive`/`engine-read`
/// have no instance id). Typed stores and the command log remain guest-side until PureCommand apply.
#[derive(Clone, Debug, Default)]
pub struct ArtifactSession {
    pub generation: u64,
    pub command_log_len: u64,
    pub artifact_schema: Option<String>,
    pub config_schema: Option<String>,
    pub draft_schema: Option<String>,
    pub document: SessionLanePack,
    pub config: SessionLanePack,
    pub draft: SessionLanePack,
    /// ⚖️ This session's own view of its local/authority `MergePolicy`, mirrored from the last
    /// `AppCommand::SetMergePolicy` this session's `exchange` calls have sent (`Normal` — the wire
    /// default — until the first one). See `WasmPluginRuntime::hello`/`set_merge_policy`.
    pub merge_policy: protocol::MergePolicy,
    /// 📨 The most recent dispatch's `protocol::MutationMessage`s — mirrored from whichever of
    /// `AppFrame::Invocation.messages` (success) or `AppFrame::Error.report` (rejected) last carried
    /// a non-empty packed `protocol::DispatchReport`. Ticket 26/08/16/MUTATION-OUTCOMES-MERGE-
    /// POLICIES-AND-FIRST-CLASS-CONFLICTS (C9): mirrors `diagnostics`'s own DSL-diagnostics
    /// surfacing convention exactly (see `AppFrame::Emit`'s `diagnostics` field, `push_app_fault`'s
    /// `fault` field) rather than a second event/callback mechanism.
    pub last_dispatch_messages: Vec<protocol::MutationMessage>,
    /// 🔀 The most recent unsolicited `AppFrame::MergeReport`, if this session has seen one.
    pub last_merge_report: Option<protocol::MergeReport>,
    /// ⚔️ This artifact's currently open conflicts, mirrored from the most recent
    /// `AppFrame::Conflicts` (pushed unsolicited after every ingest, and the reply to
    /// `AppCommand::ReadConflicts`).
    pub open_conflicts: Vec<protocol::Conflict>,
}

impl ArtifactSession {
    /// 🏗️ Empty per-instance session (no packs yet).
    pub async fn new() -> Self {
        Self::default()
    }
}

//#endregion 🔖️ArtifactSession

//#region 🔖️MutationReports
/// 🧾 Decodes a packed `protocol::DispatchReport` off the wire — the shape `AppFrame::Invocation`'s
/// trailing `messages` and `AppFrame::Error`'s trailing `report` both carry (contract-freeze.md
/// §C8/C9). Empty bytes (a pre-CHANNEL_VERSION-11 peer, or a frame that legitimately carries none)
/// decode to a message-free report under this session's own tracked policy rather than erroring —
/// callers that need to distinguish "no report" from "empty report" should check `bytes.is_empty()`
/// themselves first.
pub async fn decode_dispatch_report(bytes: &[u8]) -> Result<protocol::DispatchReport, PluginHostError> {
    if bytes.is_empty() {
        return Ok(protocol::DispatchReport { policy: protocol::MergePolicy::default(), worst: None, messages: Vec::new() });
    }
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    semio_framework_value::FromValue::from_value(value).map_err(|error| PluginHostError::Plugin(error.to_string()))
}

/// 🔀 Decodes a packed `protocol::MergeReport` — `AppFrame::MergeReport.report`.
pub async fn decode_merge_report(bytes: &[u8]) -> Result<protocol::MergeReport, PluginHostError> {
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    semio_framework_value::FromValue::from_value(value).map_err(|error| PluginHostError::Plugin(error.to_string()))
}

/// ⚔️ Decodes a packed `Vec<protocol::Conflict>` — `AppFrame::Conflicts.conflicts`.
pub async fn decode_conflicts(bytes: &[u8]) -> Result<Vec<protocol::Conflict>, PluginHostError> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let value = store::pack_rt::decode_wire_value(bytes).map_err(|error| PluginHostError::Plugin(error.to_string()))?;
    semio_framework_value::FromValue::from_value(value).map_err(|error| PluginHostError::Plugin(error.to_string()))
}

//#endregion 🔖️MutationReports

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

